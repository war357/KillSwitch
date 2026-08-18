// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Agent configuration management

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use uuid::Uuid;

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Server URL
    pub server_url: String,
    
    /// Device ID (generated on first run if not present)
    pub device_id: Option<Uuid>,
    
    /// TLS certificate path (optional, for mTLS)
    pub tls_cert_path: Option<String>,
    
    /// TLS key path (optional, for mTLS)
    pub tls_key_path: Option<String>,
    
    /// CA certificate path (optional)
    pub ca_cert_path: Option<String>,
    
    /// Poll interval in seconds
    #[serde(default = "default_poll_interval")]
    pub poll_interval_secs: u64,
    
    /// Enable verbose logging
    #[serde(default)]
    pub verbose: bool,
    
    /// Allowed wipe methods
    #[serde(default = "default_allowed_methods")]
    pub allowed_methods: Vec<String>,
    
    /// Wipe partition device (optional)
    pub wipe_partition_device: Option<String>,
}

fn default_poll_interval() -> u64 {
    30
}

fn default_allowed_methods() -> Vec<String> {
    vec!["random".to_string(), "zero".to_string()]
}

impl AgentConfig {
    /// Load configuration from file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(&path)
            .context("Failed to read config file")?;
        
        let config: Self = toml::from_str(&content)
            .context("Failed to parse config file")?;
        
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        std::fs::write(&path, content)
            .context("Failed to write config file")?;
        
        Ok(())
    }
    
    /// Generate a new device ID
    pub fn generate_device_id(&mut self) {
        self.device_id = Some(Uuid::new_v4());
    }
}

/// Load configuration from file
pub fn load<P: AsRef<Path>>(path: P) -> Result<AgentConfig> {
    AgentConfig::load(path)
}

/// Create default configuration
pub fn default_config() -> AgentConfig {
    AgentConfig {
        server_url: "https://localhost:8080".to_string(),
        device_id: None,
        tls_cert_path: None,
        tls_key_path: None,
        ca_cert_path: None,
        poll_interval_secs: 30,
        verbose: false,
        allowed_methods: default_allowed_methods(),
        wipe_partition_device: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_load_config() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let config_content = r#"
            server_url = "https://example.com:8080"
            poll_interval_secs = 60
        "#;
        writeln!(temp_file, "{}", config_content).unwrap();
        
        let config = AgentConfig::load(temp_file.path()).unwrap();
        assert_eq!(config.server_url, "https://example.com:8080");
        assert_eq!(config.poll_interval_secs, 60);
        assert!(config.device_id.is_none());
    }
    
    #[test]
    fn test_save_config() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut config = default_config();
        config.server_url = "https://test.com:8080".to_string();
        
        config.save(temp_file.path()).unwrap();
        
        let loaded = AgentConfig::load(temp_file.path()).unwrap();
        assert_eq!(loaded.server_url, "https://test.com:8080");
    }
}