# Project: rust-mcp-server

A Model Context Protocol (MCP) server for Rust development that provides a bridge between Large Language Models (LLMs) like GitHub Copilot and local Rust development tools. The server exposes comprehensive Cargo commands, code quality tools, and Rust toolchain management functionality to enable AI agents to perform actions like building, testing, analyzing, and managing Rust projects autonomously.

**Key Features:**
- Efficient schema design consuming only ~4.1k tokens per session
- Modular tool architecture with feature flags for optional tools
- Direct execution of Cargo commands with proper error handling
- Support for workspace-based operations
- Agent recommendations for improved workflows

**Tech Stack:**
- Language: Rust (Edition 2024, rust-version 1.90)
- Frameworks: rmcp 0.12.0 (Model Context Protocol), Tokio 1.45.1 (async runtime)
- Key Libs: Clap 4.5.40 (CLI), Schemars 1.1.0 (JSON schema), Serde 1.0.219 (serialization)

## Branch
The default branch is `master`.

## 🏗 Project Structure

```
rust-mcp-server/
├── src/
│   ├── main.rs              # Entry point with CLI argument parsing and server initialization
│   ├── command.rs          # Command execution logic with workspace support
│   ├── response.rs         # Response formatting and conversion to MCP protocol
│   ├── tool.rs             # Tool trait definitions and JSON schema generation
│   ├── rmcp_server.rs      # MCP server implementation and tool registration
│   ├── meta.rs             # Metadata utilities for content annotations
│   ├── serde_utils.rs      # Custom serde deserialization helpers
│   ├── version.rs          # Version information
│   └── tools/
│       ├── mod.rs          # Tool module declarations and workspace root management
│       ├── common/
│       │   ├── mod.rs      # Common tool utilities
│       │   └── docs.rs     # Documentation generation helpers
│       ├── cargo/
│       │   ├── mod.rs      # Cargo tool module and shared implementations
│       │   ├── build.rs    # cargo-build tool
│       │   ├── check.rs    # cargo-check tool
│       │   ├── test.rs     # cargo-test tool
│       │   ├── add_remove.rs # cargo-add and cargo-remove tools
│       │   ├── clippy.rs   # cargo-clippy tool
│       │   ├── doc.rs      # cargo-doc tool
│       │   ├── update.rs   # cargo-update tool
│       │   ├── search.rs   # cargo-search tool
│       │   ├── info.rs     # cargo-info tool
│       │   ├── metadata.rs # cargo-metadata tool
│       │   ├── package.rs  # cargo-package tool
│       │   └── workspace_info.rs # cargo-workspace-info tool
│       ├── rustc.rs        # rustc-explain tool
│       ├── cargo_deny.rs   # cargo-deny tools (optional feature)
│       ├── cargo_hack.rs   # cargo-hack tools (optional feature)
│       ├── cargo_machete.rs # cargo-machete tools (optional feature)
│       └── rustup.rs       # rustup tools (optional feature)
├── Cargo.toml              # Project configuration and dependencies
├── Cargo.lock              # Lockfile for dependency versions
├── README.md               # Project documentation
├── CHANGELOG.md            # Version history
├── tools.md                # Generated tools documentation
└── docs/                   # Additional documentation
```

### Key Modules

- **main**: Entry point using Clap for CLI parsing. Handles logging setup, workspace configuration, documentation generation mode, and MCP server startup via stdio transport.

- **rmcp_server**: Implements the MCP server handler. Manages tool registration, handles `list_tools` and `call_tool` requests, and provides Markdown documentation generation. Tools are stored in a HashMap with feature-flagged optional tools.

- **tool**: Defines the `Tool` and `DynTool` traits. Provides JSON schema generation using `schemars` with custom processing to remove null types (Gemini compatibility). All tools implement the `Tool` trait.

- **command**: Handles external command execution. Provides `execute_command` function that applies workspace root, logs execution, captures stdout/stderr, and formats output. Returns structured `Output` with exit status handling.

- **response**: Converts command output to MCP protocol responses. Handles content aggregation, agent recommendations, and supports disabling recommendations via flag.

- **tools**: Organized by functionality. Each tool module contains request structs with `schemars` annotations and a corresponding tool struct implementing the `Tool` trait.

## 🛠 Architecture & Rules

### 1. Patterns

