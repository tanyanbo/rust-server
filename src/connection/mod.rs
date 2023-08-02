use std::{fs, io::prelude::*, net::TcpStream, thread, time::Duration};

use crate::http::{
    req::{HttpRequest, HttpRequestError},
    res::{HttpResponse, HttpResponseStatus},
};

pub fn handle_connection(mut stream: TcpStream) -> Result<(), HttpRequestError> {
    let http_request = HttpRequest::new(&mut stream)?;

    let response = match http_request.path.as_str() {
        "/" => HttpResponse::new(
            HttpResponseStatus::Ok,
            fs::read_to_string("src/templates/index.html")
                .expect("response html template should be present"),
        ),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            HttpResponse::new(
                HttpResponseStatus::Ok,
                fs::read_to_string("src/templates/index.html")
                    .expect("response html template should be present"),
            )
        }
        _ => HttpResponse::new(
            HttpResponseStatus::NotFound,
            fs::read_to_string("src/templates/404.html")
                .expect("not found html template should be present"),
        ),
    };

    let response = response.into_bytes();

    stream.write_all(&response).unwrap();

    Ok(())
}
