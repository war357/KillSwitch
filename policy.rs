// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Policy enforcement for wipe operations

use common::proto::{DeviceId, WipeMethod};
use uuid::Uuid;

/// Wipe policy
#[derive(Debug, Clone)]
pub struct WipePolicy {
    /// Require dual approval for bulk wipes
    pub require_dual_approval: bool,
    /// Maximum number of devices that can be wiped in one request
    pub max_bulk_wipe_size: usize,
    /// Allowed wipe methods
    pub allowed_methods: Vec<WipeMethod>,
    /// Require local confirmation for full system wipes
    pub require_local_confirmation: bool,
}

impl Default for WipePolicy {
    fn default() -> Self {
        Self {
            require_dual_approval: true,
            max_bulk_wipe_size: 10,
            allowed_methods: vec![
                WipeMethod::Random,
                WipeMethod::Zero,
                WipeMethod::SecureErase,
            ],
            require_local_confirmation: true,
        }
    }
}

/// Check if wipe is allowed by policy
pub fn check_wipe_policy(
    policy: &WipePolicy,
    device_count: usize,
    method: &WipeMethod,
) -> PolicyResult {
    // Check bulk wipe limit
    if device_count > policy.max_bulk_wipe_size {
        return PolicyResult::Denied(format!(
            "Bulk wipe limit exceeded: {} > {}",
            device_count, policy.max_bulk_wipe_size
        ));
    }
    
    // Check allowed methods
    if !policy.allowed_methods.contains(method) {
        return PolicyResult::Denied(format!(
            "Wipe method {:?} not allowed",
            method
        ));
    }
    
    PolicyResult::Allowed
}

/// Policy check result
#[derive(Debug)]
pub enum PolicyResult {
    Allowed,
    Denied(String),
    RequiresApproval { approvers_needed: usize },
}

/// Check if user can wipe device
pub fn can_user_wipe_device(
    user_id: &str,
    device_id: &DeviceId,
    role: &str,
) -> bool {
    // TODO: Implement proper RBAC checks
    // For now, allow admins to wipe any device
    role == "admin"
}

/// Get pending approvals for wipe
pub fn get_pending_approvals(wipe_id: Uuid) -> Vec<String> {
    // TODO: Query database for pending approvals
    vec![]
}

/// Add approval for wipe
pub fn add_approval(wipe_id: Uuid, approver: &str) -> Result<(), String> {
    // TODO: Record approval in database
    Ok(())
}