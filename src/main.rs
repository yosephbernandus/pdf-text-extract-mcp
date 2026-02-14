use rmcp::{
    ErrorData as McpError, ServerHandler, ServiceExt,
    handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::*,
    schemars, tool, tool_handler, tool_router,
    transport::stdio,
};
use tracing_subscriber::{self, EnvFilter};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct FilePathParam {
    /// Absolute path to the PDF file
    file_path: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct ExtractPageParam {
    /// Absolute path to the PDF file
    file_path: String,
    /// Page number (0-indexed)
    page: usize,
    /// Output format: "text", "markdown", or "csv"
    format: String,
}

#[derive(Clone)]
struct PdfMcpServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl PdfMcpServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Extract all text from a PDF file as plain text (layout-aware with headings, paragraphs, and tables)")]
    async fn pdf_to_text(
        &self,
        Parameters(params): Parameters<FilePathParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = std::fs::read(&params.file_path).map_err(|e| {
            McpError::invalid_params(format!("Failed to read file '{}': {}", params.file_path, e), None)
        })?;
        let text = pdf_text_extract::pdf_to_text(&data).map_err(|e| {
            McpError::internal_error(format!("Failed to extract text: {}", e), None)
        })?;
        Ok(CallToolResult::success(vec![Content::text(text)]))
    }

    #[tool(description = "Extract all text from a PDF file as Markdown with # headings and | tables")]
    async fn pdf_to_markdown(
        &self,
        Parameters(params): Parameters<FilePathParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = std::fs::read(&params.file_path).map_err(|e| {
            McpError::invalid_params(format!("Failed to read file '{}': {}", params.file_path, e), None)
        })?;
        let md = pdf_text_extract::pdf_to_markdown(&data).map_err(|e| {
            McpError::internal_error(format!("Failed to extract markdown: {}", e), None)
        })?;
        Ok(CallToolResult::success(vec![Content::text(md)]))
    }

    #[tool(description = "Extract all text from a PDF file as CSV (best for tabular PDFs like bank statements)")]
    async fn pdf_to_csv(
        &self,
        Parameters(params): Parameters<FilePathParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = std::fs::read(&params.file_path).map_err(|e| {
            McpError::invalid_params(format!("Failed to read file '{}': {}", params.file_path, e), None)
        })?;
        let csv = pdf_text_extract::pdf_to_csv(&data).map_err(|e| {
            McpError::internal_error(format!("Failed to extract CSV: {}", e), None)
        })?;
        Ok(CallToolResult::success(vec![Content::text(csv)]))
    }

    #[tool(description = "Get the number of pages in a PDF file")]
    async fn pdf_page_count(
        &self,
        Parameters(params): Parameters<FilePathParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = std::fs::read(&params.file_path).map_err(|e| {
            McpError::invalid_params(format!("Failed to read file '{}': {}", params.file_path, e), None)
        })?;
        let mut doc = pdf_text_extract::Document::parse(&data).map_err(|e| {
            McpError::internal_error(format!("Failed to parse PDF: {}", e), None)
        })?;
        let count = doc.page_count().map_err(|e| {
            McpError::internal_error(format!("Failed to get page count: {}", e), None)
        })?;
        Ok(CallToolResult::success(vec![Content::text(count.to_string())]))
    }

    #[tool(description = "Extract a single page from a PDF in a specified format (text, markdown, or csv)")]
    async fn pdf_extract_page(
        &self,
        Parameters(params): Parameters<ExtractPageParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = std::fs::read(&params.file_path).map_err(|e| {
            McpError::invalid_params(format!("Failed to read file '{}': {}", params.file_path, e), None)
        })?;
        let mut doc = pdf_text_extract::Document::parse(&data).map_err(|e| {
            McpError::internal_error(format!("Failed to parse PDF: {}", e), None)
        })?;
        let page_count = doc.page_count().map_err(|e| {
            McpError::internal_error(format!("Failed to get page count: {}", e), None)
        })?;
        if params.page >= page_count {
            return Err(McpError::invalid_params(
                format!("Page {} out of range (document has {} pages)", params.page, page_count),
                None,
            ));
        }

        let spans = doc.extract_page_text(params.page).map_err(|e| {
            McpError::internal_error(format!("Failed to extract page {}: {}", params.page, e), None)
        })?;

        let output = match params.format.as_str() {
            "text" => {
                let elements = pdf_text_extract::classify_spans(spans);
                pdf_text_extract::elements_to_txt(&elements)
            }
            "markdown" => {
                let elements = pdf_text_extract::classify_spans(spans);
                pdf_text_extract::elements_to_markdown(&elements)
            }
            "csv" => {
                let table = pdf_text_extract::Table::from_spans(spans);
                table.to_csv()
            }
            other => {
                return Err(McpError::invalid_params(
                    format!("Unknown format '{}'. Use 'text', 'markdown', or 'csv'", other),
                    None,
                ));
            }
        };

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }
}

#[tool_handler]
impl ServerHandler for PdfMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name: "pdf-text-extract".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                title: None,
                description: None,
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "PDF text extraction server. Tools: pdf_to_text, pdf_to_markdown, pdf_to_csv, pdf_page_count, pdf_extract_page."
                    .into(),
            ),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting pdf-text-extract MCP server");

    let service = PdfMcpServer::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("serving error: {:?}", e);
        })?;

    service.waiting().await?;
    Ok(())
}
