use std::io::{self, Read, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

mod api;
mod config;

use api::{PastebinClient, CreatePasteRequest, Paste, PasteVisibility, PasteExpiration, PasteCategory};
use config::{Config, AuthConfig};

#[derive(Parser)]
#[command(
    name = "pastebin-ai",
    version,
    about = "A pretty CLI for pastebin.ai ✨",
    long_about = "Share code, logs, and text with pastebin.ai from your terminal.\nBeautiful output, zero friction."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,

    /// Disable emoji output
    #[arg(long, global = true)]
    no_emoji: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new paste
    Create(CreateArgs),

    /// Read a paste by ID
    Get(GetArgs),

    /// List your pastes (requires auth)
    List(ListArgs),

    /// Delete a paste (requires auth or delete token)
    Delete(DeleteArgs),

    /// Configure authentication
    Auth(AuthArgs),

    /// Show current configuration
    Config,
}

#[derive(Parser)]
struct CreateArgs {
    /// Content to paste (reads from stdin if not provided)
    #[arg(value_name = "CONTENT")]
    content: Option<String>,

    /// Read content from a file
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Paste title
    #[arg(short, long, value_name = "TITLE")]
    title: Option<String>,

    /// Syntax highlighting language
    #[arg(short, long, value_name = "LANG", default_value = "plaintext")]
    language: String,

    /// Visibility: public, unlisted, private
    #[arg(short, long, value_enum, default_value = "unlisted")]
    visibility: PasteVisibility,

    /// Expiration: 10m, 1h, 1d, 1w, 2w, 1m, 6m, 1y, never
    #[arg(short, long, value_enum, default_value = "1w")]
    expiration: PasteExpiration,

    /// Category: code, logs, configuration, data, documentation, other
    #[arg(long, value_enum, default_value = "code")]
    category: PasteCategory,

    /// Tags (comma-separated)
    #[arg(long, value_name = "TAGS")]
    tags: Option<String>,

    /// Maximum views (1-1000)
    #[arg(long, value_name = "N")]
    max_views: Option<u32>,

    /// Password protect the paste
    #[arg(long, value_name = "PASSWORD")]
    password: Option<String>,

    /// Burn after first read
    #[arg(long)]
    burn_after_read: bool,

    /// Copy URL to clipboard
    #[arg(short, long)]
    copy: bool,

    /// Output only the URL (for scripting)
    #[arg(long)]
    quiet: bool,
}

#[derive(Parser)]
struct GetArgs {
    /// Paste ID or URL
    #[arg(value_name = "ID")]
    id: String,

    /// Show raw content only
    #[arg(short, long)]
    raw: bool,

    /// Copy content to clipboard
    #[arg(short, long)]
    copy: bool,
}

#[derive(Parser)]
struct ListArgs {
    /// Maximum number of pastes to show
    #[arg(short, long, default_value = "20")]
    limit: usize,
}

#[derive(Parser)]
struct DeleteArgs {
    /// Paste ID
    #[arg(value_name = "ID")]
    id: String,

    /// Delete token (for anonymous pastes)
    #[arg(long, value_name = "TOKEN")]
    token: Option<String>,
}

#[derive(Parser)]
enum AuthCommand {
    /// Login with API token
    Login {
        /// API token (get from pastebin.ai/settings/api)
        #[arg(value_name = "TOKEN")]
        token: String,
    },
    /// Logout (remove stored token)
    Logout,
    /// Show current auth status
    Status,
}

#[derive(Parser)]
struct AuthArgs {
    #[command(subcommand)]
    command: AuthCommand,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.no_color {
        colored::control::set_override(false);
    }

    let emoji = !cli.no_emoji;

    match cli.command {
        Commands::Create(args) => cmd_create(args, emoji),
        Commands::Get(args) => cmd_get(args, emoji),
        Commands::List(args) => cmd_list(args, emoji),
        Commands::Delete(args) => cmd_delete(args, emoji),
        Commands::Auth(args) => cmd_auth(args, emoji),
        Commands::Config => cmd_config(emoji),
    }
}

