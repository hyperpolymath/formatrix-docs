// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
//
//! Trilium Notes bridge via ETAPI (REST API)
//!
//! Trilium's ETAPI runs on localhost:37740 by default and provides full
//! CRUD access to the note tree. Notes are stored as HTML internally;
//! this bridge converts between Formatrix AST (via Markdown renderer)
//! and Trilium's HTML.
//!
//! ETAPI docs: https://github.com/zadam/trilium/wiki/ETAPI
//!
//! Authentication: ETAPI token passed as `Authorization: <token>` header.
//!
//! Key endpoints:
//!   GET  /etapi/app-info          — health check
//!   GET  /etapi/notes             — search notes
//!   GET  /etapi/notes/{id}        — get note metadata
//!   GET  /etapi/notes/{id}/content — get note content (HTML)
//!   PUT  /etapi/notes/{id}/content — update note content
//!   POST /etapi/create-note       — create note
//!   DELETE /etapi/notes/{id}      — delete note

use crate::store::{
    ChangeKind, ChangeSet, KnowledgeStore, NoteFormat, NoteRef, StoreError, StoreResult,
};
use formatrix_core::ast::{Document, SourceFormat};
use formatrix_core::traits::{ParseConfig, RenderConfig};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trilium ETAPI bridge configuration
#[derive(Debug, Clone)]
pub struct TriliumConfig {
    /// Base URL for the ETAPI (default: http://localhost:37740)
    pub base_url: String,

    /// ETAPI authentication token
    pub token: String,

    /// The Formatrix format to use when reading Trilium notes.
    /// Trilium stores HTML; we convert to this format's AST.
    /// Default: Markdown (since HTML↔MD is the most natural conversion).
    pub read_format: SourceFormat,
}

impl Default for TriliumConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:37740".to_string(),
            token: String::new(),
            read_format: SourceFormat::Markdown,
        }
    }
}

