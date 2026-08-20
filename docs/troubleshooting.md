# Troubleshooting Guide

## Common Issues

### Daemon Won't Start

**Symptom:** `morseld start` fails or daemon doesn't stay running.

**Solutions:**

1. Check if daemon is already running:
   ```bash
   morsel status
   ```

2. Check logs:
   ```bash
   cat ~/.local/share/morsel/morseld.log
   ```

3. Check port availability:
   ```bash
   # Linux/macOS
   lsof -i :54321
   
   # Windows
   netstat -ano | findstr :54321
   ```

4. Change IPC port in config:
   ```toml
   [daemon]
   ipc_address = "127.0.0.1:54322"
   ```

5. Check configuration:
   ```bash
   morsel config show
   ```

### CLI Can't Connect to Daemon

**Symptom:** CLI commands fail with "daemon not running" error.

**Solutions:**

1. Verify daemon is running:
   ```bash
   morsel status
   ```

2. Start daemon:
   ```bash
   morseld start
   ```

3. Check IPC address:
   ```bash
   morsel config get daemon.ipc_address
   ```

4. Verify firewall isn't blocking localhost connections

### Clipboard Not Being Captured

**Symptom:** Clipboard history is empty or not updating.

**Solutions:**

1. Check daemon is running:
   ```bash
   morsel status
   ```

2. Check clipboard monitoring is enabled:
   ```bash
   morsel config get clipboard.poll_interval_ms
   ```

3. Increase polling interval:
   ```toml
   [clipboard]
   poll_interval_ms = 50
   ```

4. Check platform-specific dependencies:
   - **Linux:** Install X11 or Wayland libraries
   - **macOS:** No additional dependencies needed
   - **Windows:** No additional dependencies needed

5. Test clipboard manually:
   ```bash
   # Copy some text
   echo "test" | pbcopy  # macOS
   echo "test" | clip    # Windows
   echo "test" | xclip   # Linux (X11)
   
   # Check if captured
   morsel list
   ```

### Database Locked

**Symptom:** "database is locked" errors.

**Solutions:**

1. Check for other processes:
   ```bash
   # Linux/macOS
   lsof ~/.local/share/morsel/morsel.db
   
   # Windows
   handle.exe ~/.local/share/morsel/morsel.db
   ```

2. Disable WAL mode:
   ```toml
   [storage]
   enable_wal = false
   ```

3. Increase connection pool:
   ```toml
   [storage]
   max_connections = 20
   ```

4. Restart daemon:
   ```bash
   morseld restart
   ```

### Slow Performance

**Symptom:** Commands are slow to respond.

**Solutions:**

1. Check database size:
   ```bash
   ls -lh ~/.local/share/morsel/morsel.db
   ```

2. Clean up old items:
   ```bash
   morsel cleanup --older-than 30d
   ```

3. Reduce retention:
   ```toml
   [retention]
   max_items = 1000
   max_age_days = 30
   ```

4. Vacuum database:
   ```bash
   sqlite3 ~/.local/share/morsel/morsel.db "VACUUM;"
   ```

5. Analyze database:
   ```bash
   sqlite3 ~/.local/share/morsel/morsel.db "ANALYZE;"
   ```

### Search Returns No Results

**Symptom:** Search queries return no results even though items exist.

**Solutions:**

1. Try exact match:
   ```bash
   morsel search --exact "query"
   ```

2. Check case sensitivity:
   ```bash
   morsel search --case-sensitive "Query"
   ```

3. Remove filters:
   ```bash
   morsel search "query"
   ```

4. Check if items exist:
   ```bash
   morsel list
   ```

5. Check content type filter:
   ```bash
   morsel search --content-type text "query"
   ```

### Import/Export Fails

**Symptom:** Import or export commands fail.

**Solutions:**

1. Check file permissions:
   ```bash
   ls -la backup.json
   ```

2. Check disk space:
   ```bash
   df -h
   ```

3. Validate JSON:
   ```bash
   cat backup.json | jq .
   ```

4. Try without blobs:
   ```bash
   morsel export backup.json --no-blobs
   ```

5. Check file size:
   ```bash
   ls -lh backup.json
   ```

### High Memory Usage

**Symptom:** Morsel daemon uses excessive memory.

**Solutions:**

1. Reduce buffer size:
   ```toml
   [clipboard]
   max_buffer_size = 500
   ```

2. Reduce retention:
   ```toml
   [retention]
   max_items = 1000
   ```

3. Clean up old items:
   ```bash
   morsel cleanup --older-than 7d
   ```

4. Restart daemon:
   ```bash
   morseld restart
   ```

### Sensitive Content Not Detected

**Symptom:** Sensitive content is not being flagged.

**Solutions:**

1. Enable sensitive detection:
   ```toml
   [privacy]
   enable_sensitive_detection = true
   ```

