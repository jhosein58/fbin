# fbin 🦀💾

**Low-level binary I/O toolkit written in Rust.**  
Precise. Predictable. Bit-perfect.

---

## 🚀 About

`fbin` is a zero-dependency binary file library written in pure Rust, designed for developers who need **fine-grained control** over file I/O — from writing full `u32` values to flipping individual bits at arbitrary offsets.

Perfect for building your own:

- 📁 Custom image/audio formats
- 🧠 File-based data encoders
- 🎛️ Wave generators, synthesizers, and bit-stream tools
- 🔐 Raw binary manipulators

Inspired by the Nebula ecosystem (PHP), this is a **next-generation engine** for systems that demand clarity, precision, and no magic.

---

## 🔧 Features

- ✅ Write/read binary values (`u8`, `i16`, `i32`, etc.)
- ✅ Seek by byte or **bit-level precision**
- ✅ Modify individual bits with full control
- ✅ Modular API for extensions (e.g. wave writer, image encoder)
- ✅ Pure Rust, no external crates

---

## 📦 Getting Started

```bash
cargo add fbin
