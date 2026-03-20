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

```rust
["target", "node_modules", "__pycache__"]
```

---

### 3. Parallel Size Calculation
- Uses `rayon` for multi-threaded processing  
- Efficient aggregation of file sizes  

---

### 4. Smart Skipping
- Junk directories are measured once  
- Their contents are not traversed again  

---

### 5. Safety Classification
- `[USER]` → inside `/home`  
- `[SYSTEM]` → outside `/home`  

---

## 🔒 Safety Model

- Non-destructive by default  
- No automatic deletion  
- System paths clearly flagged  

---

## ❤️ Support

If you find this project useful, you can support it with crypto:

- **BTC:** `bc1qv9mgzgrs49jmp6zregkamyzj88s9u8ckkrst5d`  
- **ETH:** `0xc0d962c5F2DD520aA98E917cCaaf0534BA32001f`  

---

## ⚙️ Configuration (Planned)

- `.devclean.toml` config file  
- Custom ignore rules  
- User-defined junk patterns  

---

## 🚀 Roadmap

- [ ] Interactive cleaning (`devclean clean`)
- [ ] Dry-run mode
- [ ] Safe delete (move to trash)
- [ ] Output grouping
- [ ] Top-N largest junk view
- [ ] CLI flags (`--min-size`, `--top`, `--user-only`)

---

## 🤝 Contributing

Contributions are welcome.

---

## ⚠️ Disclaimer

Always verify before deleting anything.  
System file removal may break applications.

---

## 📄 License

MIT License
