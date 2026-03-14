;; SPDX-License-Identifier: PMPL-1.0-or-later
;; META.scm - Meta-level information for formatrix-docs
;; Media-Type: application/meta+scheme

(meta
  (architecture-decisions
    ("ADR-001" "Unified AST approach"
      "Single AST shared across all format parsers/renderers for lossless conversion"
      "accepted" "2025-01-02")
    ("ADR-002" "Tauri 2.0 for GUI"
      "Tauri chosen over Electron for Rust backend and smaller binary size"
      "accepted" "2025-01-02")
    ("ADR-003" "Ada for TUI"
      "Ada with AdaCurses for safety-critical terminal interface"
      "accepted" "2025-01-02"))

  (development-practices
    (code-style
      ("Rust" "rustfmt, 100 char lines")
      ("ReScript" "rescript format")
      ("Ada" "gnatpp, 3-space indent"))
    (security
      (principle "Defense in depth")
      (no-secrets-in-code #t))
    (testing
      ("Rust" "cargo test")
      ("ReScript" "Jest"))
    (versioning "SemVer")
    (documentation "AsciiDoc")
    (branching "main for stable"))

  (design-rationale
    ("format-tabs" "View same document in multiple markup formats simultaneously")
    ("unified-ast" "Lossless round-trip conversion between formats")))
