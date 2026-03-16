// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
//
//! KnowledgeStore trait — the universal interface for knowledge tool bridges.
//!
//! Every bridge (Trilium, Obsidian, Joplin, etc.) implements this trait,
//! enabling Formatrix to read/write/sync notes regardless of the underlying
//! tool's storage format or API.

use formatrix_core::ast::{Document, SourceFormat};
use std::collections::HashMap;

/// Errors from knowledge store operations
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    /// The note was not found in the store
    #[error("Note not found: {id}")]
    NotFound { id: String },

    /// Authentication failed (e.g. invalid ETAPI token)
    #[error("Authentication failed: {message}")]
    AuthError { message: String },

    /// The store is not reachable (e.g. Trilium not running)
    #[error("Store unavailable: {message}")]
    Unavailable { message: String },

    /// Network or HTTP error
    #[error("Network error: {0}")]
    Network(String),

    /// Format conversion error
    #[error("Conversion error: {0}")]
    Conversion(#[from] formatrix_core::traits::ConversionError),

    /// IO error (filesystem bridges)
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

pub type StoreResult<T> = Result<T, StoreError>;

/// The native format a knowledge tool uses for its notes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteFormat {
    /// Tool stores notes as HTML (Trilium, Joplin)
    Html,
    /// Tool stores notes as Markdown (Obsidian, Logseq, Zettlr)
    Markdown,
    /// Tool stores notes as Org Mode (Logseq org-mode)
    OrgMode,
    /// Tool stores in a proprietary format
    Proprietary,
}

impl NoteFormat {
    /// Map to the closest Formatrix SourceFormat for conversion
    pub fn to_source_format(&self) -> SourceFormat {
        match self {
            NoteFormat::Html => SourceFormat::Markdown, // We convert HTML↔MD
            NoteFormat::Markdown => SourceFormat::Markdown,
            NoteFormat::OrgMode => SourceFormat::OrgMode,
            NoteFormat::Proprietary => SourceFormat::PlainText,
        }
    }
}

/// A lightweight reference to a note in a knowledge store.
///
/// Contains enough information to list/search notes without loading
/// their full content. Use [`KnowledgeStore::read_note`] to get the
/// full document.
#[derive(Debug, Clone)]
pub struct NoteRef {
    /// Unique identifier within the store (note ID, file path, etc.)
    pub id: String,

    /// Human-readable title
    pub title: String,

    /// The format the note is stored in
    pub format: NoteFormat,

    /// Parent note ID (for hierarchical stores like Trilium)
    pub parent_id: Option<String>,

    /// Tags / labels
    pub tags: Vec<String>,

    /// Last modification time (Unix timestamp in seconds)
    pub modified_at: Option<i64>,

    /// Creation time (Unix timestamp in seconds)
    pub created_at: Option<i64>,

    /// Arbitrary metadata from the store
    pub metadata: HashMap<String, String>,
}

/// A change detected during sync
#[derive(Debug, Clone)]
pub struct ChangeSet {
    /// The note that changed
    pub note_id: String,

    /// What kind of change
    pub kind: ChangeKind,

    /// The updated document (if applicable)
    pub document: Option<Document>,

    /// Timestamp of the change
    pub timestamp: i64,
}

/// The kind of change detected
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind {
    Created,
    Modified,
    Deleted,
    Moved,
}

/// The universal interface for knowledge tool bridges.
///
/// Implementations provide bidirectional access to a knowledge tool's
/// note storage. The formatrix core's `FormatRegistry` handles AST
/// conversion; the bridge handles I/O with the tool.
///
/// # Example
///
/// ```rust,ignore
/// let trilium = TriliumBridge::new("http://localhost:37740", "my-etapi-token");
///
/// // List all notes
/// let notes = trilium.list_notes().await?;
///
/// // Read a note as a formatrix Document (AST)
/// let doc = trilium.read_note("abc123").await?;
///
/// // Convert from djot and write back
/// let djot_doc = formatrix_core::open_file("notes/idea.djot")?;
/// trilium.write_note("abc123", &djot_doc.document).await?;
/// ```
#[async_trait::async_trait]
pub trait KnowledgeStore: Send + Sync {
    /// Human-readable name of this store (e.g. "Trilium Notes", "Obsidian Vault")
    fn name(&self) -> &str;

    /// The native note format this store uses
    fn native_format(&self) -> NoteFormat;

    /// Check if the store is reachable and authenticated
    async fn health_check(&self) -> StoreResult<bool>;

    /// List all notes in the store (lightweight refs only)
    async fn list_notes(&self) -> StoreResult<Vec<NoteRef>>;

    /// Search notes by title or content
    async fn search_notes(&self, query: &str) -> StoreResult<Vec<NoteRef>>;

    /// Read a note's full content as a Formatrix Document.
    ///
    /// The bridge is responsible for converting the tool's native format
    /// (HTML, Markdown, etc.) to the Formatrix AST. Use
    /// `formatrix_core::FormatRegistry` for the conversion.
    async fn read_note(&self, id: &str) -> StoreResult<Document>;

    /// Write a Formatrix Document to the store.
    ///
    /// The bridge converts the AST to the tool's native format before
    /// writing. If the note doesn't exist, it is created.
    async fn write_note(&self, id: &str, doc: &Document) -> StoreResult<()>;

    /// Create a new note and return its ID
    async fn create_note(&self, title: &str, doc: &Document, parent_id: Option<&str>)
        -> StoreResult<String>;

    /// Delete a note
    async fn delete_note(&self, id: &str) -> StoreResult<()>;

    /// Pull changes since the last sync.
    ///
    /// Returns a list of changes (created, modified, deleted notes)
    /// since the given timestamp. Pass 0 to get all notes.
    async fn pull_changes(&self, since_timestamp: i64) -> StoreResult<Vec<ChangeSet>>;

    /// Push local changes to the store
    async fn push_changes(&self, changes: &[ChangeSet]) -> StoreResult<()>;
}
