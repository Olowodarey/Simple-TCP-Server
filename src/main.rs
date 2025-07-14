mod app;
mod client;
mod server;

use std::thread;

fn main() -> std::io::Result<()> {
    // Spawn the server in a separate thread
    thread::spawn(|| {
        app::server_run().expect("Server failed");
    });

    // Delay it  briefly to ensure server starts before client connects
    std::thread::sleep(std::time::Duration::from_millis(500));

    client::client_run()
}

// here we will run both server and client together in one terminal
