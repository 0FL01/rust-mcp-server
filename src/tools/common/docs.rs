/// Locking mode for dependency resolution.
///
/// Valid options:
/// - "locked" (default): Assert that `Cargo.lock` will remain unchanged
/// - "unlocked": Allow `Cargo.lock` to be updated
/// - "offline": Run without accessing the network
/// - "frozen": Equivalent to specifying both --locked and --offline
pub const LOCKING_MODE_DESC: &str = r#"Locking mode for dependency resolution.

Valid options:
- "locked" (default): Assert that `Cargo.lock` will remain unchanged
- "unlocked": Allow `Cargo.lock` to be updated
- "offline": Run without accessing the network
- "frozen": Equivalent to specifying both --locked and --offline"#;

/// Output verbosity level.
///
/// Valid options:
/// - "quiet" (default): Show only the essential command output
/// - "normal": Show standard output (no additional flags)
/// - "verbose": Show detailed output including build information
pub const OUTPUT_VERBOSITY_DESC: &str = r#"Output verbosity level.

Valid options:
- "quiet" (default): Show only the essential command output
- "normal": Show standard output (no additional flags)
- "verbose": Show detailed output including build information"#;
