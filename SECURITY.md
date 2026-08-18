# Security Policy

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security issue, please report it responsibly.

### How to Report

**DO NOT** create a public GitHub issue for security vulnerabilities.

Instead, please:

1. **Email**: security@yourorg.org (replace with actual contact)
2. **Include**:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### Response Timeline

- **Initial response**: Within 48 hours
- **Status update**: Within 5 business days
- **Resolution target**: Based on severity (Critical: 7 days, High: 30 days, Medium: 90 days)

### Embargo Policy

- We request a 90-day embargo from disclosure after we acknowledge the issue
- We will coordinate public disclosure with you
- CVE assignment will be coordinated if appropriate

### Security Updates

Security patches will be released as:
- Patch releases for critical/high severity issues
- Documented in release notes with appropriate detail
- Backported to supported versions

## Security Best Practices

### For Users

1. **Always use mTLS** between agent and server
2. **Enable MFA** for all admin accounts
3. **Restrict network access** to the server API
4. **Monitor audit logs** continuously
5. **Test wipe policies** in non-production first
6. **Keep systems updated** with security patches

### For Developers

1. **Never commit secrets** to the repository
2. **Use constant-time comparisons** for sensitive data
3. **Validate all inputs** from untrusted sources
4. **Log security events** (auth failures, policy violations)
5. **Follow Rust security guidelines** (memory safety, etc.)

## Threat Model

See [docs/threat-model.md](docs/threat-model.md) for detailed threat analysis and mitigations.

## Audit and Compliance

All wipe operations are logged with:
- Timestamp (UTC)
- User/device identity
- Action performed
- Target devices
- Outcome

Logs should be exported to immutable storage and monitored for anomalies.

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

Always use the latest stable version for security updates.