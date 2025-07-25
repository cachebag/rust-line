// src/http/response.rs 

pub struct Response {
    status_code: u16,
    reason: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Response {
    pub fn ok(body: String) -> Self {
        Self {
            status_code: 200,
            reason: "OK".to_string(),
            headers: vec![
                ("Content-Type".to_string(),"text/plain".to_string()),
                ("Content-Length".to_string(), body.len().to_string()),
            ],
            body,
        }
    }

    pub fn to_string(&self) -> String {}
}
