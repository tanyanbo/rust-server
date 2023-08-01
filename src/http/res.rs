use super::req::HttpVersion;

// TODO: add remaining statuses
#[allow(dead_code)]
pub enum HttpResponseStatus {
    SwitchingProtocols,
    Ok,
    Created,
    NoContent,
    MultipleChoices,
    MovedPermanently,
    NotModified,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    TooManyRequests,
    InternalServerError,
    NotImplemented,
    BadGateway,
}

impl HttpResponseStatus {
    pub fn code(&self) -> &str {
        match self {
            HttpResponseStatus::SwitchingProtocols => "101",
            HttpResponseStatus::Ok => "200",
            HttpResponseStatus::Created => "201",
            HttpResponseStatus::NoContent => "204",
            HttpResponseStatus::MultipleChoices => "300",
            HttpResponseStatus::MovedPermanently => "301",
            HttpResponseStatus::NotModified => "304",
            HttpResponseStatus::BadRequest => "400",
            HttpResponseStatus::Unauthorized => "401",
            HttpResponseStatus::Forbidden => "403",
            HttpResponseStatus::NotFound => "404",
            HttpResponseStatus::MethodNotAllowed => "405",
            HttpResponseStatus::TooManyRequests => "429",
            HttpResponseStatus::InternalServerError => "500",
            HttpResponseStatus::NotImplemented => "501",
            HttpResponseStatus::BadGateway => "502",
        }
    }

    pub fn reason_phrase(&self) -> &str {
        match self {
            HttpResponseStatus::SwitchingProtocols => "Switching Protocols",
            HttpResponseStatus::Ok => "OK",
            HttpResponseStatus::Created => "Created",
            HttpResponseStatus::NoContent => "No Content",
            HttpResponseStatus::MultipleChoices => "Multiple Choices",
            HttpResponseStatus::MovedPermanently => "Moved Permanently",
            HttpResponseStatus::NotModified => "Not Modified",
            HttpResponseStatus::BadRequest => "Bad Request",
            HttpResponseStatus::Unauthorized => "Unauthorized",
            HttpResponseStatus::Forbidden => "Forbidden",
            HttpResponseStatus::NotFound => "Not Found",
            HttpResponseStatus::MethodNotAllowed => "Method Not Allowed",
            HttpResponseStatus::TooManyRequests => "Too Many Requests",
            HttpResponseStatus::InternalServerError => "Internal Server Error",
            HttpResponseStatus::NotImplemented => "Not Implemented",
            HttpResponseStatus::BadGateway => "Bad Gateway",
        }
    }
}

pub struct HttpResponse {
    pub status: HttpResponseStatus,
    pub content: String,
}

impl HttpResponse {
    pub fn new(status: HttpResponseStatus, content: String) -> Self {
        Self { status, content }
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        let version: &str = HttpVersion::Http11.into();
        let status_line = format!(
            "{} {} {}",
            version,
            self.status.code(),
            self.status.reason_phrase()
        );

        let plain_text_response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            self.content.len(),
            self.content
        );
        plain_text_response.into_bytes()
    }
}
