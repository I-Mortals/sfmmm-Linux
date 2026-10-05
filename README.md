# SFMMM

A Tauri 2 based game mod management desktop application with support for mod browsing, installation, updates, and workshop features.

一个基于 Tauri 2 的游戏模组管理桌面应用，支持模组浏览、安装、更新及创意工坊功能。

SFMMM ワークショップ Mod マネージャー - Tauri 2 ベースのゲーム Mod 管理デスクトップアプリケーションです。

## Steam Deck / Linux Notes / 使用说明 / 注意

### English

> Read this first if you run the app on a Steam Deck (SteamOS).

**Launching the AppImage.** On SteamOS the system Mesa/EGL stack may be incompatible with the `libwayland-client.so.0` / `libepoxy.so.0` bundled inside the AppImage. The AppImage now detects SteamOS on startup and preloads the host libraries automatically, so just run it directly — no wrapper script needed.

**Loading BepInEx in-game (Proton compatibility + DLL override).** BepInEx relies on `winhttp.dll`, which Proton does not load by default; without the override below your mods will not take effect. First, in Steam right-click the game → **Properties** → **Compatibility**, tick *Force the use of a specific Steam Play compatibility tool*, and choose **Proton 8.0** or **Proton 9.0** (Experimental builds may ignore the override). Then open **Launch Options** and enter:

```
WINEDLLOVERRIDES="winhttp=n,b" %command%
```

### 中文

> 在 Steam Deck（SteamOS）上运行本应用时，请先阅读本节。

**启动 AppImage。** SteamOS 系统自带的 Mesa/EGL 环境可能与 AppImage 内置的 `libwayland-client.so.0` / `libepoxy.so.0` 不兼容。AppImage 现在会在启动时自动检测 SteamOS 并预加载宿主机库，直接运行即可，无需额外脚本。

**让游戏加载 BepInEx（Proton 兼容层 + DLL 覆写）。** BepInEx 依赖 `winhttp.dll`，而 Proton 默认不会加载它；不做下面的覆写，Mod 不会生效。请先在 Steam 中右键游戏 → **属性** → **兼容性**，勾选「强制使用特定 Steam Play 兼容性工具」，并选择 **Proton 8.0** 或 **Proton 9.0**（实验版可能不识别该覆写）；然后切到 **启动选项**，填入：

```
WINEDLLOVERRIDES="winhttp=n,b" %command%
```

### 日本語

> Steam Deck（SteamOS）で本アプリを使う場合は、まずこの節をお読みください。

**AppImage の起動。** SteamOS 標準の Mesa/EGL 環境は、AppImage に同梱された `libwayland-client.so.0` / `libepoxy.so.0` と互換性がない場合があります。AppImage は起動時に SteamOS を検出し、ホスト側のライブラリを自動でプリロードするため、ラッパースクリプトは不要でそのまま実行できます。

**BepInEx を読み込ませる（Proton 互換レイヤー + DLL オーバーライド）。** BepInEx は `winhttp.dll` に依存していますが、Proton は既定では読み込みません。以下の設定を行わないと Mod は反映されません。まず Steam でゲームを右クリック → **プロパティ** → **互換性** を開き、「特定の Steam Play 互換ツールを強制する」にチェックを入れ、**Proton 8.0** または **Proton 9.0** を選択します（実験版はこのオーバーライドを認識しない場合があります）。次に **起動オプション** に切り替え、以下を入力：

```
WINEDLLOVERRIDES="winhttp=n,b" %command%
```

## Features / 功能特性 / 機能

- **Mod Management** - Scan and manage local game mods
- **Workshop** - Browse, upload, and download community mods
- **Custom Missions** - Support for v1/v2 custom mission folders
- **Game Settings** - Configure game path and launch parameters
- **Multi-language Support** - Chinese, English, Japanese

- **模组管理** - 扫描和管理本地游戏模组
- **创意工坊** - 浏览、上传、下载社区模组
- **自定义任务** - 支持 v1/v2 版本的自定义任务文件夹
- **游戏设置** - 配置游戏路径和启动参数
- **多语言支持** - 支持中文、英文、日文

- **Mod管理** - ローカルゲームModのスキャンと管理
- **ワークショップ** - コミュニティModの閲覧、アップロード、ダウンロード
- **カスタム任務** - v1/v2 カスタム任務フォルダのサポート
- **ゲーム設定** - ゲームパスと起動パラメータの設定
- **多言語サポート** - 中国語、英語、日本語

## Tech Stack / 技术栈 / 技術スタック

- **Frontend**: React 19 + Vite + Fluent UI
- **Framework**: Tauri 2 (cross-platform desktop app)
- **Backend**: Rust + MySQL
- **Database**: SQLite (local) + MySQL (remote)
- **Editor**: TipTap (rich text editor)
- **Internationalization**: i18next

