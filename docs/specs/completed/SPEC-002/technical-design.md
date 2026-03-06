# Technical Design: Complete TypeScript Quality Templates

| Field     | Value          |
|-----------|----------------|
| Spec ID   | SPEC-002       |
| Title     | Complete TypeScript Quality Templates |
| Author    | Claude          |
| Status    | Active         |
| Created   | 2026-03-06     |
| Updated   | 2026-03-06     |

## Overview

TypeScript has build templates and gitignore support, but the quality module references `quality/eslint.config.js` and `quality/prettierrc` templates that don't exist yet. This spec creates those missing template files.

## Implementation

### Files to Create

- `templates/quality/eslint.config.js` — ESLint flat config (v9+) with TypeScript support
- `templates/quality/prettierrc` — Prettier config (JSON format)

### No Code Changes Required

The `quality.rs` module already has the TypeScript match arm (lines 63-75) that looks for these templates. We just need to create the template files.

## Task Breakdown

- [ ] Create `templates/quality/eslint.config.js` — ESLint flat config with `@typescript-eslint`
- [ ] Create `templates/quality/prettierrc` — Prettier config
- [ ] Run `make check` to verify

## Test Strategy

- **Unit tests:** `make test` — ensure no regressions
- **Manual verification:** Run `cargo run -- init` with `languages = ["typescript"]` and verify eslint.config.js and .prettierrc are generated
