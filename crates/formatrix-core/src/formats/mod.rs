// SPDX-License-Identifier: AGPL-3.0-or-later
//! Format handlers for each supported format

pub mod plaintext;
pub mod markdown;

// These will be implemented incrementally
// pub mod asciidoc;
// pub mod djot;
// pub mod orgmode;
// pub mod rst;
// pub mod typst;

pub use plaintext::PlainTextHandler;
pub use markdown::MarkdownHandler;
