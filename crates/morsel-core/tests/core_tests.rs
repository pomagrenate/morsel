//! Core data model tests for morsel-core.

use morsel_core::{ClipboardItem, ContentType, ItemId, SensitiveType};
use chrono::Utc;

#[test]
fn test_item_id_generation() {
    let id1 = ItemId::new();
    let id2 = ItemId::new();
    
    assert_ne!(id1, id2, "Item IDs should be unique");
    assert!(id1.to_string().len() > 0, "Item ID should have string representation");
}

#[test]
fn test_item_id_parsing() {
    let id = ItemId::new();
    let id_str = id.to_string();
    
    let parsed = ItemId::from_str(&id_str);
    assert!(parsed.is_ok(), "Should be able to parse Item ID from string");
    assert_eq!(parsed.unwrap(), id, "Parsed ID should match original");
}

#[test]
fn test_item_id_invalid_parsing() {
    let result = ItemId::from_str("invalid-id");
    assert!(result.is_err(), "Invalid ID string should fail to parse");
}

#[test]
fn test_content_type_detection() {
    assert_eq!(ContentType::detect("https://example.com"), ContentType::Url);
    assert_eq!(ContentType::detect("test@example.com"), ContentType::Email);
    assert_eq!(ContentType::detect("550e8400-e29b-41d4-a716-446655440000"), ContentType::Uuid);
    assert_eq!(ContentType::detect("192.168.1.1"), ContentType::IpAddress);
    assert_eq!(ContentType::detect("{\"key\": \"value\"}"), ContentType::Json);
    assert_eq!(ContentType::detect("key: value"), ContentType::Yaml);
    assert_eq!(ContentType::detect("<root></root>"), ContentType::Xml);
    assert_eq!(ContentType::detect("SELECT * FROM table"), ContentType::Sql);
    assert_eq!(ContentType::detect("fn main() {}"), ContentType::Code);
    assert_eq!(ContentType::detect("# Heading"), ContentType::Shell); // Shell commands starting with # are detected first
    assert_eq!(ContentType::detect("plain text"), ContentType::Text);
}

#[test]
fn test_content_type_display() {
    assert_eq!(ContentType::Text.to_string(), "text");
    assert_eq!(ContentType::Url.to_string(), "url");
    assert_eq!(ContentType::Email.to_string(), "email");
    assert_eq!(ContentType::Json.to_string(), "json");
}

#[test]
fn test_content_type_from_str() {
    assert_eq!("text".parse::<ContentType>().unwrap(), ContentType::Text);
    assert_eq!("url".parse::<ContentType>().unwrap(), ContentType::Url);
    assert_eq!("email".parse::<ContentType>().unwrap(), ContentType::Email);
    assert_eq!("json".parse::<ContentType>().unwrap(), ContentType::Json);
}

#[test]
fn test_content_type_invalid_from_str() {
    let result = "invalid_type".parse::<ContentType>();
    assert!(result.is_err(), "Invalid content type should fail to parse");
}

#[test]
fn test_clipboard_item_creation() {
    let item = ClipboardItem::new("test content".to_string());
    
    assert_eq!(item.content, "test content");
    assert_eq!(item.content_type, ContentType::Text);
    assert!(!item.is_favorite);
    assert!(item.tags.is_empty());
    assert!(item.collection_id.is_none());
    assert!(item.expires_at.is_none());
}

#[test]
fn test_clipboard_item_with_tags() {
    let mut item = ClipboardItem::new("test content".to_string());
    item.tags = vec!["tag1".to_string(), "tag2".to_string()];
    
    assert_eq!(item.tags.len(), 2);
    assert!(item.tags.contains(&"tag1".to_string()));
}

#[test]
fn test_clipboard_item_favorite() {
    let mut item = ClipboardItem::new("test content".to_string());
    item.is_favorite = true;
    
    assert!(item.is_favorite);
}

#[test]
fn test_sensitive_detection_jwt() {
    let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
    let detection = morsel_core::detect_sensitive(jwt);
    
    assert!(detection.detected);
    assert_eq!(detection.sensitive_type, Some(SensitiveType::Jwt));
    assert!(detection.confidence > 0.8);
}

#[test]
fn test_sensitive_detection_api_key() {
    let api_key = "sk-1234567890abcdefghijklmnopqrstuvwxyz";
    let detection = morsel_core::detect_sensitive(api_key);
    
    assert!(detection.detected);
    assert!(detection.sensitive_type.is_some());
}

#[test]
fn test_sensitive_detection_bearer_token() {
    let token = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
    let detection = morsel_core::detect_sensitive(token);
    
    assert!(detection.detected);
}

#[test]
fn test_sensitive_detection_private_key() {
    let private_key = "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC";
    let detection = morsel_core::detect_sensitive(private_key);
    
    assert!(detection.detected);
    assert_eq!(detection.sensitive_type, Some(SensitiveType::PrivateKey));
}

#[test]
fn test_sensitive_detection_password() {
    let password = "password=secret123";
    let detection = morsel_core::detect_sensitive(password);
    
    assert!(detection.detected);
    assert_eq!(detection.sensitive_type, Some(SensitiveType::Password));
}

#[test]
fn test_sensitive_detection_safe_content() {
    let safe_content = "This is just normal text with no sensitive information";
    let detection = morsel_core::detect_sensitive(safe_content);
    
    assert!(!detection.detected);
    assert!(detection.sensitive_type.is_none());
    assert_eq!(detection.confidence, 0.0);
}

#[test]
fn test_sensitive_detection_config() {
    use morsel_core::SensitiveDetectionConfig;
    
    let config = SensitiveDetectionConfig {
        enable_jwt: false,
        enable_api_key: true,
        enable_access_token: true,
        enable_bearer_token: true,
        enable_private_key: true,
        enable_password: true,
        enable_secret: true,
        confidence_threshold: 0.7,
        custom_patterns: vec![],
        whitelist_patterns: vec![],
        whitelist_content: vec![],
    };
    
    let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
    let detection = morsel_core::detect_sensitive_with_config(jwt, &config);
    
    assert!(!detection.detected, "JWT detection should be disabled");
}

#[test]
fn test_sensitive_detection_whitelist() {
    use morsel_core::SensitiveDetectionConfig;
    
    let config = SensitiveDetectionConfig {
        enable_jwt: true,
        enable_api_key: true,
        enable_access_token: true,
        enable_bearer_token: true,
        enable_private_key: true,
        enable_password: true,
        enable_secret: true,
        confidence_threshold: 0.7,
        custom_patterns: vec![],
        whitelist_patterns: vec!["test.*".to_string()],
        whitelist_content: vec!["safe-token".to_string()],
    };
    
    let safe_token = "safe-token-123";
    let detection = morsel_core::detect_sensitive_with_config(safe_token, &config);
    
    assert!(!detection.detected, "Whitelisted content should not be detected as sensitive");
}

#[test]
fn test_clipboard_item_size() {
    let item = ClipboardItem::new("test content".to_string());
    assert_eq!(item.size, 12); // "test content" is 12 bytes
}

#[test]
fn test_clipboard_item_timestamps() {
    let before = Utc::now();
    let item = ClipboardItem::new("test content".to_string());
    let after = Utc::now();
    
    assert!(item.created_at >= before);
    assert!(item.created_at <= after);
    assert!(item.last_used_at >= before);
    assert!(item.last_used_at <= after);
}
