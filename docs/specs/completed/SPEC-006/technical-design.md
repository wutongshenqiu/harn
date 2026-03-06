# Technical Design: SDD Project Diagnostics and Upgrade (harn doctor)

| Field     | Value          |
|-----------|----------------|
| Spec ID   | SPEC-006       |
| Title     | SDD Project Diagnostics and Upgrade (harn doctor) |
| Author    | Claude         |
| Status    | Draft          |
| Created   | 2026-03-07     |
| Updated   | 2026-03-07     |

## Overview

新增 `harn doctor` CLI 命令，对已有 SDD 项目执行一系列健康检查，输出诊断报告，并可选自动修复。诊断引擎定义在 `crates/core` 中，检查项实现在 `crates/modules` 中，CLI 入口在 `crates/cli` 中。

参考 PRD: `docs/specs/active/SPEC-006/prd.md`

## Implementation

### Module Structure

```
crates/
  core/src/
    lib.rs              # 新增 pub mod doctor;
    doctor.rs           # 诊断引擎：Diagnostic, Severity, AutoFix, CheckResult, report 输出
  modules/src/
    lib.rs              # 新增 pub mod sdd_checks;
    sdd_checks.rs       # SDD 专属检查项（5 个检查函数）
  cli/src/
    main.rs             # 新增 Doctor subcommand + cmd_doctor()
```

### Key Types

```rust
// crates/core/src/doctor.rs

/// 诊断严重级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

/// 可自动修复的操作
#[derive(Debug, Clone)]
pub enum AutoFix {
    /// 向 _index.md 添加缺失的 spec 条目
    AddRegistryEntry { spec_id: String, title: String, status: String, location: String },
    /// 从 _index.md 移除不存在的 spec 条目
    RemoveRegistryEntry { spec_id: String },
    /// 修正 registry 条目的 status/location 字段
    UpdateRegistryEntry { spec_id: String, field: String, old_value: String, new_value: String },
    /// 创建缺失的目录
    CreateDirectory { path: String },
    /// 用内置模板覆盖 _templates/ 中的文件
    UpdateTemplate { path: String },
}

/// 单条诊断结果
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub check: String,       // 检查类别，如 "registry-consistency"
    pub message: String,     // 人类可读描述
    pub fix: Option<AutoFix>,
}

/// 一组检查的结果
#[derive(Debug, Default)]
pub struct CheckResult {
    pub diagnostics: Vec<Diagnostic>,
}

impl CheckResult {
    pub fn push(&mut self, d: Diagnostic) { ... }
    pub fn merge(&mut self, other: CheckResult) { ... }
    pub fn has_errors(&self) -> bool { ... }
    pub fn has_warnings(&self) -> bool { ... }
    /// 返回 exit code: 0=clean, 1=warnings, 2=errors
    pub fn exit_code(&self) -> i32 { ... }
}

/// 格式化输出诊断报告（带颜色）
pub fn print_report(result: &CheckResult) { ... }

/// 执行所有 AutoFix
pub fn apply_fixes(root: &Path, result: &CheckResult) -> anyhow::Result<Vec<String>> { ... }
```

### SDD 检查项

```rust
// crates/modules/src/sdd_checks.rs

use harn_core::doctor::{CheckResult, Diagnostic, Severity, AutoFix};
use std::path::Path;

/// 检查 1: Registry ↔ 文件系统一致性
///
/// - _index.md 中列出的 spec 在文件系统中是否存在
/// - 文件系统中的 spec 目录是否在 _index.md 中注册
/// - Active/Completed 状态与目录位置是否匹配
pub fn check_registry_consistency(root: &Path) -> CheckResult { ... }

/// 检查 2: Spec 文件格式完整性
///
/// - 每个 spec 目录是否包含 prd.md
/// - prd.md 是否包含必要 section: Problem Statement, Goals, User Stories
/// - technical-design.md（如存在）是否包含: Overview, Task Breakdown, Test Strategy
/// - metadata table 是否存在且包含 Spec ID, Title, Status 字段
pub fn check_spec_format(root: &Path) -> CheckResult { ... }

/// 检查 3: 模板漂移检测
///
/// - 比较 docs/specs/_templates/*.md 与 harn 内置模板的 hash
/// - 报告哪些模板已过时
pub fn check_template_drift(root: &Path) -> CheckResult { ... }

/// 检查 4: 配置一致性
///
/// - harn.toml 存在且可解析
/// - sdd module 已启用（如果 docs/specs/ 存在）
/// - reference=true 时 docs/reference/ 是否存在
/// - playbooks=true 时 docs/playbooks/ 是否存在
pub fn check_config_consistency(root: &Path) -> CheckResult { ... }

/// 检查 5: 目录结构完整性
///
/// - docs/specs/active/ 和 docs/specs/completed/ 是否存在
/// - docs/specs/_templates/ 是否存在且包含至少 prd.md
/// - docs/specs/_index.md 是否存在
pub fn check_directory_structure(root: &Path) -> CheckResult { ... }

/// 运行所有 SDD 检查
pub fn run_all_checks(root: &Path) -> CheckResult {
    let mut result = CheckResult::default();
    result.merge(check_directory_structure(root));
    result.merge(check_registry_consistency(root));
    result.merge(check_spec_format(root));
    result.merge(check_template_drift(root));
    result.merge(check_config_consistency(root));
    result
}
```

### CLI 集成

