//! # morsel-core
//!
//! Core data types and models for the morsel clipboard manager.
//!
//! This crate provides the fundamental data structures used throughout
//! the morsel ecosystem, including clipboard items, content types,
//! and metadata models.

use base64::Engine;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;
use uuid::Uuid;

/// Sensitive content type for heuristic detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SensitiveType {
    /// API key
    ApiKey,
    /// Access token
    AccessToken,
    /// JWT token
    Jwt,
    /// Private key
    PrivateKey,
    /// Bearer token
    BearerToken,
    /// Password
    Password,
    /// Secret
    Secret,
}

/// Configuration for sensitive content detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveDetectionConfig {
    /// Enable/disable detection for each type
    pub enable_api_key: bool,
    pub enable_access_token: bool,
    pub enable_jwt: bool,
    pub enable_private_key: bool,
    pub enable_bearer_token: bool,
    pub enable_password: bool,
    pub enable_secret: bool,
    
    /// Confidence threshold (0.0 to 1.0)
    pub confidence_threshold: f32,
    
    /// Custom patterns (regex strings)
    pub custom_patterns: Vec<String>,
    
    /// Whitelist patterns to ignore (regex strings)
    pub whitelist_patterns: Vec<String>,
    
    /// Whitelist specific content strings to never flag as sensitive
    pub whitelist_content: Vec<String>,
}

impl Default for SensitiveDetectionConfig {
    fn default() -> Self {
        Self {
            enable_api_key: true,
            enable_access_token: true,
            enable_jwt: true,
            enable_private_key: true,
            enable_bearer_token: true,
            enable_password: true,
            enable_secret: true,
            confidence_threshold: 0.7,
            custom_patterns: Vec::new(),
            whitelist_patterns: Vec::new(),
            whitelist_content: Vec::new(),
        }
    }
}

impl fmt::Display for SensitiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SensitiveType::ApiKey => write!(f, "api_key"),
            SensitiveType::AccessToken => write!(f, "access_token"),
            SensitiveType::Jwt => write!(f, "jwt"),
            SensitiveType::PrivateKey => write!(f, "private_key"),
            SensitiveType::BearerToken => write!(f, "bearer_token"),
            SensitiveType::Password => write!(f, "password"),
            SensitiveType::Secret => write!(f, "secret"),
        }
    }
}

/// Result of sensitive content detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveDetection {
    /// Whether sensitive content was detected
    pub detected: bool,
    /// Type of sensitive content detected
    pub sensitive_type: Option<SensitiveType>,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
}

/// Detect sensitive content in clipboard data.
pub fn detect_sensitive(content: &str) -> SensitiveDetection {
    detect_sensitive_with_config(content, &SensitiveDetectionConfig::default())
}

