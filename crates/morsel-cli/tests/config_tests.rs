//! Configuration tests for morsel-cli.

use tempfile::NamedTempFile;
use std::fs;

#[test]
fn test_default_config() {
    let config_file = NamedTempFile::new().unwrap();
    let config_path = config_file.path();
    
    // Create empty config file
    fs::write(config_path, "{}").unwrap();
    
    // Read and parse config
    let config_content = fs::read_to_string(config_path).unwrap();
    let config: serde_json::Value = serde_json::from_str(&config_content).unwrap();
    
    assert!(config.is_object());
}

#[test]
fn test_config_with_db_path() {
    let config_file = NamedTempFile::new().unwrap();
    let config_path = config_file.path();
    
    let config_json = r#"{"db_path": "/custom/path/morsel.db"}"#;
    fs::write(config_path, config_json).unwrap();
    
    let config_content = fs::read_to_string(config_path).unwrap();
    let config: serde_json::Value = serde_json::from_str(&config_content).unwrap();
    
    assert_eq!(config["db_path"], "/custom/path/morsel.db");
}

#[test]
fn test_config_with_max_items() {
    let config_file = NamedTempFile::new().unwrap();
    let config_path = config_file.path();
    
    let config_json = r#"{"max_items": 500}"#;
    fs::write(config_path, config_json).unwrap();
    
    let config_content = fs::read_to_string(config_path).unwrap();
    let config: serde_json::Value = serde_json::from_str(&config_content).unwrap();
    
    assert_eq!(config["max_items"], 500);
}

#[test]
fn test_config_with_poll_interval() {
    let config_file = NamedTempFile::new().unwrap();
    let config_path = config_file.path();
    
    let config_json = r#"{"poll_interval": 500}"#;
    fs::write(config_path, config_json).unwrap();
    
    let config_content = fs::read_to_string(config_path).unwrap();
    let config: serde_json::Value = serde_json::from_str(&config_content).unwrap();
    
    assert_eq!(config["poll_interval"], 500);
}

#[test]
fn test_config_with_hotkey() {
    let config_file = NamedTempFile::new().unwrap();
    let config_path = config_file.path();
    
    let config_json = r#"{"hotkey": "Ctrl+Alt+M"}"#;
    fs::write(config_path, config_json).unwrap();
    
    let config_content = fs::read_to_string(config_path).unwrap();
    let config: serde_json::Value = serde_json::from_str(&config_content).unwrap();
    
    assert_eq!(config["hotkey"], "Ctrl+Alt+M");
}

#[test]
fn test_config_with_sensitive_detection() {
    let config_file = NamedTempFile::new().unwrap();
    let config_path = config_file.path();
    
    let config_json = r#"{"sensitive_detection": true, "auto_delete_sensitive": true}"#;
    fs::write(config_path, config_json).unwrap();
    
    let config_content = fs::read_to_string(config_path).unwrap();
    let config: serde_json::Value = serde_json::from_str(&config_content).unwrap();
    
    assert_eq!(config["sensitive_detection"], true);
    assert_eq!(config["auto_delete_sensitive"], true);
}

#[test]
fn test_config_with_retention_days() {
    let config_file = NamedTempFile::new().unwrap();
    let config_path = config_file.path();
    
    let config_json = r#"{"retention_days": 30}"#;
    fs::write(config_path, config_json).unwrap();
    
    let config_content = fs::read_to_string(config_path).unwrap();
    let config: serde_json::Value = serde_json::from_str(&config_content).unwrap();
    
    assert_eq!(config["retention_days"], 30);
}

