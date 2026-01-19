use std::process::Command;

use crate::{
    Response, Tool, execute_command,
    serde_utils::{deserialize_string, deserialize_string_vec},
    tools::cargo::CargoCheckRmcpTool,
};
use rmcp::ErrorData;

#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct CargoBuildRequest {
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
    release: Option<bool>,

    #[schemars(description = "")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string"
    )]
    profile: Option<String>,

    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    jobs: Option<u32>,

    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    keep_going: Option<bool>,

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

impl CargoBuildRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("build");

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
        if self.release.unwrap_or(false) {
            cmd.arg("--release");
        }

        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
        }

        if let Some(jobs) = self.jobs {
            cmd.arg("--jobs").arg(jobs.to_string());
        }

        if self.keep_going.unwrap_or(false) {
            cmd.arg("--keep-going");
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

pub struct CargoBuildRmcpTool;

impl Tool for CargoBuildRmcpTool {
    const NAME: &'static str = "cargo-build";
    const TITLE: &'static str = "cargo build";
    const DESCRIPTION: &'static str =
        "Builds a Rust project using Cargo. Usually, run without any additional arguments.";
    type RequestArgs = CargoBuildRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        let cmd = request.build_cmd()?;
        let start_time = std::time::Instant::now();
        let output = execute_command(cmd, Self::NAME)?;
        let duration = start_time.elapsed();

        let mut response: Response = output.into();
        if duration.as_secs() >= 60 {
            response.add_recommendation(format!(
                "Consider using #{} tool for faster feedback",
                CargoCheckRmcpTool::NAME
            ));
        }
        Ok(response)
    }
}
