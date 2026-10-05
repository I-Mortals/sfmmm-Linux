// 应用内网络代理设置：所有 Rust reqwest 请求统一走此代理（关闭时强制直连）。
//
// 背景：应用全部网络流量（更新检测、GitHub API、图床、订阅/BepInEx 下载等）
// 均由 Rust reqwest 发起。原先依赖 reqwest 的 system-proxy 自动检测（仅系统/
// 环境变量级），无法在应用内配置。本模块提供应用内 HTTP/HTTPS 代理：
// 配置存 SQLite config 表（proxy_enabled / proxy_url），运行期缓存在全局
// RwLock，各 client 构建处调用 `apply()` 注入。
//
// 语义：开启且地址非空 → 仅使用该代理（reqwest 设置显式代理时自动禁用系统代理）；
//      其余情况（未开启 / 地址为空）→ no_proxy() 强制直连，忽略系统与环境变量代理。
//
// 注意：仅作用于 Rust reqwest 请求。MySQL（mysql crate）与 WebView 的 fetch 不受影响。

use std::sync::{OnceLock, RwLock};

use crate::db::ApiResponse;

/// 代理配置：enabled 为开关，url 为代理地址（如 http://127.0.0.1:7890）。
#[derive(Debug, Clone, Default)]
struct ProxyState {
    enabled: bool,
    url: String,
}

static PROXY: OnceLock<RwLock<ProxyState>> = OnceLock::new();

fn state() -> &'static RwLock<ProxyState> {
    PROXY.get_or_init(|| RwLock::new(ProxyState::default()))
}

/// 更新运行期代理配置。
pub fn set(enabled: bool, url: String) {
    match state().write() {
        Ok(mut guard) => {
            guard.enabled = enabled;
            guard.url = url;
        }
        Err(poisoned) => {
            let mut guard = poisoned.into_inner();
            guard.enabled = enabled;
            guard.url = url;
        }
    }
}

/// 为 reqwest client builder 注入代理策略。
///
/// 开启且地址非空 → 使用该代理（reqwest 设置显式代理时自动禁用系统代理）；
/// 否则 → no_proxy() 强制直连。
pub fn apply(builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
    let (enabled, url) = {
        let guard = match state().read() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };
        (guard.enabled, guard.url.clone())
    };
    if enabled && !url.trim().is_empty() {
        match reqwest::Proxy::all(url.trim()) {
            Ok(proxy) => return builder.proxy(proxy),
            Err(e) => {
                log::error!("[Proxy] 代理地址无效，回退直连: {e}; url={url}");
            }
        }
    }
    builder.no_proxy()
}

/// 校验代理地址是否可解析为 reqwest 代理。
fn validate(url: &str) -> Result<(), String> {
    reqwest::Proxy::all(url.trim())
        .map(|_| ())
        .map_err(|e| format!("代理地址无效: {e}"))
}

/// 启动时从 SQLite config 表加载代理配置（缺失则默认关闭 / 直连）。
pub fn init_from_config(app: &tauri::AppHandle) {
    match load_from_config(app) {
        Ok((enabled, url)) => {
            if enabled && !url.trim().is_empty() {
                log::info!("[Proxy] 已启用应用内代理: {url}");
            }
            set(enabled, url);
        }
        Err(e) => {
            log::warn!("[Proxy] 读取代理配置失败（默认直连）: {e}");
        }
    }
}

fn load_from_config(app: &tauri::AppHandle) -> Result<(bool, String), String> {
    let conn = crate::db::subscribe::open_sqlite(app)?;
    let get = |key: &str| -> Option<String> {
        conn.query_row(
            "SELECT value FROM config WHERE `key` = ?1",
            rusqlite::params![key],
            |r| r.get::<_, String>(0),
        )
        .ok()
    };
    let enabled = get("proxy_enabled").map(|v| v == "true").unwrap_or(false);
    let url = get("proxy_url").unwrap_or_default();
    Ok((enabled, url))
}

/// 保存并应用代理设置（前端设置页调用）。
#[tauri::command]
pub async fn db_set_proxy(
    app: tauri::AppHandle,
    enabled: bool,
    url: String,
) -> Result<ApiResponse, String> {
    let url = url.trim().to_string();
    if enabled && url.is_empty() {
        return Err("启用代理时地址不能为空".into());
    }
    if enabled {
        validate(&url)?;
    }

    // 持久化到 SQLite config 表（与前端 setConfig 相同的 INSERT OR REPLACE 语义）
    {
        let conn = crate::db::subscribe::open_sqlite(&app)?;
        let write = |key: &str, value: &str| -> Result<(), String> {
            conn.execute(
                "INSERT OR REPLACE INTO config (id, `key`, value) \
                 VALUES ((SELECT id FROM config WHERE `key` = ?1), ?1, ?2)",
                rusqlite::params![key, value],
            )
            .map(|_| ())
            .map_err(|e| format!("写入配置失败: {e}"))
        };
        write("proxy_enabled", if enabled { "true" } else { "false" })?;
        write("proxy_url", &url)?;
    }

    set(enabled, url);
    Ok(ApiResponse::ok_msg(if enabled { "代理已启用" } else { "已切换为直连" }))
}
