use std::process::Command;

use crate::{
    Tool,
    command::execute_command,
    serde_utils::{deserialize_string, deserialize_string_vec},
    tools::cargo::CargoFmtRmcpTool,
};
use rmcp::ErrorData;

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoClippyRequest {
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
    no_deps: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    allow_dirty: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    fix: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    release: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    all_targets: Option<bool>,
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
impl CargoClippyRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("clippy");

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

        // Clippy-specific options
        if self.no_deps.unwrap_or(false) {
            cmd.arg("--no-deps");
        }

        if self.fix.unwrap_or(false) {
            cmd.arg("--fix");
        }

        if self.allow_dirty.unwrap_or(false) && self.fix.unwrap_or(false) {
            cmd.arg("--allow-dirty");
        }

        // Compilation options
        if self.release.unwrap_or(false) {
            cmd.arg("--release");
        }

        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
        }

        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }

        // Target selection
        if self.all_targets.unwrap_or(false) {
            cmd.arg("--all-targets");
        }

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

        if self.warnings_as_errors.unwrap_or(false) {
            cmd.env("RUSTFLAGS", "-D warnings");
        }

        Ok(cmd)
    }
}

pub struct CargoClippyRmcpTool;

impl Tool for CargoClippyRmcpTool {
    const NAME: &'static str = "cargo-clippy";
    const TITLE: &'static str = "cargo clippy";
    const DESCRIPTION: &'static str =
        "Checks a Rust package to catch common mistakes and improve code quality using Clippy";
    type RequestArgs = CargoClippyRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        let cmd = request.build_cmd()?;
        let output = execute_command(cmd, Self::NAME)?;

        let add_fix_recommendation = !request.fix.unwrap_or(false) && output.stderr.is_some();
        let add_fmt_recommendation = request.fix.unwrap_or(false);
        let mut response: crate::Response = output.into();

        if add_fix_recommendation {
            response.add_recommendation(format!(
                "Run #{} with the `fix` and `allow_dirty` options to automatically fix the issues",
                Self::NAME
            ));
        }

        if add_fmt_recommendation {
            response.add_recommendation(format!(
                "Run #{} to format code after applying fixes",
                CargoFmtRmcpTool::NAME
            ));
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_with_missing_package_field() {
        // Simulate a JSON input missing the `package` field (should be Option)
        let input = json!({
            "toolchain": null,
            "workspace": true,
            "no_deps": false,
            "allow_dirty": true,
            "fix": true,
            "release": false,
            "target": null,
            "all_targets": true,
            "lib": true,
            "bins": true,
            "examples": true,
            "tests": true,
            "features": null,
            "all_features": true,
            "no_default_features": false,
            "verbose": true,
            "quiet": false,
            "warnings_as_errors": false
        });

        let tool: Result<CargoClippyRequest, _> = serde_json::from_value(input);
        let tool = tool
            .expect("Deserialization should succeed even if `package` is missing (it's Option)");

        assert_eq!(tool.package, None);
        assert_eq!(tool.workspace, Some(true));
        assert_eq!(tool.all_features, Some(true));
        assert_eq!(tool.allow_dirty, Some(true));
    }

    #[test]
    fn test_deserialize_with_package_field() {
        // Simulate a JSON input missing the `package` field (should be Option)
        let input = json!({
            "package": ["my_package"],
        });

        let tool: Result<CargoClippyRequest, _> = serde_json::from_value(input);
        let tool = tool.expect("Deserialization should succeed");

        assert_eq!(tool.package.unwrap(), ["my_package".to_owned()]);
        assert_eq!(tool.workspace, None);
        assert_eq!(tool.all_features, None);
        assert_eq!(tool.allow_dirty, None);
    }
}
