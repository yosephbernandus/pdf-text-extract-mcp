# pdf-text-extract-mcp

MCP server that gives LLMs the ability to read PDF files. Extracts text, tables, and structured content from PDFs and returns it as plain text, Markdown, or CSV.

Built on [pdf-text-extract](https://crates.io/crates/pdf-text-extract) and the [Model Context Protocol](https://modelcontextprotocol.io/). Ships as a single binary with zero runtime dependencies.

## Installation

### Install script (macOS / Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/yosephbernandus/pdf-text-extract-mcp/master/install.sh | sh
```

### Cargo (requires Rust)

```bash
cargo install pdf-text-extract-mcp
```

### Manual download

Download the binary for your platform from the [Releases](https://github.com/yosephbernandus/pdf-text-extract-mcp/releases/latest) page:

| Platform       | Binary                     |
|----------------|----------------------------|
| Linux x86_64   | `pdf-mcp-linux-x86_64`    |
| Linux ARM64    | `pdf-mcp-linux-aarch64`   |
| macOS Intel    | `pdf-mcp-darwin-x86_64`   |
| macOS Apple Silicon | `pdf-mcp-darwin-aarch64` |
| Windows x86_64 | `pdf-mcp-windows-x86_64.exe` |

Then make it executable and move it to your PATH:

```bash
chmod +x pdf-mcp-*
sudo mv pdf-mcp-* /usr/local/bin/pdf-mcp
```

## Setup with Claude Code

```bash
claude mcp add pdf-text-extract -- pdf-mcp
```

If you installed the binary to a custom location:

```bash
claude mcp add pdf-text-extract -- /path/to/pdf-mcp
```

## Tools

### `pdf_to_text`

Extract all text from a PDF as plain text (layout-aware with headings, paragraphs, and tables).

```json
{ "file_path": "/path/to/file.pdf" }
```

### `pdf_to_markdown`

Extract all text from a PDF as Markdown with `#` headings and `|` tables. Best for structured documents like reports and invoices.

```json
{ "file_path": "/path/to/file.pdf" }
```

### `pdf_to_csv`

Extract all text from a PDF as CSV. Best for tabular PDFs like bank statements and transaction lists.

```json
{ "file_path": "/path/to/file.pdf" }
```

### `pdf_page_count`

Get the number of pages in a PDF.

```json
{ "file_path": "/path/to/file.pdf" }
```

### `pdf_extract_page`

Extract a single page in a specified format. Useful for large PDFs where you want to process one page at a time.

```json
{ "file_path": "/path/to/file.pdf", "page": 0, "format": "markdown" }
```

- `page` is 0-indexed
- `format` accepts `"text"`, `"markdown"`, or `"csv"`

## Building from source

```bash
git clone https://github.com/yosephbernandus/pdf-text-extract-mcp.git
cd pdf-text-extract-mcp
cargo build --release
```

The binary will be at `target/release/pdf-mcp`.

## License

MIT
