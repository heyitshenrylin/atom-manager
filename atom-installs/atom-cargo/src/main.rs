/// # ATOM MANAGER v0.1.0
/// author: heyitshenrylin
/// https://github.com/heyitshenrylin
///
/// # IN THIS VERSION
///
/// User input is read from the cmd line
/// Supports my installs for c++, python, and rust
/// Additional installs are easy enough to add
///
/// # HOW TO USE
///
/// Compile this file 'main.rs' to create an executable file
/// Create a shortcut to the created executable
use std::io;
use std::process;
use std::process::Command;

fn main() {
    // Display current atom installs
    println!("1 - c++");
    println!("2 - python");
    println!("3 - rust");

    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Invalid Input");

        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // For future installs or installing on another system:
        // Locate the PATH for the batch files and link them to to process::Command

        if user_input == 1 {
            Command::new("C:\\Users\\Henry\\Documents\\atom-manager\\atom-installs\\atom-c++.bat")
                .spawn()
                .expect("Something went wrong trying to open c++");
        } else if user_input == 2 {
            Command::new(
                "C:\\Users\\Henry\\Documents\\atom-manager\\atom-installs\\atom-python.bat",
            )
            .spawn()
            .expect("Something went wrong trying to open python");
        } else if user_input == 3 {
            Command::new("C:\\Users\\Henry\\Documents\\atom-manager\\atom-installs\\atom-rust.bat")
                .spawn()
                .expect("Something went wrong trying to open rust");
        }

        process::exit(0x0100);
    }
}
