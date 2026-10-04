// 跨平台路径工具。
//
// 背景：Rust 端一律用系统原生分隔符（std::path::PathBuf）。Windows 的对话框返回
// `\`，Linux/macOS 返回 `/`。前端若硬编码 `\` 拼接路径，在 AppImage/Linux 上会拼出
// `.../dir\SecretFlasherManaka.exe` 这种含字面反斜杠的非法路径，导致「文件明明存在
// 却提示未找到」。故所有拼接/归一化都必须跟随当前平台。
//
// 平台判断用 navigator.userAgent：Tauri 各平台 WebView 分别含 "Windows NT" /
// "Linux" / "Macintosh"，无需额外插件依赖。SSR/测试环境无 navigator 时退化为
// 非 Windows（POSIX），与 vitest(node) 默认环境一致。

export const IS_WINDOWS =
  typeof navigator !== 'undefined' && /Windows/i.test(navigator.userAgent || '')

/** 当前平台原生路径分隔符。 */
export const PATH_SEP = IS_WINDOWS ? '\\' : '/'

/** 去掉路径末尾的一个或多个分隔符（根路径除外）。 */
export function trimTrailingSep(p) {
  if (!p) return p ?? ''
  return p.replace(/[\\/]+$/, '')
}

/** 按平台分隔符拼接路径片段（跳过空片段）。 */
export function joinPath(...segments) {
  return segments
    .filter((s) => s !== undefined && s !== null && s !== '')
    .join(PATH_SEP)
}

/**
 * 把 base 与可能含 `/` 或 `\` 的相对路径 rel 拼接成平台原生路径。
 * rel 里的两种分隔符都会被拆开再用 PATH_SEP 重组，避免出现混合分隔符。
 */
export function joinRel(base, rel) {
  const segs = String(rel ?? '').split(/[\\/]+/).filter(Boolean)
  return joinPath(trimTrailingSep(base), ...segs)
}

/** 把路径中的分隔符统一成当前平台的原生分隔符。 */
export function toNativePath(p) {
  if (!p) return p ?? ''
  return IS_WINDOWS ? p.replace(/\//g, '\\') : p.replace(/\\/g, '/')
}
