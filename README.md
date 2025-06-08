<h1 align="center">🦀 RustShell</h1>

<p align="center">
  <b>A Minimal Unix-like Shell Written in Rust</b><br>
  <i>Build your own custom terminal experience</i>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust-orange?logo=rust" />
  <img src="https://img.shields.io/badge/Type-Custom%20Shell-green" />
  <img src="https://img.shields.io/badge/Platform-Unix%2FmacOS%2FLinux-lightgrey" />
</p>

---

## 📚 About

**RustShell** is a simple, lightweight custom shell created from scratch using Rust.  
It mimics basic Unix shell behavior — parsing commands, executing system processes, and handling built-in commands like `cd` and `exit`.

Perfect for:
- Learning systems programming
- Understanding shells
- Practicing Rust fundamentals

---

## ✨ Features

✅ Custom shell prompt (`mysh>`)  
✅ Executes built-in commands: `cd`, `exit`  
✅ Supports all external shell commands: `ls`, `pwd`, `echo`, etc.  
✅ Error handling for invalid commands  
✅ Minimal and clean architecture — beginner-friendly

---

## 🚀 Quick Demo

```bash
$ cargo run
mysh> pwd
/Users/animesh/Desktop/rust_shell
mysh> echo Hello, Rust!
Hello, Rust!
mysh> cd ..
mysh> pwd
/Users/animesh/Desktop
mysh> exit
```

---

## ⚙️ Getting Started

### 🔧 Requirements

- [Rust (via rustup)](https://rust-lang.org/tools/install)
- A Unix-like system (macOS, Linux, WSL)

### 🛠 Installation

```bash
git clone https://github.com/thisisanimesh01/rust_shell.git
cd rust_shell
cargo run
```

---

## 📂 Project Structure

```
rust_shell/
├── Cargo.toml       # Project metadata and dependencies
└── src/
    └── main.rs      # Core shell logic
```

---

## 💡 Try These Commands

```bash
cd ..
pwd
ls -la
echo Hello, Rust!
whoami
touch hello.txt
cat hello.txt
rm hello.txt
exit
```

---

## 🌟 Planned Improvements

- 🔁 Command history navigation (arrow keys)
- 💡 Autocomplete commands and paths
- 🔗 Pipe support (e.g. `ls | grep txt`)
- 🎨 Colorful prompt with current path
- 🧾 Script execution (`.mysh` files)

---

## 👨‍💻 Author

**Animesh Yadav**  
🔗 [LinkedIn]([https://www.linkedin.com/in/animesh-yadav-000](https://www.linkedin.com/in/animesh-yadav-39460b276/))  


---

## 📄 License


> ⚡ Built with ❤️ in Rust to explore how shells work at the core.

