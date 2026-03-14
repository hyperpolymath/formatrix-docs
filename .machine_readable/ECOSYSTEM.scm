;; SPDX-License-Identifier: PMPL-1.0-or-later
;; ECOSYSTEM.scm - Ecosystem position for formatrix-docs
;; Media-Type: application/vnd.ecosystem+scm

(ecosystem
  (version "1.0")
  (name "formatrix-docs")
  (type "desktop-application")
  (purpose "Cross-platform document editor with format tabs and unified AST")

  (position-in-ecosystem
    (category "developer-tools")
    (subcategory "document-editing")
    (unique-value
      "Multi-format document editing with live conversion"
      "Unified AST for lossless format round-trips"
      "Tauri + Ada dual GUI/TUI interfaces"))

  (related-projects ())

  (what-this-is
    "A cross-platform document editor supporting multiple markup formats"
    "A format conversion tool with unified AST"
    "A Tauri 2.0 desktop application with ReScript frontend")

  (what-this-is-not
    "NOT a web-based editor"
    "NOT a simple text editor"
    "NOT a markup-specific tool"))
