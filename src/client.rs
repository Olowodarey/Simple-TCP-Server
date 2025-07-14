use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

pub fn client_run() -> std::io::Result<()> {
    println!("📤 TCP Client Connecting to 127.0.0.1:7878...");

    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    let mut reader = BufReader::new(stream.try_clone()?);

    // Thread to listen for messages from server
    thread::spawn(move || {
        // Spawns a new thread. This allows the client to listen for server messages in the background, while the main thread handles user input and the move keyword captures reader into the thread.

        for line in reader.lines() {
            match line {
                Ok(msg) => println!("🧾 Server: {}", msg),
                Err(_) => {
                    println!("❌ Server disconnected.");
                    break;
                }
            }
        }
    });

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let msg = line?;
        if msg == "exit" {
            println!("👋 Disconnecting.");
            break;
        }
        stream.write_all(msg.as_bytes())?;
        stream.write_all(b"\n")?;
    }

    Ok(())
}
