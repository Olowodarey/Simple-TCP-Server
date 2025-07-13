use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use std::thread;

fn main() -> std::io::Result<()> {
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

fn handle_client(mut stream: TcpStream) {
    let peer = stream.peer_addr().unwrap(); // geting the client ip address for logging 
    let reader = BufReader::new(stream.try_clone().unwrap()); // Wrap the TCP stream in a BufReader so we can read it line by line.  try_clone() creates a copy of the stream because we'll use one for reading and one for writing. 

    for line in reader.lines() {
        match line {
            // Loop over each line the client sends.   If a message (msg) is successfully read:
            Ok(msg) => {
                println!("📨 [{}] {}", peer, msg); // print it 
                let response = format!("Echo: {}\n", msg); // Send it back to the client prefixed with "Echo: "
                stream.write_all(response.as_bytes()).unwrap();
            }

            Err(e) => {
                // If an error occurs while reading, print the error and break the loop (i.e., close the connection).
                println!("❌ Error with {}: {}", peer, e);
                break;
            }
        }
    }

    println!("🔌 Connection with {} closed.", peer); // This message shows when a client disconnects or an error ends the session.
}
