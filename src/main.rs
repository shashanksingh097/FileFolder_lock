mod cli;
mod core;

use clap::Parser;
use cli::{Cli, Commands};
use core::metadata::{Payload, FolderEntry};
use rpassword::read_password;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Lock { path, attempts } => {
            let attempts = attempts.max(1);
            let abs_path = normalize_to_absolute(&path);

            if !abs_path.exists() {
                panic!("Path does not exist: {}", abs_path.display());
            }

            let parent = abs_path.parent().unwrap_or(Path::new("."));
            let name = abs_path.file_name().unwrap().to_string_lossy().to_string();

            println!("Enter password:");
            let password = read_password().unwrap();

            let payload = if abs_path.is_file() {
                lock_single_file(&abs_path)
            } else {
                lock_folder(&abs_path)
            };

            let serialized = bincode::serialize(&payload).unwrap();
            let (salt, nonce, encrypted) =
                core::crypto::encrypt(&serialized, &password);

            let out_path = parent.join(format!("{}.lkr", name));

            let mut output = Vec::new();
            output.extend(b"LOCKR1");
            output.push(attempts);
            output.extend(salt);
            output.extend(nonce);
            output.extend(encrypted);

            fs::write(&out_path, output).unwrap();

            if abs_path.is_file() {
                fs::remove_file(&abs_path).unwrap();
            } else {
                fs::remove_dir_all(&abs_path).unwrap();
            }

            println!("Locked → {}", out_path.display());
        }

        Commands::Unlock { path } => {
            let abs_lkr = normalize_to_absolute(&path);

            if !abs_lkr.exists() {
                panic!("Locked file not found");
            }

            let restore_parent = abs_lkr.parent().unwrap_or(Path::new("."));

            let mut data = fs::read(&abs_lkr).unwrap();
            let attempts_left = data[6];

            if attempts_left == 0 {
                println!("File permanently locked");
                return;
            }

            let salt = &data[7..23];
            let nonce = &data[23..35];
            let encrypted = &data[35..];

            println!("Enter password:");
            let password = read_password().unwrap();

            match core::crypto::decrypt(encrypted, &password, salt, nonce) {
                Ok(serialized) => {
                    let payload: Payload =
                        bincode::deserialize(&serialized).unwrap();

                    restore_payload(&payload, restore_parent);
                    fs::remove_file(&abs_lkr).unwrap();

                    println!("Unlocked successfully");
                }
                Err(_) => {
                    let remaining = attempts_left - 1;
                    data[6] = remaining;
                    fs::write(&abs_lkr, &data).unwrap();

                    if remaining == 0 {
                        core::destroy::destroy_file(abs_lkr.to_str().unwrap());
                        println!("Too many attempts. Data destroyed.");
                    } else {
                        println!("Wrong password. Attempts left: {}", remaining);
                    }
                }
            }
        }
    }
}

/// Absolute + Windows-safe
fn normalize_to_absolute(input: &str) -> PathBuf {
    let path = PathBuf::from(input);
    let abs = if path.is_absolute() {
        path
    } else {
        std::env::current_dir().unwrap().join(path)
    };
    abs.canonicalize().unwrap_or(abs)
}

fn lock_single_file(path: &Path) -> Payload {
    let data = fs::read(path).unwrap();
    Payload {
        root_name: path.file_name().unwrap().to_string_lossy().to_string(),
        entries: vec![FolderEntry {
            relative_path: path.file_name().unwrap().to_string_lossy().to_string(),
            data,
        }],
    }
}

fn lock_folder(path: &Path) -> Payload {
    let root = path.file_name().unwrap().to_string_lossy().to_string();
    let mut entries = Vec::new();

    for entry in WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
    {
        if entry.file_type().is_file() {
            let full = entry.path();
            let rel = full.strip_prefix(path).unwrap();
            let data = fs::read(full).unwrap();

            entries.push(FolderEntry {
                relative_path: rel.to_string_lossy().to_string(),
                data,
            });
        }
    }

    Payload { root_name: root, entries }
}

fn restore_payload(payload: &Payload, parent: &Path) {
    let root = parent.join(&payload.root_name);
    fs::create_dir_all(&root).unwrap();

    for entry in &payload.entries {
        let full = root.join(&entry.relative_path);
        if let Some(p) = full.parent() {
            fs::create_dir_all(p).unwrap();
        }
        fs::write(full, &entry.data).unwrap();
    }
}
