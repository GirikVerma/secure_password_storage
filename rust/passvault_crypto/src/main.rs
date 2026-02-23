use std::io::{self, Read};
use std::convert::TryInto;
use serde::{Serialize, Deserialize};
use argon2::Argon2;
use chacha20poly1305::{aead::{Aead, KeyInit},ChaCha20Poly1305, Key, Nonce,};
use rand::{rngs::OsRng, RngCore};

//Constants
const VERSION: u8 = 1;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;

// Define request and response structures
#[derive(Deserialize)]
struct Request {
    mode: String,
    master_password: String,
    data: String
}

#[derive(Serialize)]
struct Response {
    status: String,
    data: Option<String>,
    error: Option<String>
}

fn encrypt(master_password: &str, plaintext_json: &[u8]) -> Result<Vec<u8>, String> {
    //Create the vector that will actually be storing all of this (version + salt + nonce + ciphertext) 
    let mut blob = Vec::with_capacity(1 + 16 + 12 + plaintext_json.len());

    //Define a 16 byte arrary for the salt and fill it with random bytes
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

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

fn decrypt(master_password: &str, ciphertext_json: &[u8]) -> Result<Vec<u8>, String>{
    //Make sure that the ciphertext is of expected length (45 in this case)
    if ciphertext_json.len() < 45 {
        return Err("Blob too short".to_string());
    }

    //Get version number 
    let version = ciphertext_json[0];
    if version != VERSION{
        return Err("Version mismatch".to_string());
    }

    //Get salt
    let salt: [u8; 16] = ciphertext_json[1..17]
        //make sure the salt is 16 bytes
        .try_into()
        .map_err(|_| "Salt parse failed".to_string())?;
    
    //Get nonce 
    let nonce: [u8; 12] = ciphertext_json[17..29]
        //Make sure its 12 bytes
        .try_into()
        .map_err(|_| "Salt parse failed".to_string())?;

    //Ciphertext 
    let ciphertext = &ciphertext_json[29..];

    //Key 
    let mut key = [0u8; 32];
    let argon2 = Argon2::default();
    argon2.hash_password_into(master_password.as_bytes(), &salt, &mut key)
        .map_err(|e| format!("Key derivation failed {}", e))?;

    //Decrypt ciphertext + tag 
    let plain_text = ChaCha20Poly1305::new(Key::from_slice(&key)).decrypt(Nonce::from_slice(&nonce), ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))?;

    Ok(plain_text)
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
            //call encrypt function
            let encrypted_bytes = encrypt(&request.master_password, &plaintext_json);
            // encode the encrypted json as base64 and send it back
            let encrypted_bytes_base64 = base64::encode(&encrypted_bytes.unwrap());
            let response = Response {status: "ok".to_string(), data: Some(encrypted_bytes_base64), error: None};
            let output = serde_json::to_string(&response).unwrap();
            println!("{}", output);
        }

        "decrypt" => {
            // decode the base64 encoded input
            let ciphertext_json = base64::decode(&request.data).unwrap();
            //call decrypt function
            let decrypted_bytes = decrypt(&request.master_password, &ciphertext_json).unwrap();
            //format response
            let decrypted_string = String::from_utf8(decrypted_bytes).unwrap();
            let response = Response {status: "ok".to_string(), data: Some(decrypted_string), error: None};
            let output = serde_json::to_string(&response).unwrap();
            println!("{}", output);
        }
        _ => {
            eprintln!("Invalid mode");
            std::process::exit(1);
        }
    }
}