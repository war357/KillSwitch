# Contributing to Remote Wipe

Thank you for considering contributing to Remote Wipe! This project aims to provide secure, auditable remote wipe capabilities for enterprise environments.

## Code of Conduct

This project adheres to the Contributor Covenant Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## Getting Started

### Prerequisites

- Rust 1.75 or later
- PostgreSQL 15+ (for server development)
- Docker (optional, for testing)

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/yourorg/remote-wipe.git
cd remote-wipe

# Build the project
cargo build

# Run tests
cargo test

# Run clippy for linting
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt --all
```

## Development Workflow

### 1. Fork and Branch

- Fork the repository
- Create a branch for your feature/fix:
  ```bash
  git checkout -b feature/your-feature-name
  ```

### 2. Make Changes

- Follow Rust idioms and best practices
- Add tests for new functionality
- Update documentation as needed
- Ensure all tests pass

### 3. Commit Messages

Use conventional commits:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `test:` Test additions/changes
- `refactor:` Code refactoring
- `chore:` Maintenance tasks

Example:
```bash
git commit -m "feat(agent): add support for selective disk wipe"
```

### 4. Pull Request

- Push your branch to your fork
- Create a Pull Request
- Fill out the PR template completely
- Request review from maintainers

## Code Style

### Rust Guidelines

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo clippy` and fix all warnings
- Use `cargo fmt` for consistent formatting
- Prefer `Result` over `panic!` in library code
- Handle errors gracefully

### Security Considerations

Given the destructive nature of this software:

1. **Never bypass safety checks** - All wipe operations require explicit confirmation
2. **Validate all inputs** - Especially disk identifiers and paths
3. **Use constant-time comparisons** - For sensitive data (tokens, secrets)
4. **Log security events** - Auth failures, policy violations, wipe attempts
5. **Test thoroughly** - Especially edge cases and error conditions

### Testing Requirements

- Unit tests for all public functions
- Integration tests for critical paths
- Document test coverage in PRs
- Test destructive operations in isolated environments only

## Documentation

### Code Documentation

- Document all public functions with `///` comments
- Include examples where helpful
- Explain security implications where relevant

Example:
```rust
/// Wipes the specified disk with random data.
///
/// # Safety
/// This function will irreversibly destroy all data on the target disk.
/// Ensure the disk identifier is correct before calling.
///
/// # Arguments
/// * `device_path` - Path to the block device (e.g., "/dev/sda")
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(e)` if wipe operation fails
pub fn wipe_disk(device_path: &str) -> Result<(), WipeError> {
    // ...
}
```

### User Documentation

- Update `docs/` for architectural changes
- Update `README.md` for user-facing changes
- Add migration guides for breaking changes

## Review Process

1. **Automated Checks**: CI must pass (tests, clippy, fmt)
2. **Code Review**: At least one maintainer approval required
3. **Security Review**: Security-sensitive changes require additional review
4. **Testing**: Destructive features require demonstration in test environment

## Release Process

Releases follow semantic versioning:
- **MAJOR**: Breaking changes
- **MINOR**: New features (backwards compatible)
- **PATCH**: Bug fixes (backwards compatible)

### Release Checklist

- [ ] All tests passing
- [ ] Changelog updated
- [ ] Documentation updated
- [ ] Security audit completed (for major releases)
- [ ] Release notes prepared
- [ ] Tags created and signed

## Questions?

- Open a GitHub Discussion for general questions
- Check existing issues and documentation first
- Be patient and respectful

## License

By contributing, you agree that your contributions will be licensed under the AGPL-3.0-or-later license.