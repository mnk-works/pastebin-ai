use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use ureq;

/// Visibility level for a paste.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PasteVisibility {
    /// Public: listed in public archive, searchable
    Public,
    /// Unlisted: accessible via direct link only (default)
    Unlisted,
    /// Private: only accessible by owner (requires authentication)
    Private,
}

impl Default for PasteVisibility {
    fn default() -> Self {
        PasteVisibility::Unlisted
    }
}

impl std::str::FromStr for PasteVisibility {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "public" => Ok(PasteVisibility::Public),
            "unlisted" => Ok(PasteVisibility::Unlisted),
            "private" => Ok(PasteVisibility::Private),
            _ => Err(format!("Invalid visibility: {}", s)),
        }
    }
}

/// Expiration duration for a paste.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PasteExpiration {
    /// 10 minutes
    #[serde(rename = "10m")]
    Min10,
    /// 1 hour
    #[serde(rename = "1h")]
    Hour1,
    /// 1 day
    #[serde(rename = "1d")]
    Day1,
    /// 1 week (default)
    #[serde(rename = "1w")]
    Week1,
    /// 2 weeks
    #[serde(rename = "2w")]
    Week2,
    /// 1 month
    #[serde(rename = "1m")]
    Month1,
    /// 6 months
    #[serde(rename = "6m")]
    Month6,
    /// 1 year
    #[serde(rename = "1y")]
    Year1,
    /// Never expires
    Never,
}

impl Default for PasteExpiration {
    fn default() -> Self {
        PasteExpiration::Week1
    }
}

impl std::str::FromStr for PasteExpiration {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "10m" => Ok(PasteExpiration::Min10),
            "1h" => Ok(PasteExpiration::Hour1),
            "1d" => Ok(PasteExpiration::Day1),
            "1w" => Ok(PasteExpiration::Week1),
            "2w" => Ok(PasteExpiration::Week2),
            "1m" => Ok(PasteExpiration::Month1),
            "6m" => Ok(PasteExpiration::Month6),
            "1y" => Ok(PasteExpiration::Year1),
            "never" => Ok(PasteExpiration::Never),
            _ => Err(format!("Invalid expiration: {}", s)),
        }
    }
}

/// Category classification for a paste.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PasteCategory {
    /// Source code snippets
    Code,
    /// Log files and stack traces
    Logs,
    /// Configuration files (YAML, TOML, etc.)
    Configuration,
    /// Data files (JSON, CSV, etc.)
    Data,
    /// Documentation and markdown
    Documentation,
    /// Other content
    Other,
}

impl Default for PasteCategory {
    fn default() -> Self {
        PasteCategory::Code
    }
}

impl std::str::FromStr for PasteCategory {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "code" => Ok(PasteCategory::Code),
            "logs" => Ok(PasteCategory::Logs),
            "configuration" => Ok(PasteCategory::Configuration),
            "data" => Ok(PasteCategory::Data),
            "documentation" => Ok(PasteCategory::Documentation),
            "other" => Ok(PasteCategory::Other),
            _ => Err(format!("Invalid category: {}", s)),
        }
    }
}

/// Request payload for creating a new paste.
#[derive(Debug, Serialize, Clone)]
pub struct CreatePasteRequest {
    /// The text content (required, max 1 MB UTF-8)
    pub content: String,
    /// Optional title for the paste
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Syntax highlighting language (e.g., "rust", "python", "json")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Visibility level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<PasteVisibility>,
    /// Expiration duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<PasteExpiration>,
    /// Category classification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<PasteCategory>,
    /// Comma- or space-separated tags (max 8 retained)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Maximum unique views (1-1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_views: Option<u32>,
    /// Password for protected access (4-128 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Delete after first read
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burn_after_read: Option<bool>,
    /// Folder ID (requires auth)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<u64>,
    /// Content is pre-encrypted ciphertext
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
}

/// Response from successful paste creation.
#[derive(Debug, Deserialize)]
pub struct PasteCreated {
    /// Unique paste ID
    pub id: String,
    /// Web UI URL for the paste
    pub url: String,
    /// Raw content URL
    pub raw_url: String,
    /// Token for anonymous deletion (save this!)
    pub delete_token: String,
    /// Expiration timestamp (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Maximum views limit (if set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_views: Option<u32>,
}

/// Paste metadata and content.
#[derive(Debug, Deserialize)]
pub struct Paste {
    /// Unique paste ID
    pub id: String,
    /// Paste title (if set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Paste content
    pub content: String,
    /// Syntax highlighting language
    pub language: String,
    /// Visibility level
    pub visibility: String,
    /// View count
    pub views: u32,
    /// Creation timestamp
    pub created_at: String,
    /// Expiration timestamp (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Maximum views limit (if set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_views: Option<u32>,
    /// Last update timestamp (if updated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Web UI URL (for owned pastes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Category (if set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Tags (if set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Burn after read flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burn_after_read: Option<bool>,
    /// Encrypted flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// Folder ID (if in folder)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    _retry_after: Option<u64>,
}

const BASE_URL: &str = "https://pastebin.ai/api/v1";

/// HTTP client for the pastebin.ai API.
///
/// Supports both anonymous and authenticated operations.
/// Use `new()` for anonymous access, `with_token()` for authenticated access,
/// or `with_delete_token()` to delete an anonymous paste using its delete token.
pub struct PastebinClient {
    agent: ureq::Agent,
    auth_token: Option<String>,
    delete_token: Option<String>,
    delete_paste_id: Option<String>,
}

