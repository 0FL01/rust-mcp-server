use std::process::Command;

use crate::{
    Tool, execute_command,
    serde_utils::{deserialize_string, deserialize_string_vec},
};
use rmcp::ErrorData;

#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct CargoUpdateRequest {
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    spec: Option<Vec<String>>,
    #[schemars(description = "")]
    #[serde(default)]
    dry_run: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    recursive: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    precise: Option<String>,
    #[schemars(description = "")]
    #[serde(default)]
    breaking: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    workspace: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    ignore_rust_version: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    config: Option<String>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    unstable_flags: Option<Vec<String>>,
}
impl CargoUpdateRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("update");

        // Package selection
        if let Some(specs) = &self.spec {
            for spec in specs {
                cmd.arg(spec);
            }
        }

        // Update options
        if self.dry_run.unwrap_or(false) {
            cmd.arg("--dry-run");
        }

        if self.recursive.unwrap_or(false) {
            cmd.arg("--recursive");
        }

        if let Some(precise) = &self.precise {
            cmd.arg("--precise").arg(precise);
        }

        if self.breaking.unwrap_or(false) {
            cmd.arg("--breaking");
        }

        if self.workspace.unwrap_or(false) {
            cmd.arg("--workspace");
        }

        if self.ignore_rust_version.unwrap_or(false) {
            cmd.arg("--ignore-rust-version");
        }

        if let Some(config) = &self.config {
            cmd.arg("--config").arg(config);
        }

        if let Some(unstable_flags) = &self.unstable_flags {
            for flag in unstable_flags {
                cmd.arg("-Z").arg(flag);
            }
        }

        Ok(cmd)
    }
}

pub struct CargoUpdateRmcpTool;

impl Tool for CargoUpdateRmcpTool {
    const NAME: &'static str = "cargo-update";
    const TITLE: &'static str = "cargo update";
    const DESCRIPTION: &'static str = "Update dependencies as recorded in the local lock file. Updates the dependencies in Cargo.lock to their latest compatible versions.";
    type RequestArgs = CargoUpdateRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        let cmd = request.build_cmd()?;
        execute_command(cmd, Self::NAME).map(Into::into)
    }
}
