# Plan: SDD Project Diagnostics and Upgrade (harn doctor)

## Specs
- [x] SPEC-006 — SDD Project Diagnostics and Upgrade (harn doctor)

## Progress Log
- 2026-03-07: SPEC-006 implemented. All 12 tasks complete.
  - Created `crates/core/src/doctor.rs` (diagnostic engine)
  - Created `crates/modules/src/sdd_checks.rs` (5 check functions)
  - Added `Doctor` subcommand to CLI
  - Added `get_embedded_content()` to TemplateEngine
  - 20 tests pass (5 core + 11 module + 4 existing)
  - `make check` passes (fmt + clippy + test)
