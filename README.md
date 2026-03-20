# 🧹 devclean

> A fast, intelligent CLI tool for scanning and cleaning development junk across your system.

`devclean` helps you reclaim disk space by identifying build artifacts, dependency folders, and cache directories commonly generated during development workflows.

---

## ✨ Features

- ⚡ **Fast scanning** with parallel filesystem traversal
- 🧠 **Smart filtering** to reduce noise and highlight meaningful results
- 🔒 **Safe-by-default** (distinguishes user vs system paths)
- 📦 Supports common dev junk:
  - Rust (`target/`)
  - Node.js (`node_modules/`)
  - Python (`__pycache__/`)
- 🚫 Ignores irrelevant/system-heavy directories automatically
- 🧭 Works across entire filesystems (`/`) or scoped paths

---

## 📸 Example

```bash
$ devclean /
```

```text
[USER]   /home/user/project/target (819 MB)
[USER]   /home/user/api/node_modules (320 MB)
[SYSTEM] /usr/lib/node_modules (40 MB)

Total reclaimable: 3.09 GB
```

---

## 🛠 Installation

### 🔧 Prerequisites

- Rust toolchain installed  
  Install via:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Then:

```bash
rustup update
```

---

### 📦 Install from source

```bash
git clone https://github.com/YOUR_USERNAME/devclean.git
cd devclean
cargo build --release
```

Binary will be located at:

```bash
target/release/devclean
```

---

### 🚀 Optional: Install globally

```bash
cargo install --path .
```

Then run anywhere:

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

### 📂 Scan specific path

```bash
devclean /home/user/projects
```

---

### 🌍 Scan entire system (read-only)

```bash
devclean /
```

> ⚠️ This may take longer depending on system size.

---

## 🧠 How It Works

`devclean` performs a recursive filesystem scan using `walkdir`, applying several layers of filtering and optimization:

---

### 1. Directory Traversal

- Recursively walks directories using a streaming iterator
- Skips ignored directories early (`.cache`, `.cargo`, etc.)
- Avoids unnecessary recursion into irrelevant paths

---

### 2. Junk Detection

Each directory is checked against a predefined rule set:

```rust
["node_modules", "target", "__pycache__"]
```

If a match is found:
- It is classified as “junk”
- Its total size is calculated

---

### 3. Parallel Size Calculation

Directory size is computed using parallel iteration:

- Uses `rayon` for multi-threaded file processing
- Aggregates file sizes efficiently
- Significantly faster than sequential scanning

---

### 4. Smart Skipping

Once a junk directory is found:

- It is measured once
- Its contents are **not traversed further**

This avoids exponential slowdown from nested directories like:

```
node_modules/node_modules/node_modules/...
```

---

### 5. Safety Classification

Each result is classified:

- `[USER]` → inside `/home` (safe to clean)
- `[SYSTEM]` → outside `/home` (potentially unsafe)

This ensures:

- visibility of system junk
- prevention of accidental damage

---

## 🔒 Safety Model

By default, `devclean` is **non-destructive**:

- It only scans and reports
- No files are deleted automatically

System paths are flagged but not filtered unless explicitly handled.

---

## ⚙️ Configuration (Planned)

Future versions may support:

- Custom ignore rules (`.devclean.toml`)
- User-defined junk patterns
- Path whitelisting / blacklisting

---

## 🚀 Roadmap

- [ ] Interactive deletion (`devclean clean`)
- [ ] Dry-run mode
- [ ] Trash support (safe delete instead of permanent)
- [ ] Output grouping (by type)
- [ ] Top-N largest junk view
- [ ] Config file support

---

## 🤝 Contributing

Contributions are welcome.

If you have ideas for:
- new junk detection rules
- performance improvements
- UX enhancements

Open an issue or PR.

---

## ⚠️ Disclaimer

`devclean` identifies directories commonly considered safe to delete, but:

> **You are responsible for what you remove.**

Always verify before deleting anything, especially outside your home directory.

---

## 📄 License

MIT License
