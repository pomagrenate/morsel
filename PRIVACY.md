# Privacy & Security Architecture

## Overview

Morsel is designed as a privacy-first clipboard manager with the following core principles:

1. **Local-Only Storage**: All clipboard data is stored locally on the user's machine by default
2. **No Account Required**: No cloud account or authentication is needed
3. **No Telemetry**: No analytics or telemetry data is collected
4. **No Mandatory Network**: The application functions entirely without network access
5. **Open Source**: All code is open source for transparency and auditability

## Data Storage

### Local Database
- **Location**: `~/.local/share/morsel/morsel.db` (configurable)
- **Format**: SQLite database
- **Encryption**: Optional (user-controlled)
- **Access**: Restricted to the current user via filesystem permissions

### Clipboard Content
- **Text Content**: Stored as plain text in the database
- **Binary Content**: Stored as blobs with deduplication via content hashing
- **Metadata**: Created timestamps, content types, tags, favorites status

### Data Retention
- **Default**: Items are retained indefinitely unless manually deleted
- **Configurable**: Users can set automatic expiration policies
- **Max Items**: Configurable limit (default: 1000 items)

## Network Behavior

### No Network Access
Morsel does not require or initiate any network connections for core functionality:

- **No Cloud Sync**: No synchronization with external servers
- **No Telemetry**: No usage statistics or crash reports sent
- **No Updates**: No automatic update checks or downloads
- **No Authentication**: No account creation or login processes

### Optional Network Features
The following features may involve network access but are entirely optional:

- **Clipboard Monitoring**: Uses local OS APIs only
- **Hotkey Registration**: Uses local OS APIs only
- **TUI Rendering**: Purely local rendering

### Third-Party Dependencies
Morsel uses the following third-party crates, none of which perform network operations:

- `tokio`: Async runtime (local only)
- `rusqlite`: SQLite database (local only)
- `ratatui`: Terminal UI (local only)
- `crossterm`: Terminal handling (local only)
- `regex`: Pattern matching (local only)
- `chrono`: Date/time handling (local only)
- `uuid`: UUID generation (local only)
- `serde`: Serialization (local only)

## Sensitive Content Detection

### Heuristic Approach
Morsel implements heuristic detection for potentially sensitive clipboard content:

- **API Keys**: Pattern matching for common API key formats
- **Access Tokens**: Detection of bearer tokens and OAuth tokens
- **JWTs**: JSON Web Token format detection
- **Private Keys**: Detection of PEM-encoded private keys
- **Secrets**: Common secret format patterns

### Limitations
- **Not a Security Guarantee**: Detection is heuristic-based and may produce false positives/negatives
- **User Control**: Users can disable detection or configure rules
- **Local Processing**: All detection happens locally, no data is sent externally

### Configuration
Users can configure:
- Enable/disable sensitive content detection
- Custom detection patterns
- False-positive handling
- Automatic deletion of detected sensitive content

## Encryption (Future)

### Planned Features
- **Optional Encryption**: User-controlled encryption of clipboard content
- **Key Management**: Secure key storage using OS keychain
- **Key Rotation**: Support for periodic key rotation
- **Encrypted Backups**: Optional encrypted backup exports

### Cryptographic Primitives
- **Algorithm**: AES-256-GCM for content encryption
- **Key Derivation**: Argon2id for key derivation from passphrases
- **Key Storage**: OS-provided keychain (Windows Credential Manager, macOS Keychain, Linux Secret Service)

## Threat Model

### Assumed Threats
1. **Local Access**: Malicious software with local file access
2. **Physical Access**: Unauthorized physical access to the machine
3. **OS Compromise**: Compromised operating system

### Mitigations
1. **Filesystem Permissions**: Database files restricted to current user
2. **Optional Encryption**: User-controlled encryption for sensitive content
3. **Memory Protection**: Clipboard content cleared from memory when not needed
4. **Secure Defaults**: Privacy-preserving defaults for all features

### Out of Scope
The following are explicitly out of scope for Morsel's threat model:

- **Cloud Attacks**: No cloud infrastructure to attack
- **Network Attacks**: No network surface to exploit
- **Supply Chain Attacks**: Mitigated through open source transparency and reproducible builds

## Privacy Configuration

### Default Settings
- **Local Storage Only**: Enabled by default
- **No Telemetry**: Disabled by default (not implemented)
- **No Network**: Disabled by default (not implemented)
- **Sensitive Detection**: Enabled by default (heuristic only)

### User Configurable Options
- Database location
- Maximum item retention
- Automatic expiration policies
- Sensitive content detection rules
- Encryption settings (future)

## Compliance

### Data Protection
Morsel is designed to comply with privacy principles:

- **Data Minimization**: Only stores clipboard data the user explicitly copies
- **User Control**: Users have full control over their clipboard history
- **Transparency**: Open source code for full transparency
- **Local Processing**: All processing happens locally

### No PII Collection
Morsel does not collect or store personally identifiable information:

- No account information
- No usage statistics
- No device identifiers
- No location data

## Auditing

### Code Review
All code is open source and available for security auditing at:
- Repository: [GitHub URL]
- License: [License]

### Security Reporting
Security vulnerabilities can be reported via:
- Private disclosure: [Security Email]
- Public issue tracker: [Issues URL]

## Best Practices

### For Users
1. **Enable Encryption**: Use encryption when storing sensitive clipboard content
2. **Regular Cleanup**: Periodically review and delete old clipboard items
3. **Secure Machine**: Ensure your OS is up to date and secure
4. **Strong Passphrases**: Use strong passphrases if enabling encryption

### For Developers
1. **No Network**: Avoid adding network dependencies unless absolutely necessary
2. **Local Processing**: Ensure all processing happens locally
3. **Privacy by Design**: Consider privacy implications for all new features
4. **Transparency**: Document all data flows and storage mechanisms

## Version History

- **v0.1.0**: Initial privacy architecture
  - Local-only storage
  - No telemetry
  - No network access
  - Heuristic sensitive content detection
