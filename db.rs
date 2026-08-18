// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Database initialization and connection pool

use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::info;

/// Global database pool
static mut POOL: Option<PgPool> = None;

/// Initialize database connection pool
pub async fn init(database_url: &str) -> Result<()> {
    info!("Connecting to database...");
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(database_url)
        .await?;
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    
    unsafe {
        POOL = Some(pool);
    }
    
    Ok(())
}

/// Get database pool
pub fn get_pool() -> &'static PgPool {
    unsafe {
        POOL.as_ref().expect("Database not initialized")
    }
}