/// Detect sensitive content with custom configuration.
pub fn detect_sensitive_with_config(content: &str, config: &SensitiveDetectionConfig) -> SensitiveDetection {
    let trimmed = content.trim();
    
    // Check whitelist content first (exact match)
    if config.whitelist_content.iter().any(|w| w == trimmed) {
        return SensitiveDetection {
            detected: false,
            sensitive_type: None,
            confidence: 0.0,
        };
    }
    
    // Check whitelist patterns
    for pattern in &config.whitelist_patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if re.is_match(trimmed) {
                return SensitiveDetection {
                    detected: false,
                    sensitive_type: None,
                    confidence: 0.0,
                };
            }
        }
    }
    
    // Check for JWT
    if config.enable_jwt {
        if let Some(confidence) = detect_jwt(trimmed) {
            if confidence >= config.confidence_threshold {
                return SensitiveDetection {
                    detected: true,
                    sensitive_type: Some(SensitiveType::Jwt),
                    confidence,
                };
            }
        }
    }
    
    // Check for API keys
    if config.enable_api_key {
        if let Some(confidence) = detect_api_key(trimmed) {
            if confidence >= config.confidence_threshold {
                return SensitiveDetection {
                    detected: true,
                    sensitive_type: Some(SensitiveType::ApiKey),
                    confidence,
                };
            }
        }
    }
    
    // Check for bearer tokens
    if config.enable_bearer_token {
        if let Some(confidence) = detect_bearer_token(trimmed) {
            if confidence >= config.confidence_threshold {
                return SensitiveDetection {
                    detected: true,
                    sensitive_type: Some(SensitiveType::BearerToken),
                    confidence,
                };
            }
        }
    }
    
    // Check for private keys
    if config.enable_private_key {
        if let Some(confidence) = detect_private_key(trimmed) {
            if confidence >= config.confidence_threshold {
                return SensitiveDetection {
                    detected: true,
                    sensitive_type: Some(SensitiveType::PrivateKey),
                    confidence,
                };
            }
        }
    }
    
    // Check for passwords
    if config.enable_password {
        if let Some(confidence) = detect_password(trimmed) {
            if confidence >= config.confidence_threshold {
                return SensitiveDetection {
                    detected: true,
                    sensitive_type: Some(SensitiveType::Password),
                    confidence,
                };
            }
        }
    }
    
    // Check for secrets
    if config.enable_secret {
        if let Some(confidence) = detect_secret(trimmed) {
            if confidence >= config.confidence_threshold {
                return SensitiveDetection {
                    detected: true,
                    sensitive_type: Some(SensitiveType::Secret),
                    confidence,
                };
            }
        }
    }
    
    // Check custom patterns
    for pattern in &config.custom_patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if re.is_match(trimmed) {
                return SensitiveDetection {
                    detected: true,
                    sensitive_type: Some(SensitiveType::Secret),
                    confidence: config.confidence_threshold,
                };
            }
        }
    }
    
    SensitiveDetection {
        detected: false,
        sensitive_type: None,
        confidence: 0.0,
    }
}

fn detect_jwt(content: &str) -> Option<f32> {
    // JWT: three base64-encoded parts separated by dots
    let parts: Vec<&str> = content.split('.').collect();
    if parts.len() == 3 {
        // Check if each part looks like base64
        let is_base64 = parts.iter().all(|p| {
            p.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        });
        if is_base64 {
            return Some(0.9);
        }
    }
    None
}

fn detect_api_key(content: &str) -> Option<f32> {
    // Common API key patterns
    let patterns = [
        r"(?i)api[_-]?key\s*[:=]\s*[a-zA-Z0-9_-]{20,}",
        r"(?i)apikey\s*[:=]\s*[a-zA-Z0-9_-]{20,}",
        r"(?i)secret[_-]?key\s*[:=]\s*[a-zA-Z0-9_-]{20,}",
        r"(?i)access[_-]?key\s*[:=]\s*[a-zA-Z0-9_-]{20,}",
        r"(?i)sk-[a-zA-Z0-9]{32,}", // OpenAI style
        r"(?i)AKIA[0-9A-Z]{16}", // AWS style
        r"(?i)xox[bap]-[0-9]{12}-[0-9]{12}-[0-9]{12}-[a-z0-9]{32}", // Slack style
    ];
    
    for pattern in &patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if re.is_match(content) {
                return Some(0.8);
            }
        }
    }
    None
}

fn detect_bearer_token(content: &str) -> Option<f32> {
    // Bearer token pattern
    let lower = content.to_lowercase();
    if lower.starts_with("bearer ") || lower.contains("authorization:") {
        return Some(0.85);
    }
    None
}

fn detect_private_key(content: &str) -> Option<f32> {
    // Private key headers
    let lower = content.to_lowercase();
    if lower.starts_with("-----begin") && lower.contains("private key") {
        return Some(0.95);
    }
    if lower.contains("rsa private key") || lower.contains("ec private key") {
        return Some(0.9);
    }
    None
}

fn detect_password(content: &str) -> Option<f32> {
    // Password patterns
    let lower = content.to_lowercase();
    if lower.starts_with("password") || lower.starts_with("passwd") {
        return Some(0.7);
    }
    if lower.contains("password:") || lower.contains("passwd:") {
        return Some(0.75);
    }
    None
}

fn detect_secret(content: &str) -> Option<f32> {
    // Secret patterns
    let lower = content.to_lowercase();
    if lower.starts_with("secret") || lower.starts_with("token") {
        return Some(0.6);
    }
    if lower.contains("secret:") || lower.contains("token:") {
        return Some(0.65);
    }
    None
}

