use std::collections::HashMap;
use reqwest::Client;

const API_URL: &str = "https://ripledd.com/api/";

//static ENDPOINTS: &'static [&str] = &[
//    "me.php",
//    "post/create.php",
//    "login.php",
//    "logout.php",
//    "user.php",
//    "users.php",
//    "post.php",
//    "signup.php",
//    "upload/avatar.php",
//    "upload/banner.php"
//];

mod lib;

pub struct RipleddClient {
    email: String,
    password: String,
    // username: String,
    channel_id: Option<String>,
    client: Client,
    // title: String,
    // content: String,
    // id: String
}

impl RipleddClient {
    pub fn new(email: String, password: String, channel_id: Option<String>) -> Self {
        RipleddClient {
            email,
            password,
            channel_id,
            client: Client::new()
        }
    }

    pub async fn create_post(&self, body: &str) -> Result<reqwest::Response, reqwest::Error> {
        let mut data = HashMap::new();
        let content = body.to_string(); // Creates a named variable for the content (Optional maybe?)

        data.insert("email", &self.email);
        data.insert("password", &self.password);
        data.insert("content", &content); // Using the named variable

        if let Some(channel_id) = &self.channel_id {
            data.insert("channel_id", channel_id);
        }

        let url = format!("{}{}", API_URL, "post/create.php"); // create.php for creating post (POST method)

        // Sends a request and awaits a response
        let response = self.client.post(&url).form(&data).send().await?;

        Ok(response)
    }
}

fn main() {
    lib::greet();
}