// SPDX-License-Identifier: MPL-2.0
//! Knowledge tool bridge implementations

#[cfg(feature = "trilium")]
pub mod trilium;

#[cfg(feature = "trilium")]
pub use trilium::TriliumBridge;
