mod args;
use args::{EntityType, Mycli, ProjectSubcommand, UserSubcommand};
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

// Simple in-memory storage for demonstration
// In a real application, you would use a database
static USERS: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create file if not exists

    // Initialize the users storage

    let args = Mycli::parse();

    match args.entity_type {
        EntityType::User(user_cmd) => match user_cmd.command {
            UserSubcommand::Create(create_user) => {
                handle_user_create(&create_user.username, &create_user.email);
            }
            UserSubcommand::Delete(delete_user) => {
                handle_user_delete(&delete_user.username);
            }
            UserSubcommand::List => {
                use tokio::io::AsyncReadExt;
                use tokio::io::AsyncWriteExt;
                use tokio::net::UnixStream;
                let socket_path = "/tmp/mycli_data.sock";

                // Connect to the server listener
                let mut stream = match UnixStream::connect(socket_path).await {
                    Ok(stream) => {
                        println!("Connected to server!");
                        stream
                    }
                    Err(e) => {
                        eprintln!("Error: Failed to connect to the server.");
                        eprintln!("Please start the server first by running:");
                        eprintln!("  cargo run -- user run");
                        eprintln!("\nOr in production:");
                        eprintln!("  mycli user run");
                        eprintln!("\nDetails: {}", e);
                        return Err(e.into());
                    }
                };

                // ---- Send a message ----
                stream.write_all(b"hello from tokio client\n").await?;
                stream.flush().await?;
                println!("Message sent!");

                // ---- Read response from server ----
                let mut buf = vec![0u8; 1024];
                let n = stream.read(&mut buf).await?;
                if n > 0 {
                    println!("Received response: {}", String::from_utf8_lossy(&buf[..n]));
                }

                // Initialize and run the TUI
                color_eyre::install()?;
                let terminal = ratatui::init();
                let result = run(terminal);
                ratatui::restore();
                result?;
            }
            UserSubcommand::Run => {
                let socket_path = "/tmp/mycli_data.sock";

                // Remove old socket if it exists
                let _ = fs::remove_file(socket_path);

                let listener = UnixListener::bind(socket_path).unwrap();
                println!("Listening on {socket_path}");

                loop {
                    match listener.accept().await {
                        Ok((stream, _addr)) => {
                            println!("new client!");
                            tokio::spawn(handle_client(stream));

                            // {
                            //     let mut users = USERS.lock().unwrap();
                            //     if users.is_none() {
                            //         *users = Some(HashMap::new());
                            //     }
                            // }
                        }
                        Err(e) => eprintln!("accept error: {e}"),
                    }
                }
            }
        },
        EntityType::Project(project_cmd) => match project_cmd.command {
            ProjectSubcommand::Create(create_project) => {
                handle_project_create(&create_project.name);
            }
            ProjectSubcommand::Delete(delete_project) => {
                handle_project_delete(&delete_project.name);
            }
        },
    }

    Ok(())
}

fn handle_user_create(username: &str, email: &str) {
    println!("Creating user...");
    println!("Username: {}", username);
    println!("Email: {}", email);

    // Add user to storage
    let mut users = USERS.lock().unwrap();
    if let Some(ref mut users_map) = *users {
        users_map.insert(username.to_string(), email.to_string());
    }

    println!("User created successfully!");
}

fn handle_user_delete(username: &str) {
    println!("Deleting user...");
    println!("Username: {}", username);

    // Remove user from storage
    let mut users = USERS.lock().unwrap();
    if let Some(ref mut users_map) = *users {
        if users_map.remove(username).is_some() {
            println!("User deleted successfully!");
        } else {
            println!("User not found!");
        }
    }
}

fn handle_user_list() {
    println!("Listing all users...");
    println!();

    let users = USERS.lock().unwrap();
    if let Some(ref users_map) = *users {
        if users_map.is_empty() {
            println!("No users found.");
        } else {
            println!("Total users: {}", users_map.len());
            println!("{:<20} {:<30}", "Username", "Email");
            println!("{}", "-".repeat(50));

            for (username, email) in users_map.iter() {
                println!("{:<20} {:<30}", username, email);
            }
        }
    }
}

fn handle_project_create(name: &str) {
    println!("Creating project...");
    println!("Project name: {}", name);
    println!("Project created successfully!");
}
fn handle_project_delete(name: &str) {
    println!("Deleting project...");
    println!("Project name: {}", name);
    println!("Project deleted successfully!");
}

async fn handle_client(mut stream: tokio::net::UnixStream) -> tokio::io::Result<()> {
    let mut buf = vec![0u8; 1024];
    loop {
        match stream.read(&mut buf).await {
            Ok(0) => {
                // client disconnected
                println!("Client disconnected");
                return Ok(());
            }
            Ok(n) => {
                let data = &buf[..n];
                println!("Received from client: {}", String::from_utf8_lossy(data));
                // Echo a simple acknowledgment back to the client
                stream.write_all(b"ack\n").await?;
                // Send another message and flush
                stream.write_all(b"hello from tokio server\n").await?;
                stream.flush().await?;
                println!("Message sent!");
            }
            Err(e) => {
                eprintln!("Failed to read from client: {}", e);
                return Err(e);
            }
        }
    }
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(ui::render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}
