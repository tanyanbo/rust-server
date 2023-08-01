use connection::handle_connection;
use std::net::TcpListener;

mod connection;
mod http;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let _ = handle_connection(stream);
    }

    Ok(())
}
