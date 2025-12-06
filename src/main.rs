mod args;
use args::{EntityType, Mycli, MycliSubcommand};
use clap::Parser;
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixListener;

use std::fs;

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
                println!("Connected to server!");

                // Ensure in-memory storage is initialized
                {
                    let mut mycli_items = MYCLITEMS.lock().unwrap();
                    if mycli_items.is_none() {
                        *mycli_items = Some(HashMap::new());
                    }
                }

                // Extract user input from the create_client struct (arraydata)
                let arraydata = create_client.arraydata.clone();
                println!("create_client.arraydata = {:?}", arraydata);

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
                println!("Message sent!");

                // ---- Optional: read response from server ----
                loop {
                    let mut buf = vec![0u8; 1024];
                    let n = stream.read(&mut buf).await?;
                    if n > 0 {
                        println!("Received response: {}", String::from_utf8_lossy(&buf[..n]));
                    }
                }
            }
            MycliSubcommand::Server => {
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
                            //     let mut mycli_items = MYCLITEMS.lock().unwrap();
                            //     if mycli_items.is_none() {
                            //         *mycli_items = Some(HashMap::new());
                            //     }
                            // }
                        }
                        Err(e) => eprintln!("accept error: {e}"),
                    }
                }
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
