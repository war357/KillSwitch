# Architecture

## Overview

Remote Wipe consists of three main components:

1. **Agent** - Runs on endpoints, communicates with server
2. **Server** - Control plane API for device management and command dispatch
3. **Wipe Partition** - Bootable 2FA-gated wipe tool

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Enterprise Network                       │
│                                                              │
│  ┌──────────────┐                                           │
│  │   Admin      │  HTTPS/mTLS                               │
│  │   Console    │◄─────────────┐                            │
│  └──────────────┘              │                            │
│                                 ▼                            │
│                        ┌─────────────────┐                   │
│                        │  Remote Wipe    │                   │
│                        │     Server      │                   │
│                        │                 │                   │
│                        │ - REST API      │                   │
│                        │ - PostgreSQL    │                   │
│                        │ - Auth (TOTP)   │                   │
│                        │ - Policy Engine │                   │
│                        └────────┬────────┘                   │
│                                 │ mTLS                        │
│              ┌──────────────────┼──────────────────┐         │
│              │                  │                  │         │
│              ▼                  ▼                  ▼         │
│     ┌─────────────┐   ┌─────────────┐   ┌─────────────┐     │
│     │   Agent     │   │   Agent     │   │   Agent     │     │
│     │  (Server 1) │   │  (Server 2) │   │  (Desktop)  │     │
│     │             │   │             │   │             │     │
│     │ - mTLS      │   │ - mTLS      │   │ - mTLS      │     │
│     │ - Wipe      │   │ - Wipe      │   │ - Wipe      │     │
│     └─────┬───────┘   └─────┬───────┘   └─────┬───────┘     │
│           │                 │                 │               │
│           ▼                 ▼                 ▼               │
│     ┌─────────┐       ┌─────────┐       ┌─────────┐         │
│     │  Disks  │       │  Disks  │       │  Disks  │         │
│     └─────────┘       └─────────┘       └─────────┘         │
│                                                              │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│              Wipe Partition (Bootable USB)                   │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  1. Boot from USB/Partition                          │   │
│  │  2. Load configuration                               │   │
│  │  3. Verify 2FA (TOTP/YubiKey)                        │   │
│  │  4. Confirm wipe (interactive)                       │   │
│  │  5. Execute wipe (dd/secure erase)                   │   │
│  │  6. Power off                                        │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Component Details

### Agent

**Location**: `crates/agent/`

**Responsibilities**:
- Maintain persistent mTLS connection to server
- Receive and execute commands (wipe, reboot, config update)
- Report status and inventory
- Execute wipe operations using system tools (dd, nvme, hdparm)
- Log all operations to local audit log

**Key Files**:
- `main.rs` - Entry point, main loop
- `channel.rs` - mTLS communication
- `commands.rs` - Command handling
- `wipe.rs` - Disk wipe operations
- `config.rs` - Configuration management

**Security**:
- mTLS for all communication
- Device identity via UUID
- Local audit logging
- No command execution without server signature

### Server

**Location**: `crates/server/`

**Responsibilities**:
- REST API for device management
- Command queue and dispatcher
- User authentication (TOTP, RBAC)
- Policy enforcement
- Audit log storage
- Database management (PostgreSQL)

**Key Files**:
- `main.rs` - Axum server, routing
- `api.rs` - HTTP handlers
- `auth.rs` - Authentication middleware
- `db.rs` - Database connection
- `policy.rs` - Policy enforcement
- `dispatcher.rs` - Command queue

**API Endpoints**:
- `GET/POST /api/v1/devices` - Device management
- `POST /api/v1/devices/:id/wipe` - Request wipe
- `GET /api/v1/agent/commands` - Agent polling
- `POST /api/v1/agent/messages` - Agent status
- `GET /api/v1/audit` - Audit logs

**Security**:
- TOTP 2FA for admin users
- mTLS for agent communication
- RBAC for all operations
- Immutable audit logging

### Wipe Partition

**Location**: `crates/wipe-partition/`

**Responsibilities**:
- Bootable minimal Linux environment
- 2FA verification before wipe
- Interactive confirmation
- Execute wipe operations
- Power off after completion

**Key Files**:
- `main.rs` - Entry point, CLI
- `totp.rs` - TOTP verification
- `wipe.rs` - Wipe operations
- `config.rs` - Configuration

**Security**:
- Requires 2FA (TOTP) before wipe
- Interactive confirmation required
- No network access (air-gapped)
- Runs from read-only media

## Data Flow

### Wipe Command Flow

1. Admin logs into server console (TOTP 2FA)
2. Admin selects device(s) and requests wipe
3. Server validates policy and permissions
4. Server queues wipe command in database
5. Agent polls server for commands
6. Agent receives wipe command
7. Agent validates command signature
8. Agent executes wipe (dd/secure erase)
9. Agent reports completion to server
10. Server logs audit event

### Reboot-to-Wipe Flow

1. Admin requests reboot-to-wipe
2. Server queues command
3. Agent receives command
4. Agent sets wipe flag in `/boot/`
5. Agent reboots system
6. System boots to wipe partition
7. Wipe partition verifies 2FA
8. User confirms wipe
9. Wipe executes
10. System powers off

## Security Model

### Threat Model

See [threat-model.md](threat-model.md) for detailed analysis.

### Key Security Properties

1. **Confidentiality**
   - mTLS for all communication
   - Encrypted audit logs
   - No secrets in config files

2. **Integrity**
   - Signed commands
   - Immutable audit logs
   - Checksums on wipe operations

3. **Availability**
   - Redundant server deployment
   - Agent reconnection logic
   - Offline command queuing

4. **Auditability**
   - All operations logged
   - Immutable audit trail
   - SIEM integration

## Deployment

### Server Deployment

- Docker container or systemd service
- PostgreSQL database
- Reverse proxy (nginx/traefik)
- TLS certificates (Let's Encrypt or internal CA)

### Agent Deployment

- Systemd service on Linux endpoints
- Scheduled task on Windows (future)
- Pre-shared token or certificate for enrollment

### Wipe Partition

- Bootable USB or dedicated partition
- GRUB entry for dual-boot
- Physical security required

## Future Enhancements

- Windows agent support
- Hardware security module (HSM) integration
- YubiKey 2FA support
- Encrypted audit log export
- Automated compliance reporting
- Integration with MDM/ITSM systems