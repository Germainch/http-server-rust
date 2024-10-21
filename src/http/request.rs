use std::str::FromStr;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: String,
    pub http_version: String,
    // pub body: String,
    // pub headers: Vec<String>,
    // pub cookies: Vec<String>,
    // pub params: Vec<String>,
}

impl Request{
    fn new(method: String, uri: String, http_version: String) -> Self {
        Self {
            method: Method::from_str(method.as_str()).unwrap(),
            uri,
            http_version,
        }
    }
}

impl FromStr for Request {
    type Err = ();

    /// Parse a request from a string
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s.split(" ").collect::<Vec<&str>>();
        if vec.len() != 3 {
            return Err(());
        }
        Ok(Self::new(vec[0].to_string(), vec[1].to_string(), vec[2].to_string()))
    }
}

#[derive(Debug)]
pub enum Method {
    POST,
    GET,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl FromStr for Method{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "POST" => Ok(Self::POST),
            "GET" => Ok(Self::GET),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "PATCH" => Ok(Self::PATCH),
            "HEAD" => Ok(Self::HEAD),
            "OPTIONS" => Ok(Self::OPTIONS),
            _ => Err(()),
        }
    }
}