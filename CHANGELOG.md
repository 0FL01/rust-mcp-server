# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0] - 2026-01-19

### Optimization (Token Usage Reduction)
Drastically reduced MCP server token consumption from ~14,000 to ~2,000 tokens per session (-85%).

#### Features & Configuration
- **Modularized Tools**: Added feature flags for optional tool categories. The following are now disabled by default:
  - `cargo-deny`
  - `cargo-hack`
  - `cargo-machete`
  - `rustup`
  - *Enable via `cargo build --features <name>`*
- **Simplified Initialization**: Removed embedded `instructions.md` and verbose response annotations (audience, priority) from server handshake and responses.

#### Schema Optimizations
- **Removed Schema Descriptions**: Stripped all documentation comments from JSON schemas using `#[schemars(description = "")]`. Schemas now contain only type information, saving ~13k characters.
- **Field Pruning**: Removed 92 rare/advanced fields from RequestArgs structures to streamline LLM usage. Removed fields include:
  - Global flags: `toolchain`, `target_dir`, `manifest_path`, `lockfile_path`, `locking_mode`, `output_verbosity`, `ignore_rust_version`.
  - Specific flags: `git`, `path`, `registry` (for `cargo-add`), `message_format`.
- **Shortened Descriptions**: Reduced length of tool descriptions for `cargo-package`, `cargo-hack`, `rustc-explain`, and `rustup` tools by ~75%.

#### Code Cleanup
- Removed unused helper functions (`locking_mode_to_cli_flags`, `output_verbosity_to_cli_flags`).
- Refactored `cargo-doc` to use default target directory resolution.
- Centralized documentation constants (internal).
