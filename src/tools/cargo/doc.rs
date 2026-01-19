use std::path::Path;
use std::process::Command;

use rmcp::{ErrorData, model::RawContent};

use crate::{
    Tool, execute_command,
    serde_utils::{deserialize_string, deserialize_string_vec},
    tools::WORKSPACE_ROOT,
};

#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct CargoDocRequest {
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
    document_private_items: Option<bool>,
    #[schemars(description = "")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    docsrs_config: Option<bool>,
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
}

impl CargoDocRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("doc");

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

        // Documentation options
        if self.no_deps.unwrap_or(true) {
            cmd.arg("--no-deps");
        }

        if self.document_private_items.unwrap_or(false) {
            cmd.arg("--document-private-items");
        }

        // Set RUSTDOCFLAGS for docs.rs configuration if enabled
        if self.docsrs_config.unwrap_or(false) {
            cmd.env("RUSTDOCFLAGS", "--cfg docsrs");
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

        Ok(cmd)
    }

    fn get_doc_path(&self) -> Option<String> {
        let base_dir = "target";

        // Get the base documentation directory
        let doc_dir = if let Some(target) = &self.target {
            format!("{base_dir}/{target}/doc")
        } else {
            format!("{base_dir}/doc")
        };

        // Get the absolute path using workspace root
        let absolute_doc_dir = if let Some(workspace_root) = WORKSPACE_ROOT.get() {
            Path::new(workspace_root).join(&doc_dir)
        } else {
            Path::new(&doc_dir).to_path_buf()
        };

        // Strategy 1: If package(s) specified, try to find documentation for the first package
        if let Some(packages) = &self.package
            && let Some(first_package) = packages.first()
        {
            // Convert package name to the format used in file paths (hyphens to underscores)
            let package_path_name = first_package.replace('-', "_");
            let package_index = absolute_doc_dir.join(&package_path_name).join("index.html");
            if package_index.exists() {
                return Some(Self::normalize_path(&package_index));
            }
        }

        // Strategy 2: Look for any index.html file under the doc directory
        if absolute_doc_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&absolute_doc_dir) {
                for entry in entries.flatten() {
                    if entry.file_type().is_ok_and(|ft| ft.is_dir()) {
                        let index_path = entry.path().join("index.html");
                        if index_path.exists() {
                            return Some(Self::normalize_path(&index_path));
                        }
                    }
                }
            }

            // Check for a top-level index.html
            let top_index = absolute_doc_dir.join("index.html");
            if top_index.exists() {
                return Some(Self::normalize_path(&top_index));
            }
        }

        None
    }

    fn normalize_path(path: &Path) -> String {
        let absolute_path = match path.canonicalize() {
            Ok(canonical) => canonical,
            Err(_) => path.to_path_buf(),
        };

        absolute_path.to_string_lossy().into_owned()
    }
}

pub struct CargoDocRmcpTool;

impl Tool for CargoDocRmcpTool {
    const NAME: &'static str = "cargo-doc";
    const TITLE: &'static str = "Build Rust documentation";
    const DESCRIPTION: &'static str = "Build documentation for a Rust package using Cargo. Recommended to use with no_deps and specific package for faster builds. Returns path to generated documentation index.";
    type RequestArgs = CargoDocRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        use rmcp::model::{AnnotateAble, Annotations};

        let cmd = request.build_cmd()?;
        let start_time = std::time::Instant::now();
        let output = execute_command(cmd, Self::NAME)?;
        let duration = start_time.elapsed();

        if !output.success() {
            return Ok(output.into());
        }

        let mut response: crate::Response = output.into();
        let doc_path = request.get_doc_path();
        let doc_info = if let Some(doc_path) = doc_path {
            format!(
                "Documentation generated successfully!\nDocumentation file: {doc_path}\nOpen this file in your browser to view the docs"
            )
        } else {
            "Documentation generated successfully!".to_owned()
        };

        response.add_content(RawContent::text(doc_info).annotate(Annotations::default()));

        if duration.as_secs() >= 30 && !request.no_deps.unwrap_or(false) {
            response.add_recommendation(
                "For faster documentation builds, consider using `no_deps: true` to build only local documentation"
            );
        }

        Ok(response)
    }
}
