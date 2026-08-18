// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! API route handlers

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use common::proto::{DeviceId, ServerMessage, AgentMessage, WipeMethod};
use uuid::Uuid;

use crate::db;
use crate::dispatcher;

/// Health check endpoint
pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// List all devices
pub async fn list_devices() -> Json<serde_json::Value> {
    // TODO: Query database for devices
    Json(serde_json::json!({
        "devices": []
    }))
}

/// Register a new device
#[derive(Deserialize)]
pub struct RegisterDeviceRequest {
    pub device_id: Option<Uuid>,
    pub inventory: common::proto::OsInfo,
}

pub async fn register_device(
    Json(req): Json<RegisterDeviceRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let device_id = req.device_id.unwrap_or_else(Uuid::new_v4);
    
    // TODO: Save to database
    Ok(Json(serde_json::json!({
        "device_id": device_id.to_string(),
        "status": "registered"
    })))
}

/// Get device details
pub async fn get_device(
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Query database
    Ok(Json(serde_json::json!({
        "device_id": id,
        "status": "online"
    })))
}

/// Wipe request
#[derive(Deserialize)]
pub struct WipeRequest {
    pub method: String,
    pub disks: Option<Vec<String>>,
    pub reason: String,
}

/// Request a wipe operation
pub async fn request_wipe(
    Path(id): Path<String>,
    Json(req): Json<WipeRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let device_id = Uuid::parse_str(&id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let method = match req.method.as_str() {
        "random" => WipeMethod::Random,
        "zero" => WipeMethod::Zero,
        "secure_erase" => WipeMethod::SecureErase,
        "dod5220" => WipeMethod::Dod5220,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    
    // Create wipe command
    let command = if let Some(disks) = req.disks {
        ServerMessage::WipeDataDisks {
            command_id: Uuid::new_v4(),
            disks,
            method,
            requested_by: "admin".to_string(), // TODO: Get from auth context
            timestamp: chrono::Utc::now(),
        }
    } else {
        ServerMessage::RebootToWipe {
            command_id: Uuid::new_v4(),
            reason: req.reason,
            requested_by: "admin".to_string(),
            timestamp: chrono::Utc::now(),
        }
    };
    
    // Queue command for dispatcher
    dispatcher::queue_command(device_id, command).await;
    
    Ok(Json(serde_json::json!({
        "status": "queued",
        "device_id": id
    })))
}

/// Get pending commands for agent
#[derive(Deserialize)]
pub struct GetCommandsQuery {
    pub device_id: String,
}

pub async fn get_commands(
    query: axum::extract::Query<GetCommandsQuery>,
) -> Result<Json<ServerMessage>, StatusCode> {
    let device_id = Uuid::parse_str(&query.device_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // TODO: Get next command from queue
    // For now, return heartbeat
    Ok(Json(ServerMessage::Heartbeat))
}

/// Agent message handler
pub async fn agent_message(
    Json(msg): Json<AgentMessage>,
) -> Result<StatusCode, StatusCode> {
    // TODO: Process agent message
    match msg {
        AgentMessage::Heartbeat { device_id, .. } => {
            tracing::info!("Heartbeat from device {}", device_id);
        }
        AgentMessage::CommandAck { command_id, status, .. } => {
            tracing::info!("Command {} status: {:?}", command_id, status);
        }
        AgentMessage::WipeComplete { command_id, success, .. } => {
            tracing::info!("Wipe {} completed: success={}", command_id, success);
        }
        _ => {}
    }
    
    Ok(StatusCode::OK)
}

/// Agent status update
#[derive(Deserialize)]
pub struct AgentStatusRequest {
    pub device_id: Uuid,
    pub status: String,
}

pub async fn agent_status(
    Json(req): Json<AgentStatusRequest>,
) -> Result<StatusCode, StatusCode> {
    tracing::info!("Status update from {}: {}", req.device_id, req.status);
    Ok(StatusCode::OK)
}

/// List admin users
pub async fn list_users() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "users": []
    }))
}

/// Create admin user
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub totp_secret: Option<String>,
}

pub async fn create_user(
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Create user in database
    Ok(Json(serde_json::json!({
        "username": req.username,
        "status": "created"
    })))
}

/// Get audit logs
#[derive(Deserialize)]
pub struct AuditQuery {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

pub async fn get_audit_logs(
    query: axum::extract::Query<AuditQuery>,
) -> Json<serde_json::Value> {
    let limit = query.limit.unwrap_or(100);
    
    // TODO: Query audit logs from database
    Json(serde_json::json!({
        "entries": [],
        "total": 0,
        "limit": limit
    }))
}