#[test]
fn test_config_with_all_fields() {
    let config_file = NamedTempFile::new().unwrap();
    let config_path = config_file.path();
    
    let config_json = r#"{
        "db_path": "/custom/path/morsel.db",
        "max_items": 1000,
        "poll_interval": 250,
        "hotkey": "Ctrl+Alt+M",
        "verbose": true,
        "sensitive_detection": true,
        "auto_delete_sensitive": false,
        "enable_encryption": false,
        "retention_days": 90
    }"#;
    fs::write(config_path, config_json).unwrap();
    
    let config_content = fs::read_to_string(config_path).unwrap();
    let config: serde_json::Value = serde_json::from_str(&config_content).unwrap();
    
    assert_eq!(config["db_path"], "/custom/path/morsel.db");
    assert_eq!(config["max_items"], 1000);
    assert_eq!(config["poll_interval"], 250);
    assert_eq!(config["hotkey"], "Ctrl+Alt+M");
    assert_eq!(config["verbose"], true);
    assert_eq!(config["sensitive_detection"], true);
    assert_eq!(config["auto_delete_sensitive"], false);
    assert_eq!(config["enable_encryption"], false);
    assert_eq!(config["retention_days"], 90);
}

#[test]
fn test_config_invalid_json() {
    let config_file = NamedTempFile::new().unwrap();
    let config_path = config_file.path();
    
    let invalid_json = r#"{"db_path": invalid}"#;
    fs::write(config_path, invalid_json).unwrap();
    
    let config_content = fs::read_to_string(config_path).unwrap();
    let result: Result<serde_json::Value, _> = serde_json::from_str(&config_content);
    
    assert!(result.is_err());
}

#[test]
fn test_config_empty_file() {
    let config_file = NamedTempFile::new().unwrap();
    let config_path = config_file.path();
    
    fs::write(config_path, "").unwrap();
    
    let config_content = fs::read_to_string(config_path).unwrap();
    let result: Result<serde_json::Value, _> = serde_json::from_str(&config_content);
    
    assert!(result.is_err());
}

#[test]
fn test_config_extra_fields() {
    let config_file = NamedTempFile::new().unwrap();
    let config_path = config_file.path();
    
    let config_json = r#"{"db_path": "/path/morsel.db", "unknown_field": "value"}"#;
    fs::write(config_path, config_json).unwrap();
    
    let config_content = fs::read_to_string(config_path).unwrap();
    let config: serde_json::Value = serde_json::from_str(&config_content).unwrap();
    
    assert_eq!(config["db_path"], "/path/morsel.db");
    assert_eq!(config["unknown_field"], "value");
}

#[test]
fn test_config_numeric_string() {
    let config_file = NamedTempFile::new().unwrap();
    let config_path = config_file.path();
    
    let config_json = r#"{"max_items": "500"}"#;
    fs::write(config_path, config_json).unwrap();
    
    let config_content = fs::read_to_string(config_path).unwrap();
    let config: serde_json::Value = serde_json::from_str(&config_content).unwrap();
    
    // JSON will parse it as a string, not a number
    assert_eq!(config["max_items"], "500");
}

#[test]
fn test_config_zero_retention() {
    let config_file = NamedTempFile::new().unwrap();
    let config_path = config_file.path();
    
    let config_json = r#"{"retention_days": 0}"#;
    fs::write(config_path, config_json).unwrap();
    
    let config_content = fs::read_to_string(config_path).unwrap();
    let config: serde_json::Value = serde_json::from_str(&config_content).unwrap();
    
    assert_eq!(config["retention_days"], 0);
}

#[test]
fn test_config_negative_values() {
    let config_file = NamedTempFile::new().unwrap();
    let config_path = config_file.path();
    
    let config_json = r#"{"max_items": -100, "retention_days": -30}"#;
    fs::write(config_path, config_json).unwrap();
    
    let config_content = fs::read_to_string(config_path).unwrap();
    let config: serde_json::Value = serde_json::from_str(&config_content).unwrap();
    
    assert_eq!(config["max_items"], -100);
    assert_eq!(config["retention_days"], -30);
}

#[test]
fn test_config_boolean_variations() {
    let config_file = NamedTempFile::new().unwrap();
    let config_path = config_file.path();
    
    let config_json = r#"{"verbose": true, "sensitive_detection": false}"#;
    fs::write(config_path, config_json).unwrap();
    
    let config_content = fs::read_to_string(config_path).unwrap();
    let config: serde_json::Value = serde_json::from_str(&config_content).unwrap();
    
    assert_eq!(config["verbose"], true);
    assert_eq!(config["sensitive_detection"], false);
}
