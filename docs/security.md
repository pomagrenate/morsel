# Security Documentation

## Overview

Morsel is designed with security in mind. This document outlines security considerations, best practices, and known limitations.

## Threat Model

Morsel is designed for single-user, local-only use. The threat model assumes:

- The user has physical access to the machine
- The user's account is not compromised
- The machine is not infected with malware
- No remote access to the daemon

## Security Features

### Local-Only Operation

- All data stored locally
- No network communication
- Localhost-only IPC binding
- No cloud sync or telemetry

### File Permissions

Database and configuration files use user-only permissions:

```bash
~/.config/morsel/config.toml      # 600 (user read/write only)
~/.local/share/morsel/morsel.db   # 600 (user read/write only)
```

### Sensitive Content Detection

Detects and can handle sensitive content:
- API keys
- Passwords
- Tokens
- Private keys
- Secrets

## Security Best Practices

### 1. Secure Your System

Morsel's security depends on your system's security:

- Keep your OS updated
- Use strong passwords
- Enable disk encryption (FileVault, BitLocker, LUKS)
- Use a firewall
- Run antivirus/antimalware

### 2. Secure Your Database

Set appropriate file permissions:

```bash
chmod 600 ~/.local/share/morsel/morsel.db
chmod 600 ~/.config/morsel/config.toml
```

### 3. Enable Sensitive Detection

```toml
[privacy]
enable_sensitive_detection = true

[sensitive_detection]
confidence_threshold = 0.9
```

### 4. Regular Cleanup

Remove old items regularly:

```bash
morsel cleanup --older-than 30d
```

### 5. Secure Backups

Encrypt backup files:

```bash
gpg --encrypt --recipient your@email.com backup.json
```

### 6. Review History

Regularly review your clipboard history:

```bash
morsel list --limit 100
```

### 7. Use Short Retention

Configure short retention periods:

```toml
[retention]
max_items = 1000
max_age_days = 7
```

## Known Limitations

### No Database Encryption

The SQLite database is not encrypted by default. If your machine is compromised, clipboard history could be accessed.

**Mitigation:**
- Use full-disk encryption
- Store database on encrypted volume
- Use short retention periods

### No IPC Authentication

IPC communication does not use authentication. Any local user can connect to the daemon.

**Mitigation:**
- Use Unix domain sockets (Linux/macOS)
- Restrict file permissions
- Run daemon only when needed

### No Audit Logging

Morsel does not log access to clipboard history.

**Mitigation:**
- Monitor system logs
- Use OS-level auditing

### No Secure Delete

Deleted items are not securely overwritten.

**Mitigation:**
- Use full-disk encryption
- Use secure delete tools for database file

## Vulnerability Reporting

If you discover a security vulnerability, please report it privately:

**Email:** security@morsel.dev

Please include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

We will respond within 48 hours and provide a timeline for the fix.

## Security Updates

### Keeping Morsel Updated

```bash
cargo install morsel
```

Or from source:

```bash
git pull
cargo install --path .
```

### Update Notifications

Subscribe to releases for security updates:
- GitHub releases: https://github.com/yourusername/morsel/releases
- RSS feed: https://github.com/yourusername/morsel/releases.atom

## Security Checklist

Use this checklist to secure your Morsel installation:

- [ ] System is updated and secured
- [ ] Full-disk encryption enabled
- [ ] Database file permissions set to 600
- [ ] Config file permissions set to 600
- [ ] Sensitive detection enabled
- [ ] Retention policy configured
- [ ] Regular cleanup scheduled
- [ ] Backups encrypted
- [ ] History reviewed regularly
- [ ] Morsel updated to latest version

## Corporate/Enterprise Security

For corporate environments:

### 1. Policy Compliance

Ensure Morsel usage complies with:
- Data retention policies
- Data classification policies
- Security policies

### 2. Configuration

Configure for corporate use:

```toml
[retention]
max_items = 500
max_age_days = 30

[privacy]
enable_sensitive_detection = true

[sensitive_detection]
confidence_threshold = 0.95
custom_patterns = [
    "COMPANY_TOKEN_[A-Z0-9]+",
    "INTERNAL_KEY_[a-f0-9]{32}"
]
```

### 3. Monitoring

Monitor for:
- Database size growth
- Sensitive content detection
- Unusual access patterns

### 4. Auditing

Regular audits:
- Review clipboard history
- Check for sensitive content
- Verify retention compliance

## Security by Design Principles

Morsel follows these security principles:

1. **Local-Only** - No cloud, no network
2. **User Control** - User controls all data
3. **Transparency** - Open source, auditable
4. **Minimalism** - Minimal attack surface
5. **Privacy** - Privacy by default

## Third-Party Dependencies

Morsel uses the following third-party dependencies:

- **rusqlite** - SQLite bindings
- **tokio** - Async runtime
- **serde** - Serialization
- **regex** - Pattern matching
- **uuid** - Unique identifiers

All dependencies are audited for security vulnerabilities. Use `cargo audit` to check for known vulnerabilities:

```bash
cargo install cargo-audit
cargo audit
```

## Secure Development Practices

### Code Review

All code changes are reviewed for security implications.

### Dependency Updates

Dependencies are updated regularly for security patches.

### Static Analysis

Static analysis tools are used:
- `cargo clippy` - Linting
- `cargo audit` - Vulnerability scanning

### Testing

Security-focused tests:
- Input validation
- Path traversal
- SQL injection
- Serialization/deserialization

## Incident Response

If you suspect a security incident:

1. **Stop the daemon**
   ```bash
   morseld stop
   ```

2. **Secure the database**
   ```bash
   chmod 600 ~/.local/share/morsel/morsel.db
   ```

3. **Review history**
   ```bash
   morsel list --limit 1000
   ```

4. **Clear if necessary**
   ```bash
   morsel clear
   ```

5. **Report**
   Email: security@morsel.dev

## FAQ

### Is Morsel secure?

Morsel is designed for local-only use and follows security best practices. However, security depends on your system's security.

### Can Morsel be hacked remotely?

No. Morsel does not listen on network interfaces and has no remote access capabilities.

### What if my computer is stolen?

If you have full-disk encryption enabled, your clipboard history is protected. Without encryption, the database file could be accessed.

### Does Morsel send my clipboard to the cloud?

No. All data is stored locally.

### Can other users on my machine access my history?

By default, no. The database file is only accessible to your user account. Ensure proper file permissions.

### Is Morsel suitable for storing passwords?

Morsel is not a password manager. While it can detect sensitive content, it's not designed for secure password storage. Use a dedicated password manager.

### Can I use Morsel in a corporate environment?

Yes, but ensure it complies with your organization's security policies. Configure appropriate retention and sensitive detection settings.

## Resources

- [OWASP Secure Coding Practices](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)
- [Rust Security Guidelines](https://doc.rust-lang.org/nomicon/security.html)
- [SQLite Security](https://www.sqlite.org/security.html)
