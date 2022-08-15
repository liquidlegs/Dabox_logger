# Dabox logger
This is a basic key logger written in rust and was created a first useful project to learn the Rust programming language.

This project also serves as a reference for me and others in how a developer might use Rust to intract with the Windows API.

The keylogger offers the following features:
- Captures keyboard activity
- Captures clipboard data
- Captures data every 30 seconds
- Data is written to the disk when the buffer exceeds 256 characters
- Writes data to a hidden file on the disk
- Tracks the the position of the cursor

# Compilation instructions
The code can be compiled using the following command:
> cargo build --release
