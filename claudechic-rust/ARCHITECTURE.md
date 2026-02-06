# Claude Chic Rust - Architecture

## High-Level Overview

```
┌─────────────────────────────────────────────────────────┐
│                    Terminal (Ratatui)                    │
├─────────────────────────────────────────────────────────┤
│                   claudechic-tui (Binary)               │
│  - Event loop        - Keybindings                      │
│  - UI rendering      - Commands                         │
│  - Widgets           - Screen management                │
├─────────────────────────────────────────────────────────┤
│              claudechic-core (Library)                  │
│  ┌──────────────────────────────────────────────────┐   │
│  │  Models (no dependencies)                        │   │
│  │  - Messages, Agents, Permissions, Events, Tools  │   │
│  └──────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  Agent Management (async logic)                  │   │
│  │  - SDK integration                               │   │
│  │  - Response processing                           │   │
│  │  - Permission handling                           │   │
│  └──────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  Persistence & Integration                      │   │
│  │  - Database (Supabase/SQLx)                      │   │
│  │  - Git operations                                │   │
│  │  - File indexing                                 │   │
│  │  - Session compaction                            │   │
│  └──────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────┤
│         External Services (Async Clients)              │
│  - Claude Agent SDK    - Supabase PostgreSQL           │
│  - Git2               - External APIs                  │
└─────────────────────────────────────────────────────────┘
```

## Crate Structure

### claudechic-core

**Purpose**: Pure business logic with no UI dependencies

**Layout**:
```
src/
├── lib.rs              # Entry point
├── error.rs            # Custom error types
├── config.rs           # Configuration loading
├── models/             # Data structures
│   ├── mod.rs
│   ├── message.rs      # Chat messages, blocks, content
│   ├── agent.rs        # Agent state and status
│   ├── permission.rs   # Permission requests and results
│   ├── events.rs       # Event types
│   └── tools.rs        # Tool use and results
├── agent/              # Agent lifecycle (Phase 3)
│   ├── mod.rs
│   ├── client.rs       # SDK client wrapper
│   ├── state.rs        # Agent state management
│   ├── response.rs     # Response processing
│   └── streaming.rs    # Text streaming
├── session/            # Session persistence (Phase 4)
│   ├── mod.rs
│   ├── persistence.rs
│   └── loader.rs
├── db/                 # Database layer (Phase 4)
│   ├── mod.rs
│   ├── models.rs
│   └── migrations.rs
├── permissions/        # Permission system (Phase 5)
│   ├── mod.rs
│   └── handler.rs
├── file_index.rs       # Fuzzy file search
├── git.rs              # Git operations
├── compact.rs          # Session compaction
└── analytics.rs        # Analytics integration
```

**Module Dependencies**:
```
error, config (no internal deps)
    ↓
models (uses error, config)
    ↓
agent, session, db, permissions
    ↓
file_index, git, compact, analytics
```

### claudechic-tui

**Purpose**: Terminal UI rendering and event handling

**Layout**:
```
src/
├── main.rs             # Entry point, CLI parsing
├── app.rs              # Main application state
├── event_handler.rs    # Event routing
├── terminal.rs         # Terminal setup/cleanup
├── ui/                 # UI components
│   ├── mod.rs
│   ├── theme.rs        # Color scheme
│   ├── layout.rs       # Layout management
│   └── screen/
│       ├── chat.rs     # Main screen
│       ├── session.rs  # Session picker
│       └── diff.rs     # Diff viewer
├── widgets/            # Reusable components
│   ├── mod.rs
│   ├── chat_message.rs
│   ├── chat_input.rs
│   ├── spinner.rs
│   ├── tool_use.rs
│   ├── footer.rs
│   ├── sidebar.rs
│   ├── prompts.rs
│   └── base.rs         # Base widget traits
└── commands/           # Command system (Phase 11)
    ├── mod.rs
    ├── parser.rs
    └── executor.rs
```

## Data Flow

### User Message Flow

```
User types message
    ↓
ChatInput captures text
    ↓
User presses Enter
    ↓
app.handle_key() → commands parsed
    ↓
app.send_message(prompt)
    ↓
agent.send(prompt) → SDK query
    ↓
SDK response stream
    ↓
Response handler processes chunks
    ↓
Events emitted: TextChunk, ToolUse, etc.
    ↓
ChatView renders updates
    ↓
Terminal frame rendered
```

### Agent Events Flow

```
agent.send() spawns response task
    ↓
SDK stream → process_response()
    ↓
match SDK message type:
    TextBlock → TextChunk event
    ToolUse → ToolUse event
    ToolResult → ToolResult event
    Complete → Complete event
    ↓
Events emitted via tokio::mpsc channel
    ↓
UI event loop receives events
    ↓
ChatView updated in response to events
    ↓
Next frame includes new content
```

