# 🧹 devclean

> Fast, intelligent CLI tool to find and clean development junk across your system.

![Rust](https://img.shields.io/badge/Rust-1.70+-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

---

## ⚡ Quick Start

```bash
cargo install --path .
devclean
```

---

## ✨ What is devclean?

Modern development environments generate massive amounts of junk:

- Rust `target/` folders  
- Node.js `node_modules/`  
- Python `__pycache__/`  
- Temporary build artifacts  

Over time, these silently consume **gigabytes of disk space**.

**devclean** scans your system and shows exactly where that space is going — fast.

---

## 📸 Example

```bash
devclean /
```

```text
[USER]   /home/eve/Desktop/RustProjects/frisk/target (819 MB)
[USER]   /home/eve/RustroverProjects/TUITemplate/target (329 MB)
[USER]   /home/eve/.local/share/opencode/bin/node_modules (21 MB)
[SYSTEM] /usr/lib/node_modules (40 MB)

Total reclaimable: 3.09 GB
```

---

## 🚀 Features

- ⚡ **Fast scanning**
  - Parallel file processing using `rayon`
- 🧠 **Smart detection**
  - Identifies common dev junk automatically
- 🔒 **Safe by default**
  - Distinguishes user vs system paths
- 🚫 **Noise reduction**
  - Ignores irrelevant directories (`.cache`, `.cargo`, etc.)
- 🧭 **Flexible scope**
  - Scan current directory or entire filesystem

---

## 🛠 Installation

### 🔧 Prerequisites

- Rust toolchain installed  
  Install with:

```bash
curl https://sh.rustup.rs -sSf | sh
rustup update
```

---

### 📦 Build from source

```bash
git clone https://github.com/EnvizyWasTaken/devclean.git
cd devclean
cargo build --release
```

Binary will be available at:

```bash
target/release/devclean
```

---

### 🌍 Install globally

```bash
cargo install --path .
```

Then run:

```bash
devclean
```

---

## 📖 Usage

### 🔍 Scan current directory

```bash
devclean
```

---

### 📂 Scan a specific path

```bash
devclean /home/eve/projects
```

---

### 🌍 Scan entire system

```bash
devclean /
```

> ⚠️ May take longer depending on disk size.

---

## 🧠 How It Works

### 1. Directory Traversal

- Uses `walkdir` to recursively scan directories
- Skips ignored paths early for performance

---

### 2. Junk Detection

Matches directories against known patterns:

```rust
["target", "node_modules", "__pycache__"]
```

---

### 3. Parallel Size Calculation

- Uses `rayon` for multi-threaded file processing
- Computes directory sizes efficiently

---

### 4. Smart Skipping

Once a junk directory is found:

- It is measured once
- Its contents are **not traversed further**

This prevents exponential slowdown from nested structures like:

```
node_modules/node_modules/node_modules/...
```

---

### 5. Safety Classification

Each result is labeled:

- `[USER]` → inside `/home` (safe to clean)
- `[SYSTEM]` → outside `/home` (potentially unsafe)

---

## 🔒 Safety Model

`devclean` is **non-destructive by default**:

- No files are deleted automatically
- System paths are flagged clearly
- You remain in control of all actions

---

## ⚙️ Configuration (Planned)

Future improvements may include:

- `.devclean.toml` config file
- Custom ignore rules
- User-defined junk patterns
- Path whitelisting / blacklisting

---

## 🚀 Roadmap

- [ ] Interactive cleaning (`devclean clean`)
- [ ] Dry-run mode
- [ ] Safe delete (move to trash)
- [ ] Output grouping (Rust / Node / Python)
- [ ] Top-N largest junk view
- [ ] CLI flags (`--min-size`, `--top`, `--user-only`)

---

## 🤝 Contributing

Contributions are welcome.

If you have ideas for:
- performance improvements
- better detection rules
- UX enhancements

Feel free to open an issue or submit a PR.

---

## ⚠️ Disclaimer

`devclean` identifies directories commonly safe to delete, but:

> **Always verify before removing anything.**

Deleting system files may break applications.

---

## 📄 License

MIT License
