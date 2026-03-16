// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
//
//! Filesystem-based knowledge store bridge
//!
//! Supports tools that store notes as plain files in a directory:
//! - Obsidian (Markdown vault)
//! - Logseq (Markdown or Org Mode)
//! - Zettlr (Markdown)
//! - Any folder of markup files
//!
//! This bridge watches for filesystem changes and maps them to the
//! KnowledgeStore trait interface.

use crate::store::{
    ChangeKind, ChangeSet, KnowledgeStore, NoteFormat, NoteRef, StoreError, StoreResult,
};
use formatrix_core::ast::Document;
use formatrix_core::file_ops;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Configuration for a filesystem-based knowledge store
#[derive(Debug, Clone)]
pub struct FsConfig {
    /// Root directory of the vault/workspace
    pub root: PathBuf,

    /// Human-readable name (e.g. "My Obsidian Vault")
    pub name: String,

    /// The native format this vault uses
    pub format: NoteFormat,

    /// File extensions to include (empty = all supported)
    pub extensions: Vec<String>,

    /// Directories to exclude (e.g. ".obsidian", ".git")
    pub exclude_dirs: Vec<String>,
}

impl FsConfig {
    /// Create an Obsidian vault config
    pub fn obsidian(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            name: "Obsidian Vault".to_string(),
            format: NoteFormat::Markdown,
            extensions: vec!["md".to_string()],
            exclude_dirs: vec![
                ".obsidian".to_string(),
                ".git".to_string(),
                ".trash".to_string(),
                ".smart-env".to_string(),
            ],
        }
    }

    /// Create a Logseq config
    pub fn logseq(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            name: "Logseq Graph".to_string(),
            format: NoteFormat::Markdown,
            extensions: vec!["md".to_string(), "org".to_string()],
            exclude_dirs: vec![
                ".git".to_string(),
                "logseq".to_string(),
                ".recycle".to_string(),
            ],
        }
    }

    /// Create a generic markup folder config
    pub fn generic(root: impl Into<PathBuf>, name: &str) -> Self {
        Self {
            root: root.into(),
            name: name.to_string(),
            format: NoteFormat::Markdown,
            extensions: Vec::new(), // All supported formats
            exclude_dirs: vec![".git".to_string()],
        }
    }
}

/// Filesystem-based knowledge store
pub struct FsBridge {
    config: FsConfig,
}

impl FsBridge {
    pub fn new(config: FsConfig) -> Self {
        Self { config }
    }

    /// Check if a path should be included based on config
    fn should_include(&self, path: &Path) -> bool {
        // Check excluded directories
        for ancestor in path.ancestors() {
            if let Some(name) = ancestor.file_name() {
                let name_str = name.to_string_lossy();
                if self.config.exclude_dirs.iter().any(|d| d == name_str.as_ref()) {
                    return false;
                }
            }
        }

        // Check extension filter
        if !self.config.extensions.is_empty() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                return self.config.extensions.iter().any(|e| e == &ext_str);
            }
            return false;
        }

        // If no extension filter, accept all supported formatrix extensions
        if let Some(ext) = path.extension() {
            file_ops::is_supported_extension(&ext.to_string_lossy())
        } else {
            false
        }
    }

    /// Scan the vault directory for all matching files
    fn scan_files(&self) -> StoreResult<Vec<PathBuf>> {
        let mut files = Vec::new();
        self.scan_dir(&self.config.root, &mut files)?;
        files.sort();
        Ok(files)
    }

    /// Recursively scan a directory
    fn scan_dir(&self, dir: &Path, files: &mut Vec<PathBuf>) -> StoreResult<()> {
        let entries = std::fs::read_dir(dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let dir_name = path.file_name().unwrap_or_default().to_string_lossy();
                if !self.config.exclude_dirs.iter().any(|d| d == dir_name.as_ref()) {
                    self.scan_dir(&path, files)?;
                }
            } else if self.should_include(&path) {
                files.push(path);
            }
        }

        Ok(())
    }

    /// Convert a file path to a note ID (relative path from root)
    fn path_to_id(&self, path: &Path) -> String {
        path.strip_prefix(&self.config.root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string()
    }

    /// Convert a note ID back to a file path
    fn id_to_path(&self, id: &str) -> PathBuf {
        self.config.root.join(id)
    }
}

