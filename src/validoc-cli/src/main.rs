use clap::{App, Arg, SubCommand};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Document {
    filename: String,
    hash: String,
}

async fn upload_file(file_path: &str, api_url: &str) -> Result<Document, Box<dyn Error>> {
    let client = Client::new();
    let file_data = std::fs::read(file_path)?;
    
    // Making a POST request to the API (we will assume a simple upload endpoint)
    let response = client
        .post(format!("{}/upload", api_url))
        .body(file_data)
        .send()
        .await?;

    if response.status().is_success() {
        let document: Document = response.json().await?;
        Ok(document)
    } else {
        Err(format!("Failed to upload: {}", response.status()).into())
    }
}

async fn hash_file(file_path: &str, api_url: &str) -> Result<String, Box<dyn Error>> {
    let file_data = std::fs::read(file_path)?;
    // Hashing the file (this is a simple example using SHA256)
    let hash = format!("{:x}", sha2::Sha256::digest(&file_data));
    
    // Optionally, send this hash to the API (depending on your use case)
    // let client = Client::new();
    // let response = client.post(format!("{}/hash", api_url))
    //                      .json(&hash)
    //                      .send()
    //                      .await?;

    Ok(hash)
}

async fn verify_file(file_path: &str, api_url: &str) -> Result<bool, Box<dyn Error>> {
    let file_data = std::fs::read(file_path)?;
    let hash = format!("{:x}", sha2::Sha256::digest(&file_data));

    // Verifying the file with the API
    let client = Client::new();
    let response = client
        .post(format!("{}/verify", api_url))
        .json(&hash)
        .send()
        .await?;

    Ok(response.status().is_success())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("validoc")
        .version("1.0")
        .about("Document verification using blockchain and IPFS")
        .arg(
            Arg::new("api-url")
                .long("api-url")
                .takes_value(true)
                .default_value("http://localhost:8080")
                .help("The URL of the API server")
        )
        .subcommand(
            SubCommand::new("upload")
                .about("Upload a file to IPFS and Ethereum")
                .arg(
                    Arg::new("file")
                        .help("The file to upload")
                        .required(true)
                        .index(1),
                )
        )
        .subcommand(
            SubCommand::new("hash")
                .about("Generate the hash of a file")
                .arg(
                    Arg::new("file")
                        .help("The file to generate a hash for")
                        .required(true)
                        .index(1),
                )
        )
        .subcommand(
            SubCommand::new("verify")
                .about("Verify a file against a stored hash")
                .arg(
                    Arg::new("file")
                        .help("The file to verify")
                        .required(true)
                        .index(1),
                )
        )
        .get_matches();

    let api_url = matches.value_of("api-url").unwrap();

    // Handle the 'upload' subcommand
    if let Some(matches) = matches.subcommand_matches("upload") {
        if let Some(file) = matches.value_of("file") {
            match upload_file(file, api_url).await {
                Ok(document) => println!("Successfully uploaded document: {:?}", document),
                Err(e) => eprintln!("Error uploading file: {}", e),
            }
        }
    }

    // Handle the 'hash' subcommand
    if let Some(matches) = matches.subcommand_matches("hash") {
        if let Some(file) = matches.value_of("file") {
            match hash_file(file, api_url).await {
                Ok(hash) => println!("File hash: {}", hash),
                Err(e) => eprintln!("Error hashing file: {}", e),
            }
        }
    }

    // Handle the 'verify' subcommand
    if let Some(matches) = matches.subcommand_matches("verify") {
        if let Some(file) = matches.value_of("file") {
            match verify_file(file, api_url).await {
                Ok(is_valid) => {
                    if is_valid {
                        println!("File verification successful.");
                    } else {
                        eprintln!("File verification failed.");
                    }
                }
                Err(e) => eprintln!("Error verifying file: {}", e),
            }
        }
    }

    Ok(())
}
