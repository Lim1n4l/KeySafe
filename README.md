# What is KeySafe

A simple **CLI password manager** built with **Rust and SQLite**.

KeySafe allows users to securely store, view, edit, and delete their saved login credentials. It also includes a built-in password generator for creating strong random passwords using different character types.

# ⚙️ How It Works

The application uses a local SQLite database to store password entries.

Passwords stored in the database are **encrypted using AES-256-GCM**, while the application password is used to securely derive the encryption key using **Argon2id**.

The database file is created automatically at:

`db/data.db`

The application provides the following features:

- View saved passwords
- Add new login entries
- Edit existing login entries
- Delete saved entries
- Decrypt and view stored passwords
- Generate strong random passwords
- Securely encrypt stored passwords

## Password Generator

The built-in password generator allows users to customize generated passwords by choosing:

- Uppercase letters
- Lowercase letters
- Numbers
- Symbols

Users can also choose the desired password length.

# 🔐 Encryption & Security

KeySafe uses modern cryptographic primitives to protect stored passwords.

## Password Encryption

Stored passwords are encrypted using:

**AES-256-GCM**

Each encrypted password uses a unique nonce, which is stored alongside the encrypted data in the database.

## Key Derivation

The application password is not directly used as an encryption key.

Instead, KeySafe derives a 256-bit encryption key using:

**Argon2id**

A cryptographically secure random salt is generated and used during key derivation.

The salt is stored in the database so that the same application password can be used to derive the encryption key when the application is opened again.

The encryption key itself is not stored in the database.

## Local Database

The SQLite database contains information required by the application, including:

- Usernames
- Encrypted passwords
- Encryption nonces
- Cryptographic salt

Stored passwords are not kept as plaintext.

⚠️ **Important:** Protect your application password carefully. Losing it may make your encrypted passwords unrecoverable.

# 🚀 Installation & Run

## 1. Clone the repository

```bash
git clone https://github.com/Lim1n4l/KeySafe.git
cd KeySafe/src
```
# 🛠️ Build the application
Make sure you have Rust and Cargo installed
```bash
cargo build --release
```
# Run the application
## ⚠️ Important: The application must currently be run from the src directory. Running the executable from another directory may cause the application to fail to locate the database correctly.
```bash
cd src
cargo run --release
```
# 💡 Feedback
Suggestions, improvements, bug reports, and contributions are welcome.
I would be happy to hear your ideas and feedback.
