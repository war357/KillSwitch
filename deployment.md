# Deployment Guide

## Prerequisites

- Rust 1.75+
- PostgreSQL 15+
- Docker (optional)
- Linux endpoints (for agent)

## Server Deployment

### Option 1: Docker Compose

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_USER: remote_wipe
      POSTGRES_PASSWORD: changeme
      POSTGRES_DB: remote_wipe
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U remote_wipe"]
      interval: 10s
      timeout: 5s
      retries: 5

  remote-wipe-server:
    image: remote-wipe-server:latest
    build:
      context: .
      dockerfile: deploy/docker/Dockerfile.server
    environment:
      DATABASE_URL: postgres://remote_wipe:changeme@postgres/remote_wipe
      RUST_LOG: info
    ports:
      - "8080:8080"
    depends_on:
      postgres:
        condition: service_healthy
    volumes:
      - audit_logs:/var/log/remote-wipe

volumes:
  postgres_data:
  audit_logs:
```

Deploy:

```bash
docker-compose up -d
```

### Option 2: Systemd Service

1. **Build server**:

```bash
cargo build --release -p server
```

2. **Create systemd service** (`/etc/systemd/system/remote-wipe-server.service`):

```ini
[Unit]
Description=Remote Wipe Server
After=network.target postgresql.service

[Service]
Type=simple
User=remote-wipe
Group=remote-wipe
ExecStart=/opt/remote-wipe/remote-wipe-server \
  --database-url "postgres://remote_wipe:password@localhost/remote_wipe" \
  --bind "0.0.0.0:8080"
Restart=on-failure
RestartSec=10s

[Install]
WantedBy=multi-user.target
```

3. **Enable and start**:

```bash
sudo systemctl daemon-reload
sudo systemctl enable remote-wipe-server
sudo systemctl start remote-wipe-server
```

### Option 3: Kubernetes

See `deploy/k8s/` for Kubernetes manifests.

## Agent Deployment

### Linux Agent

1. **Build agent**:

```bash
cargo build --release -p agent
```

2. **Create configuration** (`/etc/remote-wipe/agent.toml`):

```toml
server_url = "https://your-server:8080"
poll_interval_secs = 30
verbose = false
allowed_methods = ["random", "zero", "secure_erase"]
```

3. **Install binary**:

```bash
sudo cp target/release/remote-wipe-agent /opt/remote-wipe/
sudo chmod +x /opt/remote-wipe/remote-wipe-agent
```

4. **Create systemd service** (`/etc/systemd/system/remote-wipe-agent.service`):

```ini
[Unit]
Description=Remote Wipe Agent
After=network.target

[Service]
Type=simple
User=root
ExecStart=/opt/remote-wipe/remote-wipe-agent --config /etc/remote-wipe/agent.toml
Restart=on-failure
RestartSec=30s

[Install]
WantedBy=multi-user.target
```

5. **Enable and start**:

```bash
sudo systemctl daemon-reload
sudo systemctl enable remote-wipe-agent
sudo systemctl start remote-wipe-agent
```

### Enrollment

1. **Get enrollment token** from server admin
2. **Agent auto-enrolls** on first connection
3. **Verify device** in admin console

## Wipe Partition Deployment

### Create Bootable USB

1. **Build wipe partition tool**:

```bash
cargo build --release -p wipe-partition
```

2. **Create minimal Linux environment**:

```bash
# Use Alpine or custom initramfs
# Copy remote-wipe-partition binary
# Configure to run on boot
```

3. **Write to USB**:

```bash
dd if=remote-wipe.iso of=/dev/sdX bs=4M status=progress
```

### GRUB Entry (for dedicated partition)

Add to `/etc/grub.d/40_remote_wipe`:

```bash
menuentry "Remote Wipe" {
    set root=(hd0,3)
    linux /boot/vmlinuz quiet
    initrd /boot/initrd.img
}
```

Update GRUB:

```bash
sudo update-grub
```

## Configuration

### Server Configuration

Environment variables:

- `DATABASE_URL` - PostgreSQL connection string
- `RUST_LOG` - Log level (info, debug, warn, error)
- `BIND` - Server bind address (default: 0.0.0.0:8080)

### Agent Configuration

See `agent.toml` example above.

Key settings:

- `server_url` - Server endpoint
- `poll_interval_secs` - Command polling interval
- `allowed_methods` - Permitted wipe methods

### Wipe Partition Configuration

Create `/etc/remote-wipe/wipe.toml`:

```toml
disks = ["sda", "nvme0n1"]
require_2fa = true
power_off_after = true
default_method = "random"

# TOTP secret (base64 encoded)
totp_secret = "JBSWY3DPEHPK3PXP"
```

## Security Hardening

### Server

1. **TLS Termination**:
   - Use reverse proxy (nginx, traefik)
   - Obtain TLS certificate (Let's Encrypt)
   - Enforce HTTPS

2. **Firewall Rules**:
   ```bash
   # Allow only required ports
   sudo ufw allow 443/tcp  # HTTPS
   sudo ufw allow 8080/tcp # API (internal only)
   ```

3. **Database Security**:
   - Use strong password
   - Restrict network access
   - Enable SSL connections

### Agent

1. **File Permissions**:
   ```bash
   sudo chown root:root /opt/remote-wipe/remote-wipe-agent
   sudo chmod 700 /opt/remote-wipe/remote-wipe-agent
   sudo chmod 600 /etc/remote-wipe/agent.toml
   ```

2. **Network Restrictions**:
   - Firewall to allow only server IP
   - No inbound connections

3. **Audit Logging**:
   - Configure log rotation
   - Export logs to SIEM

## Monitoring

### Health Checks

- Server: `GET /health`
- Agent: Check systemd service status
- Database: PostgreSQL health checks

### Metrics

- Command queue length
- Wipe success/failure rates
- Agent connectivity
- Audit log volume

### Alerting

Configure alerts for:

- Failed wipe operations
- Unusual wipe patterns
- Agent disconnections
- Database errors

## Troubleshooting

### Agent Not Connecting

1. Check network connectivity
2. Verify server URL in config
3. Check TLS certificates
4. Review agent logs: `journalctl -u remote-wipe-agent`

### Wipe Fails

1. Check disk identifiers
2. Verify disk is not mounted
3. Review wipe logs
4. Try alternative wipe method

### Database Errors

1. Check PostgreSQL status
2. Verify connection string
3. Check disk space
4. Review server logs

## Backup and Recovery

### Database Backup

```bash
pg_dump -U remote_wipe remote_wipe > backup.sql
```

### Restore

```bash
psql -U remote_wipe remote_wipe < backup.sql
```

### Disaster Recovery

1. Restore database from backup
2. Redeploy server
3. Re-enroll agents
4. Verify connectivity

## Upgrades

### Server Upgrade

1. Backup database
2. Stop server
3. Deploy new version
4. Start server
5. Verify health

### Agent Upgrade

1. Deploy new binary
2. Restart service
3. Verify connectivity

## Support

For issues:

1. Check documentation
2. Review logs
3. Search GitHub issues
4. Create new issue with details