use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use ureq;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PasteVisibility {
    Public,
    Unlisted,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PasteExpiration {
    #[serde(rename = "10m")]
    Min10,
    #[serde(rename = "1h")]
    Hour1,
    #[serde(rename = "1d")]
    Day1,
    #[serde(rename = "1w")]
    Week1,
    #[serde(rename = "2w")]
    Week2,
    #[serde(rename = "1m")]
    Month1,
    #[serde(rename = "6m")]
    Month6,
    #[serde(rename = "1y")]
    Year1,
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

impl PasteExpiration {
    pub fn as_str(&self) -> &'static str {
        match self {
            PasteExpiration::Min10 => "10m",
            PasteExpiration::Hour1 => "1h",
            PasteExpiration::Day1 => "1d",
            PasteExpiration::Week1 => "1w",
            PasteExpiration::Week2 => "2w",
            PasteExpiration::Month1 => "1m",
            PasteExpiration::Month6 => "6m",
            PasteExpiration::Year1 => "1y",
            PasteExpiration::Never => "never",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PasteCategory {
    Code,
    Logs,
    Configuration,
    Data,
    Documentation,
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

#[derive(Debug, Serialize)]
pub struct CreatePasteRequest {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<PasteVisibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<PasteExpiration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<PasteCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_views: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burn_after_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PasteCreated {
    pub id: String,
    pub url: String,
    pub raw_url: String,
    pub delete_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_views: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Paste {
    pub id: String,
    pub title: Option<String>,
    pub content: String,
    pub language: String,
    pub visibility: String,
    pub views: u32,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_views: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burn_after_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_after: Option<u64>,
}

const BASE_URL: &str = "https://pastebin.ai/api/v1";

pub struct PastebinClient {
    agent: ureq::Agent,
    auth_token: Option<String>,
    delete_token: Option<String>,
    delete_paste_id: Option<String>,
}

impl PastebinClient {
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

    pub fn with_token(token: &str) -> Result<Self> {
        let mut client = Self::new()?;
        client.auth_token = Some(token.to_string());
        Ok(client)
    }

    pub fn with_delete_token(paste_id: &str, token: &str) -> Result<Self> {
        let mut client = Self::new()?;
        client.delete_token = Some(token.to_string());
        client.delete_paste_id = Some(paste_id.to_string());
        Ok(client)
    }

    fn request_with_auth(&self, req: ureq::Request) -> ureq::Request {
        if let Some(token) = &self.auth_token {
            req.set("Authorization", &format!("Bearer {}", token))
        } else {
            req
        }
    }

    pub fn create_paste(&self, request: CreatePasteRequest) -> Result<PasteCreated> {
        let mut req = self.agent.post(&format!("{}/pastes", BASE_URL)).send_json(&request)?;

        let status = req.status();
        let body = req.into_string()?;

        if (200..300).contains(&status) {
            Ok(serde_json::from_str(&body)?)
        } else {
            let err: ErrorResponse = serde_json::from_str(&body).unwrap_or(ErrorResponse {
                error: format!("HTTP {}", status),
                retry_after: None,
            });
            anyhow::bail!("API error: {}", err.error);
        }
    }

    pub fn get_paste(&self, id: &str) -> Result<Paste> {
        let resp = self.agent.get(&format!("{}/pastes/{}", BASE_URL, id)).call()?;

        let status = resp.status();
        let body = resp.into_string()?;

        if (200..300).contains(&status) {
            Ok(serde_json::from_str(&body)?)
        } else if status == 404 {
            anyhow::bail!("Paste not found: {}", id);
        } else {
            let err: ErrorResponse = serde_json::from_str(&body).unwrap_or(ErrorResponse {
                error: format!("HTTP {}", status),
                retry_after: None,
            });
            anyhow::bail!("API error: {}", err.error);
        }
    }

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
                retry_after: None,
            });
            anyhow::bail!("API error: {}", err.error);
        }
    }

    pub fn delete_paste(&self, id: &str) -> Result<()> {
        let mut req = self.agent.delete(&format!("{}/account/pastes/{}", BASE_URL, id));

        if let (Some(token), Some(paste_id)) = (&self.delete_token, &self.delete_paste_id) {
            if paste_id == id {
                req = req.set("X-Delete-Token", token);
            } else if let Some(auth) = &self.auth_token {
                req = req.set("Authorization", &format!("Bearer {}", auth));
            } else {
                anyhow::bail!("No valid authentication for delete");
            }
        } else if let Some(auth) = &self.auth_token {
            req = req.set("Authorization", &format!("Bearer {}", auth));
        } else {
            anyhow::bail!("No valid authentication for delete");
        }

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
                retry_after: None,
            });
            anyhow::bail!("API error: {}", err.error);
        }
    }
}