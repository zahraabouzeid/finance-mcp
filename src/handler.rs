use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Json},
    model::{Implementation, ServerCapabilities, ServerConfig},
    tool, tool_handler, tool_router,
};

use crate::status::{SERVER_NAME, SERVER_VERSION, ServerStatus};

#[derive(Debug, Clone)]
pub struct FinanceMcp {
    tool_router: ToolRouter<Self>,
}

impl FinanceMcp {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

impl Default for FinanceMcp {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router]
impl FinanceMcp {
    #[tool(description = "Report that the server is running, with its name and version")]
    fn server_status(&self) -> Json<ServerStatus> {
        Json(ServerStatus::current())
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for FinanceMcp {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(SERVER_NAME, SERVER_VERSION))
    }
}
