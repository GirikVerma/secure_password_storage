from datetime import datetime, timezone
import json
vault = {
    "version": 1,
    "entries": {

    }
}

#convert inputs into a json format 

def add_entry(service, username, password):
    vault["entries"][service] = {
        "username": username,
        "password": password,
        "created": datetime.now(timezone.utc).isoformat()
    }
    print(vault)

def serialize_vault():
    return json.dumps(vault)