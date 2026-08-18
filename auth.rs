// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Authentication and authorization

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use anyhow::Result;

/// Authentication state
pub struct AuthState {
    // TODO: Add JWT secret, TOTP secrets, etc.
}

/// Initialize authentication
pub async fn init() -> Result<()> {
    // TODO: Load secrets, configure JWT, etc.
    Ok(())
}

/// Authentication middleware
pub async fn auth_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // TODO: Implement proper authentication
    // For now, allow all requests (development mode)
    
    Ok(next.run(req).await)
}

/// Verify TOTP code
pub fn verify_totp(secret: &str, code: &str) -> bool {
    use totp_rs::TOTP;
    
    match TOTP::from_url(secret) {
        Ok(totp) => totp.check_current(code).unwrap_or(false),
        Err(_) => false,
    }
}

/// Generate TOTP secret
pub fn generate_totp_secret() -> String {
    use totp_rs::TOTP;
    use rand::rngs::OsRng;
    use rand::RngCore;
    
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    
    let totp = TOTP::new(
        totp_rs::Algorithm::SHA1,
        6,
        1,
        30,
        bytes.to_vec(),
    ).unwrap();
    
    totp.get_secret()
}