#[async_trait::async_trait]
impl KnowledgeStore for FsBridge {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn native_format(&self) -> NoteFormat {
        self.config.format
    }

    async fn health_check(&self) -> StoreResult<bool> {
        Ok(self.config.root.is_dir())
    }

    async fn list_notes(&self) -> StoreResult<Vec<NoteRef>> {
        let files = self.scan_files()?;
        let mut notes = Vec::with_capacity(files.len());

        for path in &files {
            let id = self.path_to_id(path);
            let title = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let metadata = std::fs::metadata(path)?;
            let modified_at = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs() as i64);

            notes.push(NoteRef {
                id,
                title,
                format: self.config.format,
                parent_id: path
                    .parent()
                    .and_then(|p| p.strip_prefix(&self.config.root).ok())
                    .map(|p| p.to_string_lossy().to_string())
                    .filter(|s| !s.is_empty()),
                tags: Vec::new(),
                modified_at,
                created_at: None,
                metadata: HashMap::new(),
            });
        }

        Ok(notes)
    }

    async fn search_notes(&self, query: &str) -> StoreResult<Vec<NoteRef>> {
        let all = self.list_notes().await?;
        let query_lower = query.to_lowercase();

        Ok(all
            .into_iter()
            .filter(|n| n.title.to_lowercase().contains(&query_lower))
            .collect())
    }

    async fn read_note(&self, id: &str) -> StoreResult<Document> {
        let path = self.id_to_path(id);

        if !path.exists() {
            return Err(StoreError::NotFound { id: id.to_string() });
        }

        let opened = file_ops::open_file(&path)
            .map_err(|e| StoreError::Other(e.to_string()))?;

        Ok(opened.document)
    }

    async fn write_note(&self, id: &str, doc: &Document) -> StoreResult<()> {
        let path = self.id_to_path(id);

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        file_ops::save_file(doc, &path)
            .map_err(|e| StoreError::Other(e.to_string()))?;

        Ok(())
    }

    async fn create_note(
        &self,
        title: &str,
        doc: &Document,
        parent_id: Option<&str>,
    ) -> StoreResult<String> {
        let ext = doc.source_format.extension();
        let filename = format!("{}.{}", title, ext);

        let path = if let Some(parent) = parent_id {
            self.config.root.join(parent).join(&filename)
        } else {
            self.config.root.join(&filename)
        };

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        file_ops::save_file(doc, &path)
            .map_err(|e| StoreError::Other(e.to_string()))?;

        Ok(self.path_to_id(&path))
    }

    async fn delete_note(&self, id: &str) -> StoreResult<()> {
        let path = self.id_to_path(id);

        if path.exists() {
            std::fs::remove_file(&path)?;
        }

        Ok(())
    }

    async fn pull_changes(&self, since_timestamp: i64) -> StoreResult<Vec<ChangeSet>> {
        let files = self.scan_files()?;
        let mut changes = Vec::new();

        for path in &files {
            let metadata = std::fs::metadata(path)?;
            let modified = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);

            if modified > since_timestamp {
                let id = self.path_to_id(path);
                let doc = file_ops::open_file(path)
                    .map_err(|e| StoreError::Other(e.to_string()))?;

                changes.push(ChangeSet {
                    note_id: id,
                    kind: ChangeKind::Modified,
                    document: Some(doc.document),
                    timestamp: modified,
                });
            }
        }

        Ok(changes)
    }

    async fn push_changes(&self, changes: &[ChangeSet]) -> StoreResult<()> {
        for change in changes {
            match change.kind {
                ChangeKind::Created | ChangeKind::Modified => {
                    if let Some(ref doc) = change.document {
                        self.write_note(&change.note_id, doc).await?;
                    }
                }
                ChangeKind::Deleted => {
                    self.delete_note(&change.note_id).await?;
                }
                ChangeKind::Moved => {
                    tracing::warn!("File move not yet handled: {}", change.note_id);
                }
            }
        }
        Ok(())
    }
}