/// Result type alias for morsel-core operations.
pub type Result<T> = std::result::Result<T, CoreError>;

/// Error types for core operations.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    #[error("Invalid clipboard content: {0}")]
    InvalidContent(String),

    #[error("Invalid content type: {0}")]
    InvalidContentType(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Invalid UUID: {0}")]
    InvalidUuid(String),

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),
}

/// Unique identifier for a clipboard item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemId(Uuid);

impl ItemId {
    /// Create a new random ItemId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create an ItemId from a UUID string.
    pub fn from_uuid_str(s: &str) -> Result<Self> {
        let uuid = Uuid::parse_str(s)
            .map_err(|e| CoreError::InvalidUuid(e.to_string()))?;
        Ok(Self(uuid))
    }

    /// Get the underlying UUID.
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    /// Get the ItemId as a string.
    pub fn as_str(&self) -> &str {
        // This is a workaround - UUID doesn't expose &str directly
        // In practice, you'd use to_string() or similar
        panic!("Use to_string() instead")
    }
}

impl Default for ItemId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ItemId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

/// Content type for clipboard items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    /// Plain text content.
    Text,
    /// Rich text (HTML, RTF, etc.).
    RichText,
    /// URL.
    Url,
    /// Image data.
    Image,
    /// File reference.
    File,
    /// Email address.
    Email,
    /// JSON data.
    Json,
    /// YAML data.
    Yaml,
    /// XML data.
    Xml,
    /// Shell command.
    Shell,
    /// SQL query.
    Sql,
    /// Code snippet.
    Code,
    /// Markdown text.
    Markdown,
    /// JWT token.
    Jwt,
    /// UUID.
    Uuid,
    /// IP address.
    IpAddress,
    /// Git commit hash.
    GitCommit,
    /// Unknown or unsupported content.
    Unknown,
}

impl ContentType {
    /// Detect content type from a string.
    pub fn detect(content: &str) -> Self {
        if content.is_empty() {
            return ContentType::Unknown;
        }

        let trimmed = content.trim();

        // Check for URL
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            return ContentType::Url;
        }

        // Check for email
        if Self::is_email(trimmed) {
            return ContentType::Email;
        }

        // Check for UUID
        if Self::is_uuid(trimmed) {
            return ContentType::Uuid;
        }

        // Check for IP address
        if Self::is_ip_address(trimmed) {
            return ContentType::IpAddress;
        }

        // Check for Git commit hash
        if Self::is_git_commit(trimmed) {
            return ContentType::GitCommit;
        }

        // Check for JWT
        if Self::is_jwt(trimmed) {
            return ContentType::Jwt;
        }

        // Check for JSON
        if Self::is_json(trimmed) {
            return ContentType::Json;
        }

        // Check for YAML
        if Self::is_yaml(trimmed) {
            return ContentType::Yaml;
        }

        // Check for XML
        if Self::is_xml(trimmed) {
            return ContentType::Xml;
        }

        // Check for SQL
        if Self::is_sql(trimmed) {
            return ContentType::Sql;
        }

        // Check for shell command
        if Self::is_shell_command(trimmed) {
            return ContentType::Shell;
        }

        // Check for Markdown
        if Self::is_markdown(trimmed) {
            return ContentType::Markdown;
        }

        // Check for code snippet
        if Self::is_code(trimmed) {
            return ContentType::Code;
        }

        // Check for file path (basic heuristic)
        if (content.contains('/') || content.contains('\\'))
            && content.contains('.') && !content.contains(' ') {
            return ContentType::File;
        }

