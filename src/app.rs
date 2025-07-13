use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use crate::server::handle_client;

pub fn server_run() -> std::io::Result<()> {
    println!("🌐 TCP Echo Server listening on 127.0.0.1:7878");

    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("📥 Connection from: {}", stream.peer_addr()?); // Print the client's IP address.
                thread::spawn(|| handle_client(stream)); // Spawn a new thread using thread::spawn() to handle this client independently — so the server can continue accepting others.
            }
            Err(e) => eprintln!("❌ Connection failed: {}", e),
        }
    }

    Ok(())
}
