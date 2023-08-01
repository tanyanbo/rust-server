use connection::handle_connection;
use std::net::TcpListener;

mod connection;
mod req;
mod res;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

    Ok(())
}
