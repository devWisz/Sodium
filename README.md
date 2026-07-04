# Sodium

Sodium is a lightweight command-line spell checker built with Rust.It scans text files word by word using a custom dictionary and detects spelling mistakes directly from the terminal.
---

Directly use from here : https://github.com/devWisz/Sodium/releases/tag/1.0

Screenshot of the Project : 
<img width="757" height="271" alt="Screenshot 2026-05-26 220102" src="https://github.com/user-attachments/assets/7303bcee-be3e-43e1-bef9-e0e2e61292f4" />


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
cd Sodium/sodium-com
```

---

## Build The Project

```bash
cargo build
```

---

# Running The Project

To run the spell checker, you need an input file to check (e.g., `input.txt`) and a dictionary file (`dictionary.txt`). **Both files must be in the current working directory from which you execute the tool.**

## Run Directly with Cargo

Run directly from the `sodium-com` directory:

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

When running the release executable, ensure that `dictionary.txt` and your target input file (e.g., `input.txt`) are present in the directory from which you run the command.

### macOS & Linux

From the `sodium-com` directory:

```bash
./target/release/sodium-com input.txt
```

Alternatively, copy the compiled binary `sodium-com` from `target/release/` to any folder containing both `dictionary.txt` and your input file, and run:

```bash
./sodium-com input.txt
```

### Windows

From the `sodium-com` directory:

```bash
.\target\release\sodium-com.exe input.txt
```

Alternatively, copy the compiled binary `sodium-com.exe` from `target/release/` to any folder containing both `dictionary.txt` and your input file, and run:

```cmd
sodium-com.exe input.txt
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
