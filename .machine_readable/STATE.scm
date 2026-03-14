;; SPDX-License-Identifier: PMPL-1.0-or-later
;; STATE.scm - Project state for formatrix-docs
;; Media-Type: application/vnd.state+scm

(state
  (metadata
    (version "0.1.0")
    (schema-version "1.0")
    (created "2025-01-02")
    (updated "2026-03-14")
    (project "formatrix-docs")
    (repo "github.com/hyperpolymath/formatrix-docs"))

  (project-context
    (name "formatrix-docs")
    (tagline "Cross-platform document editor with format tabs")
    (tech-stack
      ("Rust" "core library" "Unified AST, format parsers, renderers")
      ("Tauri 2.0" "GUI" "Desktop application framework")
      ("ReScript" "frontend" "Type-safe UI components")
      ("Ada" "TUI" "Terminal interface with AdaCurses")
      ("Nickel" "pipelines" "Import/export transformations")))

  (current-position
    (phase "initial-implementation")
    (overall-completion 15)
    (components
      (core 20 "AST, parsers scaffolded")
      (gui 10 "Tauri commands stubbed")
      (tui 5 "Ada structure created")
      (pipelines 5 "Nickel definitions started"))
    (working-features
      "Project structure"
      "Workspace Cargo.toml"
      "Core crate scaffolded"))

  (route-to-mvp
    (milestones
      ("M1" "Core format parsers"
        ("Plaintext parser" in-progress)
        ("Markdown parser" pending)
        ("AsciiDoc parser" pending))
      ("M2" "GUI shell"
        ("Tauri window" pending)
        ("Tab switching" pending))
      ("M3" "Format conversion"
        ("AST round-trip" pending))))

  (blockers-and-issues
    (critical)
    (high)
    (medium)
    (low))

  (critical-next-actions
    (immediate
      "Implement core AST types"
      "Complete plaintext parser")
    (this-week)
    (this-month))

  (session-history
    ("2025-01-02" "Project created")
    ("2026-03-14" "RSR compliance audit")))
