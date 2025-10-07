# 🌵 rust-lab

> A quiet forge where tools are made by hand — not for scale, but for understanding.

Rust-lab is a **personal laboratory of small, sharp utilities** — each one built to teach fluency through practice.  
The goal isn’t another package on crates.io. It’s *craftsmanship under pressure.*

---

## 🧭 Philosophy

Code is a mirror.  
Rust-lab exists to refine the eye that looks into it.

Each binary in this workspace is intentionally small — clear enough to study, strong enough to use.  
Every warning matters. Every import has a reason.  
The purpose is *not completion*, but *competence*.

---

## 🧱 Projects

| Tool | Purpose | Lesson |
|------|----------|--------|
| **`parhash`** | Parallel BLAKE3 directory hasher | Threading, `rayon`, CLI ergonomics |
| **`lines`** | Count lines of code by walking the filesystem | `walkdir`, iterators, error handling |
| **`assetsize`** | Human-readable file size report | Structuring output, option parsing |

---

## ⚙️ Build

```bash
cargo build --release
