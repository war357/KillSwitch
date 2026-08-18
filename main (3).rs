// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Remote Wipe Partition Tool
//!
//! Bootable 2FA-gated wipe tool for secure data destruction.
//! This runs from a dedicated partition and requires 2FA before wiping.

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use std::io::{self, Write};

mod totp;
mod wipe;
mod config;

/// Wipe Partition CLI
#[derive(Parser, Debug)]
#[command(author, version, about = "2FA-gated secure wipe tool", long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short, long, default_value = "/etc/remote-wipe/wipe.toml")]
    config: PathBuf,
    
    /// Skip 2FA verification (DANGEROUS - for testing only)
    #[arg(long)]
    skip_2fa: bool,
    
    /// Skip confirmation prompt (DANGEROUS - for automated use)
    #[arg(long)]
    no_confirm: bool,
    
    /// Wipe method (random, zero, dod5220)
    #[arg(short, long, default_value = "random")]
    method: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("=== Remote Wipe Partition Tool ===");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("⚠️  WARNING: This tool will IRREVERSIBLY DESTROY all data on selected disks.");
    println!();
    
    // Load configuration
    let cfg = config::load(&args.config)
        .unwrap_or_else(|_| config::default_config());
    
    // Verify 2FA
    if !args.skip_2fa {
        println!("🔐 2FA Verification Required");
        println!();
        
        if !totp::verify_2fa(&cfg)? {
            eprintln!("❌ 2FA verification failed. Aborting.");
            std::process::exit(1);
        }
        
        println!("✅ 2FA verified successfully");
        println!();
    } else {
        println!("⚠️  WARNING: 2FA verification SKIPPED (testing mode)");
        println!();
    }
    
    // Get disks to wipe
    let disks = &cfg.disks;
    if disks.is_empty() {
        eprintln!("❌ No disks configured for wipe");
        std::process::exit(1);
    }
    
    println!("📀 Disks to wipe:");
    for disk in disks {
        println!("   - /dev/{}", disk);
    }
    println!();
    println!("📊 Wipe method: {}", args.method);
    println!();
    
    // Confirm wipe
    if !args.no_confirm {
        print!("⚠️  Type 'WIPE' to confirm data destruction: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if input.trim() != "WIPE" {
            println!();
            println!("❌ Wipe aborted by user");
            return Ok(());
        }
    } else {
        println!("⚠️  WARNING: Confirmation prompt SKIPPED");
    }
    
    println!();
    println!("🚀 Starting wipe operation...");
    println!();
    
    // Execute wipe
    for disk in disks {
        let device_path = format!("/dev/{}", disk);
        println!("📝 Wiping {}...", device_path);
        
        match args.method.as_str() {
            "random" => wipe::wipe_with_random(&device_path, 1)?,
            "zero" => wipe::wipe_with_zeros(&device_path)?,
            "dod5220" => wipe::wipe_dod5220(&device_path)?,
            _ => {
                eprintln!("❌ Unknown wipe method: {}", args.method);
                std::process::exit(1);
            }
        }
        
        println!("✅ Wipe completed for {}", device_path);
    }
    
    println!();
    println!("✅ All disks wiped successfully");
    println!();
    println!("🔌 System will power off in 5 seconds...");
    
    std::thread::sleep(std::time::Duration::from_secs(5));
    
    // Power off
    println!("🔌 Powering off...");
    std::process::Command::new("poweroff")
        .status()
        .context("Failed to power off")?;
    
    Ok(())
}