pub struct Request {
    method: RequestMethod,
    view: String,
    message: String,
}

pub enum RequestMethod {
    GET,
    POST,
    Unsupported,
}

impl Request {
    pub fn from(request_string: String) -> Request {
        // unfinished
        println!(request_string); // delete this line

        Request::default_unsupported()
    }

    pub fn default_unsupported() -> Request { Request {
        method: RequestMethod::Unsupported,
        view: String::new(),
        message: String::new(),
    } }
}

pub struct Response {
    status: ResponseStatus,
    message: String,
}

pub enum ResponseStatus {
    OK,
    NotFound,
    Forbidden,
    InternalServerError,
}

impl Response {
    pub fn to_string(&self) -> String {
        // unfinished

        String::new()
    }
}
