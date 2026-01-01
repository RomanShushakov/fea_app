# fea_app

![Rust](https://img.shields.io/badge/Rust-stable-orange)
![WebAssembly](https://img.shields.io/badge/WebAssembly-WASM-blue)
![WebGPU](https://img.shields.io/badge/WebGPU-compute-yellow)

`fea_app` is an experimental Web-based finite element and numerical computation project, focused on implementing and understanding **iterative solvers and preconditioning techniques on the GPU using WebGPU**.

The project combines:

- numerical linear algebra (PCG, Block-Jacobi),
- sparse matrix representations (CSR),
- and GPU compute pipelines,
  with the goal of exploring how **scientific and engineering-style workloads can be executed efficiently in modern web environments**.

This repository represents a **completed WebGPU-focused milestone** rather than a full production-ready solver.

---

## Motivation

My background is in engineering and numerical methods, where I worked extensively with FEA tools (e.g. structural and stress analysis software).  
Later, I transitioned into software engineering and full-stack development.

This project grew from a personal interest in:

- understanding how FEA solvers work internally,
- bridging numerical methods with GPU computing,
- and exploring the web as a viable platform for computation-heavy workloads.

`fea_app` is therefore both:

- a learning-oriented project, and
- a hands-on engineering exploration.

---

## What is implemented

### Numerical core

- **Preconditioned Conjugate Gradient (PCG)** solver
- **Block-Jacobi preconditioner** with fixed block size (6×6)
- Sparse matrices stored in **CSR format**
- CPU-side reference implementation for validation
- **Small dense direct solver (LU-based) used as an intermediate reference**

The direct solver was used during early stages as:

- a correctness reference for validating iterative results,
- a way to test block extraction and factorization logic,
- and a baseline before moving fully to the iterative GPU-based approach.

While not intended for large systems, this step helped ensure numerical correctness and consistency across CPU and GPU implementations.

### GPU (WebGPU)

- Sparse matrix–vector multiplication (SpMV)
- Dot products with parallel reduction
- Vector operations (AXPY, scaling)
- Block-Jacobi preconditioner application on GPU
- Scalar reductions and solver control logic coordinated from the GPU

The solver is structured to minimize GPU–CPU synchronization and to batch most work into a **single command submission per iteration**.

---

## Architecture overview

- **Rust + wasm-bindgen** for core logic
- **WebGPU (via web-sys)** for GPU compute
- Dedicated GPU executors for:
  - SpMV
  - dot products
  - vector operations
  - scalar updates
  - block-Jacobi preconditioning

The code is intentionally verbose and explicit, prioritizing clarity and correctness over aggressive micro-optimizations.

---

## Scope and non-goals

This repository is **not**:

- a full FEA framework,
- a polished visualization tool,
- or a production-ready solver.

Instead, it focuses on:

- correctness,
- architectural clarity,
- and understanding GPU-based numerical workflows in the browser.

Further work (e.g. native `wgpu` backends, CLI tools, larger-scale benchmarks) is intentionally left out of this repository.

---

## Status

- WebGPU solver pipeline: **complete**
- CPU ↔ GPU validation: **complete**
- Performance tuning: **basic, exploratory**
- Documentation: **engineering-focused**

---

## License

MIT
