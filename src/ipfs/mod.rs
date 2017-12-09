mod http;

use std::path::PathBuf;

#[derive(Debug)]
pub struct Node {
    host: String,
    port: u16,
    http_client: http::Client,
}

#[derive(Debug)]
pub struct FileInfo {
    name: String,
    hash: String,
    bytes: i64,
    size: String,
}

const API_URL: &str = "/api/v0";

impl Node {
    pub fn new() -> Node {
        Node::default()
    }

    pub fn host(&mut self, host: String) -> &mut Node {
        self.host = host;
        self
    }

    pub fn port(&mut self, port: u16) -> &mut Node {
        self.port = port;
        self
    }

    pub fn url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    // API functions
    pub fn add(self, path: PathBuf) -> FileInfo {
        http_client.get()
    }
}

impl Default for Node {
    fn default() -> Node {
        Node {
            host: "localhost".to_string(),
            port: 5001,
            http_client: http::Client::new(),
        }
    }
}
