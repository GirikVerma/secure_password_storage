from vault import add_entry
from bindings import encode_vault, encrypt_vault, decrypt_vault
import os 
import json
import atexit

#checks if an existing vault configuration exists
def startup():
    if os.path.exists("vault.dat"):
        print("found!")
        print("Enter master password")
        master_password = input("")
        #Make sure password is actually correct
        decrypted_passwords = decrypt_vault(master_password)
        sanity_check = json.loads(decrypted_passwords)
        return master_password
    else:
        #create .dat file and set passwords
        print("no existing vault found, creating vault.")
        #loop until password matces requirements and is confirmed
        while True:
            print("Create a master password:")
            master_password = input("")
            #ensure password length (or any other future requirements I choose to add) 
            if len(master_password) >= 8:
                #confirm password matches
                print("Confirm password:")
                if input("") != master_password:
                    print("Passwords do not match")
                else:
                    print("*Note that if an entry is not created in this session, you will have to recreate the password next login*")
                    return master_password
            else:
                print("Password is too short!")
def cli(master_password):
    #Simple CLI setup. 
    print("Welcome to PassVault CLI!")
    print("Please choose an option:")
    print("1. Add a new password")
    print("2. Retrieve passwords")
    print("3. Reset vault")
    print("4. Exit")
    choice = input("")
    match choice:
        case "1":
            print("Enter a service name")
            service = input("")
            print("Enter the username/email")
            username = input("")
            print("Enter your password")
            password = input("")
            #create json object
            add_entry(service, username, password)
            #encode json object
            request_json = encode_vault(master_password)
            #send encoded object to rust for encryption
            encrypted_dump = encrypt_vault(request_json)
            #write encrypted dump to file
            with open("vault.dat", "w") as f:
                f.write(encrypted_dump)
        case "2":
            decrypted_passwords = decrypt_vault(master_password)
            print("Decrypted passwords: ")
            print(json.loads(decrypted_passwords))
        case "3":
            print("Are you sure that you would like to reset your password and vault? This action cannot be reversed. (y/n)")
            if input("").lower() == "y":
                os.remove("vault.dat")
                print("Vault reset successful!")
                exit()
        case "4":
            exit()

def exit_handler():
    print("Exiting PassVault...")

def main():
    master_password = startup()
    while True:
        cli(master_password)

if __name__ == "__main__":
    atexit.register(exit_handler)
    main()