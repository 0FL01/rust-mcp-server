use std::process::Command;

use crate::{Tool, execute_command};
use rmcp::ErrorData;

#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct RustcExplainRequest {
    pub error_code: String,
}

impl RustcExplainRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("rustc");
        cmd.arg("--explain").arg(&self.error_code);

        Ok(cmd)
    }
}

pub struct RustcExplainRmcpTool;

impl Tool for RustcExplainRmcpTool {
    const NAME: &'static str = "rustc-explain";
    const TITLE: &'static str = "Explain Rust error";
    const DESCRIPTION: &'static str = "Explain Rust compiler error codes (e.g., E0001, E0308). Essential for understanding and resolving compilation errors.";
    type RequestArgs = RustcExplainRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}
