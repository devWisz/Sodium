# Sodium

Sodium is a fast, lightweight command line spell checker written in Rust. It checks text files against a custom dictionary using a `HashSet` for quick lookups.

[GitHub Releases] : (https://github.com/devWisz/Sodium/releases/tag/1.0) page.

---

## Preview

<img width="757" height="271" alt="Sodium spelling checker in action" src="https://github.com/user-attachments/assets/7303bcee-be3e-43e1-bef9-e0e2e61292f4" />

---

## Features

* Fast dictionary lookups via Rust `HashSet`
* Case insensitive word matching
* Simple error handling for missing files
* Cross platform support for macOS, Linux, and Windows

---

## Installation (Manually)

### Prerequisites

* [Rust & Cargo](https://www.rust-lang.org/tools/install)
* [Git](https://git-scm.com/)

### Build from source

```bash
git clone https://github.com/devWisz/Sodium.git
cd Sodium/sodium-com
cargo build --release

```

---

## Usage

Sodium requires an input file to check and a dictionary file containing valid words (one per line). By default, it looks for `dictionary.txt` in your current directory.

### Running with Cargo

```bash
cargo run -- input.txt
cargo run -- input.txt path/to/dictionary.txt

```

### Running the compiled binary

#### macOS and Linux

```bash
./target/release/sodium-com input.txt path/to/dictionary.txt

```

#### Windows

```cmd
.\target\release\sodium-com.exe input.txt path\to\dictionary.txt

```

---

## Example

### `dictionary.txt`

```text
hello
world
rust
programming

```

### `input.txt`

```text
hello world!
programing is awesome.

```

### Output

```text
Loading dictionary.......
Checking spelling for: input.txt..

Line 2: Mistaken Word found -> "programing"

Total spelling errors found: 1

```

---

## License and Contributions
Sodium is open source. Pull requests and issues are welcome.

Developed by Sarjak Khanal (devWisZ).

