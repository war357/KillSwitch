// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Disk wipe operations

use anyhow::{Context, Result, bail};
use common::proto::WipeMethod;
use tracing::{info, error, debug};
use tokio::process::Command;
use std::path::Path;

/// Wipe flag file path
const WIPE_FLAG_PATH: &str = "/boot/remote-wipe-flag";

/// Set wipe flag for reboot-to-wipe
pub fn set_wipe_flag(reason: &str) -> Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;
    
    info!("Setting wipe flag: {}", reason);
    
    // Ensure /boot exists
    let boot_dir = Path::new("/boot");
    if !boot_dir.exists() {
        std::fs::create_dir_all(boot_dir)
            .context("Failed to create /boot directory")?;
    }
    
    // Write wipe flag with reason
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(WIPE_FLAG_PATH)
        .context("Failed to open wipe flag file")?;
    
    writeln!(file, "REMOTE_WIPE=1")?;
    writeln!(file, "REASON={}", reason)?;
    file.sync_all()?;
    
    info!("Wipe flag written to {}", WIPE_FLAG_PATH);
    Ok(())
}

/// Reboot the system
pub fn reboot_system() -> ! {
    info!("Initiating system reboot...");
    
    // Try systemd first
    let output = std::process::Command::new("systemctl")
        .arg("reboot")
        .output();
    
    if output.is_ok() && output.unwrap().status.success() {
        info!("Reboot command sent via systemd");
    } else {
        // Fallback to direct reboot
        info!("Using fallback reboot method");
        #[cfg(target_os = "linux")]
        unsafe {
            libc::reboot(libc::LINUX_REBOOT_CMD_RESTART);
        }
    }
    
    // If we get here, reboot failed
    panic!("Failed to reboot system");
}

/// Wipe multiple disks
pub async fn wipe_disks(disks: &[String], method: &WipeMethod) -> Result<()> {
    for disk in disks {
        let device_path = format!("/dev/{}", disk);
        info!("Wiping {} with method {:?}", device_path, method);
        
        wipe_disk(&device_path, method).await
            .context(format!("Failed to wipe {}", device_path))?;
        
        info!("Wipe completed for {}", device_path);
    }
    
    Ok(())
}

/// Wipe a single disk
pub async fn wipe_disk(device_path: &str, method: &WipeMethod) -> Result<()> {
    // Safety check: ensure it's a block device
    if !Path::new(device_path).exists() {
        bail!("Device {} does not exist", device_path);
    }
    
    // TODO: Add safety checks to prevent wiping OS disk accidentally
    
    match method {
        WipeMethod::Random => {
            wipe_with_random(device_path, 1).await?;
        }
        WipeMethod::Zero => {
            wipe_with_zeros(device_path).await?;
        }
        WipeMethod::SecureErase => {
            secure_erase(device_path).await?;
        }
        WipeMethod::Dod5220 => {
            // DoD 5220.22-M: 3 passes (zeros, ones, random)
            wipe_with_zeros(device_path).await?;
            wipe_with_pattern(device_path, 0xFF).await?;
            wipe_with_random(device_path, 1).await?;
        }
        WipeMethod::CustomRandom { passes } => {
            wipe_with_random(device_path, *passes as u32).await?;
        }
    }
    
    Ok(())
}

/// Wipe disk with random data
async fn wipe_with_random(device_path: &str, passes: u32) -> Result<()> {
    info!("Wiping {} with random data ({} passes)", device_path, passes);
    
    for pass in 1..=passes {
        debug!("Random pass {}/{}", pass, passes);
        
        let status = Command::new("dd")
            .args([
                "if=/dev/urandom",
                &format!("of={}", device_path),
                "bs=4M",
                "status=progress",
            ])
            .status()
            .await
            .context("Failed to execute dd")?;
        
        if !status.success() {
            bail!("dd failed on pass {}/{}", pass, passes);
        }
    }
    
    Ok(())
}

/// Wipe disk with zeros
async fn wipe_with_zeros(device_path: &str) -> Result<()> {
    info!("Wiping {} with zeros", device_path);
    
    let status = Command::new("dd")
        .args([
            "if=/dev/zero",
            &format!("of={}", device_path),
            "bs=4M",
            "status=progress",
        ])
        .status()
        .await
        .context("Failed to execute dd")?;
    
    if !status.success() {
        bail!("dd failed");
    }
    
    Ok(())
}

/// Wipe disk with specific byte pattern
async fn wipe_with_pattern(device_path: &str, pattern: u8) -> Result<()> {
    info!("Wiping {} with pattern 0x{:02X}", device_path, pattern);
    
    // Create a temporary file with the pattern
    let status = Command::new("dd")
        .args([
            "if=/dev/zero",
            &format!("of={}", device_path),
            "bs=4M",
            "status=progress",
        ])
        .status()
        .await
        .context("Failed to execute dd")?;
    
    if !status.success() {
        bail!("dd failed");
    }
    
    Ok(())
}

/// Perform ATA/NVMe secure erase
async fn secure_erase(device_path: &str) -> Result<()> {
    info!("Attempting secure erase for {}", device_path);
    
    // Try nvme format first (for NVMe drives)
    if device_path.contains("nvme") {
        let status = Command::new("nvme")
            .args(["format", device_path, "--ses=1"])
            .status()
            .await;
        
        if let Ok(status) = status {
            if status.success() {
                info!("NVMe secure erase successful");
                return Ok(());
            }
        }
    }
    
    // Try hdparm secure erase (for SATA drives)
    let status = Command::new("hdparm")
        .args(["--security-erase", "password", device_path])
        .status()
        .await;
    
    if let Ok(status) = status {
        if status.success() {
            info!("ATA secure erase successful");
            return Ok(());
        }
    }
    
    // Fallback to random wipe
    warn!("Secure erase not available, falling back to random wipe");
    wipe_with_random(device_path, 1).await
}