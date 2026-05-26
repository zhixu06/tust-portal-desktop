# TustPortal

天科大校园网自动登录桌面客户端，使用 [Tauri 2](https://v2.tauri.app/) + [Vue 3](https://vuejs.org/) 构建，驻留系统托盘，每隔 20 秒检测网络状态并自动登录校园网认证门户。

## 功能

- **自动登录** — 检测到需要登录时自动提交认证，自带重试机制（最多 3 次）
- **托盘驻留** — 最小化到系统托盘，右键菜单可触发登录、暂停/恢复、查看日志
- **手动登录** — 点击托盘图标打开设置窗口，随时手动触发登录
- **凭据保存** — 用户名和密码加密存储在本地应用数据目录
- **SSID 检测** — 自动识别 `TUST` 或 `CU_TUST` 开头的 WiFi + `10.x` 网段 IP
- **忽略 SSID** — 可关闭 SSID 检测，仅依据连通性判断是否需要登录
- **日志窗口** — 独立日志窗口，实时查看自动登录状态和错误
- **深色模式** — 自动跟随系统主题

## 使用

从 Release 中下载适合你设备的安装包进行安装。

### macOS 特殊说明

若在 macOS 上出现如 `应用已损坏` 的提示，请运行：
```bash
xattr -cr /Applications/TustPortal.app
```

## 开发

### 前置依赖

- [Rust](https://www.rust-lang.org/) (stable)
- [Node.js](https://nodejs.org/) >= 18
- [pnpm](https://pnpm.io/)
- macOS 10.15+

### 快速开始

```bash
# 安装前端依赖
pnpm install

# 启动开发模式（热更新）
pnpm tauri dev

# 构建生产版本
pnpm tauri build
```

### 技术栈

| 层 | 技术 |
|---|---|
| 桌面框架 | Tauri 2 |
| 前端 | Vue 3 + TypeScript + Vite |
| 后端 | Rust (Tokio, Reqwest) |
| 包管理 | pnpm + Cargo |

### 项目结构

```
src/                  # 前端 (Vue 3)
├── views/            # 页面组件 (设置页、日志页)
├── components/       # 通用组件
├── native/           # Tauri invoke 封装
└── router/           # 前端路由
src-tauri/src/        # 后端 (Rust)
├── main.rs           # 入口
├── lib.rs            # 应用初始化、状态管理
├── background.rs     # 后台定时自动登录
├── sign_in.rs        # 登录逻辑 & HTTP 请求
├── network_info.rs   # 网络状态检测
├── tray.rs           # 系统托盘菜单
├── log_system.rs     # 日志系统
├── js_bridge.rs      # Tauri 命令 bridge
├── store/            # 凭据 & 设置持久化
└── platform/         # 平台相关实现 (macOS)
```

### 登录流程

1. 每 20 秒检测当前 WiFi SSID 和 IP 地址，判断是否在天科大校园网环境
2. 检查外网连通性（`connectivitycheck.gstatic.com`），判断是否需要登录
3. 若需要登录且凭据已保存，自动向认证门户发送登录请求
4. 登录成功后验证百度连通性，确认网络可用

## License

[GPL 2](LICENSE)
