// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Remote Wipe Server
//!
//! Control plane API for managing devices and issuing wipe commands.

use anyhow::{Context, Result};
use clap::Parser;
use std::net::SocketAddr;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::{Router, routing::{get, post}, middleware};

mod api;
mod auth;
mod db;
mod policy;
mod dispatcher;

/// Remote Wipe Server CLI arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Database URL
    #[arg(short, long, default_value = "postgres://remote_wipe:password@localhost/remote_wipe")]
    database_url: String,
    
    /// Server bind address
    #[arg(short, long, default_value = "0.0.0.0:8080")]
    bind: String,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize logging
    let log_level = if args.verbose { "debug" } else { "info" };
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| log_level.parse().unwrap()),
        )
        .init();
    
    info!("Remote Wipe Server starting...");
    
    // Initialize database
    db::init(&args.database_url).await
        .context("Failed to initialize database")?;
    info!("Database initialized");
    
    // Initialize authentication
    auth::init().await
        .context("Failed to initialize authentication")?;
    info!("Authentication initialized");
    
    // Initialize dispatcher
    dispatcher::init();
    info!("Dispatcher initialized");
    
    // Build router
    let app = Router::new()
        .route("/health", get(api::health))
        .route("/api/v1/devices", get(api::list_devices))
        .route("/api/v1/devices", post(api::register_device))
        .route("/api/v1/devices/:id", get(api::get_device))
        .route("/api/v1/devices/:id/wipe", post(api::request_wipe))
        .route("/api/v1/agent/commands", get(api::get_commands))
        .route("/api/v1/agent/messages", post(api::agent_message))
        .route("/api/v1/agent/status", post(api::agent_status))
        .route("/api/v1/admin/users", get(api::list_users))
        .route("/api/v1/admin/users", post(api::create_user))
        .route("/api/v1/audit", get(api::get_audit_logs))
        .layer(middleware::from_fn(auth::auth_middleware));
    
    // Bind and serve
    let addr: SocketAddr = args.bind.parse()
        .context("Invalid bind address")?;
    
    info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}