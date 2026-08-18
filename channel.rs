// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Secure communication channel with server

use anyhow::{Context, Result};
use common::proto::{ServerMessage, AgentMessage};
use tracing::{info, debug, error};
use std::time::Duration;

use super::config::AgentConfig;

/// Communication channel to server
pub struct Channel {
    client: reqwest::Client,
    config: AgentConfig,
    device_id: uuid::Uuid,
}

impl Channel {
    /// Create a new channel
    pub async fn connect(config: &AgentConfig) -> Result<Self> {
        // Build TLS configuration
        let mut tls_config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_native_roots()
            .with_no_client_auth();
        
        // TODO: Load mTLS certificates if configured
        
        let connector = tokio_rustls::TlsConnector::from(tls_config);
        let client = reqwest::Client::builder()
            .use_preconfigured_tls(connector)
            .timeout(Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;
        
        // Get or generate device ID
        let device_id = config.device_id.unwrap_or_else(|| {
            info!("No device ID configured, using ephemeral ID");
            uuid::Uuid::new_v4()
        });
        
        Ok(Self {
            client,
            config: config.clone(),
            device_id,
        })
    }
    
    /// Send a message to server
    pub async fn send(&self, message: AgentMessage) -> Result<()> {
        let url = format!("{}/api/v1/agent/messages", self.config.server_url);
        
        let response = self.client
            .post(&url)
            .json(&message)
            .send()
            .await
            .context("Failed to send message")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            error!("Server returned error: {} - {}", status, text);
            anyhow::bail!("Server error: {}", status);
        }
        
        debug!("Message sent successfully");
        Ok(())
    }
    
    /// Receive next message from server
    pub async fn next(&mut self) -> Option<ServerMessage> {
        let url = format!("{}/api/v1/agent/commands?device_id={}", 
            self.config.server_url, self.device_id);
        
        match self.client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<ServerMessage>().await {
                        Ok(msg) => {
                            debug!("Received command: {:?}", msg);
                            Some(msg)
                        }
                        Err(e) => {
                            // No new commands (empty response or 204)
                            debug!("No new commands: {}", e);
                            tokio::time::sleep(Duration::from_secs(self.config.poll_interval_secs)).await;
                            None
                        }
                    }
                } else {
                    error!("Error fetching commands: {}", response.status());
                    tokio::time::sleep(Duration::from_secs(30)).await;
                    None
                }
            }
            Err(e) => {
                error!("Connection error: {}", e);
                tokio::time::sleep(Duration::from_secs(30)).await;
                None
            }
        }
    }
    
    /// Report status to server
    pub async fn report_status(&self, status: &str) -> Result<()> {
        let url = format!("{}/api/v1/agent/status", self.config.server_url);
        
        #[derive(serde::Serialize)]
        struct StatusReport {
            device_id: uuid::Uuid,
            status: String,
        }
        
        let report = StatusReport {
            device_id: self.device_id,
            status: status.to_string(),
        };
        
        self.client
            .post(&url)
            .json(&report)
            .send()
            .await
            .context("Failed to report status")?;
        
        Ok(())
    }
}

/// Connect to server
pub async fn connect(config: &AgentConfig) -> Result<Channel> {
    Channel::connect(config).await
}