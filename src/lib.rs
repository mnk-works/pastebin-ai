//! pastebin-ai library crate
//!
//! This crate provides the core API client for [pastebin.ai](https://pastebin.ai).
//! It includes types and methods for creating, retrieving, and deleting pastes,
//! both anonymously and with authentication.
//!
//! # Example
//!
//! ```no_run
//! use pastebin_ai::api::{PastebinClient, CreatePasteRequest, PasteVisibility, PasteExpiration, PasteCategory};
//!
//! let client = PastebinClient::new().unwrap();
//! let request = CreatePasteRequest {
//!     content: "Hello, pastebin.ai!".to_string(),
//!     title: Some("Test Paste".to_string()),
//!     language: Some("rust".to_string()),
//!     visibility: Some(PasteVisibility::Public),
//!     expiration: Some(PasteExpiration::Day1),
//!     category: Some(PasteCategory::Code),
//!     tags: Some("example,test".to_string()),
//!     max_views: Some(100),
//!     password: None,
//!     burn_after_read: Some(false),
//!     folder_id: None,
//!     encrypted: None,
//! };
//! let response = client.create_paste(request).unwrap();
//! println!("Paste URL: {}", response.url);
//! ```

pub mod api;
pub mod config;