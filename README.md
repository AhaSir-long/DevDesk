# DevDesk

DevDesk 是个人使用的 Windows 本机服务与运行环境管理工具。第一版设计见 [设计稿](docs/superpowers/specs/2026-11-16-devdesk-v1-design.md)。当前 Task 1 提供 Windows 服务实例登记、SQLite 持久化和基础桌面界面；自动扫描与服务操作将在后续任务实现。

## 本地开发

需要 Node.js、Rust stable（MSVC 工具链）以及 Visual Studio C++ Build Tools 和 Windows SDK。

```powershell
npm install
npm run tauri dev
```

如果当前终端找不到 `cargo`，请重开终端，让 Rust 安装器更新的 `PATH` 生效。

## 验证

```powershell
npm run test
npm run build
cargo test --manifest-path src-tauri/Cargo.toml
```

登记数据保存在 `%APPDATA%\com.devdesk.app\devdesk.sqlite`。
