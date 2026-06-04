// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Knowledge tool bridge implementations

#[cfg(feature = "trilium")]
pub mod trilium;

#[cfg(feature = "trilium")]
pub use trilium::TriliumBridge;
