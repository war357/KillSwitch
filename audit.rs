// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Audit logging utilities

use crate::proto::{AuditEntry, AuditEventType, AuditOutcome};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use std::path::Path;

/// Audit logger trait
pub trait AuditLogger {
    /// Log an audit entry
    fn log(&self, entry: AuditEntry) -> Result<(), AuditError>;
    
    /// Flush pending logs
    fn flush(&self) -> Result<(), AuditError>;
}

/// Audit error types
#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Storage error: {0}")]
    Storage(String),
}

/// File-based audit logger
pub struct FileAuditLogger {
    log_path: std::path::PathBuf,
}

impl FileAuditLogger {
    /// Create a new file audit logger
    pub fn new<P: AsRef<Path>>(log_path: P) -> Self {
        Self {
            log_path: log_path.as_ref().to_path_buf(),
        }
    }
    
    /// Write entry to file
    fn write_entry(&self, entry: &AuditEntry) -> Result<(), AuditError> {
        use std::fs::OpenOptions;
        use std::io::Write;
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;
        
        let json = serde_json::to_string(entry)?;
        writeln!(file, "{}", json)?;
        file.sync_all()?;
        
        Ok(())
    }
}

impl AuditLogger for FileAuditLogger {
    fn log(&self, entry: AuditEntry) -> Result<(), AuditError> {
        self.write_entry(&entry)
    }
    
    fn flush(&self) -> Result<(), AuditError> {
        // File writes are already synced
        Ok(())
    }
}

/// Create a new audit entry
pub fn create_audit_entry(
    event_type: AuditEventType,
    actor: &str,
    targets: Vec<Uuid>,
    details: &str,
    outcome: AuditOutcome,
) -> AuditEntry {
    AuditEntry {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        event_type,
        actor: actor.to_string(),
        targets,
        details: details.to_string(),
        outcome,
    }
}

/// In-memory audit logger (for testing)
#[derive(Default)]
pub struct MemoryAuditLogger {
    entries: std::sync::Mutex<Vec<AuditEntry>>,
}

impl MemoryAuditLogger {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn entries(&self) -> Vec<AuditEntry> {
        self.entries.lock().unwrap().clone()
    }
    
    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
    }
}

impl AuditLogger for MemoryAuditLogger {
    fn log(&self, entry: AuditEntry) -> Result<(), AuditError> {
        self.entries.lock().unwrap().push(entry);
        Ok(())
    }
    
    fn flush(&self) -> Result<(), AuditError> {
        Ok(())
    }
}