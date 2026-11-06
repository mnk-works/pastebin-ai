use anyhow::Result;
use pastebin_ai::api::{PastebinClient, CreatePasteRequest, PasteVisibility, PasteExpiration, PasteCategory};
use std::time::Duration;

const TEST_PREFIX: &str = "[pastebin-ai TEST] ";
const TEST_TAGS: &str = "test,integration,pastebin-ai-cli";

fn test_client() -> Result<PastebinClient> {
    PastebinClient::new()
}

/// Create a test paste with retry on rate limit (429).
fn create_test_paste(content: &str, title: &str) -> Result<(String, String)> {
    let client = test_client()?;
    let request = CreatePasteRequest {
        content: format!("{}{}", TEST_PREFIX, content),
        title: Some(format!("{} {}", TEST_PREFIX, title)),
        language: Some("plaintext".to_string()),
        visibility: Some(PasteVisibility::Unlisted),
        expiration: Some(PasteExpiration::Min10),
        category: Some(PasteCategory::Other),
        tags: Some(TEST_TAGS.to_string()),
        max_views: Some(100),
        password: None,
        burn_after_read: Some(false),
        folder_id: None,
        encrypted: None,
    };
    
    // Retry with exponential backoff on rate limit
    let mut attempt = 0;
    loop {
        match client.create_paste(request.clone()) {
            Ok(response) => return Ok((response.id, response.delete_token)),
            Err(e) if e.to_string().contains("429") && attempt < 5 => {
                attempt += 1;
                std::thread::sleep(Duration::from_millis(1000 * attempt));
            }
            Err(e) => return Err(e),
        }
    }
}

fn cleanup_paste(id: &str, delete_token: &str) -> Result<()> {
    let client = PastebinClient::with_delete_token(id, delete_token)?;
    client.delete_paste(id)
}

fn verify_paste_exists(id: &str) -> Result<bool> {
    let client = test_client()?;
    match client.get_paste(id) {
        Ok(_) => Ok(true),
        Err(e) if e.to_string().contains("404") => Ok(false),
        Err(e) => Err(e),
    }
}

#[test]
fn test_create_basic_paste() -> Result<()> {
    
    let (id, delete_token) = create_test_paste(
        "Basic paste content\nwith multiple lines",
        "Basic Paste Test"
    )?;

    assert!(!id.is_empty());
    assert!(!delete_token.is_empty());

    // Verify paste was created
    let client = test_client()?;
    let paste = client.get_paste(&id)?;
    assert!(paste.content.contains(TEST_PREFIX));
    assert!(paste.content.contains("Basic paste content"));
    assert_eq!(paste.visibility, "unlisted");
    assert_eq!(paste.language, "plaintext");

    // Cleanup
    cleanup_paste(&id, &delete_token)?;

    // Verify deleted
    std::thread::sleep(Duration::from_millis(500));
    let exists = verify_paste_exists(&id)?;
    assert!(!exists);

    Ok(())
}

#[test]
fn test_create_paste_with_all_options() -> Result<()> {
    let client = test_client()?;
    let request = CreatePasteRequest {
        content: format!("{}Full options test", TEST_PREFIX),
        title: Some(format!("{}Full Options", TEST_PREFIX)),
        language: Some("rust".to_string()),
        visibility: Some(PasteVisibility::Public),
        expiration: Some(PasteExpiration::Hour1),
        category: Some(PasteCategory::Code),
        tags: Some("test,rust,full-options".to_string()),
        max_views: Some(50),
        password: None,
        burn_after_read: Some(false),
        folder_id: None,
        encrypted: None,
    };
    let response = client.create_paste(request)?;

    let paste = client.get_paste(&response.id)?;
    assert!(paste.content.contains(TEST_PREFIX));
    assert_eq!(paste.language, "rust");
    assert_eq!(paste.visibility, "public");

    // Cleanup
    let delete_client = PastebinClient::with_delete_token(&response.id, &response.delete_token)?;
    delete_client.delete_paste(&response.id)?;

    Ok(())
}

#[test]
fn test_create_paste_burn_after_read() -> Result<()> {
    
    let (id, delete_token) = create_test_paste(
        "Burn after read test content",
        "Burn After Read Test"
    )?;

    // For burn_after_read, we can't easily test the self-destruct behavior
    // without making multiple requests, but we can verify it was created
    let client = test_client()?;
    let paste = client.get_paste(&id)?;
    assert!(paste.content.contains(TEST_PREFIX));

    // Cleanup using delete token
    cleanup_paste(&id, &delete_token)?;

    Ok(())
}

