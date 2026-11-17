# DevDesk task1 浅色界面 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans and superpowers:test-driven-development. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 按 2026-11-17 设计稿把 task1 改为浅色表格总览、搜索和添加服务弹窗，同时保留登记持久化。

**Architecture:** `src/App.vue` 管理列表、搜索和弹窗开关；`src/components/InstanceForm.vue` 保留字段校验和提交，增加取消及后端错误展示。复用现有 `src/api.ts` 与 Rust 命令，不增加依赖或未完成业务入口。

**Tech Stack:** Vue 3、TypeScript、Vitest、Tauri 2。

**Spec:** [浅色界面设计](../specs/2026-11-17-devdesk-ui-design.md)

## Global Constraints

- 固定浅色主题，顶部仅 DevDesk 与添加服务；单页不显示左侧导航。
- 表格仅显示实例名称、Windows 服务名、预期端口；按名称和服务名搜索。
- 弹窗沿用现有必填和端口范围校验；成功关闭并刷新，失败保留输入并报错。
- 不加入扫描、状态、启停或详情等未实现入口。
- 按项目 Git 规则保留已有暂存内容；既有代码、测试和文档仅在用户明确授权后暂存，提交与推送分别取得授权。

## Review Focus

- 空列表与搜索无结果使用不同提示。
- 保存失败时弹窗和输入保持，避免重复登记。
- 保存进行中不能重复提交或关闭弹窗导致状态不明。
- 长名称、空端口与窄窗口仍可读。
- 浅色主题不受 Windows 深色模式影响。

## Task 1: 更新 task1 首页

**Files:** Modify `src/App.vue`, `src/components/InstanceForm.vue`, `src/components/InstanceForm.test.ts`.

**Interfaces:** 保持 `listInstances()`、`saveInstance(ServiceInstanceDraft)` 与表单 `save` 事件契约。

- [x] 先写界面测试：空列表、搜索、打开与取消弹窗、成功刷新并关闭、失败保留输入、原字段校验和重复提交；运行 `npm run test` 观察预期失败（旧界面缺少入口、表格和搜索）。
- [x] 实现浅色工具栏、表格、搜索和弹窗，保持 API 与 Rust 层不变；`npm run test` 10/10 通过。
- [x] 运行 `npm run build`、Rust 回归测试，并在桌面窗口核对浅色布局、搜索无结果、弹窗、取消和真实保存；新登记实例在重启后仍显示，随后按 ID 清理了本次测试记录。窄窗口实际调整未成功，CSS 提供横向滚动。
- [x] 检查 `git diff`、`git diff --cached` 与状态；实施当时 UI 修改留在未暂存区，原有七个暂存文件保持原样，提交与推送等待后续授权。
