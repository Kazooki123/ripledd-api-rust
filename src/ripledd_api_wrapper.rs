use std::collections::HashMap;
use reqwest::Client;

const API_URL: &str = "https://ripledd.com/api/";

static ENDPOINTS: &'static [&str] = &[
    "me.php",
    "post/create.php",
    "login.php",
    "logout.php",
    "user.php",
    "users.php",
    "post.php",
    "signup.php",
    "upload/avatar.php",
    "upload/banner.php"
];

pub struct RipleddClient {
    email: String,
    password: String,
    username: String,
    channel_id: Option<String>,
    client: Client,
    title: String,
    content: String,
    id: String
}

impl RipleddClient {
    pub fn new(email: String, password: String, username: String ,channel_id: Option<String>, title: String, content: String, id: String) -> Self {
        RipleddClient {
            email,
            password,
            username,
            channel_id,
            client: Client::new(),
            title,
            content,
            id
        }
    }

    pub async fn create_post(&self, body: &str) -> Result<reqwest::Response, reqwest::Error> {
        let mut data = HashMap::new();
        data.insert("email", &self.email);
        data.insert("password", &self.password);
        data.insert("content", &body.to_string());  // Convert &str to String


        if let Some(channel_id) = &self.channel_id {
            data.insert("channel_id", channel_id);
        }

        let url = format!("{}{}", API_URL, ENDPOINTS[0]);
        self.client.post(&url).form(&data)
    }
}
