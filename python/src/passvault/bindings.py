from vault import serialize_vault
import base64
import json

def encrypt_vault(master_password) -> str:
    json_text = serialize_vault()
    json_bytes = json_text.encode("utf-8")
    b64_bytes = base64.b64encode(json_bytes)
    b64_text = b64_bytes.decode("utf-8")

    request = {
        "mode": "encrypt",
        "password": master_password,
        "data": b64_text
    }

    request_json = json.dumps(request)
    return(request_json)
