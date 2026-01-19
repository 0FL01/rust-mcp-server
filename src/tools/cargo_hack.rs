use std::process::Command;

use crate::{
    Tool, execute_command,
    serde_utils::{deserialize_string, deserialize_string_vec},
};
use rmcp::ErrorData;

fn default_check() -> String {
    "check".to_string()
}

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
#[schemars(description = "")]
pub struct CargoHackRequest {
    #[schemars(description = "")]
    #[serde(default = "default_check")]
    command: String,
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,
    #[schemars(description = "")]
    #[serde(default)]
    workspace: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    exclude: Option<Vec<String>>,
    #[schemars(description = "")]
    #[serde(default)]
    locked: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    features: Option<Vec<String>>,
    #[schemars(description = "")]
    #[serde(default)]
    each_feature: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    feature_powerset: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    optional_deps: Option<Vec<String>>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    exclude_features: Option<Vec<String>>,
    #[schemars(description = "")]
    #[serde(default)]
    exclude_no_default_features: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    exclude_all_features: Option<bool>,

    #[schemars(description = "")]
    depth: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    group_features: Option<Vec<String>>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    target: Option<Vec<String>>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    mutually_exclusive_features: Option<Vec<String>>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    include_features: Option<Vec<String>>,
    #[schemars(description = "")]
    #[serde(default)]
    no_dev_deps: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    remove_dev_deps: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    no_private: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    ignore_private: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    ignore_unknown_features: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    rust_version: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    version_range: Option<String>,

    #[schemars(description = "")]
    version_step: Option<u32>,
    #[serde(default)]
    clean_per_run: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    clean_per_version: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    keep_going: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    partition: Option<String>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    log_group: Option<String>,
    #[schemars(description = "")]
    #[serde(default)]
    print_command_list: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    no_manifest_path: Option<bool>,
}

impl CargoHackRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        // Validate command
        let allowed_commands = ["check", "test", "build", "clippy"];
        if !allowed_commands.contains(&self.command.as_str()) {
            let error_msg = format!(
                "Invalid command '{}'. Allowed commands: {}",
                self.command,
                allowed_commands.join(", ")
            );
            return Err(ErrorData::invalid_params(error_msg, None));
        }

        let mut cmd = Command::new("cargo");
        cmd.arg("hack");

        // Package selection
        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.workspace.unwrap_or(false) {
            cmd.arg("--workspace");
        }

        if let Some(excludes) = &self.exclude {
            for exclude in excludes {
                cmd.arg("--exclude").arg(exclude);
            }
        }

        if self.locked.unwrap_or(true) {
            cmd.arg("--locked");
        }

        // Feature selection
        if let Some(features) = &self.features {
            cmd.arg("--features").arg(features.join(","));
        }

        if self.each_feature.unwrap_or(false) {
            cmd.arg("--each-feature");
        }

        if self.feature_powerset.unwrap_or(false) {
            cmd.arg("--feature-powerset");
        }

        if let Some(optional_deps) = &self.optional_deps {
            if optional_deps.is_empty() {
                cmd.arg("--optional-deps");
            } else {
                cmd.arg("--optional-deps").arg(optional_deps.join(","));
            }
        }

        if let Some(exclude_features) = &self.exclude_features {
            cmd.arg("--exclude-features")
                .arg(exclude_features.join(","));
        }

        if self.exclude_no_default_features.unwrap_or(false) {
            cmd.arg("--exclude-no-default-features");
        }

        if self.exclude_all_features.unwrap_or(false) {
            cmd.arg("--exclude-all-features");
        }

        if let Some(depth) = self.depth {
            cmd.arg("--depth").arg(depth.to_string());
        }

        if let Some(group_features) = &self.group_features {
            cmd.arg("--group-features").arg(group_features.join(","));
        }

        // Target selection
        if let Some(targets) = &self.target {
            for target in targets {
                cmd.arg("--target").arg(target);
            }
        }

        // Feature constraints
        if let Some(mutually_exclusive) = &self.mutually_exclusive_features {
            cmd.arg("--mutually-exclusive-features")
                .arg(mutually_exclusive.join(","));
        }

        if let Some(include_features) = &self.include_features {
            cmd.arg("--include-features")
                .arg(include_features.join(","));
        }

        // Dependency options
        if self.no_dev_deps.unwrap_or(false) {
            cmd.arg("--no-dev-deps");
        }

        if self.remove_dev_deps.unwrap_or(false) {
            cmd.arg("--remove-dev-deps");
        }

        if self.no_private.unwrap_or(false) {
            cmd.arg("--no-private");
        }

        if self.ignore_private.unwrap_or(false) {
            cmd.arg("--ignore-private");
        }

        if self.ignore_unknown_features.unwrap_or(false) {
            cmd.arg("--ignore-unknown-features");
        }

        // Version options
        if self.rust_version.unwrap_or(false) {
            cmd.arg("--rust-version");
        }

        if let Some(version_range) = &self.version_range {
            cmd.arg("--version-range").arg(version_range);
        }

        if let Some(version_step) = self.version_step {
            cmd.arg("--version-step").arg(version_step.to_string());
        }

        // Cleanup options
        if self.clean_per_run.unwrap_or(false) {
            cmd.arg("--clean-per-run");
        }

        if self.clean_per_version.unwrap_or(false) {
            cmd.arg("--clean-per-version");
        }

        // Execution options
        if self.keep_going.unwrap_or(false) {
            cmd.arg("--keep-going");
        }

        if let Some(partition) = &self.partition {
            cmd.arg("--partition").arg(partition);
        }

        if let Some(log_group) = &self.log_group {
            cmd.arg("--log-group").arg(log_group);
        }

        if self.print_command_list.unwrap_or(false) {
            cmd.arg("--print-command-list");
        }

        if self.no_manifest_path.unwrap_or(false) {
            cmd.arg("--no-manifest-path");
        }

        // Add cargo command to run (e.g., check, test, build)
        cmd.arg(&self.command);

        Ok(cmd)
    }
}

pub struct CargoHackRmcpTool;

impl Tool for CargoHackRmcpTool {
    const NAME: &'static str = "cargo-hack";
    const TITLE: &'static str = "Run cargo-hack";
    const DESCRIPTION: &'static str = "Run cargo-hack for feature testing and CI. Supports check, test, build, clippy. Use 'check' for fast validation.";
    type RequestArgs = CargoHackRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoHackInstallRequest {}

impl CargoHackInstallRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("install").arg("cargo-hack");

        Ok(cmd)
    }
}

pub struct CargoHackInstallRmcpTool;

impl Tool for CargoHackInstallRmcpTool {
    const NAME: &'static str = "cargo-hack-install";
    const TITLE: &'static str = "Install cargo-hack";
    const DESCRIPTION: &'static str =
        "Installs cargo-hack tool for feature testing and continuous integration";
    type RequestArgs = CargoHackInstallRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}
