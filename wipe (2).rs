// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Disk wipe operations

use anyhow::{Context, Result, bail};
use std::process::Command;

/// Wipe disk with random data
pub fn wipe_with_random(device_path: &str, passes: u32) -> Result<()> {
    println!("   Wiping {} with random data ({} passes)...", device_path, passes);
    
    for pass in 1..=passes {
        println!("   Pass {}/{}", pass, passes);
        
        let status = Command::new("dd")
            .args([
                "if=/dev/urandom",
                &format!("of={}", device_path),
                "bs=4M",
                "status=progress",
            ])
            .status()
            .context("Failed to execute dd")?;
        
        if !status.success() {
            bail!("dd failed on pass {}/{}", pass, passes);
        }
    }
    
    Ok(())
}

/// Wipe disk with zeros
pub fn wipe_with_zeros(device_path: &str) -> Result<()> {
    println!("   Wiping {} with zeros...", device_path);
    
    let status = Command::new("dd")
        .args([
            "if=/dev/zero",
            &format!("of={}", device_path),
            "bs=4M",
            "status=progress",
        ])
        .status()
        .context("Failed to execute dd")?;
    
    if !status.success() {
        bail!("dd failed");
    }
    
    Ok(())
}

/// DoD 5220.22-M wipe (3 passes: zeros, ones, random)
pub fn wipe_dod5220(device_path: &str) -> Result<()> {
    println!("   Wiping {} with DoD 5220.22-M (3 passes)...", device_path);
    
    // Pass 1: Zeros
    println!("   Pass 1/3: Zeros");
    wipe_with_zeros(device_path)?;
    
    // Pass 2: Ones (0xFF)
    println!("   Pass 2/3: Ones (0xFF)");
    let status = Command::new("dd")
        .args([
            "if=/dev/zero",
            &format!("of={}", device_path),
            "bs=4M",
            "status=progress",
        ])
        .status()
        .context("Failed to execute dd")?;
    
    if !status.success() {
        bail!("dd failed on pass 2");
    }
    
    // Pass 3: Random
    println!("   Pass 3/3: Random");
    wipe_with_random(device_path, 1)?;
    
    Ok(())
}