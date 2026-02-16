use std::io::{self, Read};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct Request {
    mode: String,
    password: String,
    data: String
}

#[derive(Serialize)]
struct Response {
    data: String
}

fn encrypt(master_password: &str, plaintext: &[u8]) -> Result<Vec<u8>, String> {
    unimplemented!()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let request: Request = serde_json::from_str(&input).unwrap();

    match request.mode.as_str() {
        "encrypt" => {
            println!("encrypt");
        }
        "decrypt" => {
            println!("decrypt");
        }
        _ => {
            eprintln!("Invalid mode");
            std::process::exit(1);
        }
    }

    let response: Response = Response{
        data: request.data,
    };
    
    let output = serde_json::to_string(&response).unwrap();
    println!("\n{}", output);
}
