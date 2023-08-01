use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
pub struct HttpRequestError;

#[derive(Debug)]
pub enum HttpVersion {
    Http1,
    Http11,
}

impl HttpVersion {
    fn new(version: &str) -> Result<Self, HttpRequestError> {
        match version {
            "HTTP/1.0" => Ok(HttpVersion::Http1),
            "HTTP/1.1" => Ok(HttpVersion::Http11),
            _ => Err(HttpRequestError),
        }
    }
}

impl Into<&str> for HttpVersion {
    fn into(self) -> &'static str {
        match self {
            HttpVersion::Http1 => "HTTP/1.1",
            HttpVersion::Http11 => "HTTP/1",
        }
    }
}

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Options,
    Head,
    Trace,
    Connect,
}

impl HttpMethod {
    fn new(method: &str) -> Result<Self, HttpRequestError> {
        match method {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            "OPTIONS" => Ok(HttpMethod::Options),
            "HEAD" => Ok(HttpMethod::Head),
            "TRACE" => Ok(HttpMethod::Trace),
            "CONNECT" => Ok(HttpMethod::Connect),
            _ => Err(HttpRequestError),
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: HttpVersion,
}

impl HttpRequest {
    pub fn new(stream: &mut TcpStream) -> Result<Self, HttpRequestError> {
        let buf_reader = BufReader::new(stream);
        let mut http_request = buf_reader.lines();

        if let Some(request_line) = http_request.next() {
            let request_line = request_line.map_err(|_| HttpRequestError)?;
            let mut request_line_iter = request_line.split(" ");

            let method = request_line_iter.next().ok_or(HttpRequestError)?;

            let path = request_line_iter.next().ok_or(HttpRequestError)?;

            let version = request_line_iter.next().ok_or(HttpRequestError)?;

            return Ok(HttpRequest {
                method: HttpMethod::new(method)?,
                path: path.into(),
                version: HttpVersion::new(version)?,
            });
        }

        Err(HttpRequestError)
    }
}
