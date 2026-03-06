# PRD: SDD Project Diagnostics and Upgrade (harn doctor)

| Field     | Value          |
|-----------|----------------|
| Spec ID   | SPEC-006       |
| Title     | SDD Project Diagnostics and Upgrade (harn doctor) |
| Author    | Claude         |
| Status    | Draft          |
| Created   | 2026-03-07     |
| Updated   | 2026-03-07     |

## Problem Statement

harn 目前只支持**正向生成**：`harn init` / `harn add sdd` 初始化 SDD 结构，`harn spec` 创建新 spec。但对于**已有 SDD 项目**，没有任何工具来：

1. **诊断健康状态** — registry 与文件系统不一致、spec 格式缺失字段、孤儿 spec 等
2. **检测模板漂移** — 项目中的 `_templates/` 与当前 harn 版本的模板不同步
3. **自动修复/升级** — 将旧版 SDD 结构迁移到新版模板格式

这意味着随着 harn 版本迭代，已有项目的 SDD 结构会逐渐"腐化"，用户无法知道哪里出了问题，也无法安全地升级。

## Goals

- 提供 `harn doctor` 命令，一键诊断已有 SDD 项目的健康状态
- 检测 registry (`_index.md`) 与文件系统的一致性问题
- 验证 spec 文件格式完整性（必要 section 是否存在）
- 检测项目模板与当前 harn 内置模板的差异（模板漂移）
- 检测 `harn.toml` 配置与实际文件结构的一致性
- 提供 `--fix` 模式，自动修复可安全修复的问题
- 提供清晰的诊断报告，区分 Error / Warning / Info

## Non-Goals

- 不修改已写好的 spec 内容（PRD/TD 的具体文本是用户创作）
- 不做跨模块诊断（CI、agent 等模块的诊断留给后续迭代）
- 不做 spec 内容质量评估（如"PRD 写得好不好"）
- 不做 git history 分析（如"spec 停滞多久了"）

## User Stories

- As a developer using harn, I want to run `harn doctor` to see if my SDD project has any structural issues so that I can fix them before they cause problems.
- As a developer upgrading harn, I want to know which templates have changed so that I can decide whether to update my project templates.
- As a developer, I want `harn doctor --fix` to automatically fix safe issues (registry sync, missing dirs) so that I don't have to do it manually.
- As a developer, I want clear error messages with specific file paths and suggested fixes so that I know exactly what to do.

## Success Metrics

- `harn doctor` 能在 <1s 内完成对典型项目（~20 specs）的全量诊断
- 所有检查项有明确的 pass/warn/error 输出
- `--fix` 模式不破坏任何用户数据（spec 内容、自定义配置）
- 诊断结果的 exit code 反映健康状态（0=healthy, 1=warnings, 2=errors）

## Constraints

- 必须兼容现有 SDD 结构，不引入 breaking changes
- 诊断逻辑放在 `crates/core` 中（可被其他模块复用），CLI 集成在 `crates/cli`
- 模板漂移检测需要访问编译时嵌入的模板（通过 `harn_templates` crate）
- `--fix` 操作必须是幂等的

## Open Questions

- [ ] 是否需要 `--json` 输出格式支持 CI 集成？（建议 Phase 2）
- [ ] 是否需要 `harn upgrade` 作为独立命令，还是 `harn doctor --fix` 足够？（建议先用 `--fix`，后续按需拆分）
- [ ] 模板版本化方案：用嵌入注释 `<!-- harn-template-version: N -->` 还是用文件 hash？（建议用 hash，无侵入）

## Design Decisions

| Decision | Options Considered | Chosen | Rationale |
|----------|--------------------|--------|-----------|
| 诊断引擎位置 | core crate vs modules crate | core crate | 诊断是通用能力，未来其他模块也需要 |
| 模板漂移检测 | 版本注释 vs 文件 hash | 文件 hash | 无侵入，不需要修改现有模板格式 |
| 修复策略 | 全自动 vs 交互确认 | `--fix` 全自动 + dry-run 预览 | 符合 CLI 工具惯例，简单直接 |
| 输出格式 | plain text vs structured | plain text (colored) | 与现有 harn 输出风格一致，Phase 2 加 `--json` |
