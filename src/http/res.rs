// TODO: add remaining statuses
pub enum ResponseStatus {
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

impl ResponseStatus {
    pub fn code(&self) -> &str {
        match self {
            ResponseStatus::SwitchingProtocols => "101",
            ResponseStatus::Ok => "200",
            ResponseStatus::Created => "201",
            ResponseStatus::NoContent => "204",
            ResponseStatus::MultipleChoices => "300",
            ResponseStatus::MovedPermanently => "301",
            ResponseStatus::NotModified => "304",
            ResponseStatus::BadRequest => "400",
            ResponseStatus::Unauthorized => "401",
            ResponseStatus::Forbidden => "403",
            ResponseStatus::NotFound => "404",
            ResponseStatus::MethodNotAllowed => "405",
            ResponseStatus::TooManyRequests => "429",
            ResponseStatus::InternalServerError => "500",
            ResponseStatus::NotImplemented => "501",
            ResponseStatus::BadGateway => "502",
        }
    }

    pub fn reason_phrase(&self) -> &str {
        match self {
            ResponseStatus::SwitchingProtocols => "Switching Protocols",
            ResponseStatus::Ok => "OK",
            ResponseStatus::Created => "Created",
            ResponseStatus::NoContent => "No Content",
            ResponseStatus::MultipleChoices => "Multiple Choices",
            ResponseStatus::MovedPermanently => "Moved Permanently",
            ResponseStatus::NotModified => "Not Modified",
            ResponseStatus::BadRequest => "Bad Request",
            ResponseStatus::Unauthorized => "Unauthorized",
            ResponseStatus::Forbidden => "Forbidden",
            ResponseStatus::NotFound => "Not Found",
            ResponseStatus::MethodNotAllowed => "Method Not Allowed",
            ResponseStatus::TooManyRequests => "Too Many Requests",
            ResponseStatus::InternalServerError => "Internal Server Error",
            ResponseStatus::NotImplemented => "Not Implemented",
            ResponseStatus::BadGateway => "Bad Gateway",
        }
    }
}
