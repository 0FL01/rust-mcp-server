use std::process::Command;

use crate::{
    Tool, execute_command,
    serde_utils::{deserialize_string, deserialize_string_vec},
};
use rmcp::ErrorData;

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
#[schemars(description = "")]
pub struct CargoDenyCheckRequest {
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    which: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_string")]
    config: Option<String>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    graph: Option<String>,
    #[schemars(description = "")]
    #[serde(default)]
    hide_inclusion_graph: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    disable_fetch: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    exclude_dev: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    audit_compatible_output: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    show_stats: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    warn: Option<Vec<String>>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    allow: Option<Vec<String>>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    deny: Option<Vec<String>>,

    #[schemars(description = "")]
    feature_depth: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_string")]
    log_level: Option<String>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    format: Option<String>,
    #[schemars(description = "")]
    #[serde(default)]
    workspace: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    allow_git_index: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    exclude_unpublished: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    exclude: Option<Vec<String>>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    target: Option<Vec<String>>,
    #[schemars(description = "")]
    #[serde(default)]
    all_features: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    no_default_features: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    features: Option<Vec<String>>,
}

impl CargoDenyCheckRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("deny");

        // Apply global options first
        if let Some(log_level) = &self.log_level {
            cmd.arg("--log-level").arg(log_level);
        }

        if let Some(format) = &self.format {
            cmd.arg("--format").arg(format);
        }

        if self.workspace.unwrap_or(false) {
            cmd.arg("--workspace");
        }

        if let Some(exclude) = &self.exclude {
            for item in exclude {
                cmd.arg("--exclude").arg(item);
            }
        }

        if let Some(target) = &self.target {
            for item in target {
                cmd.arg("--target").arg(item);
            }
        }

        if self.all_features.unwrap_or(false) {
            cmd.arg("--all-features");
        }

        if self.no_default_features.unwrap_or(false) {
            cmd.arg("--no-default-features");
        }

        if let Some(features) = &self.features {
            cmd.arg("--features").arg(features.join(","));
        }

        if self.allow_git_index.unwrap_or(false) {
            cmd.arg("--allow-git-index");
        }

        if self.exclude_dev.unwrap_or(false) {
            cmd.arg("--exclude-dev");
        }

        if self.exclude_unpublished.unwrap_or(false) {
            cmd.arg("--exclude-unpublished");
        }

        // Add the subcommand
        cmd.arg("check");

        // Apply check-specific options
        if let Some(config) = &self.config {
            cmd.arg("--config").arg(config);
        }

        if let Some(graph) = &self.graph {
            cmd.arg("--graph").arg(graph);
        }

        if self.hide_inclusion_graph.unwrap_or(false) {
            cmd.arg("--hide-inclusion-graph");
        }

        if self.disable_fetch.unwrap_or(false) {
            cmd.arg("--disable-fetch");
        }

        if self.audit_compatible_output.unwrap_or(false) {
            cmd.arg("--audit-compatible-output");
        }

        if self.show_stats.unwrap_or(false) {
            cmd.arg("--show-stats");
        }

        if let Some(warn) = &self.warn {
            for item in warn {
                cmd.arg("-W").arg(item);
            }
        }

        if let Some(allow) = &self.allow {
            for item in allow {
                cmd.arg("-A").arg(item);
            }
        }

        if let Some(deny) = &self.deny {
            for item in deny {
                cmd.arg("-D").arg(item);
            }
        }

        if let Some(feature_depth) = &self.feature_depth {
            cmd.arg("--feature-depth").arg(feature_depth.to_string());
        }

        // Add the check types as positional arguments
        if let Some(which) = &self.which {
            for check in which {
                cmd.arg(check);
            }
        }

        Ok(cmd)
    }
}

pub struct CargoDenyCheckRmcpTool;

impl Tool for CargoDenyCheckRmcpTool {
    const NAME: &'static str = "cargo-deny-check";
    const TITLE: &'static str = "Check dependencies";
    const DESCRIPTION: &'static str = "Checks a project's crate graph for security advisories, license compliance, banned crates.";
    type RequestArgs = CargoDenyCheckRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoDenyInitRequest {
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    config: Option<String>,
}

impl CargoDenyInitRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("deny").arg("init");

        if let Some(config) = &self.config {
            cmd.arg(config);
        }

        Ok(cmd)
    }
}

pub struct CargoDenyInitRmcpTool;

impl Tool for CargoDenyInitRmcpTool {
    const NAME: &'static str = "cargo-deny-init";
    const TITLE: &'static str = "Initialize cargo-deny config";
    const DESCRIPTION: &'static str = "Creates a cargo-deny config from a template";
    type RequestArgs = CargoDenyInitRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoDenyListRequest {
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    config: Option<String>,

    #[schemars(description = "")]
    threshold: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_string")]
    format: Option<String>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    layout: Option<String>,
}

impl CargoDenyListRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("deny").arg("list");

        if let Some(config) = &self.config {
            cmd.arg("--config").arg(config);
        }

        if let Some(threshold) = &self.threshold {
            cmd.arg("--threshold").arg(threshold.to_string());
        }

        if let Some(format) = &self.format {
            cmd.arg("--format").arg(format);
        }

        if let Some(layout) = &self.layout {
            cmd.arg("--layout").arg(layout);
        }

        Ok(cmd)
    }
}

pub struct CargoDenyListRmcpTool;

impl Tool for CargoDenyListRmcpTool {
    const NAME: &'static str = "cargo-deny-list";
    const TITLE: &'static str = "List licenses";
    const DESCRIPTION: &'static str =
        "Outputs a listing of all licenses and the crates that use them";
    type RequestArgs = CargoDenyListRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoDenyInstallRequest {}

impl CargoDenyInstallRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("install").arg("cargo-deny");

        Ok(cmd)
    }
}

pub struct CargoDenyInstallRmcpTool;

impl Tool for CargoDenyInstallRmcpTool {
    const NAME: &'static str = "cargo-deny-install";
    const TITLE: &'static str = "Install cargo-deny";
    const DESCRIPTION: &'static str =
        "Installs cargo-deny tool for dependency graph analysis and security checks";
    type RequestArgs = CargoDenyInstallRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}
