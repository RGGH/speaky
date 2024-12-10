use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap};
use serde_json::json;
use std::env;
use std::fs::{File, read_to_string};
use std::io::copy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        env::var("OPENAI_API_KEY").expect("Environment variable OPENAI_API_KEY must be set");

    // Read text input from tts_input.txt
    let input_text = read_to_string("tts_input.txt")
        .expect("Failed to read tts_input.txt")
        .trim()
        .to_string();

    // Build the JSON payload
    let payload = json!({
        "model": "tts-1",
        "input": input_text,
        "voice": "fable"
    });

    // Create a client
    let client = reqwest::Client::new();

    // Set up headers
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);

    // Send the POST request
    let response = client
        .post("https://api.openai.com/v1/audio/speech")
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    // Check for successful response
    if response.status().is_success() {
        // Write the response body to a file
        let mut file = File::create("speech.mp3")?;
        let content = response.bytes().await?;
        copy(&mut content.as_ref(), &mut file)?;
        println!("Speech audio saved to speech.mp3");
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}

