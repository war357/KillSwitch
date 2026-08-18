// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Protocol definitions for agent-server communication

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Unique identifier for a device
pub type DeviceId = Uuid;

/// Unique identifier for a command
pub type CommandId = Uuid;

/// Messages sent from server to agent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ServerMessage {
    /// Heartbeat request
    Heartbeat,
    
    /// Command to reboot into wipe mode
    RebootToWipe {
        command_id: CommandId,
        reason: String,
        requested_by: String,
        timestamp: DateTime<Utc>,
    },
    
    /// Command to wipe specific data disks (non-OS)
    WipeDataDisks {
        command_id: CommandId,
        disks: Vec<String>,
        method: WipeMethod,
        requested_by: String,
        timestamp: DateTime<Utc>,
    },
    
    /// Command to perform full system wipe (requires reboot)
    FullSystemWipe {
        command_id: CommandId,
        method: WipeMethod,
        requested_by: String,
        timestamp: DateTime<Utc>,
    },
    
    /// Configuration update
    UpdateConfig {
        config: AgentConfig,
    },
}

/// Messages sent from agent to server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum AgentMessage {
    /// Agent heartbeat
    Heartbeat {
        device_id: DeviceId,
        uptime_secs: u64,
        load_avg: Option<f32>,
    },
    
    /// Command acknowledgment
    CommandAck {
        command_id: CommandId,
        status: CommandStatus,
        message: Option<String>,
    },
    
    /// Wipe progress update
    WipeProgress {
        command_id: CommandId,
        disk: String,
        percent_complete: u8,
        bytes_written: Option<u64>,
    },
    
    /// Wipe completion
    WipeComplete {
        command_id: CommandId,
        disks: Vec<String>,
        success: bool,
        error: Option<String>,
    },
    
    /// Device inventory
    Inventory {
        device_id: DeviceId,
        disks: Vec<DiskInfo>,
        os_info: OsInfo,
    },
}

/// Wipe method specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WipeMethod {
    /// Single pass of random data
    Random,
    /// Single pass of zeros
    Zero,
    /// ATA/NVMe secure erase
    SecureErase,
    /// DoD 5220.22-M (3 passes)
    Dod5220,
    /// Custom number of random passes
    CustomRandom { passes: u8 },
}

/// Command execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandStatus {
    Accepted,
    InProgress,
    Completed,
    Failed,
    Rejected,
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Server URL
    pub server_url: String,
    /// Poll interval in seconds
    pub poll_interval_secs: u64,
    /// Enable verbose logging
    pub verbose: bool,
    /// Allowed wipe methods
    pub allowed_methods: Vec<WipeMethod>,
}

/// Disk information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    /// Device path (e.g., "/dev/sda")
    pub device_path: String,
    /// Disk size in bytes
    pub size_bytes: u64,
    /// Disk model
    pub model: Option<String>,
    /// Disk serial number
    pub serial: Option<String>,
    /// Whether this is the OS disk
    pub is_os_disk: bool,
    /// Disk type
    pub disk_type: DiskType,
}

/// Disk type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiskType {
    HDD,
    SSD,
    NVMe,
    Unknown,
}

/// Operating system information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsInfo {
    /// OS name
    pub name: String,
    /// OS version
    pub version: String,
    /// Architecture
    pub arch: String,
    /// Kernel version
    pub kernel: String,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unique entry ID
    pub id: Uuid,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Event type
    pub event_type: AuditEventType,
    /// Actor (user or system)
    pub actor: String,
    /// Target device(s)
    pub targets: Vec<DeviceId>,
    /// Event details
    pub details: String,
    /// Outcome
    pub outcome: AuditOutcome,
}

/// Audit event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    DeviceEnrollment,
    WipeRequested,
    WipeStarted,
    WipeProgress,
    WipeCompleted,
    WipeFailed,
    ConfigChanged,
    AuthFailure,
    PolicyViolation,
}

/// Audit outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditOutcome {
    Success,
    Failure { reason: String },
    Partial { success_count: usize, failure_count: usize },
}