fn cmd_create(args: CreateArgs, emoji: bool) -> Result<()> {
    let content = get_content(args.content, args.file)?;

    let client = PastebinClient::new()?;

    let spinner = if !args.quiet {
        Some(ProgressBar::new_spinner().with_style(
            ProgressStyle::default_spinner().template("{spinner:.cyan} {msg}")
        ))
    } else {
        None
    };

    if let Some(sp) = &spinner {
        sp.set_message("Creating paste...".to_string());
        sp.enable_steady_tick(100);
    }

    let request = CreatePasteRequest {
        content,
        title: args.title,
        language: Some(args.language),
        visibility: Some(args.visibility),
        expiration: Some(args.expiration),
        category: Some(args.category),
        tags: args.tags,
        max_views: args.max_views,
        password: args.password,
        burn_after_read: Some(args.burn_after_read),
        folder_id: None,
        encrypted: None,
    };

    let response = client.create_paste(request)?;

    if let Some(sp) = spinner {
        sp.finish_and_clear();
    }

    if args.quiet {
        println!("{}", response.url);
    } else {
        print_success(emoji, &response);
    }

    if args.copy {
        copy_to_clipboard(&response.url)?;
        if !args.quiet {
            println!("{}", "📋 Copied to clipboard!".green());
        }
    }

    Ok(())
}

fn cmd_get(args: GetArgs, emoji: bool) -> Result<()> {
    let client = PastebinClient::new()?;
    let id = extract_paste_id(&args.id);

    let spinner = ProgressBar::new_spinner().with_style(
        ProgressStyle::default_spinner().template("{spinner:.cyan} {msg}")
    );
    spinner.set_message(format!("Fetching paste {}...", id));
    spinner.enable_steady_tick(100);

    let paste = client.get_paste(&id)?;

    spinner.finish_and_clear();

    if args.raw {
        print!("{}", paste.content);
    } else {
        print_paste(emoji, &paste);
    }

    if args.copy {
        copy_to_clipboard(&paste.content)?;
        if !args.raw {
            println!("\n{}", "📋 Copied to clipboard!".green());
        }
    }

    Ok(())
}

fn cmd_list(args: ListArgs, emoji: bool) -> Result<()> {
    let config = Config::load()?;
    let token = config.auth.as_ref().map(|a| a.token.as_str());

    if token.is_none() {
        println!("{}", "🔐 Authentication required. Run `pastebin-ai auth login <token>`".yellow());
        return Ok(());
    }

    let client = PastebinClient::with_token(token.unwrap())?;

    let spinner = ProgressBar::new_spinner().with_style(
        ProgressStyle::default_spinner().template("{spinner:.cyan} {msg}")
    );
    spinner.set_message("Fetching your pastes...".to_string());
    spinner.enable_steady_tick(100);

    let pastes = client.list_pastes(args.limit)?;

    spinner.finish_and_clear();

    if pastes.is_empty() {
        println!("{}", "📭 No pastes found".yellow());
        return Ok(());
    }

    println!("{}", "📋 Your Pastes".bold().cyan());
    println!("{}", "─".repeat(60).dimmed());

    for paste in pastes {
        print_paste_summary(emoji, &paste);
    }

    Ok(())
}

fn cmd_delete(args: DeleteArgs, emoji: bool) -> Result<()> {
    let id = extract_paste_id(&args.id);
    let config = Config::load()?;

    let client = if let Some(token) = args.token {
        PastebinClient::with_delete_token(&id, &token)?
    } else if let Some(auth) = config.auth {
        PastebinClient::with_token(&auth.token)?
    } else {
        println!("{}", "🔐 Authentication required. Provide --token or run `pastebin-ai auth login`".yellow());
        return Ok(());
    };

    let spinner = ProgressBar::new_spinner().with_style(
        ProgressStyle::default_spinner().template("{spinner:.cyan} {msg}")
    );
    spinner.set_message(format!("Deleting paste {}...", id));
    spinner.enable_steady_tick(100);

    client.delete_paste(&id)?;

    spinner.finish_and_clear();

    println!("{}", format!("{} Paste deleted successfully", if emoji { "✅" } else { "✓" }).green());

    Ok(())
}

fn cmd_auth(args: AuthArgs, emoji: bool) -> Result<()> {
    let mut config = Config::load()?;

    match args.command {
        AuthCommand::Login { token } => {
            if !token.starts_with("pba_") {
                println!("{}", "⚠️  Warning: Token doesn't look like a pastebin.ai token (should start with pba_)".yellow());
            }
            config.auth = Some(AuthConfig { token });
            config.save()?;
            println!("{}", format!("{} Logged in successfully!", if emoji { "🔐" } else { "✓" }).green());
        }
        AuthCommand::Logout => {
            config.auth = None;
            config.save()?;
            println!("{}", format!("{} Logged out", if emoji { "👋" } else { "✓" }).green());
        }
        AuthCommand::Status => {
            if config.auth.is_some() {
                println!("{}", format!("{} Authenticated", if emoji { "🔐" } else { "✓" }).green());
            } else {
                println!("{}", "Not authenticated".yellow());
            }
        }
    }

    Ok(())
}

