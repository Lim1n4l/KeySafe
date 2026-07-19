# What is Password Manager

A simple **CLI password manager** built with **Python and SQLite**.

This application allows users to store, view, edit, and delete their saved login credentials. It also includes a built-in password generator for creating random passwords using different character types.

# How It Works

The application uses a local SQLite database to store user credentials.

The database file is created automatically at:

`db/data.db`

The application provides the following features:

* View saved passwords
* Add new login entries
* Edit existing entries
* Delete saved entries
* Generate strong random passwords

## Password Generator

The built-in password generator allows users to customize generated passwords by choosing:

* Uppercase letters
* Lowercase letters
* Numbers
* Symbols

Users can also choose the desired password length.

# 🚀 Installation & Run

### 1. Clone the repository

```bash
git clone https://github.com/Lim1n4l/KeySafe.git
```

### 3. Run the application

```bash
python main.py
```

# Database & Security Warning ⚠️

1- **The database is NOT encrypted.**

2- All usernames and passwords are stored as plain text inside the SQLite database file:

```
db/data.db
```

3- Anyone who can access the database file can read all stored credentials.

4- Do not store sensitive or important passwords unless you understand the security risks.

5- This project is intended for learning purposes and local password management, not as a replacement for professional password managers.

# Feedback 💡

Any suggestions, improvements, or contributions are welcome.

I would be happy to hear your ideas and feedback.
