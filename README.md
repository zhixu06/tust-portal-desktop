# TustPortal

天科大校园网自动登录桌面客户端。

## 环境搭建（Windows 11）

1. **Visual Studio Build Tools** — Rust 编译需要 MSVC 链接器
   下载 [Visual Studio Build Tools](https://visualstudio.microsoft.com/zh-hans/downloads/)，安装时勾选「使用 C++ 的桌面开发」工作负载。

2. **Rust** — VS Build Tools 装完后再装，rustup 会自动检测到 MSVC
   下载 [rustup-init.exe](https://rustup.rs)，保持默认选项安装。

3. **Node.js + pnpm**
   从 https://nodejs.org 安装 LTS 版本，然后在终端运行：
   ```bash
   npm install -g pnpm
   ```

4. **CLion**
   安装 CLion 后，在插件市场搜索并安装 **Rust** 插件（由 JetBrains 官方提供）。
   用 CLion 打开项目目录即可识别 Cargo 项目。

5. **安装依赖 & 运行**
   在 CLion 内置终端中执行：
   ```bash
   pnpm install
   pnpm tauri dev
   ```

## 开发

```bash
pnpm dev          # 仅启动 Vite 前端开发服务器
pnpm build        # 构建前端
pnpm tauri dev    # 启动 Tauri 桌面应用（含热重载）
pnpm tauri build  # 打包生产版本
```
