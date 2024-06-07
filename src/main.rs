use std::collections::HashMap;
use reqwest::Client;

const API_URL: &str = "https://ripledd.com/api/";

static ENDPOINTS: &'static [&str] = &[
    "post.php",
];

pub struct RipleddClient {
    email: String,
    password: String,
    channel_id: Option<String>,
    client: Client,
}

impl RipleddClient {
    pub fn new(email: String, password: String, channel_id: Option<String>) -> Self {
        RipleddClient {
            email,
            password,
            channel_id,
            client: Client::new(),
        }
    }

    pub async fn create_post(&self, body: &str) -> Result<reqwest::Response, reqwest::Error> {
        let mut data = HashMap::new();
        data.insert("email", &self.email);
        data.insert("password", &self.password);
        data.insert("content", body);

        if let Some(channel_id) = &self.channel_id {
            data.insert("channel_id", channel_id);
        }

        let url = format!("{}{}", API_URL, ENDPOINTS[0]);
        self.client.post(&url).form(&data).await
    }
}
