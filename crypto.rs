// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Remote Wipe Contributors

//! Cryptographic utilities for secure communication

use thiserror::Error;

/// Crypto error types
#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Invalid key: {0}")]
    InvalidKey(String),
    
    #[error("Signature verification failed")]
    SignatureVerificationFailed,
    
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    
    #[error("Decryption error: {0}")]
    DecryptionError(String),
}

/// Generate a random secure token
pub fn generate_secure_token(length: usize) -> String {
    use rand::{RngCore, thread_rng};
    
    let mut bytes = vec![0u8; length];
    thread_rng().fill_bytes(&mut bytes);
    base64_encode(&bytes)
}

/// Simple base64 encoding (for tokens)
fn base64_encode(bytes: &[u8]) -> String {
    use std::fmt::Write;
    
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    let mut result = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
        let b2 = chunk.get(2).copied().unwrap_or(0) as usize;
        
        let _ = write!(result, "{}", ALPHABET[b0 >> 2] as char);
        let _ = write!(result, "{}", ALPHABET[((b0 & 0x03) << 4) | (b1 >> 4)] as char);
        
        if chunk.len() > 1 {
            let _ = write!(result, "{}", ALPHABET[((b1 & 0x0f) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            let _ = write!(result, "{}", ALPHABET[b2 & 0x3f] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}

/// Verify a message signature (placeholder - use proper crypto in production)
pub fn verify_signature(
    _message: &[u8],
    _signature: &[u8],
    _public_key: &[u8],
) -> Result<bool, CryptoError> {
    // TODO: Implement proper signature verification using ring or similar
    // This is a placeholder for the initial implementation
    Ok(true)
}

/// Sign a message (placeholder - use proper crypto in production)
pub fn sign_message(
    _message: &[u8],
    _private_key: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    // TODO: Implement proper message signing
    Ok(vec![])
}

/// Constant-time byte comparison
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_token() {
        let token1 = generate_secure_token(32);
        let token2 = generate_secure_token(32);
        
        assert_eq!(token1.len(), 44); // Base64 encoded 32 bytes
        assert_ne!(token1, token2); // Should be unique
    }
    
    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq(b"hello", b"hello"));
        assert!(!constant_time_eq(b"hello", b"world"));
        assert!(!constant_time_eq(b"hello", b"hello!"));
    }
}