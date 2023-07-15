# Minigrep

Minigrep is a simplified version of the Unix tool "grep" implemented in Rust. It allows you to search for lines containing a specific query in a given file.

This project is developed as part of the exercises in Chapter 12 of "The Rust Programming Language" book. It serves as a hands-on exercise to practice file I/O operations, error handling, and command-line argument parsing in Rust.

## Usage

To use the Minigrep program, follow the instructions below:

1. Clone or download the project source code.

2. Open a terminal and navigate to the project directory.

3. Build the project using the following command:

   ```shell
   cargo build 

4. Run the Minigrep program with the search query and file name as command-line arguments. For example, to search for the query "example" in the file "sample.txt":

   ```shell
   cargo run --release example sample.txt
   
  The program will display the lines in the file that contain the search query.
