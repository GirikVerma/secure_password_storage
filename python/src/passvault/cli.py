from vault import add_entry
from bindings import encode_vault, encrypt_vault, decrypt_vault
import os 
import json
import atexit
import getpass

#checks if an existing vault configuration exists
def startup():
    verified = False
    if os.path.exists("vault.dat"):
        print("found!")
        while verified == False:
            print("Enter master password")
            master_password = getpass.getpass("")
            #Make sure password is actually correct
            try:
                decrypted_passwords = decrypt_vault(master_password)
                sanity_check = json.loads(decrypted_passwords)
                verified = True
            except json.JSONDecodeError:
                    print("Your password was incorrect!")
        return master_password
    else:
        #create .dat file and set passwords
        print("no existing vault found, creating vault.")
        #loop until password matces requirements and is confirmed
        while True:
            print("Create a master password:")
            master_password = getpass.getpass("")
            #ensure password length (or any other future requirements I choose to add) 
            if len(master_password) >= 8:
                #confirm password matches
                print("Confirm password:")
                if getpass.getpass("") != master_password:
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
            if os.path.exists("vault.dat"):
                decrypted_passwords = decrypt_vault(master_password)
                print("Decrypted passwords: ")
                print(json.loads(decrypted_passwords))
            else:
                print("Error: file vault.dat is missing, or no password was ever encrypted")
        case "3":
            print("Are you sure that you would like to reset your password and vault? This action cannot be reversed. (y/n)")
            if input("").lower() == "y":
                os.remove("vault.dat")
                print("Vault reset successful!")
                main()
            else:
                print("Vault reset cancelled!")
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