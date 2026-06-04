<!--
SPDX-License-Identifier: MPL-2.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Post-audit Status Report: formatrix-docs
- **Date:** 2026-04-15
- **Status:** Complete (M5 Sweep)
- **Repo:** /var/mnt/eclipse/repos/formatrix-docs

## Actions Taken
1. Standard CI/Workflow Sweep: Added blocker workflows (`ts-blocker.yml`, `npm-bun-blocker.yml`) and updated `Justfile`.
2. SCM-to-A2ML Migration: Staged and committed deletions of legacy `.scm` files.
3. Lockfile Sweep: Generated and tracked missing lockfiles where manifests were present.
4. Static Analysis: Verified with `panic-attack assail`.

## Findings Summary
- 11 unwrap/expect calls in crates/formatrix-core/src/file_ops.rs
- flake.nix declares inputs without narHash, rev pinning, or sibling flake.lock — dependency revision is unpinned in nix/flake.nix
- 14 TODO/FIXME/HACK markers in contractiles/self-validating/template-hunt.k9.ncl
- flake.nix declares inputs without narHash, rev pinning, or sibling flake.lock — dependency revision is unpinned in flake.nix

## Final Grade
- **CRG Grade:** D (Promoted from E/X) - CI and lockfiles are in place.
