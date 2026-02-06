# Contributing to Claude Chic Rust

## Development Setup

### Prerequisites

- Rust 1.70+ ([install](https://rustup.rs/))
- PostgreSQL 12+ or Supabase account
- Git

### Initial Setup

```bash
# Clone repository
git clone https://github.com/mrocklin/claudechic
cd claudechic/claudechic-rust

# Verify Rust installation
rustc --version
cargo --version

# Run tests
cargo test

# Check format
cargo fmt -- --check

# Run linter
cargo clippy -- -D warnings
```

## Code Organization

### Module Guidelines

1. **Keep modules focused**
   - One responsibility per module
   - Clear, documented public API
   - Hide implementation details

2. **Naming conventions**
   - Modules: `snake_case`
   - Types: `PascalCase`
   - Functions: `snake_case`
   - Constants: `SCREAMING_SNAKE_CASE`

3. **Documentation**
   ```rust
   /// Brief description (one line)
   ///
   /// Longer explanation if needed.
   ///
   /// # Examples
   ///
   /// ```
   /// let x = do_something();
   /// ```
   pub fn function_name() {
   }
   ```

### Error Handling

- Use `anyhow::Result<T>` for main error type
- Use `thiserror::Error` for custom error enums
- Propagate with `?` operator
- Add context with `.context("message")?`

```rust
pub fn load_config(path: &Path) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read config file")?;
    serde_json::from_str(&content)
        .context("Failed to parse config JSON")
}
```

### Async Code

- Use `async fn` and `await`
- Leverage Tokio for concurrency
- Spawn tasks with `tokio::spawn`
- Use channels for communication

```rust
pub async fn process_stream(mut rx: mpsc::Receiver<Item>) {
    while let Some(item) = rx.recv().await {
        // Handle item
    }
}
```

## Testing

### Unit Tests

Place in same file or `tests/` subdirectory:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let result = function_under_test();
        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests

Create `tests/integration_test.rs`:

```rust
use claudechic_core::models::*;

#[tokio::test]
async fn test_full_workflow() {
    // Test end-to-end scenario
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests single-threaded
cargo test -- --test-threads=1
```

## Code Style

### Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting

```bash
# Run clippy
cargo clippy -- -D warnings

# Run with all features
cargo clippy --all-targets --all-features
```

### Naming

- **Files**: `snake_case.rs`
- **Modules**: `pub mod snake_case`
- **Types**: `pub struct PascalCase`
- **Methods**: `pub fn snake_case()`
- **Constants**: `const SCREAMING_SNAKE_CASE`

## Development Workflow

### Creating a New Feature

1. Create a new branch:
   ```bash
   git checkout -b feature/short-description
   ```

2. Implement in phases:
   - Design data structures
   - Implement core logic
   - Add UI layer
   - Write tests
   - Document

3. Commit frequently:
   ```bash
   git commit -m "Brief description of changes"
   ```

4. Test before pushing:
   ```bash
   cargo test
   cargo fmt
   cargo clippy
   ```

5. Push and create PR:
   ```bash
   git push origin feature/short-description
   ```

### Code Review Checklist

- [ ] Follows code style guidelines
- [ ] Tests added for new functionality
- [ ] Documentation updated
- [ ] No compiler warnings
- [ ] Clippy passes with no warnings
- [ ] Tests pass locally
- [ ] Commits have clear messages

## Documentation

### Module Documentation

Every public module should have a doc comment:

```rust
//! Brief description of module purpose.
//!
//! Longer explanation of what this module does and why.
```

### Function Documentation

```rust
/// Brief description.
///
/// # Arguments
///
/// * `param` - Description
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// Description of error conditions
///
/// # Example
///
/// ```
/// let result = function(arg)?;
/// ```
pub fn function(param: Type) -> Result<Output> {
}
```

### README Updates

Update documentation for:
- New phases completed
- New modules added
- Architecture changes
- Breaking changes

## Performance Considerations

1. **Avoid allocations in hot paths**
   - Pre-allocate when size is known
   - Use `&str` instead of `String` for parameters
   - Use iterators instead of collecting

2. **Benchmark improvements**
   ```bash
   cargo bench
   ```

3. **Profile memory usage**
   ```bash
   valgrind --leak-check=full ./target/debug/claudechic
   ```

4. **Monitor compilation time**
   ```bash
   cargo build -Z timings
   ```

## Debugging

### Adding Debug Output

Use `tracing` macros:

```rust
use tracing::{debug, info, warn, error};

debug!("Debug message");
info!("Important message");
warn!("Warning: {}", value);
error!("Error occurred");
```

### Enabling Debug Logging

```bash
RUST_LOG=debug cargo run
RUST_LOG=claudechic_core=debug,claudechic_tui=info cargo run
```

### Using Debugger

```bash
# Build for debugging
cargo build

# Run with debugger (lldb on macOS, gdb on Linux)
rust-lldb ./target/debug/claudechic
```

## Phase Implementation Guide

Each phase follows this pattern:

1. **Design** - Update ARCHITECTURE.md
2. **Implement** - Write code in modules
3. **Test** - Add unit and integration tests
4. **Document** - Update rustdoc and README
5. **Review** - Ensure quality

### Template for Phase PR

```markdown
# Phase N: [Feature Name]

## Changes
- [ ] New modules created
- [ ] Tests added
- [ ] Documentation updated
- [ ] Example code provided

## Depends On
- Phases X, Y (if any)

## Testing
- Run: `cargo test`
- Output: [Include test results]

## Notes
- Performance implications?
- Known limitations?
```

## Common Issues

### Compilation Errors

**Error**: `unresolved import`
- Solution: Check module is exported in parent `mod.rs`

**Error**: `cannot borrow as mutable`
- Solution: Add `mut` keyword to binding
- Or: Use interior mutability (`Mutex`, `Cell`, etc.)

**Error**: `lifetime mismatch`
- Solution: Check reference lifetimes match
- Or: Use owned values instead

### Testing Issues

**Async test hangs**
- Check: Is receiver being dropped?
- Check: Are channels unbounded?

**Flaky tests**
- Check: Timing-dependent assertions
- Check: Unordered collection comparisons

## Useful Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/)
- [Ratatui Guide](https://ratatui.rs/)
- [SQLx Guide](https://github.com/launchbadge/sqlx)
- [API Guidelines](https://rust-lang.github.io/api-guidelines/)

## Questions?

- Check existing issues and PRs
- Read architecture docs
- Refer to equivalent Python code
- Ask in discussions

Thank you for contributing!
