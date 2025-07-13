use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) {
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

// note
// to run server type the below on a new home  terminal
// nc 127.0.0.1 7878