impl PastebinClient {
    /// Create a new anonymous client.
    pub fn new() -> Result<Self> {
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(10))
            .timeout_read(Duration::from_secs(30))
            .timeout_write(Duration::from_secs(30))
            .user_agent("pastebin-ai-cli/0.1.0")
            .build();
        Ok(Self {
            agent,
            auth_token: None,
            delete_token: None,
            delete_paste_id: None,
        })
    }

    /// Create an authenticated client with a Bearer token.
    ///
    /// Get your token from <https://pastebin.ai/settings/api> (starts with `pba_`).
    pub fn with_token(token: &str) -> Result<Self> {
        let mut client = Self::new()?;
        client.auth_token = Some(token.to_string());
        Ok(client)
    }

    /// Create a client for deleting a specific anonymous paste.
    ///
    /// Use the delete token returned when creating the paste.
    pub fn with_delete_token(paste_id: &str, token: &str) -> Result<Self> {
        let mut client = Self::new()?;
        client.delete_token = Some(token.to_string());
        client.delete_paste_id = Some(paste_id.to_string());
        Ok(client)
    }

    /// Create a new paste.
    ///
    /// Returns [`PasteCreated`] with the paste URL, raw URL, and delete token.
    pub fn create_paste(&self, request: CreatePasteRequest) -> Result<PasteCreated> {
        let req = self.agent.post(&format!("{}/pastes", BASE_URL)).send_json(&request)?;

        let status = req.status();
        let body = req.into_string()?;

        if (200..300).contains(&status) {
            Ok(serde_json::from_str(&body)?)
        } else {
            let err: ErrorResponse = serde_json::from_str(&body).unwrap_or(ErrorResponse {
                error: format!("HTTP {}", status),
                _retry_after: None,
            });
            anyhow::bail!("API error: {}", err.error);
        }
    }

    /// Retrieve a paste by ID.
    ///
    /// Works for public and unlisted pastes. Private/password-protected/encrypted
    /// pastes require authentication and return 404.
    pub fn get_paste(&self, id: &str) -> Result<Paste> {
        let url = format!("{}/pastes/{}", BASE_URL, id);

        let resp = self.agent.get(&url).call()?;

        let status = resp.status();
        let body = resp.into_string()?;

        if (200..300).contains(&status) {
            Ok(serde_json::from_str(&body)?)
        } else if status == 404 {
            anyhow::bail!("Paste not found: {}", id);
        } else {
            let err: ErrorResponse = serde_json::from_str(&body).unwrap_or(ErrorResponse {
                error: format!("HTTP {}", status),
                _retry_after: None,
            });
            anyhow::bail!("API error: {}", err.error);
        }
    }

    /// List pastes owned by the authenticated account.
    ///
    /// Requires authentication via `with_token()`. Returns up to `limit` pastes.
    pub fn list_pastes(&self, limit: usize) -> Result<Vec<Paste>> {
        let token = self.auth_token.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Authentication required"))?;

        let resp = self.agent
            .get(&format!("{}/account/pastes", BASE_URL))
            .set("Authorization", &format!("Bearer {}", token))
            .call()?;

        let status = resp.status();
        let body = resp.into_string()?;

        if (200..300).contains(&status) {
            let pastes: Vec<Paste> = serde_json::from_str(&body)?;
            Ok(pastes.into_iter().take(limit).collect())
        } else {
            let err: ErrorResponse = serde_json::from_str(&body).unwrap_or(ErrorResponse {
                error: format!("HTTP {}", status),
                _retry_after: None,
            });
            anyhow::bail!("API error: {}", err.error);
        }
    }

    /// Delete a paste.
    ///
    /// For anonymous pastes created with this client's delete token, uses the
    /// public delete endpoint with `X-Delete-Token` header.
    /// For authenticated pastes, uses the account endpoint with Bearer token.
    pub fn delete_paste(&self, id: &str) -> Result<()> {
        // Use different endpoint depending on auth type
        let (_url, req) = if let (Some(token), Some(paste_id)) = (&self.delete_token, &self.delete_paste_id) {
            if paste_id == id {
                // Anonymous paste: use public endpoint with X-Delete-Token
                let url = format!("{}/pastes/{}", BASE_URL, id);
                let req = self.agent.delete(&url).set("X-Delete-Token", token);
                (url, req)
            } else if let Some(auth) = &self.auth_token {
                // Wrong delete token for this paste, fall back to auth
                let url = format!("{}/account/pastes/{}", BASE_URL, id);
                let req = self.agent.delete(&url).set("Authorization", &format!("Bearer {}", auth));
                (url, req)
            } else {
                anyhow::bail!("No valid authentication for delete");
            }
        } else if let Some(auth) = &self.auth_token {
            // Authenticated paste: use account endpoint with Bearer token
            let url = format!("{}/account/pastes/{}", BASE_URL, id);
            let req = self.agent.delete(&url).set("Authorization", &format!("Bearer {}", auth));
            (url, req)
        } else {
            anyhow::bail!("No valid authentication for delete");
        };

        let resp = req.call()?;
        let status = resp.status();

        if (200..300).contains(&status) || status == 204 {
            Ok(())
        } else if status == 404 {
            anyhow::bail!("Paste not found: {}", id);
        } else {
            let body = resp.into_string()?;
            let err: ErrorResponse = serde_json::from_str(&body).unwrap_or(ErrorResponse {
                error: format!("HTTP {}", status),
                _retry_after: None,
            });
            anyhow::bail!("API error: {}", err.error);
        }
    }
}