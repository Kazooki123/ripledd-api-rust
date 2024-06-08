use ripledd_api_wrapper::RipleddClient;

fn main() -> Result<(), reqwest::Error> {
    // Replace with your Ripledd email and password (consider using environment variables for security)
    let email = "example@example.com";
    let password = "12345";

    // Create a Ripledd client
    let mut ripledd = RipleddClient::new(email.to_string(), password.to_string(), None);

    // Create a post
    let post_body = "The post from the API";
    let response = ripledd.create_post(post_body).await?;

    // Check for successful response
    if response.status().is_success() {
        println!("Post created successfully!");
    } else {
        println!("Error creating post: {}", response.status());
    }

    Ok(())
}