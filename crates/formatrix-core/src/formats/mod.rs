// SPDX-License-Identifier: AGPL-3.0-or-later
//! Format handlers for each supported format

pub mod plaintext;
pub mod markdown;
pub mod djot;
pub mod orgmode;

// FD-S02, FD-S03: SHOULD requirement implementations
pub mod rst;
pub mod typst;

// FD-S01: AsciiDoc - to be implemented
// pub mod asciidoc;

pub use plaintext::PlainTextHandler;
pub use markdown::MarkdownHandler;
pub use djot::DjotHandler;
pub use orgmode::OrgModeHandler;

// SHOULD handlers
pub use rst::RstHandler;
pub use typst::TypstHandler;
