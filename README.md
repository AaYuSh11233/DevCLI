# DevCLI

**Developer Command-Line Toolkit — by Ninja**

DevCLI is a professional-grade, fast, and extensible developer CLI tool, written in Rust. It’s designed to streamline your development workflow by combining Git, Node.js, Rust, Python, system monitoring, and project scaffolding into one seamless command-line experience.

---

## 🚀 Features

✅ Git Workflow (init, status, add, commit, push, fetch, merge, branch, remote)  
✅ Node.js Commands (npm install/remove, npx run)  
✅ Rust Commands (cargo build/run/check/fmt)  
✅ Python Virtual Environments & Pip  
✅ Project Initialization with boilerplate  
✅ Real-time System Information (CPU, RAM)  
✅ Lightweight, blazing fast — written in Rust

---

## 🧰 Installation

### From source

```bash
git clone https://github.com/AaYuSh11233/DevPilot.git
cd DevPilot
cargo install --path .
```

This will install `devcli.exe` into your `~/.cargo/bin` (on Windows: `C:\Users\<you>\.cargo\bin\devcli.exe`).

### Add to PATH (if needed)
Ensure the following directory is in your system PATH:
```
C:\Users\<you>\.cargo\bin
```

---

## 📦 Usage

Run from anywhere:

### Help
```bash
devcli --help
```

### Commands

#### 📁 Project Scaffolding
```bash
devcli init my-new-app
```

#### 🧬 Git Commands
```bash
devcli git-init
devcli git-status
devcli git-add README.md src/
devcli git-commit -m "Initial commit"
devcli git-push
devcli git-fetch
devcli git-merge dev-branch
devcli git-branch
devcli git-remote
```

#### 🪄 Node.js Commands
```bash
devcli npm-install express
devcli npm-remove express
devcli npx create-react-app my-app
```

#### 🦀 Rust Commands
```bash
devcli cargo-build
devcli cargo-run
devcli cargo-check
devcli cargo-fmt
```

#### 🐍 Python Commands
```bash
devcli py-venv-create
devcli pip-install flask
```

#### 🖥️ System Info
```bash
devcli sysinfo
```

---

## 🔷 Why DevCLI?

- Combines common developer tools in one binary
- Saves time and avoids context switching
- Cross-platform and minimal dependencies
- Open source and extensible

---
