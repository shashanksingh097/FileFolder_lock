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
            let data = fs::read(&path).expect("Failed to read file");

            println!("Enter password:");
            let password = read_password().unwrap();

            let payload = Payload {
                attempts_left: attempts,
                original_name: path.clone(),
                file_data: data,
            };

            let serialized = bincode::serialize(&payload).unwrap();
            let (salt, nonce, encrypted) =
                core::crypto::encrypt(&serialized, &password);

            let mut output = Vec::new();
            output.extend(b"LOCKR1");
            output.extend(salt);
            output.extend(nonce);
            output.extend(encrypted);

            fs::write(format!("{}.lkr", path), output).unwrap();
            fs::remove_file(&path).unwrap();

            println!("File locked successfully");
        }

        Commands::Unlock { path } => {
            let data = fs::read(&path).expect("Failed to read file");

            let salt = &data[6..22];
            let nonce = &data[22..34];
            let encrypted = &data[34..];

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
                    println!("Wrong password");
                }
            }
        }
    }
}
