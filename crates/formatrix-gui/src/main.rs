// SPDX-License-Identifier: PMPL-1.0-or-later
//! Formatrix Docs - Gossamer desktop application
//!
//! Cross-platform document editor with format tabs.
//! Migrated from Tauri 2.0 to Gossamer webview shell.

use gossamer_rs::App;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod commands;

fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "formatrix_gui=debug,gossamer=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Formatrix Docs v{}", env!("CARGO_PKG_VERSION"));

    App::new()
        .command("load_document", commands::load_document)
        .command("save_document", commands::save_document)
        .command("convert_to_format", commands::convert_to_format)
        .command("get_document_events", commands::get_document_events)
        .command("clear_document_events", commands::clear_document_events)
        .command("parse_document", commands::parse_document)
        .command("render_document", commands::render_document)
        .command("detect_format", commands::detect_format)
        .command("get_supported_formats", commands::get_supported_formats)
        .run();
}
