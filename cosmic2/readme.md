Here’s a clean, professional `README.md` for your app:

---

# Hash Tool (COSMIC / Rust)

A simple desktop hashing utility built in Rust using the COSMIC ecosystem.

This application allows you to quickly generate cryptographic hashes from text input, view multiple results, copy them, and append them to a file.

---

## Features

* Generate hashes from text input
* Multiple algorithms:

  * SHA-256
  * SHA-512
  * SHA3-256
  * SHA3-512
  * BLAKE3
* Selectable & copyable hash output
* Scrollable list of generated hashes
* Append all generated hashes to `hash.txt`

  * Creates the file if it does not exist
  * Appends if the file already exists
* Clean, minimal dark theme UI
* Lightweight and fast

---

## Why Rust + COSMIC?

This app demonstrates how straightforward it is to build desktop utilities in Rust.

Because COSMIC is built in Rust:

* You can directly use Rust crates like `sha2`, `sha3`, and `blake3`
* No language bridges or bindings are required
* Cargo manages everything cleanly
* The entire stack stays consistent and simple

Adding cryptographic functionality was as easy as adding dependencies and calling them directly.

---

## Building the App

### 1. Install Rust

If you don’t have Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Then restart your terminal.

---

### 2. Clone the Project

```bash
git clone <your-repo-url>
cd <project-folder>
```

---

### 3. Build and Run

```bash
cargo run
```

To build a release version:

```bash
cargo build --release
```

The binary will be in:

```
target/release/
```

---

## How It Works

1. Enter text in the input field.
2. Choose a hashing algorithm.
3. Click **Generate Hash**.
4. The result appears in the list below.
5. You can:

   * Select and copy the hash
   * Generate multiple hashes
   * Clear the list
   * Append all hashes to `hash.txt`

The `hash.txt` file is created in the same directory where the application is run.

---

## Dependencies

* `iced` (GUI)
* `sha2`
* `sha3`
* `blake3`

All dependencies are managed via Cargo.

---

## License

MIT (or your chosen license)

---

If you’d like, I can also:

* Make it more “GitHub polished” with badges
* Add screenshots section formatting
* Add a COSMIC-specific section
* Or make it shorter and minimalist

Just tell me the vibe you want 🙂
