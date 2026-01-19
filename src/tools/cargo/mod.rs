mod add_remove;
mod build;
mod check;
mod clippy;
mod doc;
mod info;
mod metadata;
mod package;
mod search;
mod test;
mod update;
mod workspace_info;

pub use add_remove::{CargoAddRmcpTool, CargoRemoveRmcpTool};
pub use build::CargoBuildRmcpTool;
pub use check::CargoCheckRmcpTool;
pub use clippy::CargoClippyRmcpTool;
pub use doc::CargoDocRmcpTool;
pub use info::CargoInfoRmcpTool;
pub use metadata::CargoMetadataRmcpTool;
pub use package::CargoPackageRmcpTool;
pub use search::CargoSearchRmcpTool;
pub use test::CargoTestRmcpTool;
pub use update::CargoUpdateRmcpTool;
pub use workspace_info::CargoWorkspaceInfoRmcpTool;

use std::process::Command;

use crate::{
    Tool, execute_command,
    serde_utils::{deserialize_string, deserialize_string_vec},
};
use rmcp::ErrorData;

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoGenerateLockfileRequest {
    #[schemars(description = "")]
    #[serde(default)]
    ignore_rust_version: Option<bool>,
}

impl CargoGenerateLockfileRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("generate-lockfile");

        if self.ignore_rust_version.unwrap_or(false) {
            cmd.arg("--ignore-rust-version");
        }

        Ok(cmd)
    }
}

pub struct CargoGenerateLockfileRmcpTool;

impl Tool for CargoGenerateLockfileRmcpTool {
    const NAME: &'static str = "cargo-generate_lockfile";
    const TITLE: &'static str = "Generate Cargo.lock";
    const DESCRIPTION: &'static str = "Generates or updates the Cargo.lock file for a Rust project. Usually, run without any additional arguments.";
    type RequestArgs = CargoGenerateLockfileRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoCleanRequest {
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    profile: Option<String>,
    #[schemars(description = "")]
    #[serde(default)]
    doc: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    dry_run: Option<bool>,
    #[schemars(description = "")]
    #[serde(default)]
    release: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    target: Option<String>,
}

impl CargoCleanRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("clean");

        // Package selection
        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        // Compilation options
        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
        }

        if self.doc.unwrap_or(false) {
            cmd.arg("--doc");
        }

        if self.dry_run.unwrap_or(false) {
            cmd.arg("--dry-run");
        }

        if self.release.unwrap_or(false) {
            cmd.arg("--release");
        }

        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }

        Ok(cmd)
    }
}

pub struct CargoCleanRmcpTool;

impl Tool for CargoCleanRmcpTool {
    const NAME: &'static str = "cargo-clean";
    const TITLE: &'static str = "Clean Cargo artifacts";
    const DESCRIPTION: &'static str = "Cleans the target directory for a Rust project using Cargo. By default, it cleans the entire workspace.";
    type RequestArgs = CargoCleanRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoFmtRequest {
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,
    #[schemars(description = "")]
    #[serde(default)]
    all: bool,
    #[schemars(description = "")]
    #[serde(default)]
    check: bool,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    message_format: Option<String>,
}

impl CargoFmtRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("fmt");

        // Package selection
        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.all {
            cmd.arg("--all");
        }

        // Formatting options
        if self.check {
            cmd.arg("--check");
        }

        if let Some(message_format) = &self.message_format {
            cmd.arg("--message-format").arg(message_format);
        }

        Ok(cmd)
    }
}

pub struct CargoFmtRmcpTool;

impl Tool for CargoFmtRmcpTool {
    const NAME: &'static str = "cargo-fmt";
    const TITLE: &'static str = "Format Rust code";
    const DESCRIPTION: &'static str =
        "Formats Rust code using rustfmt. Usually, run without any additional arguments.";
    type RequestArgs = CargoFmtRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        let output = execute_command(request.build_cmd()?, Self::NAME)?;
        let failed = !output.success();
        let mut response: crate::Response = output.into();

        if failed && request.check {
            response.add_recommendation(format!(
                "Run #{} with `check: false` to automatically format the code",
                Self::NAME
            ));
        }

        Ok(response)
    }
}

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoNewRequest {
    pub path: String,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    pub name: Option<String>,
    #[schemars(description = "")]
    #[serde(default)]
    pub bin: bool,
    #[schemars(description = "")]
    #[serde(default)]
    pub lib: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    pub edition: Option<String>,

    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    pub vcs: Option<String>,
    #[schemars(description = "")]
    #[serde(default, deserialize_with = "deserialize_string")]
    pub registry: Option<String>,
}

impl CargoNewRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("new");

        // Add the path argument (required)
        cmd.arg(&self.path);

        // Template options
        if self.bin {
            cmd.arg("--bin");
        }
        if self.lib.unwrap_or(false) {
            cmd.arg("--lib");
        }

        // Package options
        if let Some(name) = &self.name {
            cmd.arg("--name").arg(name);
        }
        if let Some(edition) = &self.edition {
            cmd.arg("--edition").arg(edition);
        }
        if let Some(registry) = &self.registry {
            cmd.arg("--registry").arg(registry);
        }

        // VCS options
        if let Some(vcs) = &self.vcs {
            cmd.arg("--vcs").arg(vcs);
        }

        Ok(cmd)
    }
}

pub struct CargoNewRmcpTool;

impl Tool for CargoNewRmcpTool {
    const NAME: &'static str = "cargo-new";
    const TITLE: &'static str = "Create new Rust project";
    const DESCRIPTION: &'static str = "Create a new cargo package at <path>. Creates a new Rust project with the specified name and template.";
    type RequestArgs = CargoNewRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoListRequest {}

impl CargoListRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("--list");
        Ok(cmd)
    }
}

pub struct CargoListRmcpTool;

impl Tool for CargoListRmcpTool {
    const NAME: &'static str = "cargo-list";
    const TITLE: &'static str = "List cargo commands";
    const DESCRIPTION: &'static str = "Lists installed cargo commands using 'cargo --list'.";
    type RequestArgs = CargoListRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}