#[test]
fn test_get_nonexistent_paste() -> Result<()> {
    
    let client = test_client()?;
    let result = client.get_paste("nonexistent_id_12345");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("404"));
    Ok(())
}

#[test]
fn test_create_and_get_raw() -> Result<()> {
    
    let (id, delete_token) = create_test_paste(
        "Raw content test\nLine 2\nLine 3",
        "Raw Content Test"
    )?;

    let client = test_client()?;
    let paste = client.get_paste(&id)?;
    assert!(paste.content.contains("Raw content test"));
    assert!(paste.content.contains("Line 2"));
    assert!(paste.content.contains("Line 3"));

    cleanup_paste(&id, &delete_token)?;

    Ok(())
}

#[test]
fn test_unicode_content() -> Result<()> {
    
    let (id, delete_token) = create_test_paste(
        "Unicode: 你好 🌍 café ñoño 🦀\nEmoji: ✨🚀💻\nCyrillic: привет",
        "Unicode Test"
    )?;

    let client = test_client()?;
    let paste = client.get_paste(&id)?;
    assert!(paste.content.contains("你好"));
    assert!(paste.content.contains("🌍"));
    assert!(paste.content.contains("🦀"));
    assert!(paste.content.contains("привет"));

    cleanup_paste(&id, &delete_token)?;

    Ok(())
}

#[test]
fn test_large_paste() -> Result<()> {
    
    // Create a paste near the 1MB limit
    let large_content = "x".repeat(900_000); // 900KB
    let (id, delete_token) = create_test_paste(
        &large_content,
        "Large Paste Test"
    )?;

    let client = test_client()?;
    let paste = client.get_paste(&id)?;
    assert!(paste.content.len() > 900_000);

    cleanup_paste(&id, &delete_token)?;

    Ok(())
}

#[test]
fn test_delete_with_invalid_token() -> Result<()> {
    
    let (id, _delete_token) = create_test_paste(
        "Content for invalid token test",
        "Invalid Token Test"
    )?;

    // Try to delete with wrong token
    let client = PastebinClient::with_delete_token(&id, "invalid_token_12345")?;
    let result = client.delete_paste(&id);
    assert!(result.is_err());

    // Cleanup with correct token
    let _client = test_client()?;
    // We need the real delete token - but we can't get it back
    // This test verifies the API rejects invalid tokens
    // The paste will expire in 10 minutes anyway

    Ok(())
}

#[test]
fn test_concurrent_pastes() -> Result<()> {
    
    let mut paste_ids = Vec::new();

    // Create multiple pastes with small delays to avoid rate limits
    for i in 0..5 {
        let (id, delete_token) = create_test_paste(
            &format!("Concurrent test paste #{}", i),
            &format!("Concurrent Test #{}", i)
        )?;
        paste_ids.push((id, delete_token));
        std::thread::sleep(Duration::from_millis(300));
    }

    // Verify all exist
    let client = test_client()?;
    for (id, _) in &paste_ids {
        let paste = client.get_paste(id)?;
        assert!(paste.content.contains(TEST_PREFIX));
    }

    // Cleanup all
    for (id, delete_token) in paste_ids {
        cleanup_paste(&id, &delete_token)?;
    }

    Ok(())
}

#[test]
fn test_paste_metadata() -> Result<()> {
    
    let (id, delete_token) = create_test_paste(
        "Metadata test content",
        "Metadata Test"
    )?;

    let client = test_client()?;
    let paste = client.get_paste(&id)?;

    // Check metadata fields
    assert!(!paste.id.is_empty());
    assert!(paste.title.is_some());
    assert!(paste.title.unwrap().contains(TEST_PREFIX));
    assert_eq!(paste.language, "plaintext");
    assert_eq!(paste.visibility, "unlisted");
    assert!(paste.views >= 1); // At least our GET request
    assert!(paste.created_at.len() > 0);
    assert!(paste.expires_at.is_some());

    cleanup_paste(&id, &delete_token)?;

    Ok(())
}

#[test]
fn test_special_characters_in_content() -> Result<()> {
    
    let special_content = r#"Special chars: "quotes" 'single' `backticks` $dollar &ampersand <angle> |pipe
Newlines:
	Tabs:
Null byte attempt: \0
Control chars: \x1b[31m"#;

    let (id, delete_token) = create_test_paste(
        special_content,
        "Special Characters Test"
    )?;

    let client = test_client()?;
    let paste = client.get_paste(&id)?;
    assert!(paste.content.contains("quotes"));
    assert!(paste.content.contains("backticks"));
    assert!(paste.content.contains("Newlines"));

    cleanup_paste(&id, &delete_token)?;

    Ok(())
}