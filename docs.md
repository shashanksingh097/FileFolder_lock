# Locker — Current State (v0.1.1)

## Release Status

- Latest release: v0.1.1
- Purpose: Stable stopping point
- Use case: Files encrypted with this version can always be recovered by checking out this Git tag
- Documentation included: Yes

---

## Core Functionality (Working & Stable)

### Lock

- Lock single files
- Lock entire folders (recursive)
- Lock multiple files via wildcard (for example: *.png)
- Password-based encryption
- Original file or folder is deleted after locking
- Output is a portable .lkr file
- Works correctly on Windows paths

---

### Unlock

- Unlock a single .lkr file
- Restore happens next to the .lkr file
- Correct behavior:
  - Single file restores as a file
  - Folder restores as a folder
- Password attempt limits are enforced
- Data is permanently destroyed after attempts are exhausted

---

## Safety and UX Protections

- --force flag for destructive operations
- Confirmation prompt when --force is not used
- Absolute path normalization
- Clear failure modes (wrong password, missing file, etc.)
- Portable .lkr files (can be moved or renamed safely)

---

## Cryptography (High Level)

- Argon2 for password to key derivation
- AES-GCM for authenticated encryption
- Unique salt and nonce per lock operation
- Encrypted payload contains:
  - Metadata
  - Directory structure (relative paths)
  - File contents
- Integrity protection is enforced (tampering breaks decryption)

---

## Data Model (Important)

### Payload Structure (Stable for This Release)

- is_dir — distinguishes file vs folder
- root_name — restored name
- entries — list of relative paths with file bytes

### Fixes Enabled by This Model

- File restoring as a folder bug is fixed
- Correct restore semantics for both files and folders

---

## Known-Good Workflow

### For Files Encrypted With v0.1.1

git checkout v0.1.1  
cargo run -- unlock file.lkr  

### Migration Strategy for Future Versions

1. Checkout the old tag
2. Unlock files
3. Checkout the new version
4. Re-lock files

---

## Known Limitations (Intentional)

- No batch unlock (*.lkr) yet
- No overwrite protection on restore
- No format version byte inside .lkr
- No GUI
- No long-term backward compatibility guarantee beyond Git tags

---

## What Was Consciously Deferred

These features are planned, not forgotten:

- Format versioning (LOCKR1, LOCKR2, etc.)
- Batch unlock support
- --dry-run mode
- Overwrite detection on restore
- Secure zeroization of secrets
- GUI frontend

---

## Big-Picture Achievement

You now have:

- A usable encryption tool
- A frozen, recoverable release
- A clean Git history
- A documented stopping point
- A safe development pause

This is exactly how real security tools should be handled.

---

## Status

Development is paused cleanly at v0.1.1.  
It is safe to encrypt data with this version.

When development resumes, the first logical step will be:

Add format versioning so future releases can automatically handle old .lkr files.
