use std::io::{self, Read};
use serde::{Serialize, Deserialize};

// Define request and response structures
#[derive(Deserialize)]
struct Request {
    mode: String,
    master_password: String,
    data: String
}

#[derive(Serialize)]
struct Response {
    data: String
}

fn encrypt(master_password: &str, plaintext_json: &[u8]) -> Result<Vec<u8>, String> {
    Ok(plaintext_json.to_vec())
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let request: Request = serde_json::from_str(&input).unwrap();

    // check what mode we are in and call the appropriate function
    match request.mode.as_str() {

        "encrypt" => {  
            // decode the base64 encoded input
            let plaintext_json = base64::decode(&request.data).unwrap();
            let encrypted_bytes = encrypt(&request.master_password, &plaintext_json);
            // encode the encrypted json as base64 and send it back
            let encrypted_bytes_base64 = base64::encode(&encrypted_bytes.unwrap());
            let response = Response {data: encrypted_bytes_base64};
            let output = serde_json::to_string(&response).unwrap();
            println!("{}", output);
        }

        "decrypt" => {
            println!("decrypt");
        }
        _ => {
            eprintln!("Invalid mode");
            std::process::exit(1);
        }
    }
}