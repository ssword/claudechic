# Quick Start Guide

Get up and running with Claude Chic Rust development.

## Installation

### Prerequisites

- **Rust 1.70+**: [Install from rustup.rs](https://rustup.rs/)
- **PostgreSQL 12+** or **Supabase account**
- **Git**

Verify installation:
```bash
rustc --version
cargo --version
```

### Clone & Setup

```bash
# Navigate to project
cd claudechic/claudechic-rust

# Verify structure
ls -la

# Check Rust toolchain
cat rust-toolchain.toml
```

## Building

### Debug Build (Fast iteration)

```bash
cargo build
./target/debug/claudechic
```

### Release Build (Optimized)

```bash
cargo build --release
./target/release/claudechic
```

### Check Without Building

```bash
cargo check
```

## Testing

### Run All Tests

```bash
cargo test
```

### Run Specific Test

```bash
cargo test test_agent_creation
```

### Run with Output

```bash
cargo test -- --nocapture
```

## Code Quality

### Format Code

```bash
# Format all Rust files
cargo fmt

# Check if formatting needed
cargo fmt -- --check
```

### Lint with Clippy

```bash
# Run linter
cargo clippy

# Strict mode (all warnings)
cargo clippy -- -D warnings

# With all features
cargo clippy --all-targets --all-features
```

### Combined Check

```bash
# Format, lint, and test
cargo fmt && cargo clippy -- -D warnings && cargo test
```

## Project Structure

```
claudechic-rust/
├── claudechic-core/          # Core library
│   └── src/
│       ├── models/           # Data structures (complete Phase 2)
│       ├── agent/            # Agent management (Phase 3)
│       ├── session/          # Persistence (Phase 4)
│       ├── error.rs          # Error types
│       ├── config.rs         # Configuration
│       └── lib.rs            # Library entry
│
├── claudechic-tui/           # Terminal UI (Ratatui)
│   └── src/
│       ├── main.rs           # Binary entry point
│       ├── app.rs            # Main app
│       ├── ui/               # Theme and layout
│       ├── widgets/          # UI components
│       └── commands/         # Command system
│
└── docs/
    ├── README.md             # Overview
    ├── ARCHITECTURE.md       # Design decisions
    ├── CONTRIBUTING.md       # Development guide
    └── IMPLEMENTATION_STATUS # Progress tracking
```

## Development Workflow

### 1. Explore Code

Start with reading key files:

```bash
# Understand project structure
cat README.md
cat ARCHITECTURE.md

# See current models
cat claudechic-core/src/models/message.rs
cat claudechic-core/src/models/agent.rs

# Check main binary
cat claudechic-tui/src/main.rs
```

### 2. Make Changes

Pick a small feature or bug:

```bash
# Create feature branch
git checkout -b feature/my-feature

# Edit files
vim claudechic-core/src/models/agent.rs

# Check changes compile
cargo check
```

### 3. Test Changes

```bash
# Run tests
cargo test

# Run specific test
cargo test test_agent_new

# See output
cargo test -- --nocapture --test-threads=1
```

### 4. Format & Lint

```bash
# Format code
cargo fmt

# Check linting
cargo clippy -- -D warnings
```

### 5. Commit & Push

```bash
# Commit changes
git commit -m "Add feature: description"

# Push to GitHub
git push origin feature/my-feature
```

## Common Tasks

### Add a New Module

1. Create file: `claudechic-core/src/new_module.rs`
2. Add to `lib.rs`: `pub mod new_module;`
3. Add unit tests
4. Run: `cargo test`

### Add a Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        let result = function();
        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = async_fn().await;
        assert!(result.is_ok());
    }
}
```

### Debug Async Code

Enable debug logging:
```bash
RUST_LOG=debug cargo run
```

### Check Dependencies

```bash
# See dependency tree
cargo tree

# Check for updates
cargo outdated

# Check security
cargo audit
```

## Environment Setup

### Create `.env` file

```bash
cat > .env << EOF
DATABASE_URL=postgresql://user:password@localhost/claudechic
SUPABASE_URL=https://xxx.supabase.co
SUPABASE_KEY=xxx
ANTHROPIC_API_KEY=sk-...
RUST_LOG=debug
EOF
```

### Load in Shell

```bash
# Load from .env
export $(cat .env | xargs)

# Or use direnv
direnv allow
```

## Useful Commands Reference

```bash
# Development
cargo check              # Quick syntax check
cargo build              # Build debug binary
cargo build --release    # Build optimized binary
cargo run                # Build and run
cargo test               # Run all tests

# Code Quality
cargo fmt                # Format code
cargo fmt --check        # Check formatting
cargo clippy             # Lint code
cargo clippy -- -D warnings  # Strict linting

# Dependencies
cargo tree               # Show dependency tree
cargo tree --depth=1     # Top-level dependencies
cargo outdated           # Check for updates
cargo audit              # Security audit

# Documentation
cargo doc --open         # Generate and open docs
cargo doc                # Generate docs

# Profiling
cargo build --release    # For profiling
cargo bench              # Run benchmarks (when added)
```

## Phase-Specific Tasks

### Phase 2: Models

```bash
# View message models
cat claudechic-core/src/models/message.rs

# Run model tests
cargo test models

# Generate docs for models
cargo doc --open
# Navigate to claudechic_core::models
```

### Phase 3: SDK (When Starting)

```bash
# Create new agent module
touch claudechic-core/src/agent.rs

# Add imports and stubs
# Run cargo check frequently
cargo check
```

## Troubleshooting

### Build Fails with "cargo: command not found"

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Tests Hang

Check for:
- Dropped channel receivers
- Deadlocks in async code
- Unbounded channel fills

Run single-threaded:
```bash
cargo test -- --test-threads=1
```

### Weird Compilation Errors

Clean and rebuild:
```bash
cargo clean
cargo build
```

### Clippy Errors

Update Rust:
```bash
rustup update
cargo clippy
```

## Resources

- **Rust Book**: https://doc.rust-lang.org/book/
- **Tokio Guide**: https://tokio.rs/tokio/tutorial
- **Ratatui**: https://ratatui.rs/
- **SQLx Docs**: https://github.com/launchbadge/sqlx
- **Error Handling**: https://docs.rs/anyhow/latest/anyhow/

## Getting Help

1. **Check docs**: `cargo doc --open`
2. **Read architecture**: `cat ARCHITECTURE.md`
3. **Review examples**: Look at existing tests
4. **Search issues**: GitHub issues and discussions
5. **Ask questions**: Create a discussion

## Next Steps

- [x] Install Rust and clone project
- [ ] Run `cargo test` to verify setup
- [ ] Read ARCHITECTURE.md
- [ ] Run `cargo build --release`
- [ ] Pick a task from IMPLEMENTATION_STATUS.md
- [ ] Start coding!

---

**Happy coding! 🦀**
