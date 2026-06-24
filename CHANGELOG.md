<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath)
-->

# Changelog

All notable changes to `formatrix-docs` will be documented in this file.

This file is generated from conventional commits by the
[`changelog-reusable.yml`](https://github.com/hyperpolymath/standards/blob/main/.github/workflows/changelog-reusable.yml)
workflow (`hyperpolymath/standards#206`). Adopt the workflow in this repo's CI to keep this file in sync automatically — see
[`templates/cliff.toml`](https://github.com/hyperpolymath/standards/blob/main/templates/cliff.toml)
for the canonical config.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/);
this project aims to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- feat(crg): add crg-grade and crg-badge justfile recipes
- feat: add Stapeln container configuration
- feat: add UX Justfile with doctor, tour, help-me, assail recipes
- feat: deploy UX Manifesto infrastructure
- feat: replace Tauri with Gossamer — gossamer-rs backend (mirror of docmatrix)
- feat: Gossamer migration — RuntimeBridge, gossamer.conf.json, Tauri→Gossamer conversion
- feat: add formatrix-bridges crate and missing AST module
- feat: add CLADE.a2ml — clade taxonomy declaration
- feat: add mirror.yml workflow for GitLab/Bitbucket mirroring
- feat: customize fuzz target with repo-specific logic

### Fixed

- fix(ci): bump a2ml/k9-validate-action pins to canonical (standards#85) (#10)
- fix(ci): sync hypatia-scan.yml to canonical (kill cd-scanner build drift) (#9)
- fix(ci): adopt canonical hypatia-scan.yml (env.HOME/scanner-layout + Comment-step gate) (#8)
- fix(ci): bump erlef/setup-beam SHA for ubuntu24 runner support (#5)
- fix(ci): move secret-scanner Cargo.toml gate from job-level if: to step-level (#6)
- fix(ci): Resolve workflow-linter self-matching and metadata issues
- fix: correct email jonathan.jewell → j.d.a.jewell
- fix: global AGPL-3.0-or-later → PMPL-1.0-or-later replacement
- fix(license): SPDX AGPL-3.0 → PMPL-1.0-or-later in dotfiles
- fix: remove duplicate SCM files from root

### Changed

- refactor: migrate 6SCM → 6A2 (.scm → .a2ml format)

### Documentation

- docs: add post-audit status report for M5 sweep
- docs: add TEST-NEEDS.md (CRG C)
- docs: add EXPLAINME.adoc — prove-it file backing README claims
- docs: add SECURITY.md for vulnerability reporting
- docs: add checkpoint files for state tracking
- docs: rename to DocMatrix
- docs: mark FD-M07 complete
- docs: add v1 ecosystem publish roadmap
- docs: mark FD-C07 Gist library as done
- docs: update seam check status after SHOULDs

### CI

- ci(rust): convert rust-ci.yml to thin wrapper (standards#174) (#14)
- ci: redistribute concurrency-cancel guard to read-only check workflows (#12)
- ci: bump actions/upload-artifact SHA to current v4 (#4)
- ci: SHA-pin hyperpolymath validate-actions in dogfood-gate
- ci: deploy dogfood-gate, fix hypatia-scan, add pre-commit hooks

## Pre-history

Prior commits to this file's introduction are recorded in git history but not formally classified into Keep-a-Changelog sections. To backfill, run `git cliff -o CHANGELOG.md` locally using the canonical [`cliff.toml`](https://github.com/hyperpolymath/standards/blob/main/templates/cliff.toml) — this is one-shot mechanical work.

---

<!-- This file was seeded by the 2026-05-26 estate tech-debt audit follow-up (Row-2 Phase 3); see [`hyperpolymath/standards/docs/audits/2026-05-26-estate-documentation-debt.md`](https://github.com/hyperpolymath/standards/blob/main/docs/audits/2026-05-26-estate-documentation-debt.md). -->
