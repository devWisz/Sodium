# Sodium

Sodium is a lightweight, high-performance command-line spell checker built with Rust. It scans text files word-by-word, checking each against a custom dictionary list using a fast `HashSet` lookup.

You can download the pre-compiled binaries directly from the [GitHub Releases](https://github.com/devWisz/Sodium/releases/tag/1.0).

---

## Preview

<img width="757" height="271" alt="Sodium spelling checker in action" src="https://github.com/user-attachments/assets/7303bcee-be3e-43e1-bef9-e0e2e61292f4" />

---

## Features

- **Fast Lookups**: Leverages Rust's standard `HashSet` for highly optimized dictionary lookups.
- **Minimalist Design**: Zero unnecessary bloat or complex configurations.
- **Case-Insensitive matching**: Detects misspellings regardless of word capitalization.
- **Graceful Error Handling**: Clearly notifies you of missing dictionary or input files instead of crashing.
- **Cross-Platform**: Run the compiled binary natively on macOS, Linux, or Windows.

---

## Manual Installation & Setup

### Prerequisites

Ensure you have the following installed on your machine:
- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/)

### 1. Clone the Repository
```bash
git clone https://github.com/devWisz/Sodium.git
```

### 2. Navigate to the Cargo Project
```bash
cd Sodium/sodium-com
```

### 3. Build the Project
```bash
cargo build
```

---

## Running the Tool

To perform spell-checking, you need:
1. **Your target file** (e.g., `input.txt`) — The file you want to check.
2. **`dictionary.txt`** — The list of correct words (one word per line). By default, the tool looks for `dictionary.txt` in the current working directory, but you can optionally specify a custom dictionary path as a second argument.

### Option A: Run directly with Cargo
You can run the project using Cargo from the `sodium-com` directory:
```bash
# Looks for dictionary.txt in the current directory:
cargo run -- input.txt

# Specifying a custom dictionary path:
cargo run -- input.txt path/to/dictionary.txt
```

### Option B: Build and run the optimized release binary
Build the release executable:
```bash
cargo build --release
```

Then, run the compiled binary:

#### macOS & Linux
```bash
# Run from any folder by specifying paths to both files:
./target/release/sodium-com path/to/input.txt path/to/dictionary.txt

# Or run from the folder containing both files:
./target/release/sodium-com input.txt
```

#### Windows
```cmd
:: Run from any folder by specifying paths to both files:
.\target\release\sodium-com.exe path\to\input.txt path\to\dictionary.txt

:: Or run from the folder containing both files:
.\target\release\sodium-com.exe input.txt
```

---

## File Formats & Example

### Dictionary Format (`dictionary.txt`)
A plain text file containing one valid lowercase/uppercase word per line.
```text
hello
world
rust
programming
is
awesome
computer
science
```

### Input File Format (`input.txt`)
The text document you want to check for spelling errors.
```text
hello world!
programing is asome.
computer science.
rust programming is awesome.
```

### Expected Output
```text
Loading dictionary.......
Checking spelling for: input.txt..

Line 2: Mistaken Word found -> "programing"
Line 2: Mistaken Word found -> "asome"

Total spelling errors found: 2
```

---

## Open Source & Contributions

Sodium is fully open-source. Contributions, forks, and feature requests are welcome!

Developed with ❤️ by **devWisZ** (Sarjak Khanal).
