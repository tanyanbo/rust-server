use connection::handle_connection;
use std::{net::TcpListener};
use thread_pool::ThreadPool;

mod connection;
mod http;
mod thread_pool;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.exec(Box::new(|| {
            let _ = handle_connection(stream);
        }));
    }

    Ok(())
}
