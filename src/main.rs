mod args;
use args::{EntityType, Mycli, MycliSubcommand};
use clap::Parser;
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixListener;

use std::fs;

mod ui;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;

// ClientData struct for sharing data between async handlers and UI
#[derive(Debug, Clone)]
pub struct ClientData {
    entries: Vec<(String, u64)>,
    last_updated: std::time::SystemTime,
}

impl ClientData {
    fn new() -> Self {
        ClientData {
            entries: vec![],
            last_updated: std::time::SystemTime::now(),
        }
    }

    fn from_string(data: &str) -> Result<Self, String> {
        let tokens: Vec<&str> = data.trim().split_whitespace().collect();

        if tokens.is_empty() {
            return Err("Empty data".to_string());
        }

        if tokens.len() % 2 != 0 {
            return Err("Odd number of tokens, expected key-value pairs".to_string());
        }

        let mut entries = Vec::new();
        for chunk in tokens.chunks(2) {
            if chunk.len() == 2 {
                let key = chunk[0].to_string();
                let value: u64 = chunk[1].parse()
                    .map_err(|_| format!("Invalid number: {}", chunk[1]))?;
                entries.push((key, value));
            }
        }

        Ok(ClientData {
            entries,
            last_updated: std::time::SystemTime::now(),
        })
    }

    fn as_vec(&self) -> Vec<(&str, u64)> {
        self.entries.iter()
            .map(|(k, v)| (k.as_str(), *v))
            .collect()
    }

    fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

// Thread-safe shared state for client data
static CLIENT_DATA: Mutex<ClientData> = Mutex::new(ClientData {
    entries: vec![],
    last_updated: std::time::UNIX_EPOCH,
});

// Thread-safe shared state for server logs
pub static SERVER_LOGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
pub static LOG_VIEW_HEIGHT: Mutex<u16> = Mutex::new(0);
pub static SCROLL_STATE: Mutex<(u16, u16)> = Mutex::new((0, 0));

// Simple in-memory storage for demonstration
// In a real application, you would use a database
static MYCLITEMS: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create file if not exists

    // Initialize the mycli storage

    let args = Mycli::parse();

    match args.entity_type {
        EntityType::Mycli(mycli_cmd) => match mycli_cmd.command {
            MycliSubcommand::Create(create_mycli) => {
                handle_mycli_create(&create_mycli.username, &create_mycli.email);
            }
            MycliSubcommand::Client(create_client) => {
                use tokio::io::AsyncReadExt;
                use tokio::io::AsyncWriteExt;
                use tokio::net::UnixStream;
                let socket_path = "/tmp/mycli_data.sock";

                // Connect to the server listener
                let mut stream = UnixStream::connect(socket_path).await?;
                // println!("Connected to server!");

                // Extract user input from the create_client struct (arraydata)
                let arraydata = create_client.arraydata.clone();
                // println!("create_client.arraydata = {:?}", arraydata);

                // Build a single message from the array (join with spaces)
                // This can be sent to the server later
                let client_message = if arraydata.is_empty() {
                    "No data from client\n".to_string()
                } else {
                    format!("{}\n", arraydata.join(" "))
                };

                // ---- Send a message ----
                stream.write_all(client_message.as_bytes()).await?;
                stream.flush().await?;
                // println!("Message sent!");

                // ---- Optional: read response from server ----
                loop {
                    let mut buf = vec![0u8; 1024];
                    let n = stream.read(&mut buf).await?;
                    if n > 0 {
                        // println!("Received response: {}", String::from_utf8_lossy(&buf[..n]));
                    }
                }
            }
            MycliSubcommand::Server => {
                let socket_path = "/tmp/mycli_data.sock";

                // Remove old socket if it exists
                let _ = fs::remove_file(socket_path);

                let listener = UnixListener::bind(socket_path).unwrap();
                SERVER_LOGS
                    .lock()
                    .unwrap()
                    .push(format!("Listening on {}", socket_path));

                // Initialize UI ONCE at startup
                color_eyre::install()?;
                let terminal = ratatui::init();

                // Spawn listener as background task
                let listener_handle = tokio::spawn(async move {
                    loop {
                        match listener.accept().await {
                            Ok((stream, _addr)) => {
                                tokio::spawn(handle_client(stream));
                            }
                            Err(e) => eprintln!("accept error: {e}"),
                        }
                    }
                });

                // Run UI in main task
                let ui_result = run(terminal).await;
                ratatui::restore();

                // Clean up listener
                listener_handle.abort();
                ui_result?;
            }
        },
    }

    Ok(())
}

fn handle_mycli_create(username: &str, email: &str) {
    println!("Creating mycli...");
    println!("Username: {}", username);
    println!("Email: {}", email);

    // Add mycli to storage
    let mut mycli_items = MYCLITEMS.lock().unwrap();
    if let Some(ref mut mycli_map) = *mycli_items {
        mycli_map.insert(username.to_string(), email.to_string());
    }

    println!("Mycli created successfully!");
}

async fn handle_client(mut stream: tokio::net::UnixStream) -> tokio::io::Result<()> {
    let mut buf = vec![0u8; 1024];
    loop {
        match stream.read(&mut buf).await {
            Ok(0) => {
                SERVER_LOGS
                    .lock()
                    .unwrap()
                    .push(format!("Client Disconnected"));
                // client disconnected
                // println!("Client disconnected");
                return Ok(());
            }
            Ok(n) => {
                SERVER_LOGS
                    .lock()
                    .unwrap()
                    .push(format!("Client connected"));
                let data_str = String::from_utf8_lossy(&buf[..n]);
                SERVER_LOGS
                    .lock()
                    .unwrap()
                    .push(format!("Received from client: {}", data_str.trim()));

                // Parse and store client data
                match ClientData::from_string(&data_str) {
                    Ok(client_data) => {
                        // println!("Parsed data: {:?}", client_data);
                        *CLIENT_DATA.lock().unwrap() = client_data;
                        stream.write_all(b"ack\n").await?;
                    }
                    Err(e) => {
                        let error_msg = format!("Parse error: {}", e);
                        eprintln!("{}", error_msg);
                        stream.write_all(b"err: invalid format\n").await?;
                    }
                }
                stream.flush().await?;
            }
            Err(e) => {
                SERVER_LOGS
                    .lock()
                    .unwrap()
                    .push(format!("Failed to read from client"));
                eprintln!("Failed to read from client: {}", e);
                return Err(e);
            }
        }
    }
}

async fn run(mut terminal: DefaultTerminal) -> Result<()> {
    use crossterm::event::KeyCode;
    use std::time::Duration;

    loop {
        terminal.draw(ui::render)?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Up => {
                        let mut scroll = SCROLL_STATE.lock().unwrap();
                        scroll.0 = scroll.0.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        let mut scroll = SCROLL_STATE.lock().unwrap();
                        let num_log_lines = SERVER_LOGS.lock().unwrap().len() as u16;
                        let view_height = *LOG_VIEW_HEIGHT.lock().unwrap();
                        if scroll.0 < num_log_lines.saturating_sub(view_height.saturating_sub(2)) {
                            scroll.0 = scroll.0.saturating_add(1);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
