// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Shared types and utilities for Remote Wipe
//!
//! This crate contains common data structures, protocol definitions,
//! and utility functions used across the agent, server, and wipe-partition components.

pub mod proto;
pub mod audit;
pub mod crypto;

pub use proto::*;
pub use audit::*;