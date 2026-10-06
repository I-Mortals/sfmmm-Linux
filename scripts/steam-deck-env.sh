#!/bin/sh
# SteamOS / Steam Deck 启动包装器。
#
# 该脚本在 AppImage 内部作为 AppRun 运行（由 CI 在打包后替换，原始二进制
# 被重命名为 AppRun.bin）。仅当运行环境是 SteamOS 时，才把宿主机的
# Wayland 客户端与 libepoxy 预加载进来，随后把控制权交还给原始的
# AppRun 二进制。
#
# 之所以放在 AppRun 层（而不是 Rust 的 main()）是因为 Tauri 构建出的
# AppImage 使用预编译的 AppRun 二进制，main() 执行时 Wayland 连接早已
# 初始化，设置 LD_PRELOAD 为时已晚；且需要 preload 的是宿主机的库。

set -eu

APPDIR="${APPDIR:-$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)}"
export APPDIR

# SteamOS 检测（Steam Deck 的 /etc/os-release 中含 ID=steamos）
if [ -f /etc/os-release ] && grep -q '^ID=steamos$' /etc/os-release; then
    PRELOAD=""
    # 逐个检查文件是否存在，避免未来某个库缺失时启动直接失败
    for lib in /usr/lib/libwayland-client.so.0 /usr/lib/libepoxy.so.0; do
        if [ -f "$lib" ]; then
            PRELOAD="${PRELOAD:+$PRELOAD:}$lib"
        fi
    done
    if [ -n "$PRELOAD" ]; then
        LD_PRELOAD="$PRELOAD${LD_PRELOAD:+:$LD_PRELOAD}"
        export LD_PRELOAD
    fi
fi

exec "$APPDIR/AppRun.bin" "$@"
