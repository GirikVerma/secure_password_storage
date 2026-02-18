use std::io::{self, Read};
use serde::{Serialize, Deserialize};
use argon2::Argon2;
use chacha20poly1305::{aead::{Aead, KeyInit},ChaCha20Poly1305, Key, Nonce,};
use rand::{rngs::OsRng, RngCore};

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
    //Define a 16 byte arrary for the salt and fill it with random bytes
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    //Create a version number
    const VERSION: u8 = 1;

    //Create the vector that will actually be storing all of this (version + salt + nonce + ciphertext) 
    let mut blob = Vec::with_capacity(1 + 16 + 12 + plaintext_json.len());

    //Derive key
    let mut key = [0u8; 32];
    let argon2 = Argon2::default();
    argon2.hash_password_into(master_password.as_bytes(), &salt, &mut key);

    //generate nonce
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);

    //generate ciphertext using ChaCha20Poly1305
    let cipher_text = ChaCha20Poly1305::new(Key::from_slice(&key)).encrypt(Nonce::from_slice(&nonce), plaintext_json)
        .map_err(|e| format!("Encryption failed: {}", e))?;



    //Fill in the vector space
    blob.push(VERSION);
    blob.extend_from_slice(&salt);
    blob.extend_from_slice(&nonce);
    blob.extend_from_slice(&cipher_text);


    //Return blob 
    Ok(blob)
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