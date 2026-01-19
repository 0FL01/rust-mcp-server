use std::process::Command;

use crate::{
    Tool, execute_command,
    serde_utils::{deserialize_string, deserialize_string_vec},
    tools::cargo::CargoWorkspaceInfoRmcpTool,
};
use rmcp::ErrorData;
#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct CargoMetadataRequest {
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    filter_platform: Option<String>,
    #[schemars(description = "")]
    #[serde(default)]
    no_deps: Option<bool>,

    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    config: Option<String>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    features: Option<Vec<String>>,
    #[schemars(description = "")]
    #[serde(default)]
    all_features: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    no_default_features: Option<bool>,
}
impl CargoMetadataRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("metadata");
        cmd.arg("--format-version").arg("1");

        // Package/dependency filtering
        if let Some(triple) = &self.filter_platform {
            cmd.arg("--filter-platform").arg(triple);
        }

        if self.no_deps.unwrap_or(false) {
            cmd.arg("--no-deps");
        }

        if let Some(config) = &self.config {
            cmd.arg("--config").arg(config);
        }

        // Feature selection
        if let Some(features) = &self.features
            && !features.is_empty()
        {
            cmd.arg("--features").arg(features.join(","));
        }

        if self.all_features.unwrap_or(false) {
            cmd.arg("--all-features");
        }

        if self.no_default_features.unwrap_or(false) {
            cmd.arg("--no-default-features");
        }

        Ok(cmd)
    }
}

pub struct CargoMetadataRmcpTool;

impl Tool for CargoMetadataRmcpTool {
    const NAME: &'static str = "cargo-metadata";
    const TITLE: &'static str = "cargo metadata";
    const DESCRIPTION: &'static str = "Outputs a listing of a project's resolved dependencies and metadata in machine-readable format (JSON).";
    type RequestArgs = CargoMetadataRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        let cmd = request.build_cmd()?;
        let mut response: crate::Response = execute_command(cmd, Self::NAME)?.into();

        if !request.no_deps.unwrap_or(false) {
            response.add_recommendation(
                "Set no_deps=true to return only workspace member metadata, reducing output size and token usage",
            );
        }

        response.add_recommendation(format!(
            "Use #{} if you don't need full metadata",
            CargoWorkspaceInfoRmcpTool::NAME
        ));

        Ok(response)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_with_features_array() {
        let input = json!({
            "features": ["serde", "tokio"],
        });

        let tool: Result<CargoMetadataRequest, _> = serde_json::from_value(input);
        let tool = tool.expect("Deserialization should succeed with features array");

        assert_eq!(
            tool.features.unwrap(),
            ["serde".to_owned(), "tokio".to_owned()]
        );
    }

    #[test]
    fn test_deserialize_with_single_feature_string() {
        let input = json!({
            "features": "serde",
        });

        let tool: Result<CargoMetadataRequest, _> = serde_json::from_value(input);
        let tool = tool.expect("Deserialization should succeed with single feature string");

        assert_eq!(tool.features.unwrap(), ["serde".to_owned()]);
    }

    #[test]
    fn test_deserialize_with_features_string_array() {
        let input = json!({
            "features": "[\"serde\",\"tokio\"]",
        });

        let tool: Result<CargoMetadataRequest, _> = serde_json::from_value(input);
        let tool = tool
            .expect("Deserialization should succeed with features string that looks like array");

        assert_eq!(tool.features.unwrap(), ["[\"serde\",\"tokio\"]".to_owned()]);
    }
}