### Permission Flow

```
SDK callback: can_use_tool(tool_name, input)
    ↓
Agent checks permission mode
    ↓
Match permission mode:
    Default → queue PermissionRequest
    AcceptEdits → auto-allow Edit/Write
    Plan → allow only plan-safe operations
    ↓
If queued: UI shows SelectionPrompt
    ↓
User selects: Allow, Deny, AllowSession, AllowAll
    ↓
Result sent via oneshot channel
    ↓
Callback returns PermissionResult
    ↓
SDK continues or stops
```

## Module Responsibilities

### Models (`claudechic-core/src/models/`)

- **message.rs**: ChatItem, MessageContent (User/Assistant), blocks
- **agent.rs**: Agent state, status, permission mode
- **permission.rs**: PermissionRequest, PermissionResult
- **events.rs**: Event enum for agent events
- **tools.rs**: ToolUse, ToolResult

Requirements: `serde`, `chrono`, `uuid`

### Agent Management (`claudechic-core/src/agent/`)

- **client.rs**: Wraps SDK client with connection management
- **state.rs**: Agent state updates and lifecycle
- **response.rs**: Process SDK response stream
- **streaming.rs**: Text chunk accumulation and buffering

Requirements: `tokio`, `futures`, models

### Persistence (`claudechic-core/src/session/` + `db/`)

- **session/persistence.rs**: Save/load agent state
- **session/loader.rs**: Load historical sessions
- **db/models.rs**: SQLx row mappers
- **db/migrations.rs**: Schema management

Requirements: `sqlx`, PostgreSQL

### Terminal UI (`claudechic-tui/`)

- **app.rs**: Main app state and event loop
- **widgets/**: Individual UI components
- **ui/theme.rs**: Color definitions
- **commands/**: Slash command handling

Requirements: `ratatui`, `crossterm`, core

## Event System

### Event Channels

**Agent → UI** (per-agent):
```rust
tokio::sync::mpsc::channel(1000) for AgentEvent
```

**UI Event Loop** (keyboard, mouse, resize):
```rust
crossterm event channel with Tokio tasks
```

### Event Types

```rust
enum AgentEvent {
    TextChunk { agent_id, text },
    ToolUse { agent_id, tool },
    ToolResult { agent_id, result },
    Complete { agent_id },
    Error { agent_id, message },
    StatusChanged { agent_id, message },
    PermissionNeeded { agent_id, request },
}
```

## Concurrency Model

### Single Agent
```
Main event loop
    ↓
[Response task] ← spawned on agent.send()
    ↓ (emits events)
[Event handler] ← processes events
    ↓
UI update
```

### Multi-Agent
```
Main event loop
    ↓
[Agent 1 response task]
[Agent 2 response task]
[Agent 3 response task]
    ↓ (broadcast channel)
Merge events
    ↓
UI updates (only active agent shown)
```

## Key Design Decisions

### Separation of Concerns

1. **Models** - Pure data structures, no logic
2. **Core** - Business logic, no UI
3. **TUI** - UI only, calls into core

Benefit: Core testable without UI, reusable in other frontends

### Observer Pattern with Channels

Instead of callbacks:
- Agent emits events
- UI subscribed to events
- Decouples timing and locking

Benefit: Natural async handling, no deadlocks

### Modular Widgets

Each widget self-contained:
- Input handling
- State management
- Rendering

Benefit: Easy to test, compose, reuse

### Database-First Sessions

Sessions stored in Supabase, not local files:
- Centralized history
- Cross-device access
- Future collaboration
- Backups automatic

## Testing Strategy

### Unit Tests
- Model constructors and methods
- Utility functions (file indexing, compaction)
- Permission logic

### Integration Tests
- Agent lifecycle with mock SDK
- Session save/load roundtrips
- Widget rendering
- Command parsing

### Property Tests
- Message parsing with randomized input
- Session compaction preserves state
- Diff generation consistency

## Performance Targets

| Metric | Target | Current |
|--------|--------|---------|
| Startup time | <2s | TBD |
| Memory (idle) | <50MB | TBD |
| Memory (with session) | <100MB | TBD |
| First message latency | <100ms | TBD |
| Widget render time | <16ms | TBD |
| Chat scroll smoothness | 60 FPS | TBD |

## Future Extensions

1. **Remote protocol** - Control from another terminal
2. **Plugin system** - Custom commands/widgets
3. **Web UI** - Parallel web frontend
4. **Collaboration** - Shared sessions
5. **Local-first** - Offline capability