/// Trilium ETAPI response types
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppInfo {
    app_version: String,
    db_version: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TriliumNote {
    note_id: String,
    title: String,
    #[serde(rename = "type")]
    note_type: String,
    mime: Option<String>,
    is_protected: bool,
    date_created: Option<String>,
    date_modified: Option<String>,
    utc_date_created: Option<String>,
    utc_date_modified: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchResult {
    results: Vec<TriliumNote>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateNoteRequest {
    parent_note_id: String,
    title: String,
    #[serde(rename = "type")]
    note_type: String,
    content: String,
}

/// Trilium Notes bridge
///
/// Connects to a running Trilium instance via its ETAPI to read, write,
/// and sync notes. Converts between Trilium's native HTML storage and
/// Formatrix's unified AST.
pub struct TriliumBridge {
    config: TriliumConfig,
    client: Client,
}

impl TriliumBridge {
    /// Create a new Trilium bridge with the given base URL and ETAPI token.
    pub fn new(base_url: &str, token: &str) -> Self {
        Self {
            config: TriliumConfig {
                base_url: base_url.trim_end_matches('/').to_string(),
                token: token.to_string(),
                ..Default::default()
            },
            client: Client::new(),
        }
    }

    /// Create from a full config
    pub fn from_config(config: TriliumConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    /// Build the full URL for an ETAPI endpoint
    fn url(&self, path: &str) -> String {
        format!("{}/etapi{}", self.config.base_url, path)
    }

    /// Send an authenticated GET request
    async fn get(&self, path: &str) -> StoreResult<reqwest::Response> {
        self.client
            .get(&self.url(path))
            .header("Authorization", &self.config.token)
            .send()
            .await
            .map_err(|e| StoreError::Network(e.to_string()))
    }

    /// Send an authenticated PUT request with a body
    async fn put(&self, path: &str, body: &str) -> StoreResult<reqwest::Response> {
        self.client
            .put(&self.url(path))
            .header("Authorization", &self.config.token)
            .header("Content-Type", "text/html")
            .body(body.to_string())
            .send()
            .await
            .map_err(|e| StoreError::Network(e.to_string()))
    }

    /// Send an authenticated POST request with JSON
    async fn post_json<T: Serialize>(&self, path: &str, body: &T) -> StoreResult<reqwest::Response> {
        self.client
            .post(&self.url(path))
            .header("Authorization", &self.config.token)
            .json(body)
            .send()
            .await
            .map_err(|e| StoreError::Network(e.to_string()))
    }

    /// Send an authenticated DELETE request
    async fn delete(&self, path: &str) -> StoreResult<reqwest::Response> {
        self.client
            .delete(&self.url(path))
            .header("Authorization", &self.config.token)
            .send()
            .await
            .map_err(|e| StoreError::Network(e.to_string()))
    }

    /// Convert a TriliumNote to a NoteRef
    fn note_to_ref(note: &TriliumNote) -> NoteRef {
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), note.note_type.clone());
        if let Some(ref mime) = note.mime {
            metadata.insert("mime".to_string(), mime.clone());
        }

        NoteRef {
            id: note.note_id.clone(),
            title: note.title.clone(),
            format: NoteFormat::Html,
            parent_id: None, // Would need a separate API call
            tags: Vec::new(), // Labels require /notes/{id}/attributes
            modified_at: note
                .utc_date_modified
                .as_ref()
                .and_then(|d| parse_trilium_date(d)),
            created_at: note
                .utc_date_created
                .as_ref()
                .and_then(|d| parse_trilium_date(d)),
            metadata,
        }
    }

    /// Convert HTML content to a Formatrix Document via Markdown intermediate.
    ///
    /// Trilium stores notes as HTML. We parse it as Markdown (since simple
    /// HTML is close enough to Markdown for most notes). For rich HTML,
    /// a dedicated HTML parser would be better — future enhancement.
    fn html_to_document(&self, html: &str, title: &str) -> StoreResult<Document> {
        // For now, treat HTML content as Markdown-ish and parse it.
        // A proper implementation would use an HTML-to-AST parser.
        let handler = formatrix_core::formats::MarkdownHandler::new();
        let config = ParseConfig::default();

        let mut doc = handler
            .parse(html, &config)
            .map_err(StoreError::Conversion)?;

        // Set the title from Trilium metadata
        doc.meta.title = Some(title.to_string());
        doc.source_format = self.config.read_format;

        Ok(doc)
    }

    /// Convert a Formatrix Document to HTML for Trilium storage
    fn document_to_html(&self, doc: &Document) -> StoreResult<String> {
        // Render to Markdown first, then wrap in simple HTML.
        // A proper implementation would render to HTML directly.
        let handler = formatrix_core::formats::MarkdownHandler::new();
        let config = RenderConfig::default();

        let md = handler
            .render(doc, &config)
            .map_err(StoreError::Conversion)?;

        // Simple Markdown → HTML wrapping. In production, use a proper
        // MD→HTML renderer (comrak can do this directly).
        Ok(md)
    }
}

#[async_trait::async_trait]
impl KnowledgeStore for TriliumBridge {
    fn name(&self) -> &str {
        "Trilium Notes"
    }

    fn native_format(&self) -> NoteFormat {
        NoteFormat::Html
    }

    async fn health_check(&self) -> StoreResult<bool> {
        let resp = self.get("/app-info").await?;

        if resp.status() == 401 {
            return Err(StoreError::AuthError {
                message: "Invalid ETAPI token".to_string(),
            });
        }

        if !resp.status().is_success() {
            return Err(StoreError::Unavailable {
                message: format!("HTTP {}", resp.status()),
            });
        }

        let info: AppInfo = resp
            .json()
            .await
            .map_err(|e| StoreError::Network(e.to_string()))?;

        tracing::info!(
            "Trilium connected: v{} (db v{})",
            info.app_version,
            info.db_version
        );

        Ok(true)
    }

    async fn list_notes(&self) -> StoreResult<Vec<NoteRef>> {
        // ETAPI search with empty query returns all notes
        self.search_notes("").await
    }

    async fn search_notes(&self, query: &str) -> StoreResult<Vec<NoteRef>> {
        let encoded = urlencoding::encode(query);
        let path = format!("/notes?search={}", encoded);
        let resp = self.get(&path).await?;

        if !resp.status().is_success() {
            return Err(StoreError::Network(format!("HTTP {}", resp.status())));
        }

        let result: SearchResult = resp
            .json()
            .await
            .map_err(|e| StoreError::Network(e.to_string()))?;

        Ok(result.results.iter().map(Self::note_to_ref).collect())
    }

    async fn read_note(&self, id: &str) -> StoreResult<Document> {
        // Get note metadata
        let meta_resp = self.get(&format!("/notes/{}", id)).await?;

        if meta_resp.status().as_u16() == 404 {
            return Err(StoreError::NotFound { id: id.to_string() });
        }

        let note: TriliumNote = meta_resp
            .json()
            .await
            .map_err(|e| StoreError::Network(e.to_string()))?;

        // Get note content
        let content_resp = self.get(&format!("/notes/{}/content", id)).await?;
        let content = content_resp
            .text()
            .await
            .map_err(|e| StoreError::Network(e.to_string()))?;

        // Convert HTML → AST
        self.html_to_document(&content, &note.title)
    }

    async fn write_note(&self, id: &str, doc: &Document) -> StoreResult<()> {
        let html = self.document_to_html(doc)?;

        let resp = self.put(&format!("/notes/{}/content", id), &html).await?;

        if resp.status().as_u16() == 404 {
            return Err(StoreError::NotFound { id: id.to_string() });
        }

        if !resp.status().is_success() {
            return Err(StoreError::Network(format!(
                "Failed to write note: HTTP {}",
                resp.status()
            )));
        }

        Ok(())
    }

    async fn create_note(
        &self,
        title: &str,
        doc: &Document,
        parent_id: Option<&str>,
    ) -> StoreResult<String> {
        let html = self.document_to_html(doc)?;

        let req = CreateNoteRequest {
            parent_note_id: parent_id.unwrap_or("root").to_string(),
            title: title.to_string(),
            note_type: "text".to_string(),
            content: html,
        };

        let resp = self.post_json("/create-note", &req).await?;

        if !resp.status().is_success() {
            return Err(StoreError::Network(format!(
                "Failed to create note: HTTP {}",
                resp.status()
            )));
        }

        let created: TriliumNote = resp
            .json()
            .await
            .map_err(|e| StoreError::Network(e.to_string()))?;

        Ok(created.note_id)
    }

    async fn delete_note(&self, id: &str) -> StoreResult<()> {
        let resp = self.delete(&format!("/notes/{}", id)).await?;

        if !resp.status().is_success() {
            return Err(StoreError::Network(format!(
                "Failed to delete note: HTTP {}",
                resp.status()
            )));
        }

        Ok(())
    }

    async fn pull_changes(&self, _since_timestamp: i64) -> StoreResult<Vec<ChangeSet>> {
        // Trilium ETAPI doesn't have a native change feed.
        // We'd need to poll /notes and compare timestamps.
        // For now, return empty — full sync via list_notes + read_note.
        tracing::warn!("Trilium pull_changes not yet implemented — use list_notes + read_note");
        Ok(Vec::new())
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
                    tracing::warn!(
                        "Note move not yet supported for Trilium: {}",
                        change.note_id
                    );
                }
            }
        }
        Ok(())
    }
}

