# DevDesk 项目约定

- 当前目录是 Tauri 2 + Vue 3 + TypeScript + Rust 桌面应用；Git `origin` 指向 `https://github.com/AhaSir-long/DevDesk.git`。
- 已确认的业务术语见 `CONTEXT.md`。临时需求文档和访谈中的建议均不自动视为最终需求。
- 第一版技术路线为 Tauri 2 + Vue 3 + TypeScript + Rust；登记信息与操作历史由 Rust 的 SQLite 存储层处理。
- 前端测试使用 `npm run test`，前端构建使用 `npm run build`，Rust 测试使用 `cargo test --manifest-path src-tauri/Cargo.toml`，桌面调试使用 `npm run tauri dev`。
- 实现前先确认第一版范围和验收场景。
