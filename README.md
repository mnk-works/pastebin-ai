# pastebin-ai 📄✨

A pretty CLI for [pastebin.ai](https://pastebin.ai) — share code, logs, and text from your terminal with beautiful output.

```
✨ Paste created!
──────────────────────────────────────────────────
🔗 https://pastebin.ai/p/abc123
📄 https://pastebin.ai/raw/abc123
⏰ Expires: 2026-11-13T17:30:19Z
🗑️ Delete token: abc123...
```

## Features

- 🎨 **Beautiful output** — colors, emojis, spinners
- ⚡ **Fast** — built with Rust, minimal dependencies
- 🔐 **Authentication** — secure token storage
- 📋 **Clipboard integration** — copy URLs/content with `-c`
- 📁 **File input** — `pastebin-ai create -f main.rs`
- 📥 **Stdin support** — `cat file.txt | pastebin-ai create`
- 🔍 **Get pastes** — fetch by ID or URL
- 📋 **List pastes** — see your history (with auth)
- 🗑️ **Delete pastes** — with token or auth
- 🤫 **Quiet mode** — just the URL for scripting

## Installation

### Cargo (recommended)

```bash
cargo install pastebin-ai
```

### Binary releases

Download from [GitHub Releases](https://github.com/mnk-works/pastebin-ai/releases).

### Build from source

```bash
git clone https://github.com/mnk-works/pastebin-ai
cd pastebin-ai
cargo install --path .
```

## Quick Start

```bash
# Create a paste from stdin
echo "hello world" | pastebin-ai create

# Create from a file with syntax highlighting
pastebin-ai create -f main.rs -l rust

# Create with options
pastebin-ai create "code here" -t "My Title" -l python -v public -e 1d

# Get a paste
pastebin-ai get abc123

# Get raw content only
pastebin-ai get abc123 --raw

# List your pastes (requires auth)
pastebin-ai auth login pba_your_token
pastebin-ai list

# Delete a paste
pastebin-ai delete abc123 --token delete_token_here
```

## Commands

| Command | Description |
|---------|-------------|
| `create` | Create a new paste |
| `get` | Read a paste by ID |
| `list` | List your pastes (auth required) |
| `delete` | Delete a paste |
| `auth` | Manage authentication |
| `config` | Show configuration |

## Options

### Create Options

```bash
pastebin-ai create [CONTENT] [OPTIONS]

Options:
  -f, --file <FILE>           Read content from file
  -t, --title <TITLE>         Paste title
  -l, --language <LANG>       Syntax highlighting (default: plaintext)
  -v, --visibility <VIS>      public | unlisted | private (default: unlisted)
  -e, --expiration <EXP>      10m | 1h | 1d | 1w | 2w | 1m | 6m | 1y | never (default: 1w)
  -c, --category <CAT>        code | logs | configuration | data | documentation | other (default: code)
  --tags <TAGS>               Comma-separated tags
  --max-views <N>             Maximum views (1-1000)
  --password <PASS>           Password protect
  --burn-after-read           Delete after first view
  --copy                      Copy URL to clipboard
  -q, --quiet                 Output only URL
```

### Global Options

```bash
--no-color    Disable colored output
--no-emoji    Disable emoji output
```

## Authentication

Get your API token from [pastebin.ai/settings/api](https://pastebin.ai/settings/api) (starts with `pba_`):

```bash
pastebin-ai auth login pba_your_token_here
```

Tokens are stored in `~/.config/pastebin-ai/config.toml`.

## Examples

```bash
# Share a Rust file with syntax highlighting
pastebin-ai create -f src/main.rs -l rust -t "My Rust snippet"

# Pipe command output
cargo build 2>&1 | pastebin-ai create -t "Build log" -l text -c logs

# Create a temporary paste (10 minutes)
pastebin-ai create "temp data" -e 10m

# Create a password-protected paste
pastebin-ai create "secret" --password hunter2

# Burn after read (self-destructing)
pastebin-ai create "one-time secret" --burn-after-read

# Script-friendly: just the URL
url=$(pastebin-ai create "content" -q)
echo "Share: $url"
```

## Configuration

Config file: `~/.config/pastebin-ai/config.toml`

```toml
auth = { token = "pba_..." }
```

## License

MIT OR Apache-2.0