fn cmd_config(_emoji: bool) -> Result<()> {
    let config = Config::load()?;

    println!("{}", "⚙️  Configuration".bold().cyan());
    println!("{}", "─".repeat(40).dimmed());

    if let Some(auth) = &config.auth {
        let masked = format!("{}****", &auth.token[..4.min(auth.token.len())]);
        println!("{}: {}", "Auth Token".bold(), masked.cyan());
    } else {
        println!("{}: {}", "Auth Token".bold(), "Not set".yellow());
    }

    Ok(())
}

fn get_content(content_arg: Option<String>, file_arg: Option<PathBuf>) -> Result<String> {
    if let Some(file) = file_arg {
        std::fs::read_to_string(file).context("Failed to read file")
    } else if let Some(content) = content_arg {
        Ok(content)
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        if buffer.trim().is_empty() {
            anyhow::bail!("No content provided. Use argument, --file, or pipe to stdin.");
        }
        Ok(buffer)
    }
}

fn extract_paste_id(input: &str) -> String {
    if input.contains("pastebin.ai") {
        input.split('/').last().unwrap_or(input).to_string()
    } else {
        input.to_string()
    }
}

fn print_success(emoji: bool, resp: &api::PasteCreated) {
    let check = if emoji { "✨" } else { "✓" };
    println!("{} {}", check.bold().green(), "Paste created!".bold().green());
    println!("{}", "─".repeat(50).dimmed());
    println!("{} {}", "🔗".cyan(), resp.url.bold().underline());
    println!("{} {}", "📄".cyan(), resp.raw_url.dimmed());
    if let Some(expires) = &resp.expires_at {
        println!("{} {} {}", "⏰".cyan(), "Expires:".dimmed(), expires);
    }
    println!("{} {}", "🗑️".cyan(), format!("Delete token: {}", resp.delete_token).dimmed());
}

fn print_paste(emoji: bool, paste: &Paste) {
    let lang = &paste.language;
    println!("{} {}", if emoji { "📄" } else { "" }, paste.title.as_deref().unwrap_or("Untitled").bold());
    println!("{}", "─".repeat(60).dimmed());
    println!("{} {}", "🆔".cyan(), paste.id);
    println!("{} {}", "🏷️".cyan(), lang.cyan());
    println!("{} {}", "👁️".cyan(), paste.views.to_string().dimmed());
    println!("{} {}", "🔒".cyan(), paste.visibility);
    if let Some(expires) = &paste.expires_at {
        println!("{} {}", "⏰".cyan(), expires.dimmed());
    }
    println!("{}", "─".repeat(60).dimmed());
    println!("{}", paste.content);
}

fn print_paste_summary(emoji: bool, paste: &Paste) {
    let title = paste.title.as_deref().unwrap_or("Untitled");
    let lang = &paste.language;
    println!("{} {} {}", if emoji { "📄" } else { "" }, title.bold(), format!("({})", lang).dimmed());
    println!("   {} • {} views • {}", paste.id.cyan(), paste.views.to_string().yellow(), paste.visibility.dimmed());
    if let Some(expires) = &paste.expires_at {
        println!("   {} {}", "⏰".dimmed(), expires.dimmed());
    }
    println!();
}

fn copy_to_clipboard(text: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        let mut child = std::process::Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()?;
        child.stdin.as_mut().unwrap().write_all(text.as_bytes())?;
        child.wait()?;
    }
    #[cfg(target_os = "linux")]
    {
        let mut child = std::process::Command::new("xclip")
            .args(["-selection", "clipboard"])
            .stdin(std::process::Stdio::piped())
            .spawn()?;
        child.stdin.as_mut().unwrap().write_all(text.as_bytes())?;
        child.wait()?;
    }
    #[cfg(target_os = "windows")]
    {
        let mut child = std::process::Command::new("clip")
            .stdin(std::process::Stdio::piped())
            .spawn()?;
        child.stdin.as_mut().unwrap().write_all(text.as_bytes())?;
        child.wait()?;
    }
    Ok(())
}