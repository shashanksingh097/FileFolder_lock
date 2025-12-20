mod cli;
mod core;

use clap::Parser;
use cli::{Cli, Commands};
use core::metadata::Payload;
use rpassword::read_password;
use std::fs;

fn main() {
    let cli = Cli::parse();

    match cli.command {
            Commands::Lock { path, attempts } => {
            let attempts = attempts.max(1);

            let data = fs::read(&path).expect("Failed to read file");

            println!("Enter password:");
            let password = read_password().unwrap();

            let payload = Payload {
                original_name: path.clone(),
                file_data: data,
            };

            let serialized = bincode::serialize(&payload).unwrap();
            let (salt, nonce, encrypted) =
                core::crypto::encrypt(&serialized, &password);

            let mut output = Vec::new();
            output.extend(b"LOCKR1");
            output.push(attempts); // attempts_left (PLAIN HEADER)
            output.extend(salt);
            output.extend(nonce);
            output.extend(encrypted);

            fs::write(format!("{}.lkr", path), output).unwrap();
            fs::remove_file(&path).unwrap();

            println!("File locked successfully");
        }

        Commands::Unlock { path } => {
            let mut data = fs::read(&path).expect("Failed to read file");

            let attempts_left = data[6];
            let salt = &data[7..23];
            let nonce = &data[23..35];
            let encrypted = &data[35..];

            if attempts_left == 0 {
                println!("File permanently locked");
                return;
            }

            println!("Enter password:");
            let password = read_password().unwrap();

            match core::crypto::decrypt(encrypted, &password, salt, nonce) {
                Ok(serialized) => {
                    let payload: Payload =
                        bincode::deserialize(&serialized).unwrap();

                    fs::write(&payload.original_name, payload.file_data).unwrap();
                    fs::remove_file(&path).unwrap();

                    println!("Unlocked successfully");
                }
                Err(_) => {
                    let remaining = attempts_left - 1;
                    data[6] = remaining;

                    fs::write(&path, &data).unwrap();

                    if remaining == 0 {
                        core::destroy::destroy_file(&path);
                        println!("Too many attempts. File destroyed permanently.");
                    } else {
                        println!("Wrong password. Attempts left: {}", remaining);
                    }
                }
            }
        }
    }

}
