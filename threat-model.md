# Threat Model

## Overview

This document analyzes potential threats to Remote Wipe and describes mitigations implemented in the system.

## Assets to Protect

1. **Data on endpoint devices** - Primary asset to be destroyed on demand
2. **Audit logs** - Evidence of wipe operations
3. **Server infrastructure** - Control plane for wipe operations
4. **Agent credentials** - Device identity and authentication
5. **Admin credentials** - Access to issue wipe commands

## Threat Actors

1. **External Attackers** - No authorized access, attempting to compromise systems
2. **Malicious Insiders** - Authorized users attempting unauthorized wipes
3. **Compromised Admins** - Admin credentials stolen or misused
4. **Network Attackers** - MITM, replay, or interception attacks

## Threat Analysis

### T1: Unauthorized Wipe Command

**Description**: Attacker issues wipe command without authorization.

**Attack Vectors**:
- Compromised admin credentials
- API vulnerability
- Command injection

**Impact**: CRITICAL - Data loss

**Mitigations**:
- ✅ TOTP 2FA for all admin actions
- ✅ RBAC and least privilege
- ✅ Dual approval for bulk wipes (policy)
- ✅ Audit logging of all wipe requests
- ✅ Rate limiting on wipe commands

**Residual Risk**: LOW - Requires compromise of multiple factors

---

### T2: Command Interception/Modification

**Description**: Attacker intercepts or modifies wipe commands in transit.

**Attack Vectors**:
- MITM attack on agent-server communication
- DNS spoofing
- TLS vulnerability

**Impact**: CRITICAL - Data loss or wipe prevention

**Mitigations**:
- ✅ mTLS for all agent-server communication
- ✅ Certificate pinning
- ✅ Command signatures
- ✅ Replay protection (timestamps, nonces)

**Residual Risk**: VERY LOW - Requires breaking TLS

---

### T3: Agent Impersonation

**Description**: Attacker impersonates a legitimate agent to receive wipe commands.

**Attack Vectors**:
- Stolen device credentials
- Cloned agent binary
- Compromised endpoint

**Impact**: HIGH - Targeted data loss

**Mitigations**:
- ✅ Device-specific credentials (UUID + certificates)
- ✅ mTLS client certificates
- ✅ Inventory verification
- ✅ Anomaly detection (future)

**Residual Risk**: LOW - Requires credential theft

---

### T4: Audit Log Tampering

**Description**: Attacker modifies or deletes audit logs to hide unauthorized wipes.

**Attack Vectors**:
- Direct database access
- Log file modification
- Backup deletion

**Impact**: HIGH - Loss of accountability

**Mitigations**:
- ✅ Immutable audit log storage (append-only)
- ✅ External SIEM integration
- ✅ Write-once storage for logs
- ✅ Regular log exports

**Residual Risk**: MEDIUM - Requires privileged access

---

### T5: Denial of Service

**Description**: Attacker prevents wipe commands from being executed.

**Attack Vectors**:
- DDoS on server
- Network partition
- Agent crash

**Impact**: MEDIUM - Delayed incident response

**Mitigations**:
- ✅ Agent reconnection logic
- ✅ Command queuing (offline support)
- ✅ Redundant server deployment (future)
- ✅ Health monitoring

**Residual Risk**: MEDIUM - Network-dependent

---

### T6: Malicious Insider

**Description**: Authorized user issues unauthorized wipes.

**Attack Vectors**:
- Legitimate admin credentials
- Policy bypass
- Bulk wipe abuse

**Impact**: CRITICAL - Data loss

**Mitigations**:
- ✅ Dual approval for bulk wipes
- ✅ Comprehensive audit logging
- ✅ Alerting on unusual patterns
- ✅ Role separation (requester vs approver)
- ✅ HR/ITSM integration (future)

**Residual Risk**: MEDIUM - Requires trusted insider

---

### T7: Physical Access Attack

**Description**: Attacker with physical access bypasses wipe protections.

**Attack Vectors**:
- Boot from external media
- Disk removal
- Hardware tampering

**Impact**: HIGH - Data theft or destruction

**Mitigations**:
- ✅ Wipe partition requires 2FA
- ✅ BIOS/UEFI passwords (recommended)
- ✅ Secure Boot (recommended)
- ✅ Physical security policies

**Residual Risk**: MEDIUM - Physical security dependent

---

### T8: Supply Chain Attack

**Description**: Compromised dependencies or build process.

**Attack Vectors**:
- Malicious crate dependency
- Compromised CI/CD
- Binary replacement

**Impact**: CRITICAL - Backdoor in all deployments

**Mitigations**:
- ✅ AGPL license (source disclosure)
- ✅ Reproducible builds (future)
- ✅ Dependency auditing (cargo-audit)
- ✅ Signed releases (future)
- ✅ Minimal dependencies

**Residual Risk**: MEDIUM - Industry-wide challenge

---

## Security Boundaries

```
┌─────────────────────────────────────────────────────────┐
│                  Untrusted Network                       │
│  ┌─────────────────────────────────────────────────┐    │
│  │              External Attackers                  │    │
│  └─────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
                          │
                          │ mTLS + Auth
                          ▼
┌─────────────────────────────────────────────────────────┐
│               Trusted Network (Enterprise)               │
│                                                          │
│  ┌──────────────┐         ┌──────────────┐             │
│  │    Server    │◄───────►│  PostgreSQL  │             │
│  │   (API)      │         │   Database   │             │
│  └──────┬───────┘         └──────────────┘             │
│         │ mTLS                                         │
│         ▼                                              │
│  ┌──────────────┐                                     │
│  │    Agent     │                                     │
│  │  (Endpoint)  │                                     │
│  └──────┬───────┘                                     │
│         │                                              │
│         ▼                                              │
│  ┌──────────────┐                                     │
│  │  Disk Data   │  ← Primary Asset                    │
│  └──────────────┘                                     │
│                                                          │
│  ┌──────────────┐                                     │
│  │   Wipe       │  ← Air-gapped                        │
│  │  Partition   │                                     │
│  └──────────────┘                                     │
└─────────────────────────────────────────────────────────┘
```

## Security Recommendations

### For Operators

1. **Network Security**
   - Deploy server in isolated network segment
   - Use firewall rules to restrict agent access
   - Monitor network traffic for anomalies

2. **Access Control**
   - Enforce MFA for all admin accounts
   - Implement least-privilege RBAC
   - Regular access reviews

3. **Monitoring**
   - Enable comprehensive audit logging
   - Export logs to external SIEM
   - Alert on unusual wipe patterns

4. **Physical Security**
   - Secure server infrastructure
   - Control physical access to endpoints
   - Use BIOS/UEFI passwords

5. **Incident Response**
   - Document wipe procedures
   - Test wipe functionality regularly
   - Maintain offline backups of critical data

### For Developers

1. **Code Security**
   - Follow Rust security best practices
   - Use constant-time comparisons for secrets
   - Validate all inputs

2. **Dependency Management**
   - Regular `cargo audit`
   - Minimize dependencies
   - Pin dependency versions

3. **Testing**
   - Security-focused code review
   - Fuzz testing for parsers
   - Penetration testing before releases

## Compliance Considerations

Remote Wipe can support compliance with:

- **GDPR** - Right to erasure, data minimization
- **HIPAA** - PHI destruction
- **PCI DSS** - Cardholder data destruction
- **SOC 2** - Security controls, audit trails
- **ISO 27001** - Information security management

Organizations should validate specific compliance requirements with their auditors.

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2026-08-18 | Initial threat model |