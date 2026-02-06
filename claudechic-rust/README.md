# Claude Chic - Rust Rewrite

A high-performance terminal UI for Claude Code, built with Rust for improved performance, memory safety, and native compilation.

## Status

**Phase 1 Complete**: Project Setup and Structure ✓

Current phase: Phase 2 - Core Data Models and Types (In Progress)

## Features

- ✓ Modular Rust architecture with separate core and UI crates
- ✓ Foundational data models (messages, agents, permissions, events, tools)
- ✓ Type-safe error handling with custom error types
- ✓ Tokio-based async runtime
- ✓ Ratatui terminal UI framework
- 🔄 Multi-agent support
- 🔄 Supabase integration for session persistence
- 🔄 Claude Agent SDK integration
- 🔄 Git worktree support
- 🔄 Session management and compaction
- 🔄 Permission system with plan mode support

## Project Structure

```
claudechic-rust/
├── claudechic-core/        # Core business logic (no UI dependencies)
│   └── src/
│       ├── models/         # Data structures
│       ├── agent/          # Agent lifecycle (Phase 3)
│       ├── session/        # Session persistence (Phase 4)
│       ├── db/             # Database integration (Phase 4)
│       ├── permissions/    # Permission handling (Phase 5)
│       └── config.rs       # Configuration
├── claudechic-tui/         # Terminal UI (Ratatui)
│   └── src/
│       ├── app.rs          # Main application
│       ├── widgets/        # UI components
│       ├── ui/             # Theme and layout
│       └── commands/       # Command system
└── README.md
```

## Development

### Prerequisites

- Rust 1.70+ (see `rust-toolchain.toml`)
- PostgreSQL 12+ (or Supabase)

### Building

```bash
# Build the project
cargo build

# Build with optimizations
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint with clippy
cargo clippy -- -D warnings
```

### Project Configuration

- **Workspace**: Multi-crate workspace for separation of concerns
- **Profiles**:
  - `dev`: Fast iteration with minimal optimization
  - `release`: Full optimization with LTO
- **Dependencies**: Minimal, well-maintained crates from Tokio ecosystem

## Phases Overview

### ✓ Phase 1: Project Setup (Complete)
- Workspace structure with core and UI crates
- Base Cargo configuration
- Modular file organization
- Initial module structure

### Phase 2: Core Data Models (In Progress)
- Message content types (user, assistant, tool use)
- Agent state management
- Permission request handling
- Event types
- Tool tracking
- Serialization support

### Phase 3: SDK and Connection
- Claude Agent SDK integration
- Agent lifecycle management
- Response processing and streaming
- Permission callbacks
- Event emission

### Phase 4: Supabase Integration
- Database schema and migrations
- SQLx connection management
- Session persistence
- Message history storage
- Query optimization

### Phase 5+: UI, Features, Testing, Deployment

See `RUST_REWRITE_PLAN.md` for complete 15-phase plan.

## Key Differences from Python

- **Performance**: Compiled binary vs interpreted
- **Memory**: Typically 50MB vs 150-300MB
- **Startup**: <2s target vs 3-5s Python
- **Concurrency**: Native Tokio async vs asyncio
- **Type Safety**: Compile-time guarantees
- **Distribution**: Single binary vs runtime dependency

## Architecture Highlights

### Pure Logic Separation
Core library (`claudechic-core`) contains zero UI dependencies and can be tested independently.

### Observer Pattern
Events flow through Tokio channels:
- Agent events → UI updates
- Permission requests → UI prompts
- AgentManager events → sidebar updates

### Multi-Agent Support
Each agent runs in its own Tokio task with concurrent event processing.

### Database First
Sessions stored in Supabase, not local files. Enables cross-device resume and future collaboration features.

## Environment Variables

```bash
DATABASE_URL=postgresql://user:pass@localhost/claudechic
SUPABASE_URL=https://xxx.supabase.co
SUPABASE_KEY=xxx
ANTHROPIC_API_KEY=xxx
```

## Next Steps

1. ✓ Phase 1: Project setup
2. Complete Phase 2: Data models (in progress)
3. Phase 3: SDK integration
4. Phase 4: Database schema and migrations
5. Phase 5+: UI and features

## Contributing

See CONTRIBUTING.md for development guidelines.

## License

MIT - See LICENSE in parent directory

## References

- [Ratatui Documentation](https://ratatui.rs/)
- [Tokio Guide](https://tokio.rs/tokio/tutorial)
- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [Rust Book](https://doc.rust-lang.org/book/)
