# KillSwitch

**Open-source remote wipe and decommissioning platform for servers and workstations**

KillSwitch is an AGPL-licensed, enterprise-grade solution for secure data destruction, incident response, and device decommissioning. It provides cryptographically secure remote wipe capabilities with comprehensive audit logging and policy enforcement.

## ⚠️ Warning

This software performs **irreversible data destruction**. Use only in controlled environments with proper authorization and testing.

## Features

- 🔐 **Cryptographically secure** - mTLS communication, signed commands
- 🛡️ **Policy-driven** - RBAC, approval workflows, audit trails
- 🖥️ **Multi-platform** - Linux and Windows agents (Windows WIP)
- 🔄 **Flexible wipe methods** - Full disk, selective, secure erase
- 📊 **Comprehensive audit** - Immutable logging, SIEM integration
- 🔑 **2FA support** - TOTP, YubiKey, remote approval

## Architecture

[See docs/architecture.md for detailed architecture]

## Quick Start

### Prerequisites

- Rust 1.75+
- PostgreSQL 15+
- Docker (optional, for deployment)

### Build

```bash
git clone https://github.com/yourorg/killswitch.git
cd killswitch
cargo build --release
```

## Documentation

- [Architecture](docs/architecture.md)
- [Threat Model](docs/threat-model.md)
- [Deployment](docs/deployment.md)

## License

GNU Affero General Public License v3.0 (AGPL-3.0-or-later)

## Disclaimer

This software is provided "as is" without warranty. Users assume all risk of data loss.
