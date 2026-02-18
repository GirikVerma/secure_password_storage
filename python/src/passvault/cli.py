from vault import add_entry
from bindings import encode_vault, encrypt_vault
import os 

#checks if an existing vault configuration exists
def startup():
    if os.path.exists("vault.dat"):
        print("found!")
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
                    return master_password
            else:
                print("Password is too short!")
def cli(master_password):
    #Simple CLI setup. 
    print("Welcome to PassVault CLI!")
    print("Please choose an option:")
    print("1. Add a new password")
    print("2. Retrieve passwords")
    print("3. Exit")
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
            print("Not implemented yet")
        case "3":
            exit()
def main():
    master_password = startup()
    while True:
        cli(master_password)

if __name__ == "__main__":
    main()