        // Default to text
        ContentType::Text
    }

    fn is_email(content: &str) -> bool {
        // Basic email regex
        let email_regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(content)
    }

    fn is_uuid(content: &str) -> bool {
        // UUID format: 8-4-4-4-12 hex digits
        let uuid_regex = regex::Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$").unwrap();
        uuid_regex.is_match(content)
    }

    fn is_ip_address(content: &str) -> bool {
        // IPv4
        let ipv4_regex = regex::Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap();
        if ipv4_regex.is_match(content) {
            return true;
        }
        // IPv6 (simplified)
        let ipv6_regex = regex::Regex::new(r"^[0-9a-fA-F:]+$").unwrap();
        if ipv6_regex.is_match(content) && content.contains(':') {
            return true;
        }
        false
    }

    fn is_git_commit(content: &str) -> bool {
        // Git commit hash: 40 hex characters (or 7+ for short hashes)
        let git_regex = regex::Regex::new(r"^[0-9a-fA-F]{7,40}$").unwrap();
        git_regex.is_match(content)
    }

    fn is_jwt(content: &str) -> bool {
        // JWT: three base64-encoded parts separated by dots
        let parts: Vec<&str> = content.split('.').collect();
        if parts.len() == 3 {
            // Check if each part looks like base64
            parts.iter().all(|p| {
                p.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
            })
        } else {
            false
        }
    }

    fn is_json(content: &str) -> bool {
        // JSON starts with { or [
        let trimmed = content.trim();
        trimmed.starts_with('{') && trimmed.ends_with('}') ||
        trimmed.starts_with('[') && trimmed.ends_with(']')
    }

    fn is_yaml(content: &str) -> bool {
        // YAML typically has key: value pairs or starts with ---
        let trimmed = content.trim();
        trimmed.starts_with("---") || trimmed.contains(':') && !trimmed.contains('{')
    }

    fn is_xml(content: &str) -> bool {
        // XML starts with < and ends with >
        let trimmed = content.trim();
        trimmed.starts_with('<') && trimmed.ends_with('>')
    }

    fn is_sql(content: &str) -> bool {
        // SQL keywords
        let upper = content.to_uppercase();
        let sql_keywords = ["SELECT", "INSERT", "UPDATE", "DELETE", "CREATE", "ALTER", "DROP", "FROM", "WHERE", "JOIN"];
        sql_keywords.iter().any(|&keyword| upper.starts_with(keyword))
    }

    fn is_shell_command(content: &str) -> bool {
        // Shell commands start with common prefixes
        let trimmed = content.trim();
        let prefixes = ["sudo", "apt", "yum", "npm", "pip", "cargo", "git", "docker", "kubectl", "curl", "wget"];
        prefixes.iter().any(|&prefix| trimmed.starts_with(prefix)) ||
        trimmed.starts_with('#') || trimmed.starts_with('$')
    }

    fn is_markdown(content: &str) -> bool {
        // Markdown headers, lists, or code blocks
        let trimmed = content.trim();
        trimmed.starts_with('#') || 
        trimmed.starts_with("- ") || 
        trimmed.starts_with("* ") ||
        trimmed.starts_with("```") ||
        trimmed.contains("**") && trimmed.contains("**")
    }

    fn is_code(content: &str) -> bool {
        // Code indicators: function definitions, imports, etc.
        let code_patterns = [
            "fn ", "def ", "function ", "class ", "import ", "from ", "use ",
            "pub fn ", "async fn ", "const ", "let ", "var ", "=>", "() ->",
        ];
        code_patterns.iter().any(|&pattern| content.contains(pattern))
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentType::Text => write!(f, "text"),
            ContentType::RichText => write!(f, "rich_text"),
            ContentType::Url => write!(f, "url"),
            ContentType::Image => write!(f, "image"),
            ContentType::File => write!(f, "file"),
            ContentType::Email => write!(f, "email"),
            ContentType::Json => write!(f, "json"),
            ContentType::Yaml => write!(f, "yaml"),
            ContentType::Xml => write!(f, "xml"),
            ContentType::Shell => write!(f, "shell"),
            ContentType::Sql => write!(f, "sql"),
            ContentType::Code => write!(f, "code"),
            ContentType::Markdown => write!(f, "markdown"),
            ContentType::Jwt => write!(f, "jwt"),
            ContentType::Uuid => write!(f, "uuid"),
            ContentType::IpAddress => write!(f, "ip_address"),
            ContentType::GitCommit => write!(f, "git_commit"),
            ContentType::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::str::FromStr for ContentType {
    type Err = CoreError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "text" => Ok(ContentType::Text),
            "rich_text" => Ok(ContentType::RichText),
            "url" => Ok(ContentType::Url),
            "image" => Ok(ContentType::Image),
            "file" => Ok(ContentType::File),
            "email" => Ok(ContentType::Email),
            "json" => Ok(ContentType::Json),
            "yaml" => Ok(ContentType::Yaml),
            "xml" => Ok(ContentType::Xml),
            "shell" => Ok(ContentType::Shell),
            "sql" => Ok(ContentType::Sql),
            "code" => Ok(ContentType::Code),
            "markdown" => Ok(ContentType::Markdown),
            "jwt" => Ok(ContentType::Jwt),
            "uuid" => Ok(ContentType::Uuid),
            "ip_address" => Ok(ContentType::IpAddress),
            "git_commit" => Ok(ContentType::GitCommit),
            "unknown" => Ok(ContentType::Unknown),
            _ => Err(CoreError::DeserializationError(format!("Unknown content type: {}", s))),
        }
    }
}

