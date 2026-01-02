// SPDX-License-Identifier: AGPL-3.0-or-later
//! Tauri commands for document operations

use formatrix_core::{Document, ParseConfig, RenderConfig, SourceFormat};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMeta {
    pub path: Option<String>,
    pub format: String,
    pub modified: bool,
    pub word_count: usize,
    pub char_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentData {
    pub content: String,
    pub meta: DocumentMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub content: String,
    pub warnings: Vec<String>,
}

/// Load a document from the filesystem
#[tauri::command]
pub async fn load_document(path: String) -> Result<DocumentData, String> {
    let content = tokio::fs::read_to_string(&path)
        .await
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Detect format from extension
    let format = std::path::Path::new(&path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|ext| match ext {
            "txt" => "txt",
            "md" | "markdown" => "md",
            "adoc" | "asciidoc" => "adoc",
            "dj" | "djot" => "djot",
            "org" => "org",
            "rst" => "rst",
            "typ" => "typ",
            _ => "txt",
        })
        .unwrap_or("txt")
        .to_string();

    let word_count = content.split_whitespace().count();
    let char_count = content.chars().count();

    Ok(DocumentData {
        content,
        meta: DocumentMeta {
            path: Some(path),
            format,
            modified: false,
            word_count,
            char_count,
        },
    })
}

/// Save a document to the filesystem
#[tauri::command]
pub async fn save_document(
    path: String,
    content: String,
    format: String,
) -> Result<DocumentMeta, String> {
    tokio::fs::write(&path, &content)
        .await
        .map_err(|e| format!("Failed to write file: {}", e))?;

    let word_count = content.split_whitespace().count();
    let char_count = content.chars().count();

    Ok(DocumentMeta {
        path: Some(path),
        format,
        modified: false,
        word_count,
        char_count,
    })
}

/// Convert document content from one format to another
#[tauri::command]
pub async fn convert_to_format(
    content: String,
    from_format: String,
    to_format: String,
) -> Result<ConversionResult, String> {
    use formatrix_core::formats::{MarkdownHandler, PlainTextHandler};
    use formatrix_core::traits::{Parser, Renderer};

    // For now, just return the content as-is if converting to same format
    if from_format == to_format {
        return Ok(ConversionResult {
            content,
            warnings: Vec::new(),
        });
    }

    // Parse source format
    let parse_config = ParseConfig::default();
    let render_config = RenderConfig::default();

    let doc = match from_format.as_str() {
        "txt" => PlainTextHandler::new()
            .parse(&content, &parse_config)
            .map_err(|e| e.to_string())?,
        "md" => MarkdownHandler::new()
            .parse(&content, &parse_config)
            .map_err(|e| e.to_string())?,
        _ => {
            return Err(format!("Unsupported source format: {}", from_format));
        }
    };

    // Render to target format
    let output = match to_format.as_str() {
        "txt" => PlainTextHandler::new()
            .render(&doc, &render_config)
            .map_err(|e| e.to_string())?,
        "md" => MarkdownHandler::new()
            .render(&doc, &render_config)
            .map_err(|e| e.to_string())?,
        _ => {
            return Err(format!("Unsupported target format: {}", to_format));
        }
    };

    Ok(ConversionResult {
        content: output,
        warnings: Vec::new(),
    })
}
