# WASM vs JavaScript – real-world performance benchmarks

This repository contains a set of **hands-on, browser-based benchmarks** comparing
**pure JavaScript** with **Rust compiled to WebAssembly (WASM)**.

It contains **real workloads**:
heavy numeric loops, and CPU-intensive image processing pipelines.

Live demo:  
👉 https://sylwia-lask.github.io/wasm-js-bench/

---

## Why this project exists

JavaScript engines have become *very* fast thanks to JIT optimizations.
At the same time, WebAssembly promises near-native performance in the browser.

So the real question is:

> When does WASM actually outperform JavaScript?  
> And when does it not?

This project explores that boundary using:
- identical algorithms,
- identical data,
- the same browser,
- and no cheating (no BigInt-only tricks, no artificial slowdowns).

---

## What is benchmarked?

### 1. Numeric workloads

**Matrix-style O(n³) computation**

- Deeply nested loops
- Integer-heavy arithmetic with modulo
- Very friendly to JS JIT

Result:  
➡️ JavaScript is often *competitive*, sometimes even faster.

---

### 2. Factorial-style modulo arithmetic

Inspired by classic competitive programming tasks like:

```

n! mod 1_000_000_007

```

- Tight integer loops
- No memory allocations
- Deterministic cost per iteration

Result:  
➡️ WASM clearly wins as `n` grows.

---

### 3. Heavy image processing pipeline (the fun one)

A real CPU-heavy pipeline applied to full-resolution images:

```

box blur
→ gaussian blur
→ grayscale
→ downscale
→ upscale
→ wave distortion
→ repeated Sobel edge detection
→ nonlinear edge glow
→ intensity hash mixing (modulo arithmetic)
→ final blending

```

Characteristics:
- Millions of pixel operations
- Branches + math + memory access
- Same algorithm implemented in JS and Rust

Result:  
➡️ **WASM consistently crushes JS** for large images.

---

## Project structure

```

WASM-JS-BENCH/
├── frontend/          # Svelte + Vite app (UI & benchmarks)
│   ├── src/
│   │   ├── logic/     # JavaScript implementations
│   │   ├── wasm/      # wasm-bindgen output (committed artifacts)
│   │   └── components
│   └── vite.config.ts
│
├── rust-wasm/         # Rust source code
│   ├── src/
│   │   ├── numeric.rs
│   │   ├── image_pipeline.rs
│   │   └── lib.rs
│   └── Cargo.toml
│
└── .github/workflows/ # GitHub Pages deployment

````

---

## Tech stack

- **Frontend:** Svelte + TypeScript + Vite
- **WASM:** Rust + wasm-bindgen
- **Deployment:** GitHub Pages via GitHub Actions
- **Target environment:** modern browsers (Chrome, Firefox)

---

## Notes on benchmarking

- All measurements happen **in the same browser tab**
- No workers, no offloading
- Timings use `performance.now()`
- Results depend on:
  - CPU
  - browser engine
  - image resolution

Because of this:
> **Absolute numbers vary — relative trends matter.**

---

## Key takeaways

- JavaScript is *excellent* for many numeric workloads  
- WASM shines when:
  - computation is very heavy
  - loops dominate
  - memory access patterns are stable
- Image processing is a **sweet spot for WASM**
- WASM is not a magic bullet — but when it hits, it really hits

---

## Running locally

```bash
git clone https://github.com/sylwia-lask/wasm-js-bench.git
cd wasm-js-bench/frontend
npm install
npm run dev
````

---

## Building WASM

From `rust-wasm/`:

```bash
wasm-pack build --release --target web --out-dir ../frontend/src/wasm
```

The generated artifacts are committed so CI does not need Rust.

---

## License

MIT
Do whatever you want — experiments welcome 🙂

---
