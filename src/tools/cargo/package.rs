use std::process::Command;

use crate::{
    Tool, execute_command,
    serde_utils::{deserialize_string, deserialize_string_vec},
};
use rmcp::ErrorData;

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoPackageRequest {
    #[schemars(description = "")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_vec"
    )]
    package: Option<Vec<String>>,

    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    workspace: Option<bool>,

    #[schemars(description = "")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_vec"
    )]
    exclude: Option<Vec<String>>,

    #[schemars(description = "")]
    #[serde(default)]
    list: bool,

    #[schemars(description = "")]
    #[serde(default)]
    no_verify: bool,

    #[schemars(description = "")]
    #[serde(default)]
    no_metadata: bool,

    #[schemars(description = "")]
    #[serde(default)]
    allow_dirty: bool,

    #[schemars(description = "")]
    #[serde(default)]
    exclude_lockfile: bool,

    #[schemars(description = "")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_vec"
    )]
    features: Option<Vec<String>>,

    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    all_features: Option<bool>,

    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    no_default_features: Option<bool>,

    #[schemars(description = "")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string"
    )]
    target: Option<String>,

    #[schemars(description = "")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string"
    )]
    target_dir: Option<String>,

    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    jobs: Option<u32>,

    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    keep_going: Option<bool>,
}

impl CargoPackageRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("package");

        // Package selection
        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.workspace.unwrap_or(false) {
            cmd.arg("--workspace");
        }

        if let Some(exclude) = &self.exclude {
            for excluded in exclude {
                cmd.arg("--exclude").arg(excluded);
            }
        }

        // Operation modes
        if self.list {
            cmd.arg("--list");
        }

        if self.no_verify {
            cmd.arg("--no-verify");
        }

        if self.no_metadata {
            cmd.arg("--no-metadata");
        }

        if self.allow_dirty {
            cmd.arg("--allow-dirty");
        }

        if self.exclude_lockfile {
            cmd.arg("--exclude-lockfile");
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

        // Compilation options
        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }

        if let Some(target_dir) = &self.target_dir {
            cmd.arg("--target-dir").arg(target_dir);
        }

        if let Some(jobs) = self.jobs {
            cmd.arg("--jobs").arg(jobs.to_string());
        }

        if self.keep_going.unwrap_or(false) {
            cmd.arg("--keep-going");
        }

        Ok(cmd)
    }
}

pub struct CargoPackageRmcpTool;

impl Tool for CargoPackageRmcpTool {
    const NAME: &'static str = "cargo-package";
    const TITLE: &'static str = "cargo package";
    const DESCRIPTION: &'static str = "Assemble the local package into a distributable tarball for publishing. Validates build by default. Usually run without arguments for single-package projects.";
    type RequestArgs = CargoPackageRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        let cmd = request.build_cmd()?;
        execute_command(cmd, Self::NAME).map(Into::into)
    }
}
