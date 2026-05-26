# Sodium

Sodium is a lightweight command-line spell checker built with Rust.It scans text files word by word using a custom dictionary and detects spelling mistakes directly from the terminal.
---

Directly use from here : https://github.com/devWisz/Sodium/releases/tag/1.0

# Features

- Fast command-line spell checking
- Custom dictionary support
- Detects invalid or unknown words
- Case-insensitive word matching
- Lightweight and minimal architecture
- Uses `HashSet` for fast lookups
- Clean terminal output
- Cross-platform support


---

# Installation (Manually)

## Prerequisites

Make sure the following tools are installed:

- Rust
- Cargo
- Git

---

## Clone The Repository

```bash
git clone https://github.com/devWisz/Sodium.git
```

---

## Move Into The Project Directory

```bash
cd Sodium
```

---

## Build The Project

```bash
cargo build
```

---

# Running The Project

## Run Directly

```bash
cargo run -- input.txt
```

---

## Run Optimized Release Build

```bash
cargo build --release
```

---

## Run Release Executable

### Windows

```bash
./target/release/sodium.exe 
```


---

# Dictionary Format

The dictionary file should contain one valid word per line.

Example:

```txt
hello
world
rust
programming
awesome
computer
science
```

---

# Example Input File

```txt
hello world!
programing is asome.
computer science.
rust programming is awesome.
```

---

# Example Output

```bash
Loading dictionary.......

Checking spelling for: input.txt..

Line 2: Mistaken Word found -> "programing"
Line 2: Mistaken Word found -> "asome"

Total spelling errors found: 2
```



# Open Source

Sodium is fully open source.Contributions, improvements, and forks are always welcome.

Developed by devWisZ aka Sarjak Khanal.