- **前端**: React 19 + Vite + Fluent UI
- **框架**: Tauri 2 (跨平台桌面应用)
- **后端**: Rust + MySQL
- **数据库**: SQLite (本地) + MySQL (远程)
- **编辑器**: TipTap (富文本编辑器)
- **国际化**: i18next

- **フロントエンド**: React 19 + Vite + Fluent UI
- **フレームワーク**: Tauri 2 (クロスプラットフォームデスクトップアプリ)
- **バックエンド**: Rust + MySQL
- **データベース**: SQLite (ローカル) + MySQL (リモート)
- **エディタ**: TipTap (リッチテキストエディタ)
- **国際化**: i18next

## Quick Start / 快速开始 / クイックスタート

### Prerequisites / 前置要求 / 前提条件

- Node.js (>= 18)
- pnpm
- Rust (>= 1.75)

### Install Dependencies / 安装依赖 / 依存関係のインストール

```bash
pnpm install
```

### Development / 开发模式 / 開発モード

```bash
# Frontend only
pnpm dev

# Desktop app
pnpm tauri dev
```

```bash
# 仅运行前端
pnpm dev

# 运行桌面应用
pnpm tauri dev
```

```bash
# フロントエンドのみ
pnpm dev

# デスクトップアプリ
pnpm tauri dev
```

### Build / 构建 / ビルド

```bash
# Build frontend
pnpm build

# Build desktop app installer
pnpm tauri build
```

```bash
# 构建前端
pnpm build

# 构建桌面应用安装包
pnpm tauri build
```

```bash
# フロントエンドをビルド
pnpm build

# デスクトップアプリのインストーラーをビルド
pnpm tauri build
```

### Lint / 代码检查 / コードチェック

```bash
pnpm lint
```

## Project Structure / 项目结构 / プロジェクト構造

```
├── src/                    # Frontend source
│   ├── components/         # Common components
│   ├── contexts/           # React Context
│   ├── hooks/              # Custom Hooks
│   ├── i18n/               # Internationalization
│   ├── modules/            # Feature modules
│   ├── services/           # API services
│   ├── App.jsx             # App entry
│   └── main.jsx            # React entry
├── src-tauri/              # Tauri Rust source
│   ├── src/
│   │   ├── main.rs         # Tauri entry
│   │   ├── lib.rs          # Plugin registration
│   │   └── db.rs           # MySQL operations
│   └── tauri.conf.json     # Tauri config
├── public/                 # Static assets
└── package.json            # Frontend dependencies
```

```
├── src/                    # 前端源码
│   ├── components/         # 通用组件
│   ├── contexts/           # React Context
│   ├── hooks/              # 自定义 Hooks
│   ├── i18n/               # 国际化配置
│   ├── modules/            # 功能模块
│   ├── services/           # API 服务
│   ├── App.jsx             # 应用入口
│   └── main.jsx            # React 渲染入口
├── src-tauri/              # Tauri Rust 源码
│   ├── src/
│   │   ├── main.rs         # Tauri 入口
│   │   ├── lib.rs          # 插件注册和命令定义
│   │   └── db.rs           # MySQL 数据库操作
│   └── tauri.conf.json     # Tauri 配置
├── public/                 # 静态资源
└── package.json            # 前端依赖
```

```
├── src/                    # フロントエンドソース
│   ├── components/         # 共通コンポーネント
│   ├── contexts/           # React Context
│   ├── hooks/              # カスタム Hooks
│   ├── i18n/               # 国際化設定
│   ├── modules/            # 機能モジュール
│   ├── services/           # API サービス
│   ├── App.jsx             # アプリケーションエントリ
│   └── main.jsx            # React レンダリングエントリ
├── src-tauri/              # Tauri Rust ソース
│   ├── src/
│   │   ├── main.rs         # Tauri エントリ
│   │   ├── lib.rs          # プラグイン登録とコマンド定義
│   │   └── db.rs           # MySQL データベース操作
│   └── tauri.conf.json     # Tauri 設定
├── public/                 # 静的アセット
└── package.json            # フロントエンド依存関係
```

## Environment Variables / 环境变量 / 環境変数

Create `src-tauri/.env` file with the following variables:

创建 `src-tauri/.env` 文件并配置以下变量：

`src-tauri/.env` ファイルを作成し、以下の変数を設定します：

```env
DB_URL=mysql://user:password@host:port/database
```

See `src-tauri/.env.example` for reference.

参考 `src-tauri/.env.example`。

`src-tauri/.env.example` を参照してください。

## License / 许可证 / ライセンス

MIT


---
