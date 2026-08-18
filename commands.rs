// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Command handling and execution

use anyhow::{Context, Result};
use common::proto::{ServerMessage, AgentMessage, CommandStatus, AuditEventType, AuditOutcome};
use common::audit::{self, AuditLogger, FileAuditLogger};
use tracing::{info, warn, error};
use std::path::PathBuf;

use super::config::AgentConfig;
use super::wipe;

/// Audit log path
const AUDIT_LOG_PATH: &str = "/var/log/remote-wipe/audit.log";

/// Handle incoming server message
pub async fn handle_message(msg: ServerMessage, config: &AgentConfig) -> Result<()> {
    let audit_logger = FileAuditLogger::new(AUDIT_LOG_PATH);
    
    match msg {
        ServerMessage::Heartbeat => {
            info!("Received heartbeat request");
            // Send heartbeat response
            let response = AgentMessage::Heartbeat {
                device_id: config.device_id.unwrap_or_default(),
                uptime_secs: 0, // TODO: Track actual uptime
                load_avg: None,
            };
            // In production, send this back to server
            info!("Heartbeat sent");
        }
        
        ServerMessage::RebootToWipe { 
            command_id, 
            reason, 
            requested_by,
            timestamp,
        } => {
            info!("Received RebootToWipe command from {}", requested_by);
            info!("Reason: {}", reason);
            
            // Log audit event
            let entry = audit::create_audit_entry(
                AuditEventType::WipeRequested,
                &requested_by,
                vec![config.device_id.unwrap_or_default()],
                &format!("Reboot to wipe: {}", reason),
                AuditOutcome::Success,
            );
            if let Err(e) = audit_logger.log(entry) {
                error!("Failed to log audit event: {}", e);
            }
            
            // Set wipe flag and reboot
            if let Err(e) = wipe::set_wipe_flag(&reason) {
                error!("Failed to set wipe flag: {}", e);
                
                // Report failure
                let response = AgentMessage::CommandAck {
                    command_id,
                    status: CommandStatus::Failed,
                    message: Some(format!("Failed to set wipe flag: {}", e)),
                };
                // TODO: Send response to server
                return Err(e);
            }
            
            info!("Wipe flag set, rebooting in 5 seconds...");
            
            // Give time for final logging
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            
            // Reboot system
            wipe::reboot_system();
        }
        
        ServerMessage::WipeDataDisks { 
            command_id,
            disks,
            method,
            requested_by,
            timestamp,
        } => {
            info!("Received WipeDataDisks command from {}", requested_by);
            info!("Disks: {:?}, Method: {:?}", disks, method);
            
            // Log audit event
            let entry = audit::create_audit_entry(
                AuditEventType::WipeRequested,
                &requested_by,
                vec![config.device_id.unwrap_or_default()],
                &format!("Wipe data disks: {:?}", disks),
                AuditOutcome::Success,
            );
            if let Err(e) = audit_logger.log(entry) {
                error!("Failed to log audit event: {}", e);
            }
            
            // Acknowledge command
            let response = AgentMessage::CommandAck {
                command_id,
                status: CommandStatus::Accepted,
                message: None,
            };
            // TODO: Send response to server
            
            // Execute wipe
            match wipe::wipe_disks(&disks, &method).await {
                Ok(_) => {
                    info!("Wipe completed successfully");
                    
                    let complete = AgentMessage::WipeComplete {
                        command_id,
                        disks: disks.clone(),
                        success: true,
                        error: None,
                    };
                    // TODO: Send completion to server
                }
                Err(e) => {
                    error!("Wipe failed: {}", e);
                    
                    let complete = AgentMessage::WipeComplete {
                        command_id,
                        disks: disks.clone(),
                        success: false,
                        error: Some(e.to_string()),
                    };
                    // TODO: Send failure to server
                    return Err(e);
                }
            }
        }
        
        ServerMessage::FullSystemWipe { 
            command_id,
            method,
            requested_by,
            timestamp,
        } => {
            warn!("Received FullSystemWipe command - requires reboot");
            // This should trigger a reboot to wipe partition
            // Similar to RebootToWipe but with full system wipe
        }
        
        ServerMessage::UpdateConfig { config: new_config } => {
            info!("Received configuration update");
            // TODO: Update runtime configuration
            // Note: Some changes may require restart
        }
    }
    
    Ok(())
}