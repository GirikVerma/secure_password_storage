from vault import add_entry
from bindings import encrypt_vault
def main():
    while True:
        print("Welcome to PassVault CLI!")
        print("Please choose an option:")
        print("1. Add a new password")
        print("2. Retrieve a password")
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
                add_entry(service, username, password)
            case "2":
                print("Not implemented yet")
            case "3":
                exit()
if __name__ == "__main__":
    main()