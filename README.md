# Ripledd Rust API Wrapper

![Banner](docs/images/banner.png)

An API Wrapper for Ripledd API's services💫

Note: This is the Rust version of the **original** Ripledd API wrapper from [Foxy](https://github.com/foxy4096)

## Installation

To use the Ripledd Rust library in your project, add it as a dependency in your `Cargo.toml` file:

```Ini, TOML
[dependencies]
ripledd_api_wrapper = "0.1.0"
```

Then, run `cargo build` to download and compile the library.

## Usage Example

Here's an example of how to use the Ripledd Rust library to create a post:

```Rust
use ripledd_api_wrapper::RipleddClient;

fn main() -> Result<(), reqwest::Error> {
    // Replace with your Ripledd email and password (consider using environment variables for security)
    let email = "your_email@example.com";
    let password = "your_password";

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
```

## Explanation

1. We import the RipleddClient struct from your ripledd_api_wrapper library.
In the main function, we define placeholder values for email and password (consider using environment variables for security).
2. We create a new RipleddClient instance.
3. We define the post content (post_body).
4. We call create_post on the client with the post body and await the asynchronous response.
5. We check the response status code. If successful, we print a success message. Otherwise, we print an error message with the status code.

### Running the Example

1. Save the code snippet above as a Rust file (e.g., demo.rs).
2. Make sure you have Rust and Cargo installed.
3. In your terminal, navigate to the directory where you saved `demo.rs`.
4. Run the following command to compile and run the example:

```bash
cargo run
```

This will build the library and execute the example code, printing the response from the Ripledd API.