/// A clipboard item with its content and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardItem {
    /// Unique identifier for this item.
    pub id: ItemId,
    /// The content of the clipboard item.
    pub content: String,
    /// The type of content.
    pub content_type: ContentType,
    /// When this item was created.
    pub created_at: DateTime<Utc>,
    /// When this item was last used/accessed.
    pub last_used_at: DateTime<Utc>,
    /// Size of the content in bytes.
    pub size: usize,
    /// Whether this item is marked as a favorite.
    pub is_favorite: bool,
    /// Tags associated with this item.
    pub tags: Vec<String>,
    /// Collection ID if this item belongs to a collection.
    pub collection_id: Option<ItemId>,
    /// Expiration time if applicable.
    pub expires_at: Option<DateTime<Utc>>,
    /// Source application metadata (where supported).
    pub source: Option<String>,
}

impl ClipboardItem {
    /// Create a new clipboard item.
    pub fn new(content: String) -> Self {
        let content_type = ContentType::detect(&content);
        let size = content.len();
        let now = Utc::now();

        Self {
            id: ItemId::new(),
            content,
            content_type,
            created_at: now,
            last_used_at: now,
            size,
            is_favorite: false,
            tags: Vec::new(),
            collection_id: None,
            expires_at: None,
            source: None,
        }
    }

    /// Create a clipboard item with a specific content type.
    pub fn with_type(content: String, content_type: ContentType) -> Self {
        let size = content.len();
        let now = Utc::now();

        Self {
            id: ItemId::new(),
            content,
            content_type,
            created_at: now,
            last_used_at: now,
            size,
            is_favorite: false,
            tags: Vec::new(),
            collection_id: None,
            expires_at: None,
            source: None,
        }
    }

    /// Mark this item as used (update last_used_at).
    pub fn mark_used(&mut self) {
        self.last_used_at = Utc::now();
    }

    /// Add a tag to this item.
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Remove a tag from this item.
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    /// Check if this item has a specific tag.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    /// Clear all tags from this item.
    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    /// Set the favorite status.
    pub fn set_favorite(&mut self, is_favorite: bool) {
        self.is_favorite = is_favorite;
    }

    /// Set the collection ID.
    pub fn set_collection(&mut self, collection_id: Option<ItemId>) {
        self.collection_id = collection_id;
    }

    /// Set expiration time.
    pub fn set_expires_at(&mut self, expires_at: Option<DateTime<Utc>>) {
        self.expires_at = expires_at;
    }

    /// Check if this item has expired.
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }
}

/// A collection of clipboard items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    /// Unique identifier for this collection.
    pub id: ItemId,
    /// Collection name.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// Tags for this collection.
    pub tags: Vec<String>,
    /// Whether this collection is marked as favorite.
    pub is_favorite: bool,
    /// IDs of items in this collection.
    pub item_ids: Vec<ItemId>,
    /// When this collection was created.
    pub created_at: DateTime<Utc>,
    /// When this collection was last modified.
    pub updated_at: DateTime<Utc>,
}

