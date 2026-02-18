from vault import serialize_vault
import base64
import json
import platform

def encode_vault(master_password) -> str:
    #handle the encoding prcoesses so rust and python communicate correctly
    json_text = serialize_vault()
    json_bytes = json_text.encode("utf-8")
    b64_bytes = base64.b64encode(json_bytes)
    b64_text = b64_bytes.decode("utf-8")
    #define the request sent to rust for encryption
    request = {
        "mode": "encrypt",
        "master_password": master_password,
        "data": b64_text
    }

    request_json = json.dumps(request)
    return(request_json)

def encrypt_vault(request_json: str) -> str:
    #creates a subprocess calling the rust program
    import subprocess
    #create subprocess depending on what OS the user is on
    system = platform.system()

    if system == "Windows":
        result = subprocess.run(["rust/passvault_crypto/target/debug/passvault_crypto.exe"], input=request_json, text=True, capture_output=True)
    else:
        result = subprocess.run(["rust/passvault_crypto/target/debug/passvault_crypto"], input=request_json, text=True, capture_output=True)
    return result.stdout
