# Playbook: Write PRD & TD (Anti-Forgetting)

## Overview

Complex PRD/TD writing involves large amounts of intermediate research (competitor analysis, tech stack evaluation, trade-off analysis). Long conversations cause context compression, losing earlier findings. This playbook solves this by splitting the work into 4 independent phases, each persisting results to disk.

**Key principle**: Each phase can run in a **separate conversation**. Later phases read prior phase outputs from files, not from conversation history.

## Prerequisites

- Spec directory exists: `docs/specs/active/SPEC-NNN/`
- Spec is registered in `docs/specs/_index.md`

## Phases

### Phase 1: Competitor Research

**Input**: Spec directory path, problem domain description
**Output**: `docs/specs/active/SPEC-NNN/research-competitors.md`

1. Read the spec directory to understand the problem domain
2. Identify 3-5 relevant competitors or prior art
3. For each competitor, research:
   - Core features and positioning
   - Technical approach and stack
   - Strengths and weaknesses
   - Relevance to our project
4. **Write findings to `research-competitors.md` incrementally** — do not wait until the end
5. Summarize with a comparison table and preliminary recommendations
6. Mark the research document status as "Complete"

**Completion criteria**: `research-competitors.md` exists with filled competitor table, detailed analysis per competitor, and summary recommendations.

### Phase 2: Technical Feasibility Analysis

**Input**: Spec directory path (reads `research-competitors.md` from Phase 1)
**Output**: `docs/specs/active/SPEC-NNN/research-tech.md`

1. **Read `research-competitors.md`** to understand the landscape
2. Read existing codebase structure (`Cargo.toml`, key modules) to understand current architecture
3. Identify 2-4 candidate technical approaches
4. For each approach, evaluate:
   - Implementation complexity
   - Compatibility with existing architecture
   - Performance implications
   - Maintenance burden
5. **Write findings to `research-tech.md` incrementally**
6. Produce a comparison table with recommendation
7. Mark the research document status as "Complete"

**Completion criteria**: `research-tech.md` exists with approach comparison table, detailed pros/cons per approach, and a clear recommended approach with justification.

### Phase 3: PRD Writing

**Input**: Spec directory path (reads both research docs from Phases 1-2)
**Output**: `docs/specs/active/SPEC-NNN/prd.md`

1. **Read `research-competitors.md` and `research-tech.md`**
2. Read any existing PRD template or draft in the spec directory
3. Write the PRD with:
   - Problem statement (informed by competitor gaps)
   - Goals and non-goals
   - User stories
   - Success metrics
   - Scope boundaries (informed by technical feasibility)
4. **Write to `prd.md` incrementally** — start with structure, then fill sections
5. Cross-reference research docs for evidence-based decisions

**Completion criteria**: `prd.md` exists with all sections filled, references to research findings, and clear scope.

### Phase 4: TD Writing

**Input**: Spec directory path (reads PRD + both research docs)
**Output**: `docs/specs/active/SPEC-NNN/technical-design.md`

1. **Read `prd.md`, `research-competitors.md`, and `research-tech.md`**
2. Read relevant source code for the areas being modified
3. Write the TD with:
   - Architecture overview
   - API design (informed by competitor patterns)
   - Implementation plan (based on recommended technical approach)
   - Task breakdown with dependencies
   - Testing strategy
   - Revision log
4. **Write to `technical-design.md` incrementally**
5. Ensure task breakdown aligns with PRD scope

**Completion criteria**: `technical-design.md` exists with all sections filled, task breakdown with dependencies, and alignment with PRD goals.

## Anti-Forgetting Rules

1. **Incremental writes**: Every phase writes to its output file as it progresses, not just at the end. If the conversation is interrupted, partial progress is preserved.
2. **File-based handoff**: Each phase reads only from files, never from prior conversation context. This means phases can safely run in separate conversations.
3. **Structured output**: Research docs use tables and structured formats so downstream phases can quickly extract key conclusions.
4. **Self-contained sections**: Each section in a research doc should be understandable without reading the full conversation that produced it.

## Quick Reference

```
Phase 1: research-competitors.md  (can run independently)
Phase 2: research-tech.md         (depends on Phase 1 output)
Phase 3: prd.md                   (depends on Phases 1-2 output)
Phase 4: technical-design.md      (depends on Phases 1-3 output)
```