impl Collection {
    /// Create a new collection.
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: ItemId::new(),
            name,
            description: None,
            tags: Vec::new(),
            is_favorite: false,
            item_ids: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Set the description.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Add an item to this collection.
    pub fn add_item(&mut self, item_id: ItemId) {
        if !self.item_ids.contains(&item_id) {
            self.item_ids.push(item_id);
            self.updated_at = Utc::now();
        }
    }

    /// Remove an item from this collection.
    pub fn remove_item(&mut self, item_id: ItemId) {
        self.item_ids.retain(|id| *id != item_id);
        self.updated_at = Utc::now();
    }

    /// Check if this collection contains a specific item.
    pub fn has_item(&self, item_id: ItemId) -> bool {
        self.item_ids.contains(&item_id)
    }

    /// Get the number of items in this collection.
    pub fn item_count(&self) -> usize {
        self.item_ids.len()
    }

    /// Clear all items from this collection.
    pub fn clear_items(&mut self) {
        self.item_ids.clear();
        self.updated_at = Utc::now();
    }

    /// Check if this collection is empty.
    pub fn is_empty(&self) -> bool {
        self.item_ids.is_empty()
    }

    /// Set the favorite status.
    pub fn set_favorite(&mut self, is_favorite: bool) {
        self.is_favorite = is_favorite;
        self.updated_at = Utc::now();
    }

    /// Add a tag to this collection.
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    /// Remove a tag from this collection.
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
        self.updated_at = Utc::now();
    }
}

/// Metadata for binary content (images, files, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobMetadata {
    /// Unique identifier for this blob.
    pub id: ItemId,
    /// MIME type of the blob.
    pub mime_type: String,
    /// Size in bytes.
    pub size: usize,
    /// When this blob was created.
    pub created_at: DateTime<Utc>,
    /// Hash of the blob content for deduplication.
    pub hash: String,
}

impl BlobMetadata {
    /// Create new blob metadata.
    pub fn new(mime_type: String, size: usize, hash: String) -> Self {
        Self {
            id: ItemId::new(),
            mime_type,
            size,
            created_at: Utc::now(),
            hash,
        }
    }
}

/// Backup format for morsel clipboard data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backup {
    /// Backup format version
    pub version: u32,
    /// When the backup was created
    pub created_at: DateTime<Utc>,
    /// Clipboard items
    pub items: Vec<ClipboardItem>,
    /// Binary blobs
    pub blobs: Vec<BackupBlob>,
    /// Backup metadata
    pub metadata: BackupMetadata,
}

/// Binary blob in backup format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupBlob {
    /// Unique identifier
    pub id: ItemId,
    /// MIME type
    pub mime_type: String,
    /// Size in bytes
    pub size: usize,
    /// Hash for deduplication
    pub hash: String,
    /// Base64-encoded data
    pub data: String,
}

/// Backup metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    /// Total number of items
    pub item_count: usize,
    /// Total number of blobs
    pub blob_count: usize,
    /// Total size in bytes
    pub total_size: usize,
    /// Morsel version that created the backup
    pub morsel_version: String,
    /// Optional encryption info
    pub encryption: Option<EncryptionInfo>,
}

/// Encryption information for encrypted backups.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionInfo {
    /// Encryption algorithm used
    pub algorithm: String,
    /// Key derivation function
    pub kdf: String,
    /// Salt for key derivation (hex-encoded)
    pub salt: String,
    /// Nonce for encryption (hex-encoded)
    pub nonce: String,
}

impl Backup {
    /// Create a new backup from clipboard items.
    pub fn new(items: Vec<ClipboardItem>, blobs: Vec<BackupBlob>) -> Self {
        let item_count = items.len();
        let blob_count = blobs.len();
        let total_size = items.iter().map(|i| i.size).sum::<usize>() 
            + blobs.iter().map(|b| b.size).sum::<usize>();
        
        Self {
            version: 1,
            created_at: Utc::now(),
            items,
            blobs,
            metadata: BackupMetadata {
                item_count,
                blob_count,
                total_size,
                morsel_version: env!("CARGO_PKG_VERSION").to_string(),
                encryption: None,
            },
        }
    }

