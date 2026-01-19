use std::process::Command;

use crate::{Tool, execute_command, serde_utils::deserialize_string};
use rmcp::ErrorData;

#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct CargoSearchRequest {
    pub query: String,
    pub limit: Option<u32>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    pub registry: Option<String>,
}
impl CargoSearchRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("search");
        cmd.arg(&self.query);
        if let Some(limit) = self.limit {
            cmd.arg("--limit").arg(limit.to_string());
        }
        if let Some(registry) = &self.registry {
            cmd.arg("--registry").arg(registry);
        }
        Ok(cmd)
    }
}

pub struct CargoSearchRmcpTool;

impl Tool for CargoSearchRmcpTool {
    const NAME: &'static str = "cargo-search";
    const TITLE: &'static str = "cargo search";
    const DESCRIPTION: &'static str = "Search packages in the registry. Default registry is crates.io. Equivalent to 'cargo search <code>QUERY</code>'.";
    type RequestArgs = CargoSearchRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        let cmd = request.build_cmd()?;
        execute_command(cmd, Self::NAME).map(Into::into)
    }
}
