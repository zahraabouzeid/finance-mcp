use std::{
    io,
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

use finance_mcp::server::{self, MCP_PATH};
use reqwest::{StatusCode, header};
use rmcp::{
    ServiceExt,
    model::CallToolRequestParams,
    serde_json::json,
    transport::{
        StreamableHttpClientTransport, streamable_http_client::StreamableHttpClientTransportConfig,
    },
};
use tokio::{net::TcpStream, task::JoinHandle, time::timeout};
use tokio_util::sync::CancellationToken;

const INITIALIZE_REQUEST: &str = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"test","version":"0.0.0"}}}"#;

struct RunningServer {
    address: SocketAddr,
    shutdown: CancellationToken,
    task: JoinHandle<io::Result<()>>,
}

impl RunningServer {
    async fn start() -> Self {
        let listener = server::bind(0).await.unwrap();
        let address = listener.local_addr().unwrap();
        let shutdown = CancellationToken::new();
        let task = tokio::spawn(server::serve(listener, shutdown.clone().cancelled_owned()));
        Self {
            address,
            shutdown,
            task,
        }
    }

    fn url(&self) -> String {
        format!("http://{}{}", self.address, MCP_PATH)
    }

    async fn post_initialize(
        &self,
        extra_header: Option<(header::HeaderName, &str)>,
    ) -> StatusCode {
        let mut request = reqwest::Client::new()
            .post(self.url())
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/json, text/event-stream")
            .body(INITIALIZE_REQUEST);
        if let Some((name, value)) = extra_header {
            request = request.header(name, value);
        }
        request.send().await.unwrap().status()
    }
}

#[tokio::test]
async fn binds_to_loopback_only() {
    let server = RunningServer::start().await;

    assert_eq!(server.address.ip(), Ipv4Addr::LOCALHOST);
}

#[tokio::test]
async fn client_initializes_and_sees_tools_capability() {
    let server = RunningServer::start().await;
    let transport = StreamableHttpClientTransport::from_config(
        StreamableHttpClientTransportConfig::with_uri(server.url()),
    );

    let client = ().serve(transport).await.unwrap();
    let info = client.peer_info().unwrap();

    assert!(info.capabilities.tools.is_some());
    assert_eq!(info.server_info.as_ref().unwrap().name, "finance-mcp");
}

#[tokio::test]
async fn client_lists_server_status_without_required_parameters() {
    let server = RunningServer::start().await;
    let transport = StreamableHttpClientTransport::from_config(
        StreamableHttpClientTransportConfig::with_uri(server.url()),
    );
    let client = ().serve(transport).await.unwrap();

    let tools = client.list_all_tools().await.unwrap();
    let tool = tools
        .iter()
        .find(|tool| tool.name == "server_status")
        .unwrap();

    assert!(tool.description.is_some());
    let required = tool
        .input_schema
        .get("required")
        .and_then(|value| value.as_array())
        .map_or(0, Vec::len);
    assert_eq!(required, 0);
}

#[tokio::test]
async fn client_calls_server_status() {
    let server = RunningServer::start().await;
    let transport = StreamableHttpClientTransport::from_config(
        StreamableHttpClientTransportConfig::with_uri(server.url()),
    );
    let client = ().serve(transport).await.unwrap();

    let result = client
        .call_tool(CallToolRequestParams::new("server_status"))
        .await
        .unwrap();

    assert_ne!(result.is_error, Some(true));
    assert_eq!(
        result.structured_content,
        Some(json!({
            "status": "ok",
            "name": "finance-mcp",
            "version": env!("CARGO_PKG_VERSION"),
        }))
    );
}

#[tokio::test]
async fn accepts_loopback_host_headers() {
    let server = RunningServer::start().await;
    let localhost = format!("localhost:{}", server.address.port());

    assert!(server.post_initialize(None).await.is_success());
    assert!(
        server
            .post_initialize(Some((header::HOST, &localhost)))
            .await
            .is_success()
    );
}

#[tokio::test]
async fn rejects_foreign_host_header() {
    let server = RunningServer::start().await;

    let status = server
        .post_initialize(Some((header::HOST, "attacker.example")))
        .await;

    assert!(status.is_client_error());
}

#[tokio::test]
async fn rejects_requests_with_origin_header() {
    let server = RunningServer::start().await;

    let status = server
        .post_initialize(Some((header::ORIGIN, "https://attacker.example")))
        .await;

    assert!(status.is_client_error());
}

#[tokio::test]
async fn stops_after_shutdown_is_triggered() {
    let server = RunningServer::start().await;

    server.shutdown.cancel();
    let result = timeout(Duration::from_secs(5), server.task).await;

    assert!(result.unwrap().unwrap().is_ok());
    assert!(TcpStream::connect(server.address).await.is_err());
}
