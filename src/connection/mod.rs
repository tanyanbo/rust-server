use std::{
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use crate::req::{HttpRequest, HttpRequestError};

pub fn handle_connection(mut stream: TcpStream) -> Result<(), HttpRequestError> {
    let http_request = HttpRequest::new(&mut stream)?;

    println!("{:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("src/templates/index.html")
        .expect("response html template should be present");
    let response = format!(
        "{status_line}\r\nContent-Length: {}\r\n\r\n{contents}",
        contents.len()
    );

    stream.write_all(response.as_bytes()).unwrap();

    Ok(())
}
