# Technical Design: Add Docker Templates for All Languages

| Field     | Value          |
|-----------|----------------|
| Spec ID   | SPEC-005       |
| Title     | Add Docker Templates for All Languages |
| Author    | Claude          |
| Status    | Active         |
| Created   | 2026-03-06     |
| Updated   | 2026-03-06     |

## Overview

The Docker module code (`docker.rs`) already handles language-specific Dockerfile selection, but no Docker templates exist yet. This spec creates Dockerfile templates for all supported languages plus a generic fallback and a docker-compose.yml template.

## Implementation

### Files to Create

All files under `templates/docker/`:

- `Dockerfile.rust` — Multi-stage build with `rust:slim` + `debian:slim` runtime
- `Dockerfile.go` — Multi-stage build with `golang:alpine` + `alpine` runtime (static binary)
- `Dockerfile.typescript` — Multi-stage build with `node:alpine` (build) + `node:alpine` (runtime)
- `Dockerfile.dart` — Multi-stage build with `dart:stable` + `scratch` runtime (AOT compiled)
- `Dockerfile.python` — `python:slim` with uv for dependency management
- `Dockerfile.java` — Multi-stage with `eclipse-temurin:21-jdk` + `eclipse-temurin:21-jre`  (shared with SPEC-003)
- `Dockerfile.cpp` — Multi-stage with `gcc:latest` + `debian:slim` runtime (shared with SPEC-004)
- `Dockerfile.generic` — Generic fallback (comment-based template)
- `docker-compose.yml` — Basic compose template with app service

### No Code Changes Required

The `docker.rs` module already handles:
- Language-specific Dockerfile lookup: `docker/Dockerfile.{primary_lang}`
- Fallback to `docker/Dockerfile.generic`
- docker-compose.yml rendering from `docker/docker-compose.yml`

All templates use minijinja variables: `{{ project_name }}`, `{{ primary_language }}`

## Task Breakdown

- [ ] Create `templates/docker/Dockerfile.rust`
- [ ] Create `templates/docker/Dockerfile.go`
- [ ] Create `templates/docker/Dockerfile.typescript`
- [ ] Create `templates/docker/Dockerfile.dart`
- [ ] Create `templates/docker/Dockerfile.python`
- [ ] Create `templates/docker/Dockerfile.generic`
- [ ] Create `templates/docker/docker-compose.yml`
- [ ] Run `make check` to verify

## Note

`Dockerfile.java` and `Dockerfile.cpp` are created by SPEC-003 and SPEC-004 respectively. If those specs haven't been implemented yet when this spec runs, create them here.

## Test Strategy

- **Unit tests:** `make test` — ensure no regressions
- **Manual verification:** Run `cargo run -- init` with docker enabled for each language and verify Dockerfile is generated
