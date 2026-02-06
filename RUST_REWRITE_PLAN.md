# Claude Chic Rust Rewrite Plan

A comprehensive plan to rewrite the Claude Chic terminal UI from Python to Rust, creating an independent project with improved performance, memory safety, and compile-time guarantees.

**Status**: Planning
**Target Directory**: `claudechic-rust/` (independent folder at repository root)
**Target Branch**: `rust-rewrite`
**SDK**: `claude-agent-sdk-rs`
**Database**: Supabase PostgreSQL
**TUI Framework**: Ratatui
**Async Runtime**: Tokio

---

## Table of Contents

1. [Overview](#overview)
2. [Project Architecture](#project-architecture)
3. [Phase 1: Project Setup and Structure](#phase-1-project-setup-and-structure)
4. [Phase 2: Core Data Models and Types](#phase-2-core-data-models-and-types)
5. [Phase 3: SDK and Connection Layer](#phase-3-sdk-and-connection-layer)
6. [Phase 4: Supabase Integration Layer](#phase-4-supabase-integration-layer)
7. [Phase 5: Message History and Event Processing](#phase-5-message-history-and-event-processing)
8. [Phase 6: Terminal UI Foundation](#phase-6-terminal-ui-foundation)
9. [Phase 7: Core UI Widgets](#phase-7-core-ui-widgets)
10. [Phase 8: Multi-Agent Support](#phase-8-multi-agent-support)
11. [Phase 9: Advanced Widgets and Features](#phase-9-advanced-widgets-and-features)
12. [Phase 10: File System and Git Integration](#phase-10-file-system-and-git-integration)
13. [Phase 11: Command System and Keybindings](#phase-11-command-system-and-keybindings)
14. [Phase 12: Permission System](#phase-12-permission-system)
15. [Phase 13: Session Management and Persistence](#phase-13-session-management-and-persistence)
16. [Phase 14: Testing Infrastructure](#phase-14-testing-infrastructure)
17. [Phase 15: Documentation and Deployment](#phase-15-documentation-and-deployment)
18. [Technical Decisions](#technical-decisions)
19. [Dependencies](#dependencies)
20. [File Structure](#file-structure)

---

## Overview

Claude Chic is a stylish terminal UI for the Claude Code agent SDK. The current implementation in Python provides a sophisticated multi-agent experience with advanced features like session management, git worktree support, and real-time streaming.

This rewrite to Rust will:
- **Improve Performance**: Native compilation with zero-cost abstractions
- **Enhance Memory Safety**: Leverage Rust's type system and borrow checker
- **Maintain Feature Parity**: Support all existing Python version features
- **Improve Deployment**: Single binary distribution without runtime dependencies
- **Use Modern Async**: Tokio-based concurrent agent support
- **Centralize Persistence**: Supabase replaces local JSONL files

### Key Goals

1. Achieve 100% feature parity with Python version
2. Maintain identical user experience and keybindings
3. Improve startup time and memory consumption
4. Enable easier distribution and installation
5. Create foundation for future enhancements
6. Establish Rust best practices in the codebase

---

## Project Architecture

### Crate Structure

```
claudechic-rust/
├── Cargo.toml                 # Workspace root
├── claudechic-core/           # Core business logic (no UI deps)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── agent/
│   │   ├── session/
│   │   ├── permissions/
│   │   └── models/
│   └── tests/
├── claudechic-tui/            # Terminal UI layer
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── app.rs
│   │   ├── ui/
│   │   ├── widgets/
│   │   └── events/
│   └── tests/
├── claudechic/                # CLI binary wrapper
│   └── Cargo.toml
└── README.md
```

### Module Organization

**Pure Logic (no dependencies on UI or SDK)**:
- `models/` - Data structures (ChatItem, UserContent, ToolUse, etc.)
- `session/` - Session persistence, loading, listing
- `file_index/` - Fuzzy file search, git integration
- `permissions/` - Permission request queuing
- `compact/` - Session compaction logic

**Agent Layer** (depends on SDK, but not UI):
- `agent/` - Agent lifecycle, message history, status
- `agent_manager/` - Multi-agent coordination
- `permissions/handlers/` - Permission callback system

**UI Layer** (depends on everything):
- `app.rs` - Main event loop and application state
- `ui/` - Screen layouts and rendering
- `widgets/` - Reusable UI components
- `events/` - Event handling and routing

### Data Flow

```
User Input
    ↓
Event Handler (app.rs)
    ↓
Command Router / Widget Handler
    ↓
Agent.send() or Agent.interrupt()
    ↓
claude-agent-sdk-rs Client
    ↓
SDK Response Stream
    ↓
Message Handler (_handle_sdk_message)
    ↓
History Update & Supabase Save
    ↓
UI Update (ChatView rendering)
    ↓
Ratatui Frame Render
    ↓
Terminal Output
```

### Observer Pattern

Events flow through async channels:

```
Agent Events (tokio::sync::mpsc)
├── on_text_chunk()
├── on_tool_use()
├── on_tool_result()
├── on_complete()
├── on_status_changed()
├── on_prompt_added()
└── on_permission_mode_changed()

AgentManager Events (tokio::sync::broadcast)
├── on_agent_created()
├── on_agent_switched()
└── on_agent_closed()
```

---

## Phase 1: Project Setup and Structure

**Objective**: Establish project foundation, dependencies, and folder structure

### Tasks

1. Create project directory structure
   - Create `claudechic-rust/` folder at repository root
   - Initialize workspace `Cargo.toml` with members
   - Create `claudechic-core` library crate
   - Create `claudechic-tui` binary crate
   - Create `claudechic` CLI wrapper crate

2. Configure Cargo dependencies
   - Add core dependencies: `tokio`, `ratatui`, `serde`, `clap`
   - Add SDK dependency: `claude-agent-sdk-rs`
   - Add database: `sqlx` with Postgres feature
   - Add serialization: `serde_json`, `serde_yaml`
   - Add error handling: `anyhow`, `thiserror`
   - Add logging: `tracing`, `tracing-subscriber`
   - Add utilities: `async-trait`, `futures`, `once_cell`

3. Set up configuration files
   - Create `.cargo/config.toml` for optimization profiles
   - Create `rust-toolchain.toml` pinning stable channel
   - Create `.rustfmt.toml` for code formatting
   - Create `clippy.toml` for linting rules
   - Create pre-commit hooks for format/lint checks

4. Initialize module structure
   - Create `claudechic-core/src/lib.rs` with module declarations
   - Create `claudechic-tui/src/main.rs` with basic structure
   - Create subdirectories for logical grouping
   - Add module documentation comments

5. Create build configuration
   - Set up release profile with LTO and codegen-units=1
   - Configure debug profile for faster compilation
   - Set up feature flags for optional functionality
   - Create build scripts if needed for code generation

6. Documentation setup
   - Create root `README.md` for Rust version
   - Create `ARCHITECTURE.md` documenting design decisions
   - Create `CONTRIBUTING.md` for development guidelines
   - Create changelog template

### Deliverables

- Working Cargo workspace with all crates compiling
- Basic project structure matching Python organization
- Clean separation between core, UI, and binary layers
- Ready for phase 2 implementation

---

## Phase 2: Core Data Models and Types

**Objective**: Define all fundamental data structures used throughout the application

### Tasks

1. Define message content types
   ```rust
   // In claudechic-core/src/models/message.rs

   pub struct ImageAttachment {
       pub path: String,
       pub filename: String,
       pub media_type: String,
       pub base64_data: String,
   }

   pub struct UserContent {
       pub text: String,
       pub images: Vec<ImageAttachment>,
   }

   pub struct TextBlock {
       pub text: String,
   }

   pub struct ToolUse {
       pub id: String,
       pub name: String,
       pub input: serde_json::Value,
       pub parent_tool_use_id: Option<String>,
       pub result: Option<String>,
       pub is_error: bool,
   }

   pub enum AssistantBlock {
       Text(TextBlock),
       ToolUse(Box<ToolUse>),
   }

   pub struct AssistantContent {
       pub blocks: Vec<AssistantBlock>,
   }

   pub enum MessageContent {
       User(UserContent),
       Assistant(AssistantContent),
   }

   pub struct ChatItem {
       pub role: Role, // enum: User, Assistant
       pub content: MessageContent,
   }
   ```

2. Define agent state types
   ```rust
   // In claudechic-core/src/models/agent.rs

   #[derive(Clone, Copy, PartialEq, Eq)]
   pub enum AgentStatus {
       Idle,
       Busy,
       NeedsInput,
   }

   #[derive(Clone, PartialEq, Eq)]
   pub enum PermissionMode {
       Default,
       AcceptEdits,
       Plan,
   }

   pub struct Agent {
       pub id: String,
       pub name: String,
       pub cwd: PathBuf,
       pub worktree: Option<String>,
       pub status: AgentStatus,
       pub messages: Vec<ChatItem>,
       pub permission_mode: PermissionMode,
       // ... additional fields
   }
   ```

3. Define permission types
   ```rust
   // In claudechic-core/src/models/permission.rs

   #[derive(Clone, Copy)]
   pub enum PermissionChoice {
       Allow,
       Deny,
       AllowSession,
       AllowAll,
   }

   pub struct PermissionRequest {
       pub tool_name: String,
       pub tool_input: serde_json::Value,
       pub choice: tokio::sync::oneshot::Receiver<PermissionChoice>,
   }
   ```

4. Define tool types
   ```rust
   // In claudechic-core/src/models/tools.rs

   pub enum ToolName {
       Edit,
       Write,
       Bash,
       NotebookEdit,
       Task,
       TodoWrite,
       // ... all other tools
   }

   pub struct ToolResult {
       pub tool_use_id: String,
       pub content: String,
       pub is_error: bool,
   }
   ```

5. Create serialization implementations
   - Implement `Serialize` and `Deserialize` for all types
   - Create custom serializers for complex types (e.g., base64 images)
   - Set up JSON schema for session storage
   - Create database row mappers for sqlx

6. Add validation and constructors
   - Implement builder patterns for complex types
   - Add validation in constructors
   - Create factory functions for common patterns
   - Add Display and Debug implementations

### Deliverables

- Complete type definitions for all domain objects
- Serialization support for Supabase persistence
- Clean API for constructing and manipulating domain objects
- Well-documented type hierarchy

---

## Phase 3: SDK and Connection Layer

**Objective**: Integrate claude-agent-sdk-rs and manage agent lifecycle

### Tasks

1. Wrap SDK client
   ```rust
   // In claudechic-core/src/agent/client.rs

   pub struct SdkClient {
       client: ClaudeSDKClient,
       connection: tokio::sync::Mutex<Option<ClientConnection>>,
   }

   impl SdkClient {
       pub async fn connect(options: ClaudeAgentOptions) -> Result<Self>;
       pub async fn disconnect(&self) -> Result<()>;
       pub async fn query(&self, prompt: &str) -> Result<impl Stream<Item=Message>>;
       pub async fn interrupt(&self) -> Result<()>;
       pub async fn set_permission_mode(&self, mode: PermissionMode) -> Result<()>;
   }
   ```

2. Implement Agent lifecycle
   ```rust
   // In claudechic-core/src/agent/mod.rs

   pub struct Agent {
       // Identity
       pub id: String,
       pub name: String,
       pub cwd: PathBuf,
       pub worktree: Option<String>,

       // SDK
       client: Option<SdkClient>,
       session_id: Option<String>,
       response_task: Option<JoinHandle<()>>,

       // Status and state
       status: AgentStatus,
       messages: Vec<ChatItem>,
       pending_prompts: VecDeque<PermissionRequest>,

       // Permissions
       permission_mode: PermissionMode,
       session_allowed_tools: HashSet<String>,

       // Observers
       observer: Option<Box<dyn AgentObserver>>,
       permission_handler: Option<Box<dyn PermissionHandler>>,

       // Channels for events
       event_tx: mpsc::Sender<AgentEvent>,
   }

   impl Agent {
       pub async fn connect(&mut self, options: ClaudeAgentOptions, resume: Option<String>) -> Result<()>;
       pub async fn disconnect(&mut self) -> Result<()>;
       pub async fn send(&mut self, prompt: &str) -> Result<()>;
       pub async fn interrupt(&mut self) -> Result<()>;
       pub async fn load_history(&mut self, cwd: &Path) -> Result<()>;
   }
   ```

3. Implement permission callback
   ```rust
   // In claudechic-core/src/permissions/handler.rs

   pub async fn handle_permission(
       agent: &mut Agent,
       tool_name: &str,
       tool_input: serde_json::Value,
   ) -> Result<PermissionResult>;
   ```

4. Create response processor
   ```rust
   // In claudechic-core/src/agent/response.rs

   pub async fn process_response(
       agent: &mut Agent,
       stream: impl Stream<Item=SdkMessage>,
   ) -> Result<()>;

   async fn handle_sdk_message(
       agent: &mut Agent,
       message: SdkMessage,
   ) -> Result<()>;
   ```

5. Implement message streaming
   - Handle `TextBlock` messages with streaming display
   - Track `ToolUse` blocks with pending tool dictionary
   - Process `ToolResult` blocks with error detection
   - Manage message flushing to history

6. Add plan mode instruction handling
   - Build plan mode system message
   - Prepend to prompts when in plan mode
   - Handle plan file path resolution
   - Implement read-only enforcement for blocked tools

7. Set up event emission
   - Create channel-based event system
   - Emit events for text chunks, tool uses, completions
   - Implement observer trait for UI integration
   - Add async-trait for trait objects

### Deliverables

- Working SDK client integration
- Agent lifecycle management (connect, disconnect, send, interrupt)
- Message history tracking and streaming support
- Permission callback system
- Event emission for UI layer

---

## Phase 4: Supabase Integration Layer

**Objective**: Set up database persistence and session management

### Tasks

1. Create database schema
   ```sql
   -- migrations/001_create_sessions.sql
   CREATE TABLE sessions (
       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
       name TEXT NOT NULL,
       cwd TEXT NOT NULL,
       session_id TEXT UNIQUE,
       created_at TIMESTAMPTZ DEFAULT now(),
       updated_at TIMESTAMPTZ DEFAULT now()
   );

   CREATE TABLE session_messages (
       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
       session_id UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
       role TEXT NOT NULL, -- 'user' or 'assistant'
       message_type TEXT NOT NULL, -- 'text', 'tool_use'
       content JSONB NOT NULL,
       checkpoint_uuid TEXT,
       created_at TIMESTAMPTZ DEFAULT now(),
       INDEX session_idx (session_id),
       INDEX checkpoint_idx (checkpoint_uuid)
   );
   ```

2. Set up sqlx integration
   ```rust
   // In claudechic-core/src/db/mod.rs

   pub struct Database {
       pool: sqlx::PgPool,
   }

   impl Database {
       pub async fn connect(database_url: &str) -> Result<Self>;
       pub async fn save_session(&self, session: &SessionRecord) -> Result<()>;
       pub async fn load_session_messages(&self, session_id: &str) -> Result<Vec<ChatItem>>;
       pub async fn list_sessions(&self, cwd: Option<&Path>) -> Result<Vec<SessionInfo>>;
       pub async fn delete_session(&self, session_id: &str) -> Result<()>;
   }
   ```

3. Implement session persistence
   ```rust
   // In claudechic-core/src/session/persistence.rs

   pub async fn save_agent_state(db: &Database, agent: &Agent) -> Result<()>;
   pub async fn load_agent_state(db: &Database, session_id: &str) -> Result<Agent>;
   pub async fn create_new_session(db: &Database, name: &str, cwd: &Path) -> Result<String>;
   ```

4. Create migration system
   - Write SQL migration files for schema
   - Implement migration runner using sqlx-cli
   - Version migrations with timestamps
   - Create rollback procedures

5. Implement session listing
   - Query sessions by directory
   - Support filtering and sorting
   - Cache session metadata
   - Handle session cleanup

6. Add checkpoint tracking
   - Store message UUIDs for rewind capability
   - Link checkpoints to specific messages
   - Enable message restoration from checkpoints

7. Set up connection pooling
   - Configure pool size based on environment
   - Handle connection errors gracefully
   - Implement reconnection logic
   - Add metrics for pool utilization

### Deliverables

- Supabase database schema created
- SQLx integration with connection pooling
- Session save and load functionality
- Session listing and filtering
- Checkpoint tracking for rewind support

---

## Phase 5: Message History and Event Processing

**Objective**: Implement robust message handling and streaming support

### Tasks

1. Implement text chunk accumulation
   ```rust
   // In claudechic-core/src/agent/streaming.rs

   pub struct StreamingState {
       current_text_buffer: String,
       current_assistant: Option<AssistantContent>,
       needs_new_message: bool,
   }

   impl StreamingState {
       pub fn handle_text_chunk(&mut self, text: &str) -> Option<ChatMessage>;
       pub fn flush_text(&mut self) -> Option<TextBlock>;
   }
   ```

2. Create tool use tracking
   ```rust
   // In claudechic-core/src/agent/tools.rs

   pub struct ToolTracker {
       pending_tools: HashMap<String, ToolUse>,
       active_tasks: HashMap<String, String>,
   }

   impl ToolTracker {
       pub fn add_tool(&mut self, tool: ToolUse) -> Result<()>;
       pub fn add_tool_result(&mut self, tool_id: &str, result: &str, is_error: bool) -> Result<Option<ToolUse>>;
       pub fn get_task_output(&self, task_id: &str) -> Option<&str>;
       pub fn accumulate_task_text(&mut self, task_id: &str, text: &str);
   }
   ```

3. Implement SDK message dispatcher
   ```rust
   // In claudechic-core/src/agent/message_handler.rs

   pub async fn handle_stream_event(
       agent: &mut Agent,
       event: StreamEvent,
   ) -> Result<()>;

   pub async fn handle_assistant_message(
       agent: &mut Agent,
       message: AssistantMessage,
   ) -> Result<()>;

   pub async fn handle_tool_result_block(
       agent: &mut Agent,
       block: ToolResultBlock,
   ) -> Result<()>;
   ```

4. Create error handling
   - Distinguish connection errors vs API errors
   - Implement auto-reconnect logic
   - Store error context for debugging
   - Emit error events to UI

5. Implement message flushing
   - Flush text buffers to history when needed
   - Ensure tool uses are complete before flushing
   - Maintain proper message ordering
   - Persist to Supabase after each flush

6. Add plan mode instruction handling
   - Build system reminder for plan mode
   - Prepend to user prompts
   - Extract and apply plan file constraints
   - Handle plan path resolution

7. Create event emission
   - Define `AgentEvent` enum for all event types
   - Create event channel in Agent struct
   - Emit events for all state changes
   - Implement backpressure handling for high-frequency events

### Deliverables

- Streaming text accumulation with proper buffering
- Tool use and task tracking
- SDK message dispatching and routing
- Error handling and recovery
- Event emission system for UI integration

---

## Phase 6: Terminal UI Foundation

**Objective**: Build the base terminal UI infrastructure using Ratatui

### Tasks

1. Set up Ratatui application
   ```rust
   // In claudechic-tui/src/app.rs

   pub struct App {
       // State
       agents: Vec<Agent>,
       current_agent_idx: usize,
       mode: AppMode, // Normal, Insert, Command

       // Event channels
       agent_event_rx: mpsc::Receiver<AgentEvent>,
       ui_event_tx: mpsc::Sender<UiEvent>,

       // UI state
       scroll_pos: u16,
       input_buffer: String,

       // Database
       db: Database,
   }

   impl App {
       pub fn new(db: Database) -> Self;
       pub async fn update(&mut self, event: Event) -> Result<()>;
       pub fn render(&self) -> Vec<ratatui::Frame>;
   }
   ```

2. Create event loop
   ```rust
   // In claudechic-tui/src/main.rs

   pub async fn run_app() -> Result<()> {
       let terminal = Terminal::new(
           CrosstermBackend::new(stdout())
       )?;

       let mut app = App::new(database)?;
       let mut event_reader = EventReader::new();

       loop {
           terminal.draw(|f| app.render(f))?;

           match event_reader.next()? {
               Event::Key(key) => app.handle_key(key).await?,
               Event::Mouse(mouse) => app.handle_mouse(mouse).await?,
               Event::Resize(w, h) => app.handle_resize(w, h)?,
               _ => {}
           }

           if app.should_quit {
               break;
           }
       }

       Ok(())
   }
   ```

3. Implement terminal initialization
   - Enable raw mode on startup
   - Disable canonical mode for real-time input
   - Set up panic handler for cleanup
   - Restore terminal on exit

4. Create color scheme and theming
   ```rust
   // In claudechic-tui/src/theme.rs

   pub struct Theme {
       pub user_message: Color,    // Orange #cc7700
       pub assistant_message: Color, // Blue #334455
       pub tool_use: Color,        // Gray #333333
       pub error: Color,           // Red
       pub success: Color,         // Green
       // ... more colors
   }
   ```

5. Implement viewport management
   - Track scroll position
   - Handle screen resizing
   - Calculate visible area
   - Implement scrolling with arrow keys/Page Up/Down

6. Build frame rendering pipeline
   - Create layout manager for widget positioning
   - Implement constraint system for responsive design
   - Support horizontal and vertical splits
   - Handle overlays and modals

7. Add focus management
   - Track which widget has focus
   - Route events to focused widget
   - Support Tab key for focus cycling
   - Highlight focused widgets

### Deliverables

- Working Ratatui application with event loop
- Terminal setup and cleanup
- Color scheme and theme system
- Viewport and scroll management
- Frame rendering infrastructure

---

## Phase 7: Core UI Widgets

**Objective**: Build reusable UI components for displaying content

### Tasks

1. Create ChatMessage widget
   ```rust
   // In claudechic-tui/src/widgets/chat_message.rs

   pub struct ChatMessageWidget<'a> {
       message: &'a ChatItem,
       theme: &'a Theme,
   }

   impl Widget for ChatMessageWidget<'_> {
       fn render(self, area: Rect, buf: &mut Buffer);
   }
   ```
   - Display user messages with orange left border
   - Display assistant messages with blue left border
   - Wrap text at viewport width
   - Support inline images (display as [Image: filename])

2. Create ChatInput widget
   ```rust
   // In claudechic-tui/src/widgets/chat_input.rs

   pub struct ChatInputWidget {
       text: String,
       cursor_pos: usize,
       history: Vec<String>,
       history_pos: usize,
   }

   impl ChatInputWidget {
       pub fn handle_char(&mut self, c: char);
       pub fn handle_key(&mut self, key: KeyCode) -> Option<String>; // Returns Some when Enter pressed
   }
   ```
   - Support multiline input
   - Implement history with Up/Down arrows
   - Show character count
   - Support Ctrl+U (clear line), Ctrl+W (delete word)

3. Create ThinkingIndicator widget
   ```rust
   // In claudechic-tui/src/widgets/spinner.rs

   pub struct ThinkingIndicator {
       frame: usize,
       animation: &'static [&'static str],
   }

   impl Widget for ThinkingIndicator {
       fn render(self, area: Rect, buf: &mut Buffer);
   }
   ```
   - Implement animated spinner with smooth transitions
   - Cycle through frames at 10Hz

4. Create ToolUseWidget
   ```rust
   // In claudechic-tui/src/widgets/tool_use.rs

   pub struct ToolUseWidget<'a> {
       tool: &'a ToolUse,
       expanded: bool,
       theme: &'a Theme,
   }

   impl Widget for ToolUseWidget<'_> {
       fn render(self, area: Rect, buf: &mut Buffer);
   }
   ```
   - Display tool name and input
   - Show collapsible details
   - Display result when available
   - Highlight errors in red

5. Create StatusFooter widget
   ```rust
   // In claudechic-tui/src/widgets/footer.rs

   pub struct StatusFooter<'a> {
       agent: &'a Agent,
       context_percent: f32,
       cpu_percent: f32,
   }

   impl Widget for StatusFooter<'_> {
       fn render(self, area: Rect, buf: &mut Buffer);
   }
   ```
   - Show current model
   - Display context usage percentage
   - Show permission mode indicator
   - Display typing indicator when sending

6. Create syntax highlighting
   - Integrate syntect for code highlighting
   - Cache language detection results
   - Support common languages (rust, python, js, etc.)
   - Fallback to monospace when no highlighting

7. Create copy button and hover effects
   - Show copy button on hover over code blocks
   - Implement clipboard integration with `copypasta` or similar
   - Show visual feedback on successful copy
   - Add keyboard shortcut for copy (Ctrl+C in selection)

### Deliverables

- ChatMessage widget with proper formatting
- ChatInput widget with history and editing
- ThinkingIndicator animated spinner
- ToolUseWidget with expandable details
- StatusFooter with indicators
- Syntax highlighting for code blocks

---

## Phase 8: Multi-Agent Support

**Objective**: Implement concurrent multi-agent coordination and UI

### Tasks

1. Create AgentManager
   ```rust
   // In claudechic-core/src/agent_manager.rs

   pub struct AgentManager {
       agents: Vec<Agent>,
       current_idx: usize,
       event_tx: broadcast::Sender<ManagerEvent>,
   }

   impl AgentManager {
       pub async fn create_agent(&mut self, name: &str, cwd: &Path) -> Result<&mut Agent>;
       pub async fn close_agent(&mut self, idx: usize) -> Result<()>;
       pub fn switch_agent(&mut self, idx: usize) -> Result<()>;
       pub fn list_agents(&self) -> Vec<&Agent>;
   }
   ```

2. Build agent switching logic
   - Map Ctrl+1-9 to agent indices
   - Save input text when switching away
   - Restore input text when switching back
   - Emit agent switched event

3. Implement concurrent execution
   - Spawn separate response task per agent
   - Use broadcast channels for UI events
   - Handle agent status updates independently
   - Manage task cancellation on agent close

4. Create AgentSidebar widget
   ```rust
   // In claudechic-tui/src/widgets/sidebar.rs

   pub struct AgentSidebar<'a> {
       manager: &'a AgentManager,
       current_idx: usize,
   }

   impl Widget for AgentSidebar<'_> {
       fn render(self, area: Rect, buf: &mut Buffer);
   }
   ```
   - List all agents with names
   - Show status indicator (○ idle, ● gray busy, ● orange needs input)
   - Highlight current agent
   - Show keyboard shortcuts

5. Implement agent persistence
   - Save agent list to Supabase
   - Load agents on startup
   - Persist agent preferences per session
   - Store worktree associations

6. Add agent lifecycle events
   - Emit on_agent_created event
   - Emit on_agent_switched event
   - Emit on_agent_closed event
   - Update UI for each event

7. Handle concurrent event streams
   - Merge events from multiple agents
   - Route to appropriate UI element
   - Handle timing issues with concurrent updates
   - Implement proper cleanup on disconnect

### Deliverables

- AgentManager coordinating multiple agents
- Concurrent agent execution with tokio
- Agent switching with Ctrl+1-9
- AgentSidebar display showing all agents
- Multi-agent event handling

---

## Phase 9: Advanced Widgets and Features

**Objective**: Build sophisticated UI components for advanced features

### Tasks

1. Create SelectionPrompt widget
   ```rust
   // In claudechic-tui/src/widgets/prompts.rs

   pub struct SelectionPrompt {
       title: String,
       options: Vec<String>,
       selected: usize,
       tool_name: String,
   }
   ```
   - Display permission request
   - Show options: Allow, Deny, Allow Session, Allow All
   - Highlight selected option
   - Return choice on Enter

2. Create QuestionPrompt widget
   ```rust
   pub struct QuestionPrompt {
       questions: Vec<Question>,
       current_q: usize,
       answers: HashMap<String, String>,
   }

   pub struct Question {
       id: String,
       text: String,
       input_type: QuestionType,
   }

   pub enum QuestionType {
       Text,
       YesNo,
       Selection(Vec<String>),
   }
   ```
   - Render AskUserQuestion prompts
   - Support multiple question types
   - Navigate between questions
   - Collect and return answers

3. Create DiffWidget
   ```rust
   // In claudechic-tui/src/widgets/diff.rs

   pub struct DiffWidget<'a> {
       diff: &'a str,
       scroll_pos: u16,
   }

   impl Widget for DiffWidget<'_> {
       fn render(self, area: Rect, buf: &mut Buffer);
   }
   ```
   - Render git diffs with syntax highlighting
   - Color added lines green, removed lines red
   - Show file headers and hunks
   - Support scrolling

4. Create TodoPanel widget
   ```rust
   // In claudechic-tui/src/widgets/todo.rs

   pub struct TodoPanel<'a> {
       todos: &'a [TodoItem],
   }

   pub struct TodoItem {
       content: String,
       status: TodoStatus, // Pending, InProgress, Completed
       active_form: Option<String>,
   }
   ```
   - List todos from TodoWrite tool
   - Show status indicators
   - Color code by status
   - Update as todos change

5. Create ProcessPanel widget
   ```rust
   // In claudechic-tui/src/widgets/processes.rs

   pub struct ProcessPanel<'a> {
       processes: &'a [BackgroundProcess],
   }

   pub struct BackgroundProcess {
       pid: u32,
       command: String,
       status: ProcessStatus,
   }
   ```
   - List running background tasks
   - Show command and PID
   - Display CPU and memory usage
   - Allow clicking for details

6. Create ContextReport widget
   - Render 2D grid of context usage
   - Show usage by message type
   - Color code by utilization
   - Support zooming

7. Create UsageReport widget
   - Display API rate limit information
   - Show requests used vs quota
   - Visualize with progress bars
   - Color code by threshold (dim → yellow → red)

### Deliverables

- SelectionPrompt for permission handling
- QuestionPrompt for AskUserQuestion tool
- DiffWidget for viewing changes
- TodoPanel for task tracking
- ProcessPanel for background tasks
- ContextReport and UsageReport visualizations

---

## Phase 10: File System and Git Integration

**Objective**: Integrate git operations and file system access

### Tasks

1. Integrate git2-rs
   ```rust
   // In claudechic-core/src/git/mod.rs

   pub struct GitRepo {
       repo: git2::Repository,
   }

   impl GitRepo {
       pub fn open(path: &Path) -> Result<Self>;
       pub fn get_status(&self) -> Result<Vec<(String, FileStatus)>>;
       pub fn get_diff(&self, path: Option<&str>) -> Result<String>;
       pub async fn list_worktrees(&self) -> Result<Vec<Worktree>>;
       pub async fn create_worktree(&self, name: &str, branch: &str) -> Result<String>;
   }
   ```

2. Implement file indexing
   ```rust
   // In claudechic-core/src/file_index.rs

   pub struct FileIndex {
       files: Vec<String>,
       root: PathBuf,
   }

   impl FileIndex {
       pub async fn refresh(&mut self) -> Result<()>;
       pub fn search(&self, query: &str) -> Vec<&str>;
   }
   ```
   - Use git ls-files for project files
   - Cache results with TTL
   - Support fuzzy matching
   - Background refresh on file changes

3. Create file system watcher
   ```rust
   // In claudechic-core/src/fs_watcher.rs

   pub struct FileWatcher {
       tx: mpsc::Sender<FileChange>,
   }

   pub enum FileChange {
       Added(PathBuf),
       Modified(PathBuf),
       Deleted(PathBuf),
   }
   ```
   - Watch for git status changes
   - Watch for session file updates
   - Debounce rapid changes
   - Emit events to UI

4. Implement worktree support
   ```rust
   // In claudechic-core/src/worktree.rs

   pub struct Worktree {
       pub name: String,
       pub path: PathBuf,
       pub branch: String,
       pub is_locked: bool,
   }

   pub async fn list_worktrees(repo: &GitRepo) -> Result<Vec<Worktree>>;
   pub async fn create_worktree(repo: &GitRepo, name: &str, branch: &str) -> Result<Worktree>;
   pub async fn remove_worktree(repo: &GitRepo, name: &str) -> Result<()>;
   ```

5. Create diff generation
   - Generate unified diffs
   - Parse diff output
   - Cache diff results
   - Support partial diffs

6. Implement session compaction
   ```rust
   // In claudechic-core/src/compact.rs

   pub async fn compact_session(
       db: &Database,
       session_id: &str,
       dry_run: bool,
   ) -> Result<CompactionStats>;
   ```
   - Summarize old tool uses
   - Remove redundant messages
   - Preserve critical context
   - Report savings

7. Add file attachment support
   - Support image path validation
   - Encode images as base64
   - Store attachment metadata
   - Display images in chat

### Deliverables

- Git2 integration for repository operations
- File indexing with fuzzy search
- File system watcher
- Worktree management
- Diff generation and display
- Session compaction
- Image attachment support

---

## Phase 11: Command System and Keybindings

**Objective**: Implement command routing and keyboard shortcuts

### Tasks

1. Create command parser
   ```rust
   // In claudechic-tui/src/commands/mod.rs

   pub enum Command {
       Agent(AgentCommand),
       Resume(String),
       Clear,
       Shell(String),
       Exit,
       Usage,
       Compactish(CompactFlags),
       Worktree(WorktreeCommand),
   }

   pub fn parse_command(input: &str) -> Result<Command>;
   ```

2. Implement agent commands
   ```rust
   pub enum AgentCommand {
       List,
       Create { name: String, path: Option<PathBuf> },
       Close(String),
   }
   ```
   - `/agent` - List all agents
   - `/agent <name>` - Create new agent
   - `/agent <name> <path>` - Create in directory
   - `/agent close <name>` - Close agent

3. Implement session commands
   ```rust
   pub struct ResumeCommand {
       session_id: Option<String>,
   }
   ```
   - `/resume` - Show session picker
   - `/resume <id>` - Resume specific session

4. Implement utility commands
   - `/clear` - Clear chat UI only
   - `/shell <cmd>` - Run shell command (suspend TUI)
   - `/exit` - Quit application
   - `/usage` - Show API usage

5. Implement worktree commands
   ```rust
   pub enum WorktreeCommand {
       List,
       Create { name: String, branch: String },
       Switch { name: String },
       Finish,
   }
   ```
   - `/worktree list` - Show all worktrees
   - `/worktree create <name> <branch>` - Create new
   - `/worktree switch <name>` - Switch worktree
   - `/worktree finish` - Clean up on completion

6. Implement compaction command
   - `/compactish` - Compact session
   - `/compactish -n` - Dry run
   - Show compression stats

7. Set up keybindings
   ```rust
   // In claudechic-tui/src/keybindings.rs

   pub struct KeyBindings {
       send_message: Key,        // Enter
       quit: Key,                // Ctrl+C
       clear_chat: Key,          // Ctrl+L
       search_history: Key,      // Ctrl+R
       cycle_permission_mode: Key, // Shift+Tab
       // ... more bindings
   }
   ```
   - Enter: Send message
   - Ctrl+C: Quit (×2 for confirmation)
   - Ctrl+L: Clear chat
   - Ctrl+R: Reverse history search
   - Shift+Tab: Cycle permission modes
   - Ctrl+1-9: Switch agent
   - Page Up/Down: Scroll chat
   - Arrow keys: Navigate in prompts

8. Create help system
   - Show available commands with `/?`
   - Display keybindings with help command
   - Context-sensitive help in prompts
   - Accessible from input via special key

### Deliverables

- Command parser and router
- All agent, session, utility commands implemented
- Worktree command support
- Complete keybinding system
- Help system with available commands

---

## Phase 12: Permission System

**Objective**: Implement tool approval and permission modes

### Tasks

1. Build permission request queue
   ```rust
   // In claudechic-core/src/permissions/mod.rs

   pub struct PermissionRequest {
       pub tool_name: String,
       pub tool_input: serde_json::Value,
       pub tx: oneshot::Sender<PermissionResult>,
   }

   pub enum PermissionResult {
       Allow,
       Deny,
       AllowSession,
       AllowAll,
   }
   ```

2. Implement permission callback
   ```rust
   pub async fn handle_permission_request(
       agent: &mut Agent,
       tool_name: &str,
       tool_input: &serde_json::Value,
   ) -> Result<PermissionResult>;
   ```
   - Auto-allow EnterPlanMode
   - Auto-allow MCP tools
   - Block plan mode tools
   - Queue user request otherwise

3. Create plan mode tool blocking
   - Maintain PLAN_MODE_BLOCKED_TOOLS set
   - Allow Edit/Write to ~/.claude/plans/ directory
   - Deny other write operations
   - Return helpful error message

4. Implement acceptEdits mode
   - Auto-approve Edit and Write tools
   - Log approved operations
   - Maintain audit trail
   - Support toggle via Shift+Tab

5. Create session tool allowlist
   ```rust
   pub struct PermissionMode {
       mode: PermissionModeType,
       session_allowed_tools: HashSet<String>,
   }

   pub enum PermissionModeType {
       Default,
       AcceptEdits,
       Plan,
   }
   ```
   - Track allowed tools per session
   - Add tool on AllowSession response
   - Persist to Supabase

6. Implement AskUserQuestion handling
   - Detect AskUserQuestion tool
   - Show QuestionPrompt widget
   - Collect answers from user
   - Return answers in updated_input

7. Add alternative message flow
   - Support PermissionResultDeny with alternative message
   - Don't interrupt on denied with alternative
   - Display alternative to model
   - Let model continue instead of failing

### Deliverables

- Permission request queuing system
- Permission callback with proper routing
- Plan mode tool blocking
- AcceptEdits mode with auto-approval
- Session tool allowlists
- AskUserQuestion prompt handling

---

## Phase 13: Session Management and Persistence

**Objective**: Implement session lifecycle and cross-device resume

### Tasks

1. Build session save workflow
   ```rust
   // In claudechic-core/src/session/save.rs

   pub async fn save_agent_response(
       db: &Database,
       agent: &Agent,
   ) -> Result<()>;

   pub async fn save_user_message(
       db: &Database,
       session_id: &str,
       message: &UserContent,
   ) -> Result<()>;
   ```
   - Save after each user message
   - Save after complete response
   - Update modified timestamp
   - Handle concurrent saves

2. Build session resume
   ```rust
   // In claudechic-core/src/session/resume.rs

   pub async fn load_session(
       db: &Database,
       session_id: &str,
   ) -> Result<Agent>;
   ```
   - Load full chat history
   - Restore agent state
   - Handle missing sessions gracefully
   - Load from Supabase or local fallback

3. Create session picker UI
   ```rust
   // In claudechic-tui/src/widgets/session_picker.rs

   pub struct SessionPickerWidget {
       sessions: Vec<SessionInfo>,
       selected: usize,
   }
   ```
   - List recent sessions
   - Filter by directory
   - Sort by date
   - Preview session info

4. Implement session compaction
   ```rust
   pub async fn compact_session(
       db: &Database,
       session_id: &str,
       dry_run: bool,
   ) -> Result<CompactionStats> {
       let messages = db.load_session_messages(session_id).await?;
       let compacted = compress_old_messages(&messages);

       if !dry_run {
           db.save_compacted_session(session_id, &compacted).await?;
       }

       Ok(stats)
   }
   ```
   - Summarize old tool uses
   - Merge redundant messages
   - Preserve recent context
   - Show compression ratio

5. Implement checkpoint system
   - Store message UUIDs in Supabase
   - Link to specific messages for rewind
   - Enable message restoration
   - Support `/rewind` command

6. Add analytics integration
   ```rust
   // In claudechic-core/src/analytics.rs

   pub async fn send_analytics_event(
       event_name: &str,
       properties: &serde_json::Value,
   ) -> Result<()>;
   ```
   - PostHog integration (fire-and-forget)
   - Track: session_started, message_sent, tool_used
   - Include: model, tool_name, success/failure
   - Use background task to avoid blocking UI

7. Create session cleanup
   - Delete old sessions after retention period
   - Archive to cold storage if needed
   - Handle cleanup errors gracefully
   - Log cleanup operations

### Deliverables

- Session save after each response
- Session resume with full history
- Session picker UI
- Session compaction with dry-run
- Checkpoint system for rewind
- Analytics event tracking
- Session cleanup and retention

---

## Phase 14: Testing Infrastructure

**Objective**: Build comprehensive test suite

### Tasks

1. Set up testing framework
   - Configure `tokio-test` for async tests
   - Create mock SDK client
   - Create test database fixtures
   - Set up test logger with tracing

2. Create mock SDK client
   ```rust
   // In claudechic-core/tests/mocks/

   pub struct MockSdkClient {
       responses: Vec<SdkMessage>,
   }

   impl MockSdkClient {
       pub fn with_response(response: SdkMessage) -> Self;
       pub async fn query(&self, prompt: &str) -> Result<impl Stream<Item=SdkMessage>>;
   }
   ```

3. Write agent lifecycle tests
   - Test connect/disconnect
   - Test send message flow
   - Test interrupt handling
   - Test session resume

4. Write permission tests
   - Test auto-approval logic
   - Test permission queue
   - Test plan mode blocking
   - Test acceptEdits auto-approve

5. Write widget tests
   - Test ChatMessage rendering
   - Test ChatInput handling
   - Test ToolUseWidget expansion
   - Test StatusFooter updates

6. Write integration tests
   - Test full user interaction flow
   - Test multi-agent coordination
   - Test session persistence
   - Test command execution

7. Create property-based tests
   - Test message parsing with randomized input
   - Test session compaction preserves state
   - Test diff generation consistency
   - Use `proptest` for generation

8. Add benchmarks
   ```rust
   // In claudechic-core/benches/

   #[bench]
   fn bench_message_streaming(b: &mut Bencher) {
       // Measure text chunk processing
   }

   #[bench]
   fn bench_widget_render(b: &mut Bencher) {
       // Measure UI rendering
   }
   ```
   - Benchmark message streaming
   - Benchmark widget rendering
   - Benchmark session compaction
   - Track performance regressions

### Deliverables

- Unit tests for all modules
- Integration tests for workflows
- Mock SDK client for testing
- Property-based tests
- Performance benchmarks
- CI configuration for tests

---

## Phase 15: Documentation and Deployment

**Objective**: Complete documentation and prepare for release

### Tasks

1. Write API documentation
   - Comprehensive rustdoc for all public APIs
   - Include examples in doc comments
   - Document error cases
   - Link to related types

2. Create user guide
   - Installation instructions
   - Basic usage tutorial
   - Command reference
   - Keybinding reference
   - Troubleshooting section

3. Write architecture documentation
   - Component overview with diagrams
   - Data flow diagrams
   - Event flow documentation
   - Module organization explanation

4. Create migration guide
   - Python to Rust feature mapping
   - Behavior differences
   - Upgrade instructions
   - Known limitations

5. Set up CI/CD pipeline
   ```yaml
   # In .github/workflows/test.yml
   name: Test
   on: [push, pull_request]
   jobs:
     test:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v3
         - uses: dtolnay/rust-toolchain@stable
         - run: cargo test --all
         - run: cargo clippy -- -D warnings
         - run: cargo fmt -- --check
   ```
   - Build on push
   - Run tests
   - Run clippy linter
   - Check formatting

6. Create release process
   - Version bumping with cargo-release
   - Generate changelog
   - Create GitHub release
   - Publish to crates.io

7. Build binary distribution
   - Cross-compile for multiple platforms
   - Create installer scripts
   - Support GitHub releases
   - Document installation methods

8. Create example projects
   - Simple "hello world" agent
   - Multi-agent coordination example
   - Custom tool integration example
   - Session management example

### Deliverables

- Complete rustdoc documentation
- User guide with examples
- Architecture documentation
- Migration guide from Python
- Working CI/CD pipeline
- Release automation
- Multi-platform binary distribution

---

## Technical Decisions

### 1. SDK Integration: claude-agent-sdk-rs

**Decision**: Use `claude-agent-sdk-rs` for native Rust integration.

**Rationale**:
- Native async/await support with Rust's type system
- No Python runtime dependency
- Better error handling with Result types
- Compile-time guarantees on correctness

**Alternative Considered**: Subprocess wrapper for Python SDK
- Would require shipping Python runtime
- Extra IPC overhead
- Harder to integrate permission callbacks

### 2. Database: Supabase PostgreSQL

**Decision**: Use Supabase PostgreSQL with SQLx for persistence.

**Rationale**:
- Centralized session storage accessible across devices
- Automatic backups and maintenance
- Built-in authentication with Supabase Auth
- Enables future features (sharing, collaboration)
- Scales better than local files

**Alternative Considered**: Keep local JSONL files
- Simpler for single-user workflows
- But harder to share sessions
- Backup responsibility on user

### 3. Async Runtime: Tokio

**Decision**: Use Tokio for async task spawning.

**Rationale**:
- Industry standard for Rust async
- Excellent performance
- Rich ecosystem
- Integrates well with Ratatui

### 4. Terminal Framework: Ratatui

**Decision**: Use Ratatui for terminal UI rendering.

**Rationale**:
- High-level abstraction over raw terminal
- Widget system for component reuse
- Active maintenance and community
- Cross-platform support

**Alternative Considered**: Cursive, Druid, others
- Ratatui has best balance of features and simplicity

### 5. Architecture: Modular Crates

**Decision**: Separate core logic from UI in distinct crates.

**Rationale**:
- Core logic testable without UI
- UI can be replaced independently
- Clear separation of concerns
- Reusable core library

### 6. Permission Handling: Observer Pattern with Channels

**Decision**: Use tokio channels for event-driven permission handling.

**Rationale**:
- Decouples SDK from UI
- Supports async permission callbacks
- Scales to multiple agents
- Clean error handling

### 7. Session Persistence: Row-based with Supabase

**Decision**: Store sessions in Supabase with SQLx connection pooling.

**Rationale**:
- Automatic connection management
- Type-safe queries
- Transaction support if needed
- Query result mapping

### 8. Error Handling: Custom Error Type with Thiserror

**Decision**: Define domain-specific Error enum with thiserror.

**Rationale**:
- Ergonomic error conversion
- Rich context information
- Nested cause chains
- Integrates with anyhow for main error propagation

---

## Dependencies

### Core Dependencies

```toml
# Async runtime
tokio = { version = "1", features = ["full"] }
tokio-util = "0.7"

# Terminal UI
ratatui = "0.28"
crossterm = "0.28"

# Claude SDK
claude-agent-sdk-rs = "0.1"  # When available

# Database
sqlx = { version = "0.8", features = ["postgres", "uuid", "json"] }
tokio-postgres = "0.7"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# CLI
clap = { version = "4.0", features = ["derive"] }

# Utilities
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
once_cell = "1.19"
futures = "0.3"
async-trait = "0.1"
```

### Development Dependencies

```toml
[dev-dependencies]
tokio-test = "0.4"
proptest = "1.4"
criterion = "0.5"
mockito = "1.2"
```

### Optional Dependencies

```toml
[features]
default = ["analytics", "git"]
analytics = ["posthog"]  # When available
git = ["git2"]

[dependencies.git2]
version = "0.18"
optional = true

[dependencies.copypasta]
version = "0.10"
optional = true

[dependencies.syntect]
version = "5.1"
optional = true
```

---

## File Structure

```
claudechic-rust/
├── Cargo.toml                          # Workspace root
├── README.md                           # Project overview
├── ARCHITECTURE.md                     # Design documentation
├── CONTRIBUTING.md                     # Development guide
│
├── claudechic-core/                    # Core library crate
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── agent/
│   │   │   ├── mod.rs
│   │   │   ├── client.rs              # SDK wrapper
│   │   │   ├── state.rs               # Agent state management
│   │   │   ├── response.rs            # Response processing
│   │   │   ├── streaming.rs           # Text streaming
│   │   │   ├── tools.rs               # Tool tracking
│   │   │   └── message_handler.rs     # SDK message routing
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── message.rs
│   │   │   ├── agent.rs
│   │   │   ├── permission.rs
│   │   │   ├── tools.rs
│   │   │   └── events.rs
│   │   ├── session/
│   │   │   ├── mod.rs
│   │   │   ├── persistence.rs
│   │   │   ├── loader.rs
│   │   │   └── saver.rs
│   │   ├── db/
│   │   │   ├── mod.rs
│   │   │   ├── models.rs
│   │   │   └── migrations.rs
│   │   ├── permissions/
│   │   │   ├── mod.rs
│   │   │   ├── handler.rs
│   │   │   └── modes.rs
│   │   ├── file_index.rs
│   │   ├── git.rs
│   │   ├── compact.rs
│   │   ├── analytics.rs
│   │   ├── error.rs
│   │   └── config.rs
│   │
│   ├── tests/
│   │   ├── agent_lifecycle.rs
│   │   ├── permissions.rs
│   │   ├── session_persistence.rs
│   │   └── mocks/
│   │       └── sdk_client.rs
│   │
│   └── benches/
│       ├── streaming.rs
│       └── message_processing.rs
│
├── claudechic-tui/                     # TUI binary crate
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs                     # Entry point
│   │   ├── app.rs                      # Main application
│   │   ├── event_handler.rs            # Event routing
│   │   ├── ui/
│   │   │   ├── mod.rs
│   │   │   ├── layout.rs               # Layout management
│   │   │   ├── colors.rs               # Color scheme
│   │   │   ├── theme.rs                # Theme system
│   │   │   └── screen/
│   │   │       ├── chat.rs             # Main chat screen
│   │   │       ├── session.rs          # Session picker
│   │   │       └── diff.rs             # Diff viewer
│   │   ├── widgets/
│   │   │   ├── mod.rs
│   │   │   ├── chat_message.rs
│   │   │   ├── chat_input.rs
│   │   │   ├── spinner.rs
│   │   │   ├── tool_use.rs
│   │   │   ├── footer.rs
│   │   │   ├── sidebar.rs
│   │   │   ├── prompts.rs              # SelectionPrompt, QuestionPrompt
│   │   │   ├── diff.rs
│   │   │   ├── todo.rs
│   │   │   ├── processes.rs
│   │   │   ├── syntax.rs               # Syntax highlighting
│   │   │   └── base.rs                 # Base widget traits
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── parser.rs
│   │   │   ├── agent.rs
│   │   │   ├── session.rs
│   │   │   ├── worktree.rs
│   │   │   └── executor.rs
│   │   ├── keybindings.rs
│   │   └── terminal.rs                 # Terminal init/cleanup
│   │
│   ├── tests/
│   │   ├── widget_rendering.rs
│   │   ├── command_parsing.rs
│   │   └── integration.rs
│   │
│   └── benches/
│       └── rendering.rs
│
├── claudechic/                         # CLI wrapper (optional)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
│
├── migrations/
│   └── 001_create_sessions.sql
│
├── .github/
│   └── workflows/
│       ├── test.yml
│       ├── release.yml
│       └── docs.yml
│
└── docs/
    ├── user-guide.md
    ├── architecture.md
    ├── migration-guide.md
    └── troubleshooting.md
```

---

## Implementation Order

### Phase 1-3 (Foundation)
- **Start here**: Project setup, data models, SDK integration
- **Can be done in parallel**: Database schema

### Phase 4-5 (Backend Logic)
- **Depends on**: Phases 1-3
- **Can test without UI**: Yes

### Phase 6-7 (Basic UI)
- **Depends on**: Phase 5
- **Can iterate on independently**: Yes

### Phase 8-9 (Advanced Features)
- **Depends on**: Phases 6-7
- **Can develop in parallel**: Some widgets

### Phase 10-13 (Integration)
- **Depends on**: All previous phases
- **Ties everything together**: Sessions, permissions, commands

### Phase 14-15 (Quality)
- **Can start earlier**: Testing infrastructure
- **Should be last**: Documentation

---

## Success Criteria

- [ ] All Python features implemented in Rust
- [ ] Feature parity confirmed with manual testing
- [ ] Performance benchmarks show 2x+ improvement over Python
- [ ] Memory usage < 50MB for typical session
- [ ] Startup time < 2 seconds
- [ ] Test coverage > 80%
- [ ] Zero unsafe code outside necessary FFI
- [ ] Documentation complete and clear
- [ ] Binary distribution working cross-platform
- [ ] User migration path documented

---

## Next Steps

1. Validate claude-agent-sdk-rs availability and API
2. Set up Supabase database and schema
3. Begin Phase 1: Project setup
4. Create initial GitHub branch `rust-rewrite`
5. Establish development process and review guidelines