/// Parse Trilium's date format (ISO 8601) to Unix timestamp
fn parse_trilium_date(date_str: &str) -> Option<i64> {
    // Trilium uses format like "2024-01-15 14:30:00.000Z"
    // For now, return None — proper chrono parsing would be better
    let _ = date_str;
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = TriliumConfig::default();
        assert_eq!(config.base_url, "http://localhost:37740");
        assert_eq!(config.read_format, SourceFormat::Markdown);
    }

    #[test]
    fn test_url_building() {
        let bridge = TriliumBridge::new("http://localhost:37740", "test-token");
        assert_eq!(
            bridge.url("/app-info"),
            "http://localhost:37740/etapi/app-info"
        );
        assert_eq!(
            bridge.url("/notes/abc123"),
            "http://localhost:37740/etapi/notes/abc123"
        );
    }

    #[test]
    fn test_note_to_ref() {
        let note = TriliumNote {
            note_id: "abc123".to_string(),
            title: "Test Note".to_string(),
            note_type: "text".to_string(),
            mime: Some("text/html".to_string()),
            is_protected: false,
            date_created: Some("2024-01-15".to_string()),
            date_modified: Some("2024-01-16".to_string()),
            utc_date_created: None,
            utc_date_modified: None,
        };

        let note_ref = TriliumBridge::note_to_ref(&note);
        assert_eq!(note_ref.id, "abc123");
        assert_eq!(note_ref.title, "Test Note");
        assert_eq!(note_ref.format, NoteFormat::Html);
    }
}