    /// Validate the backup integrity.
    pub fn validate(&self) -> Result<()> {
        // Check version compatibility
        if self.version > 1 {
            return Err(CoreError::DeserializationError(
                format!("Unsupported backup version: {}", self.version)
            ));
        }

        // Check item count matches
        if self.items.len() != self.metadata.item_count {
            return Err(CoreError::DeserializationError(
                format!("Item count mismatch: expected {}, got {}",
                    self.metadata.item_count, self.items.len())
            ));
        }

        // Check blob count matches
        if self.blobs.len() != self.metadata.blob_count {
            return Err(CoreError::DeserializationError(
                format!("Blob count mismatch: expected {}, got {}",
                    self.metadata.blob_count, self.blobs.len())
            ));
        }

        Ok(())
    }
}

impl BackupBlob {
    /// Create a new backup blob from raw data.
    pub fn new(id: ItemId, mime_type: String, data: &[u8]) -> Self {
        let size = data.len();
        let hash = format!("{:x}", md5::compute(data));
        let engine = base64::engine::general_purpose::STANDARD;
        let data_base64 = engine.encode(data);

        Self {
            id,
            mime_type,
            size,
            hash,
            data: data_base64,
        }
    }

    /// Decode the base64 data.
    pub fn decode_data(&self) -> Result<Vec<u8>> {
        let engine = base64::engine::general_purpose::STANDARD;
        engine.decode(&self.data)
            .map_err(|e| CoreError::DeserializationError(format!("Failed to decode blob data: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_id_new() {
        let id1 = ItemId::new();
        let id2 = ItemId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_item_id_from_uuid_str() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let id = ItemId::from_uuid_str(uuid_str).unwrap();
        assert_eq!(id.to_string(), uuid_str);
    }

    #[test]
    fn test_content_type_detect_text() {
        assert_eq!(ContentType::detect("hello world"), ContentType::Text);
    }

    #[test]
    fn test_content_type_detect_url() {
        assert_eq!(ContentType::detect("https://example.com"), ContentType::Url);
        assert_eq!(ContentType::detect("http://example.com"), ContentType::Url);
    }

    #[test]
    fn test_content_type_detect_file() {
        assert_eq!(ContentType::detect("/path/to/file.txt"), ContentType::File);
    }

    #[test]
    fn test_clipboard_item_new() {
        let item = ClipboardItem::new("test content".to_string());
        assert_eq!(item.content, "test content");
        assert_eq!(item.content_type, ContentType::Text);
        assert!(!item.is_favorite);
        assert!(item.tags.is_empty());
    }

    #[test]
    fn test_clipboard_item_mark_used() {
        let mut item = ClipboardItem::new("test".to_string());
        let original_last_used = item.last_used_at;
        
        // Small delay to ensure timestamp difference
        std::thread::sleep(std::time::Duration::from_millis(10));
        item.mark_used();
        
        assert!(item.last_used_at > original_last_used);
    }

    #[test]
    fn test_clipboard_item_tags() {
        let mut item = ClipboardItem::new("test".to_string());
        item.add_tag("rust".to_string());
        item.add_tag("clipboard".to_string());
        
        assert_eq!(item.tags.len(), 2);
        assert!(item.tags.contains(&"rust".to_string()));
        
        item.remove_tag("rust");
        assert_eq!(item.tags.len(), 1);
        assert!(!item.tags.contains(&"rust".to_string()));
    }

    #[test]
    fn test_clipboard_item_favorite() {
        let mut item = ClipboardItem::new("test".to_string());
        assert!(!item.is_favorite);
        
        item.set_favorite(true);
        assert!(item.is_favorite);
        
        item.set_favorite(false);
        assert!(!item.is_favorite);
    }

    #[test]
    fn test_clipboard_item_expiration() {
        let mut item = ClipboardItem::new("test".to_string());
        assert!(!item.is_expired());
        
        let expires_at = Utc::now() - chrono::Duration::hours(1);
        item.set_expires_at(Some(expires_at));
        assert!(item.is_expired());
        
        let future = Utc::now() + chrono::Duration::hours(1);
        item.set_expires_at(Some(future));
        assert!(!item.is_expired());
    }
}
