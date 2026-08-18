// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! TOTP 2FA verification

use anyhow::{Context, Result};
use std::io::{self, Write};
use totp_rs::TOTP;

use super::config::WipeConfig;

/// Verify TOTP 2FA
pub fn verify_2fa(cfg: &WipeConfig) -> Result<bool> {
    // Get TOTP secret from config
    let totp_secret = cfg.totp_secret.as_ref()
        .context("TOTP secret not configured")?;
    
    // Parse TOTP from secret
    let totp = TOTP::from_url(totp_secret)
        .or_else(|_| {
            // Try as raw secret
            TOTP::new(
                totp_rs::Algorithm::SHA1,
                6,
                1,
                30,
                totp_secret.as_bytes().to_vec(),
            ).map_err(|e| anyhow::anyhow!("Invalid TOTP secret: {}", e))
        })?;
    
    // Prompt for TOTP code
    print!("Enter 6-digit TOTP code: ");
    io::stdout().flush()?;
    
    let mut code = String::new();
    io::stdin().read_line(&mut code)?;
    let code = code.trim();
    
    // Verify code
    match totp.check_current(code) {
        Ok(true) => Ok(true),
        Ok(false) => Ok(false),
        Err(e) => {
            eprintln!("TOTP verification error: {}", e);
            Ok(false)
        }
    }
}

/// Generate a new TOTP secret (for initial setup)
pub fn generate_totp_secret() -> Result<String> {
    use rand::{RngCore, thread_rng};
    
    let mut bytes = [0u8; 32];
    thread_rng().fill_bytes(&mut bytes);
    
    let totp = TOTP::new(
        totp_rs::Algorithm::SHA1,
        6,
        1,
        30,
        bytes.to_vec(),
    )?;
    
    Ok(totp.get_secret())
}