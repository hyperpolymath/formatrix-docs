// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
//
//! Formatrix Bridges — Knowledge tool interop layer
//!
//! Provides bidirectional synchronisation between Formatrix's unified AST and
//! external knowledge management tools (Trilium, Obsidian, Joplin, Logseq, etc.).
//!
//! Each bridge implements the [`KnowledgeStore`] trait, enabling:
//! - Reading notes from any tool into Formatrix's format-agnostic AST
//! - Writing AST documents back to any tool in its native format
//! - Syncing changes bidirectionally (pull/push)
//! - Format-transparent round-trips (e.g. write in djot, tool sees markdown)
//!
//! # Architecture
//!
//! ```text
//! ┌──────────────┐     ┌────────────────┐     ┌──────────────┐
//! │ Trilium      │     │   Formatrix    │     │ Obsidian     │
//! │ (SQLite+REST)│◄───►│  Unified AST   │◄───►│ (filesystem) │
//! └──────────────┘     │  + FormatReg   │     └──────────────┘
//!                      │                │
//! ┌──────────────┐     │                │     ┌──────────────┐
//! │ Joplin       │◄───►│                │◄───►│ Logseq       │
//! │ (REST API)   │     └────────────────┘     │ (filesystem) │
//! └──────────────┘                            └──────────────┘
//! ```

#![forbid(unsafe_code)]

pub mod store;

#[cfg(feature = "trilium")]
pub mod bridges;

#[cfg(feature = "filesystem")]
pub mod fs_bridge;

pub use store::{
    ChangeKind, ChangeSet, KnowledgeStore, NoteFormat, NoteRef, StoreError, StoreResult,
};