2. Lower confidence threshold:
   ```toml
   [sensitive_detection]
   confidence_threshold = 0.7
   ```

3. Enable specific detectors:
   ```toml
   [sensitive_detection]
   enable_api_key = true
   enable_password = true
   ```

4. Add custom patterns:
   ```toml
   [sensitive_detection]
   custom_patterns = [
       "my_pattern_[a-z]+"
   ]
   ```

### Configuration Not Applied

**Symptom:** Configuration changes don't take effect.

**Solutions:**

1. Restart daemon:
   ```bash
   morseld restart
   ```

2. Verify config file location:
   ```bash
   morsel config get
   ```

3. Check config syntax:
   ```bash
   cat ~/.config/morsel/config.toml
   ```

4. Reinitialize config:
   ```bash
   morsel config init
   ```

## Platform-Specific Issues

### Linux

#### X11 Clipboard Issues

**Symptom:** Clipboard not working on X11.

**Solutions:**

1. Install X11 libraries:
   ```bash
   sudo apt install libx11-dev libxext-dev  # Debian/Ubuntu
   sudo dnf install libX11-devel libXext-devel  # Fedora
   ```

2. Check X11 is running:
   ```bash
   echo $DISPLAY
   ```

3. Test clipboard:
   ```bash
   xclip -selection clipboard
   ```

#### Wayland Clipboard Issues

**Symptom:** Clipboard not working on Wayland.

**Solutions:**

1. Install Wayland libraries:
   ```bash
   sudo apt install libwayland-dev  # Debian/Ubuntu
   sudo dnf install wayland-devel  # Fedora
   ```

2. Use Wayland clipboard tool:
   ```bash
   wl-copy
   ```

### macOS

#### Permission Issues

**Symptom:** Clipboard access denied.

**Solutions:**

1. Grant accessibility permissions in System Preferences
2. Check Terminal has accessibility permissions
3. Restart Terminal after granting permissions

#### Notarization Issues

**Symptom:** App won't open due to security restrictions.

**Solutions:**

1. Right-click app and select "Open"
2. Go to System Preferences > Security & Privacy
3. Click "Open Anyway"

### Windows

#### Clipboard Access Denied

**Symptom:** Clipboard access denied.

**Solutions:**

1. Run as administrator
2. Check Windows clipboard history is enabled
3. Restart Windows Explorer

#### Firewall Blocking

**Symptom:** Daemon can't bind to port.

**Solutions:**

1. Allow morseld through Windows Firewall
2. Change port in config:
   ```toml
   [daemon]
   ipc_address = "127.0.0.1:54322"
   ```

## Getting Help

### Logs

Check daemon logs for detailed error information:

```bash
cat ~/.local/share/morsel/morseld.log
```

Enable debug logging:

```toml
[daemon]
log_level = "debug"
```

### Verbose Mode

Run CLI with verbose output:

```bash
morsel --verbose list
```

### Debug Mode

Run daemon in foreground:

```bash
morseld run
```

### Report Issues

If you can't resolve the issue:

1. Gather information:
   - Morsel version: `morsel --version`
   - OS version
   - Error message
   - Relevant logs
   - Configuration: `morsel config show`

2. Report on GitHub:
   - https://github.com/yourusername/morsel/issues

## Recovery

### Database Corruption

**Symptom:** "database disk image is malformed"

**Solutions:**

1. Export what you can:
   ```bash
   morsel export backup.json
   ```

2. Delete database:
   ```bash
   rm ~/.local/share/morsel/morsel.db
   ```

3. Restart daemon:
   ```bash
   morseld restart
   ```

4. Import backup:
   ```bash
   morsel import backup.json
   ```

### Lost Configuration

**Symptom:** Configuration file is corrupted or missing.

**Solutions:**

1. Reinitialize:
   ```bash
   morsel config init
   ```

2. Restore from backup if available

### Lost History

**Symptom:** Clipboard history is missing.

**Solutions:**

1. Check database file exists:
   ```bash
   ls -la ~/.local/share/morsel/morsel.db
   ```

2. Check for backup:
   ```bash
   ls -la ~/.local/share/morsel/*.backup
   ```

3. Restore from backup:
   ```bash
   cp ~/.local/share/morsel/morsel.db.backup ~/.local/share/morsel/morsel.db
   ```

## Prevention

### Regular Backups

Set up regular backups:

```bash
# Backup script
#!/bin/bash
DATE=$(date +%Y%m%d)
morsel export ~/.backups/morsel-$DATE.json
```

Add to cron/systemd timer for automatic backups.

### Monitor Disk Space

Monitor database size:

```bash
watch -n 60 'ls -lh ~/.local/share/morsel/morsel.db'
```

### Regular Cleanup

Schedule regular cleanup:

```bash
# Cron job
0 0 * * * morsel cleanup --older-than 30d
```

### Update Regularly

Keep Morsel updated:

```bash
cargo install morsel
```
