use std::process::Command;

use crate::{
    Tool, execute_command,
    serde_utils::{deserialize_string, deserialize_string_vec},
};
use rmcp::ErrorData;

#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct CargoCheckRequest {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lib: Option<bool>,

    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bins: Option<bool>,

    #[schemars(description = "")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string"
    )]
    bin: Option<String>,

    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    examples: Option<bool>,

    #[schemars(description = "")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string"
    )]
    example: Option<String>,

    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tests: Option<bool>,

    #[schemars(description = "")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string"
    )]
    test: Option<String>,

    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    benches: Option<bool>,

    #[schemars(description = "")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string"
    )]
    bench: Option<String>,

    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    all_targets: Option<bool>,

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    jobs: Option<u32>,

    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    keep_going: Option<bool>,

    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    release: Option<bool>,

    #[schemars(description = "")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string"
    )]
    profile: Option<String>,

    #[schemars(description = "")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string"
    )]
    target: Option<String>,

    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    warnings_as_errors: Option<bool>,
}

impl CargoCheckRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("check");

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

        // Target selection
        if self.lib.unwrap_or(false) {
            cmd.arg("--lib");
        }

        if self.bins.unwrap_or(false) {
            cmd.arg("--bins");
        }

        if let Some(bin) = &self.bin {
            cmd.arg("--bin").arg(bin);
        }

        if self.examples.unwrap_or(false) {
            cmd.arg("--examples");
        }

        if let Some(example) = &self.example {
            cmd.arg("--example").arg(example);
        }

        if self.tests.unwrap_or(false) {
            cmd.arg("--tests");
        }

        if let Some(test) = &self.test {
            cmd.arg("--test").arg(test);
        }

        if self.benches.unwrap_or(false) {
            cmd.arg("--benches");
        }

        if let Some(bench) = &self.bench {
            cmd.arg("--bench").arg(bench);
        }

        if self.all_targets.unwrap_or(false) {
            cmd.arg("--all-targets");
        }

        // Feature selection
        if let Some(features) = &self.features {
            cmd.arg("--features").arg(features.join(","));
        }

        if self.all_features.unwrap_or(false) {
            cmd.arg("--all-features");
        }

        if self.no_default_features.unwrap_or(false) {
            cmd.arg("--no-default-features");
        }

        // Compilation options
        if let Some(jobs) = self.jobs {
            cmd.arg("--jobs").arg(jobs.to_string());
        }

        if self.keep_going.unwrap_or(false) {
            cmd.arg("--keep-going");
        }

        if self.release.unwrap_or(false) {
            cmd.arg("--release");
        }

        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
        }

        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }

        if self.warnings_as_errors.unwrap_or(false) {
            cmd.env("RUSTFLAGS", "-D warnings");
        }
        Ok(cmd)
    }
}

pub struct CargoCheckRmcpTool;

impl Tool for CargoCheckRmcpTool {
    const NAME: &'static str = "cargo-check";
    const TITLE: &'static str = "cargo check";
    const DESCRIPTION: &'static str = "Checks a Rust package and all of its dependencies for errors. Usually, run without any additional arguments.";
    type RequestArgs = CargoCheckRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        let cmd = request.build_cmd()?;
        execute_command(cmd, Self::NAME).map(Into::into)
    }
}
