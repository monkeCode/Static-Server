#[derive(Copy, Clone)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Connect,
    Options,
    Trace,
    Patch,
}

impl Method {
    fn from_str(method: &str) -> Self {
        match method {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "HEAD" => Method::Head,
            "CONNECT" => Method::Connect,
            "OPTIONS" => Method::Options,
            "TRACE" => Method::Trace,
            "PATCH" => Method::Patch,
            _ => panic!("http method not found")
        }
    }

    fn to_string(&self) -> String {
        let s = match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Head => "HEAD",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Patch => "PATCH",
        };
        String::from(s)
    }
}

pub struct Request {
    pub method: Method,
    pub path: String,
    pub host: String,
    pub version: String,
    pub connection: String,
    pub user_agent: String,
    pub accept: String,
    pub accept_encoding: String,
    pub accept_language: String,
    pub sec_fetch_site: String,
    pub sec_fetch_user: String,
    pub sec_fetch_dest: String,
    pub referer: String,
    pub request: String,
}

impl Request {
    pub fn new(request: &Vec<String>) -> Self {
        let data = request[0].split(' ').collect::<Vec<&str>>();
        //TODO: parse request
        return Request {
            method: Method::from_str(data[0]),
            path: String::from(data[1]),
            version: String::from(data[2]),
            host: String::from(""),
            connection: String::from(""),
            user_agent: String::from(""),
            accept: String::from(""),
            accept_encoding: String::from(""),
            accept_language: String::from(""),
            sec_fetch_site: String::from(""),
            sec_fetch_user: String::from(""),
            sec_fetch_dest: String::from(""),
            referer: String::from(""),
            request: request.join("\r\n"),
        };
    }
}