```rust
// crates/cli/src/main.rs — 新增部分

#[derive(Subcommand)]
enum Commands {
    // ... existing commands ...

    /// Diagnose SDD project health
    Doctor {
        /// Project directory
        #[arg(default_value = ".")]
        directory: PathBuf,

        /// Auto-fix safe issues
        #[arg(long)]
        fix: bool,
    },
}

fn cmd_doctor(directory: PathBuf, fix: bool) -> Result<()> {
    let root = resolve_root(directory)?;

    // 检查是否是 SDD 项目
    if !root.join("docs/specs").exists() {
        eprintln!("Error: No SDD structure found. Run `harn add sdd` first.");
        std::process::exit(1);
    }

    println!("Checking SDD project health...\n");

    let result = harn_modules::sdd_checks::run_all_checks(&root);

    harn_core::doctor::print_report(&result);

    if fix && result.has_fixable() {
        println!("\nApplying fixes...\n");
        let fixed = harn_core::doctor::apply_fixes(&root, &result)?;
        for f in &fixed {
            println!("  Fixed: {f}");
        }
        println!("\nRe-checking...\n");
        let recheck = harn_modules::sdd_checks::run_all_checks(&root);
        harn_core::doctor::print_report(&recheck);
        std::process::exit(recheck.exit_code());
    }

    std::process::exit(result.exit_code());
}
```

### Flow

1. 用户运行 `harn doctor [-d path] [--fix]`
2. CLI 解析参数，确认目标目录包含 SDD 结构
3. 依次执行 5 项检查，收集 `Diagnostic` 列表
4. 输出带颜色的诊断报告
5. 如果 `--fix`，执行所有 `AutoFix` 操作，然后重新检查并输出

### 预期输出示例

```
$ harn doctor

Checking SDD project health...

[Directory Structure]
  ok  docs/specs/_index.md exists
  ok  docs/specs/_templates/ exists (3 templates)
  ok  docs/specs/active/ exists
  ok  docs/specs/completed/ exists

[Registry Consistency]
  warn  SPEC-003 listed as Active but directory is in completed/
  err   SPEC-006 directory exists in active/ but has no registry entry
  ok    4/6 registry entries consistent

[Spec Format]
  warn  SPEC-002/prd.md missing section: "Success Metrics"
  ok    11/12 spec files pass format check

[Template Drift]
  warn  _templates/prd.md differs from harn built-in (hash mismatch)
  ok    _templates/technical-design.md matches built-in
  ok    _templates/research.md matches built-in

[Config Consistency]
  ok    harn.toml valid and SDD module enabled
  ok    reference docs present (reference=true)
  ok    playbooks present (playbooks=true)

Result: 1 error, 2 warnings
Run `harn doctor --fix` to auto-fix 2 issues.
```

## Configuration Changes

无新增配置项。`harn doctor` 读取现有 `harn.toml` 进行配置一致性检查。

## Alternative Approaches

| Approach | Pros | Cons | Verdict |
|----------|------|------|---------|
| 纯 slash command（AI agent 执行） | 零代码，灵活 | 不可重复、速度慢、依赖 AI | 否 — 诊断应该确定性执行 |
| 独立 `harn lint-specs` 命令 | 语义清晰 | 与 `harn doctor` 职责重叠 | 否 — `doctor` 更通用 |
| Module trait 新增 `diagnose()` 方法 | 每个模块可自带检查 | 改动大，当前只需 SDD | 未来 — Phase 2 可扩展 |

## Task Breakdown

- [ ] Task 1: 在 `crates/core/src/doctor.rs` 中定义 `Severity`, `AutoFix`, `Diagnostic`, `CheckResult` 类型和 `print_report()`, `apply_fixes()` 函数
- [ ] Task 2: 在 `crates/modules/src/sdd_checks.rs` 中实现 `check_directory_structure()`
- [ ] Task 3: 在 `crates/modules/src/sdd_checks.rs` 中实现 `check_registry_consistency()` — 解析 `_index.md` 表格，与文件系统对比
- [ ] Task 4: 在 `crates/modules/src/sdd_checks.rs` 中实现 `check_spec_format()` — 扫描 spec 文件的必要 section
- [ ] Task 5: 在 `crates/modules/src/sdd_checks.rs` 中实现 `check_template_drift()` — 比较文件 hash 与内置模板
- [ ] Task 6: 在 `crates/modules/src/sdd_checks.rs` 中实现 `check_config_consistency()`
- [ ] Task 7: 在 `crates/modules/src/sdd_checks.rs` 中实现 `run_all_checks()` 聚合函数
- [ ] Task 8: 在 `crates/cli/src/main.rs` 中新增 `Doctor` subcommand 和 `cmd_doctor()` 入口
- [ ] Task 9: 在 `crates/core/src/doctor.rs` 中实现 `apply_fixes()` — 执行 AutoFix 操作
- [ ] Task 10: 编写单元测试（registry 解析、format 检查、template hash 比较）
- [ ] Task 11: 编写集成测试（在 tempdir 中构造各种异常场景，验证诊断输出）
- [ ] Task 12: `make check` 通过（fmt + clippy + test）

## Test Strategy

- **Unit tests:**
  - `doctor.rs`: `CheckResult` 的 `exit_code()`, `has_errors()`, `has_warnings()` 方法
  - `sdd_checks.rs`: 每个 check 函数用 tempdir 构造正常/异常场景
    - registry 解析：正常表格、空表格、格式错误
    - format 检查：完整 spec、缺失 section 的 spec
    - template drift：匹配/不匹配的模板文件
    - config 一致性：有/无 harn.toml、配置与文件不匹配
- **Integration tests:**
  - 在 tempdir 中用 `harn add sdd` 初始化，然后手动制造问题，验证 `run_all_checks()` 输出
  - 验证 `apply_fixes()` 修复后重新检查通过
- **Manual verification:**
  - 在 harn 项目自身运行 `harn doctor`，确认输出合理
  - 手动制造 registry 不一致，验证检测和修复

## Revision Log

| Date | Section | Change | Reason |
|------|---------|--------|--------|
| 2026-03-07 | All | Initial draft | SPEC-006 creation |