- **Tool-Based Architecture**: Each Rust command or tool is represented as a separate tool implementing the `Tool` trait. Tools are registered in the server and exposed via MCP protocol.
- **Request-Response Pattern**: Each tool has a request struct (with `schemars::JsonSchema`) and a tool struct that implements `call_rmcp_tool` returning `Result<Response, ErrorData>`.
- **Feature Flags**: Optional tools (cargo-deny, cargo-hack, cargo-machete, rustup) are behind Cargo features to reduce token usage when not needed.
- **Command Building Pattern**: Request structs have a `build_cmd()` method that constructs the actual `std::process::Command` with appropriate arguments.

### 2. Conventions

- **Error Handling**: Use `Result<T, ErrorData>` for MCP protocol errors. Command execution uses `std::process::Command` with proper error handling for missing commands (`NotFound` error kind).
- **JSON Schema Optimization**: All tool request structs use `#[schemars(description = "")]` to minimize token usage. Optional fields are serialized with `skip_serializing_if = "Option::is_none"`.
- **Logging**: Use `tracing` crate for structured logging. Logs include command execution details, success/failure status, and diagnostic information.
- **Testing**: Unit tests are included in relevant modules (e.g., `tool.rs` for JSON schema generation, `response.rs` for response formatting).
- **Workspace Support**: Tools can operate on a specific workspace via `--workspace` argument. The workspace root is set once via `tools::set_workspace_root()` and applied to all commands via `apply_workspace_root()`.
- **Token Optimization**: The project is optimized for minimal token consumption (~4.1k tokens). Schema descriptions are empty, verbose metadata is removed from responses, and 92 rare/advanced fields have been pruned from request arguments.

### 3. Adding New Tools

When adding a new tool:

1. Create request struct with `#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]`
2. Add `#[schemars(description = "")]` to all fields to minimize tokens
3. Implement `build_cmd()` method to construct the `std::process::Command`
4. Create tool struct implementing `Tool` trait with `NAME`, `TITLE`, `DESCRIPTION`, and `RequestArgs`
5. In `call_rmcp_tool()`, call `execute_command()` and return `Response`
6. Register tool in `rmcp_server.rs` `Server::new()` method
7. If tool is optional, wrap registration in `#[cfg(feature = "...")]`
8. Add feature flag in `Cargo.toml` `[features]` section if needed

### 4. Build & Release

- **Development**: `cargo build`
- **Release**: Release profile uses LTO, single codegen unit, panic abort, and symbol stripping for optimal binary size
- **Features**: Default build includes core Cargo tools. Optional features: `cargo-deny`, `cargo-hack`, `cargo-machete`, `rustup`
- **Testing**: Run with `cargo test`
- **Documentation**: Generate tools documentation with `--generate-docs <output_file>` flag

### 5. Command Line Interface

Key CLI arguments:
- `--log-level`: Set logging verbosity (error, warn, info, debug, trace)
- `--log-file`: Write logs to file instead of stderr
- `--disable-tool <name>`: Disable specific tools (can be used multiple times)
- `--workspace <path>`: Set workspace root for operations
- `--generate-docs <output>`: Generate Markdown documentation and exit
- `--no-recommendations`: Disable agent recommendations in responses

### 6. MCP Protocol

- **Transport**: stdio (standard input/output)
- **Capabilities**: Tools only (no prompts, resources, logging, completions, or experimental features)
- **Handshake**: Provides server info including name, version, and capabilities
- **Tool Calls**: Receives `CallToolRequestParam` with tool name and arguments, returns `CallToolResult` with content items

### 7. Dependencies

- **rmcp**: Core MCP protocol implementation
- **tokio**: Async runtime with current_thread flavor
- **clap**: Command-line argument parsing
- **schemars**: JSON schema generation for tool inputs
- **serde/serde_json**: Serialization/deserialization
- **tracing**: Structured logging framework
- **anyhow**: Error handling with context

### 8. Tool Categories

- **Core Cargo Commands**: build, check, test, doc, fmt, clippy, clean
- **Project Management**: new, generate-lockfile, package, list
- **Dependency Management**: add, remove, update, metadata, search, info, workspace-info
- **Code Quality & Security** (optional): cargo-deny, cargo-machete
- **Advanced Testing** (optional): cargo-hack
- **Rust Toolchain Management** (optional): rustup, rustc-explain

### 9. Response Format

Tool responses include:
1. Command line executed (with `Executed command:` prefix)
2. stdout (if present)
3. stderr (if present)
4. Exit status with emoji (✅ for success, ❌ for failure)
5. Agent recommendations (optional, can be disabled)
6. Additional custom content (optional)

### 10. Versioning

- Current version: 0.3.1
- Follows semantic versioning
- Changelog maintained in CHANGELOG.md
- Version displayed via `-V` or `--version` flag
