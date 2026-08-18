// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Remote Wipe Agent
//!
//! This agent runs on endpoints and communicates with the Remote Wipe server
//! to receive and execute wipe commands securely.

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use tracing::{info, error, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod channel;
mod commands;
mod wipe;

/// Remote Wipe Agent CLI arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short, long, default_value = "/etc/remote-wipe/agent.toml")]
    config: PathBuf,
    
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
    
    info!("Remote Wipe Agent starting...");
    
    // Load configuration
    let cfg = config::load(&args.config)
        .context(format!("Failed to load config from {:?}", args.config))?;
    
    info!("Configuration loaded from {:?}", args.config);
    info!("Server URL: {}", cfg.server_url);
    info!("Device ID: {}", cfg.device_id);
    
    // Initialize device ID if not present
    if cfg.device_id.is_none() {
        warn!("Device ID not configured, generating new one...");
        // In production, this would be saved to config
    }
    
    // Connect to server
    let mut ch = channel::connect(&cfg).await
        .context("Failed to connect to server")?;
    
    info!("Connected to server, waiting for commands...");
    
    // Main command loop
    loop {
        match ch.next().await {
            Some(msg) => {
                if let Err(e) = commands::handle_message(msg, &cfg).await {
                    error!("Error handling command: {}", e);
                    // Continue processing other commands
                }
            }
            None => {
                warn!("Connection closed, reconnecting in 30 seconds...");
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                
                match channel::connect(&cfg).await {
                    Ok(new_ch) => {
                        ch = new_ch;
                        info!("Reconnected to server");
                    }
                    Err(e) => {
                        error!("Reconnection failed: {}", e);
                    }
                }
            }
        }
    }
}