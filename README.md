# Hack-Assembler
A 2-Pass Assembler built in Rust to compile Hack assembly code into binary machine code for the [Nand2Tetris](https://www.nand2tetris.org/) Hack computer.

---

## 📖 Overview
This project implements a **two-pass assembler** in Rust for the Hack computer system.  
It translates Hack assembly (`.asm`) programs into their corresponding binary (`.hack`) format.  

- **First Pass**: Resolves labels and updates the symbol table.  
- **Second Pass**: Translates instructions (`A`, `C`, and variables) into binary.  

---

## ⚡ Features
- Converts `.asm` files into `.hack` files.  
- Supports variables and labels with a symbol table.  
- Strips comments and whitespace automatically.  
- Outputs binaries into an `output/` directory.  
- Error handling for invalid instructions.  
- Handles multiple input files (up to a configurable maximum).  

---

## 🛠 Project Structure
```
src/
├── assembler.rs # Core assembler logic
├── parser.rs # Breaks instructions into variants
├── translator.rs # Translates A and C instructions to binary
├── symbol_handler.rs # Manages labels & variables (symbol table)
├── lib.rs # Library entry point
└── main.rs # CLI entry point
```

---

## 🚀 Getting Started

### 1. Clone the repository
```bash
git clone https://github.com/your-username/hack-assembler.git
```
### 2. Enter the repo directory
```bash
cd hack-assembler
```

### 3. Build
```bash
cargo build --release
```
### 4. Run
Pass one or more `.asm` files as arguments (Max 10 args)
```bash
cargo run -- ./input/Add.asm ./input/Max.asm
```
OR
```bash
cargo run -- "./input/Add.asm" "./input/Max.asm"
```
Using the quotes `"file_path"` helps to prevent the user from mistakenly joining the two or more filepaths together while results in invalid file referencing.

If the input files are in the Root directory of this project, you can reference them directly:
```bash
cargo run -- Add.asm Max.asm
```

The Output `.hack` files will be created in the `output/` directory which is automatically created if it does not exist.