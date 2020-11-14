const NEXT_LINE: &str = "\r\n";
const VOID_LINE: &str = "\r\n\r\n";
const FIRST_LINE_ENDING: &str = " HTTP/1.1";

pub struct Request {
    pub method: RequestMethod,
    pub view: String,
    pub message: String,
}

pub enum RequestMethod {
    GET,
    POST,
    Unsupported,
} impl RequestMethod {
    pub fn from(raw_str: &str) -> RequestMethod { match raw_str {
        "GET" => RequestMethod::GET,
        "POST" => RequestMethod::POST,
        _ => RequestMethod::Unsupported,
    } }
}

impl Request {
    pub fn from(request_str: &str) -> Request {
        // parse first line.
        let lines: Vec<&str> = request_str.split(NEXT_LINE).collect();
        let first_line = lines[0];

        // filter, if not in correct format.
        if !first_line.ends_with(FIRST_LINE_ENDING) {
            return Request::default_unsupported()
        }

        // parse parameters in the first line and filter incorrect.
        let parameters: Vec<&str> = first_line.split(" ").collect();
        if !parameters.len() == 3 {
            return Request::default_unsupported()
        }

        // when everything ok, then generate the object.
        Request {
            method: RequestMethod::from(parameters[0]),
            view: parameters[1].to_string(),
            message: Request::parse_message(request_str),
        }
    }

    pub fn default_unsupported() -> Request { Request {
        method: RequestMethod::Unsupported,
        view: String::new(),
        message: String::new(),
    } }

    fn parse_message(request_str: &str) -> String {
        match request_str.find(VOID_LINE) {
            None => return String::new(),
            Some(split_index) => {
                request_str.split_at(
                    split_index + VOID_LINE.len()
                ).1.to_string()
            }
        }
    }
}

pub struct Response {
    pub status: ResponseStatus,
    pub message: String,
}

pub enum ResponseStatus {
    OK,
    NotFound,
    Forbidden,
    InternalServerError,
} impl ResponseStatus {
    pub fn to_first_line(&self) -> &str { match &self {
        ResponseStatus::OK => "HTTP/1.1 200 OK\r\n\r\n",
        ResponseStatus::NotFound => "HTTP/1.1 404 Not Found \r\n\r\n",
        ResponseStatus::Forbidden => "HTTP/1.1 403 Forbidden \r\n\r\n",
        ResponseStatus::InternalServerError =>
            "HTTP/1.1 500 Internal Server Error\r\n\r\n",
    } }
}

impl Response {
    pub fn to_string(&self) -> String { format!(
        "{}{}", self.status.to_first_line(), self.message
    ) }
}
