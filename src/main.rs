mod app;
mod server;

fn main() -> std::io::Result<()> {
    app::server_run()
}
