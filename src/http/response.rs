pub struct Response {
    pub(crate) status_code: u16,
    pub(crate) body: String,
}

impl Response {
    pub fn new(status_code: u16, body: String) -> Self {
        Self {
            status_code,
            body,
        }
    }

    pub fn from_file(status_code: u16, filepath: &str) -> Self {

        if let Ok(contents) = std::fs::read_to_string(filepath) {
            Self {
                status_code,
                body: contents,
            }
        } else {
            Self {
                status_code: 404,
                body: "404 Not Found".to_string(),
            }
        }
    }

    pub fn into_string(self) -> String {
        format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}", self.status_code, self.body.len(), self.body)
    }
}