// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
//
//! Unified AST for document representation
//!
//! All document formats parse to this AST and render from it.
//! The AST is format-agnostic — it represents the semantic structure
//! of a document, not its syntactic surface form.

use serde::{Deserialize, Serialize};

/// Source format of a document
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceFormat {
    PlainText,
    Markdown,
    AsciiDoc,
    Djot,
    OrgMode,
    ReStructuredText,
    Typst,
}

impl SourceFormat {
    /// Get the canonical file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            SourceFormat::PlainText => "txt",
            SourceFormat::Markdown => "md",
            SourceFormat::AsciiDoc => "adoc",
            SourceFormat::Djot => "djot",
            SourceFormat::OrgMode => "org",
            SourceFormat::ReStructuredText => "rst",
            SourceFormat::Typst => "typ",
        }
    }

    /// Get the MIME type for this format
    pub fn mime_type(&self) -> &'static str {
        match self {
            SourceFormat::PlainText => "text/plain",
            SourceFormat::Markdown => "text/markdown",
            SourceFormat::AsciiDoc => "text/asciidoc",
            SourceFormat::Djot => "text/djot",
            SourceFormat::OrgMode => "text/org",
            SourceFormat::ReStructuredText => "text/x-rst",
            SourceFormat::Typst => "text/typst",
        }
    }
}

/// Document metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocumentMeta {
    /// Document title (extracted from first heading or frontmatter)
    pub title: Option<String>,

    /// Author(s) from frontmatter
    pub authors: Vec<String>,

    /// Date from frontmatter
    pub date: Option<String>,

    /// Arbitrary key-value metadata from frontmatter
    pub frontmatter: std::collections::HashMap<String, String>,

    /// Tags / keywords
    pub tags: Vec<String>,
}

/// Source span for error reporting and lossless round-trip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    /// Start byte offset
    pub start: usize,
    /// End byte offset
    pub end: usize,
    /// Line number (1-based)
    pub line: u32,
    /// Column number (1-based)
    pub column: u32,
}

/// A complete document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// The format this document was parsed from
    pub source_format: SourceFormat,

    /// Document-level metadata
    pub meta: DocumentMeta,

    /// Block-level content
    pub content: Vec<Block>,

    /// Raw source text for lossless round-trip (if preserved)
    #[serde(skip)]
    pub raw_source: Option<String>,
}

/// A list item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    /// Content blocks within the list item
    pub content: Vec<Block>,

    /// Whether this item is checked (for task lists)
    pub checked: Option<bool>,
}

/// Block-level content elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Block {
    /// A paragraph of inline content
    Paragraph {
        content: Vec<Inline>,
        span: Option<Span>,
    },

    /// A heading (h1–h6)
    Heading {
        level: u8,
        content: Vec<Inline>,
        id: Option<String>,
        span: Option<Span>,
    },

    /// A fenced or indented code block
    CodeBlock {
        language: Option<String>,
        content: String,
        span: Option<Span>,
    },

    /// A block quote
    BlockQuote {
        content: Vec<Block>,
        span: Option<Span>,
    },

    /// An ordered or unordered list
    List {
        ordered: bool,
        start: Option<u32>,
        items: Vec<ListItem>,
        span: Option<Span>,
    },

    /// A thematic break / horizontal rule
    ThematicBreak {
        span: Option<Span>,
    },

    /// A table
    Table {
        headers: Vec<Vec<Inline>>,
        rows: Vec<Vec<Vec<Inline>>>,
        alignments: Vec<Alignment>,
        span: Option<Span>,
    },

    /// Raw content in a specific format (passthrough)
    Raw {
        format: Option<String>,
        content: String,
        span: Option<Span>,
    },

    /// A definition list
    DefinitionList {
        items: Vec<(Vec<Inline>, Vec<Block>)>,
        span: Option<Span>,
    },

    /// An admonition / callout (note, warning, tip, etc.)
    Admonition {
        kind: String,
        title: Option<Vec<Inline>>,
        content: Vec<Block>,
        span: Option<Span>,
    },

    /// A footnote definition
    FootnoteDefinition {
        label: String,
        content: Vec<Block>,
        span: Option<Span>,
    },
}

/// Table column alignment
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Default,
}

/// Inline content elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Inline {
    /// Plain text
    Text { content: String },

    /// Emphasized text (italic)
    Emphasis { content: Vec<Inline> },

    /// Strong text (bold)
    Strong { content: Vec<Inline> },

    /// Inline code
    Code {
        content: String,
        language: Option<String>,
    },

    /// A hyperlink
    Link {
        url: String,
        title: Option<String>,
        content: Vec<Inline>,
    },

    /// An image
    Image {
        url: String,
        alt: String,
        title: Option<String>,
    },

    /// A hard line break
    LineBreak,

    /// A soft line break (typically rendered as a space)
    SoftBreak,

    /// Strikethrough text
    Strikethrough { content: Vec<Inline> },

    /// Superscript
    Superscript { content: Vec<Inline> },

    /// Subscript
    Subscript { content: Vec<Inline> },

    /// A footnote reference
    FootnoteReference { label: String },

    /// Raw inline content (e.g. HTML)
    RawInline {
        format: Option<String>,
        content: String,
    },

    /// Math (inline)
    Math { content: String },

    /// Math (display/block)
    DisplayMath { content: String },
}
