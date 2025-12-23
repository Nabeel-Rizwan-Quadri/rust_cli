# MyCLI Project Overview

## Project Summary
MyCLI is a Rust-based command-line application that implements a client-server architecture using Unix domain sockets for inter-process communication, with a terminal-based user interface.

## Tech Stack
- **Language**: Rust (Edition 2024)
- **CLI Framework**: Clap v4 (with derive features)
- **Async Runtime**: Tokio v1 (full features)
- **Terminal UI**: Ratatui v0.28
- **Error Handling**: color-eyre v0.6.3
- **Terminal Control**: crossterm v0.28.1

## Architecture

### Components

#### 1. Command Structure (src/args.rs)
The CLI is organized with the following command hierarchy:
- `mycli mycli create <username> <email>` - Create a new user
- `mycli mycli server` - Start the server
- `mycli mycli client <arraydata...>` - Connect as a client and send data

#### 2. Server Mode (src/main.rs:78-102)
- Listens on Unix socket at `/tmp/mycli_data.sock`
- Accepts multiple client connections
- Spawns a new async task for each client connection
- Displays a terminal UI (bar chart) when clients connect
- Echoes acknowledgments back to clients

#### 3. Client Mode (src/main.rs:34-77)
- Connects to the server via Unix socket
- Sends array data (joined with spaces) to the server
- Receives and displays server responses in a loop

#### 4. User Management (src/main.rs:109-121)
- Stores user data in-memory using a HashMap
- Maps username -> email
- Thread-safe storage using Mutex

#### 5. Terminal UI (src/ui.rs)
- Renders a bordered block with green foreground
- Displays a bar chart with sample data (A, B, C, D values)
- Yellow-styled bars with black text on yellow background
- Title: "My Blocky Boy"

## Data Flow

```
[Client] ---> Unix Socket ---> [Server] ---> [Terminal UI]
   |                               |
   v                               v
Send data                    Handle client
                             Echo "ack"
                             Send "hello from tokio server"
```

## In-Memory Storage
- Global static `MYCLITEMS`: `Mutex<Option<HashMap<String, String>>>`
- Stores username-email mappings
- Initialized on first use
- Not persisted to disk (demonstration only)

## Key Features
1. Async TCP-like communication over Unix sockets
2. Multi-client support with concurrent handling
3. Real-time terminal UI updates
4. Simple user management system
5. Message echoing and acknowledgment

## File Structure
```
mycli/
├── Cargo.toml          # Project dependencies and metadata
├── src/
│   ├── main.rs         # Main application logic, server/client handlers
│   ├── args.rs         # CLI argument definitions using Clap
│   └── ui.rs           # Terminal UI rendering with Ratatui
└── PROJECT_OVERVIEW.md # This file
```

## Current State
According to git status:
- Working on main branch
- `src/main.rs` has uncommitted modifications
- Recent commits show refactoring of user and project management into unified mycli commands

## Potential Use Cases
- Learning Rust async programming with Tokio
- Understanding Unix socket communication
- Building terminal-based monitoring tools
- Prototyping client-server architectures
- Creating interactive CLI applications

## Notes
- The storage is in-memory only and will be lost on restart
- Unix socket path is hardcoded to `/tmp/mycli_data.sock`
- UI launches when server accepts a connection
- No authentication or encryption on the socket
