// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Wipe partition configuration

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Wipe configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WipeConfig {
    /// Disks to wipe (device names without /dev/ prefix)
    pub disks: Vec<String>,
    
    /// TOTP secret for 2FA
    pub totp_secret: Option<String>,
    
    /// Default wipe method
    #[serde(default = "default_method")]
    pub default_method: String,
    
    /// Require 2FA
    #[serde(default = "default_true")]
    pub require_2fa: bool,
    
    /// Power off after wipe
    #[serde(default = "default_true")]
    pub power_off_after: bool,
}

fn default_method() -> String {
    "random".to_string()
}

fn default_true() -> bool {
    true
}

impl WipeConfig {
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
}

/// Load configuration
pub fn load<P: AsRef<Path>>(path: P) -> Result<WipeConfig> {
    WipeConfig::load(path)
}

/// Default configuration
pub fn default_config() -> WipeConfig {
    WipeConfig {
        disks: vec![],
        totp_secret: None,
        default_method: "random".to_string(),
        require_2fa: true,
        power_off_after: true,
    }
}