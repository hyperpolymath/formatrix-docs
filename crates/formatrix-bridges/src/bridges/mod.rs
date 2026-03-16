// SPDX-License-Identifier: PMPL-1.0-or-later
//! Knowledge tool bridge implementations

#[cfg(feature = "trilium")]
pub mod trilium;

#[cfg(feature = "trilium")]
pub use trilium::TriliumBridge;
