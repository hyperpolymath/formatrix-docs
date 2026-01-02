// SPDX-License-Identifier: AGPL-3.0-or-later
//! Formatrix Core - Unified document AST and format converters
//!
//! This crate provides:
//! - A unified AST that all document formats convert to/from
//! - Parser and renderer traits for format handlers
//! - Implementations for 7 formats: TXT, MD, ADOC, DJOT, ORG, RST, TYP
//! - C FFI exports for the Ada TUI

pub mod ast;
pub mod formats;
pub mod traits;

#[cfg(feature = "ffi")]
pub mod ffi;

pub use ast::{Block, Document, DocumentMeta, Inline, SourceFormat};
pub use traits::{ConversionError, ParseConfig, Parser, RenderConfig, Renderer, Result};
