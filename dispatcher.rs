// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Command dispatcher for agent communication

use common::proto::{DeviceId, ServerMessage};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Command queue per device
static COMMAND_QUEUE: RwLock<HashMap<DeviceId, Vec<ServerMessage>>> = RwLock::const_new(HashMap::new());

/// Initialize dispatcher
pub fn init() {
    tracing::info!("Dispatcher initialized");
}

/// Queue a command for a device
pub async fn queue_command(device_id: DeviceId, command: ServerMessage) {
    let mut queue = COMMAND_QUEUE.write().await;
    queue.entry(device_id).or_insert_with(Vec::new).push(command);
    tracing::info!("Command queued for device {}", device_id);
}

/// Get next command for a device
pub async fn get_next_command(device_id: DeviceId) -> Option<ServerMessage> {
    let mut queue = COMMAND_QUEUE.write().await;
    queue.get_mut(&device_id).and_then(|cmds| {
        if cmds.is_empty() {
            None
        } else {
            Some(cmds.remove(0))
        }
    })
}

/// Get queue length for a device
pub async fn get_queue_length(device_id: DeviceId) -> usize {
    let queue = COMMAND_QUEUE.read().await;
    queue.get(&device_id).map(|v| v.len()).unwrap_or(0)
}