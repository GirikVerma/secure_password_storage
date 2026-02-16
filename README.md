# secure_password_storage

#small project to learn about secure storage practices

passvault init creates a vault file on disk, storing KDF parameters + salt + encrypted payload.

passvault unlock prompts for master password, derives a key, decrypts the vault into memory, then opens an interactive session (a “shell”) so you don’t re-enter the master password every command.

Inside the session: add, list, get, delete, lock, exit.

On lock/exit it re-encrypts and writes the vault back to disk.
