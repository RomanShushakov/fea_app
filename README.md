# FEA App — WebGPU & WASM Finite Element Solver Showcase

This repository is a technical showcase of a browser-based **Finite Element Analysis (FEA)** application focused on high-performance numerical computing in the modern web stack.

The project demonstrates how **WebAssembly (Rust)** and **WebGPU** can be combined to implement iterative solvers, sparse linear algebra, and preconditioners in a frontend environment, while keeping the UI responsive and the architecture clean.

This is **not a toy demo** — the code mirrors real solver pipelines and GPU execution models commonly used in HPC and scientific computing.

---

## What This Project Demonstrates

- Rust → WebAssembly for numerical kernels and domain logic  
- WebGPU compute pipelines for iterative solvers and vector operations  
- Sparse matrix algorithms implemented for GPU execution  
- Block-Jacobi preconditioning with LU-factored dense blocks  
- Single-submit GPU iteration design (minimized CPU ↔ GPU synchronization)  
- Worker-based execution model for non-blocking UI  
- Clean separation between frontend orchestration and compute backends  

---

## Architecture Overview

The application follows a layered design similar to desktop FEA software, adapted to the browser:

UI (Web Components)
↓
JS Orchestration Layer
↓
Web Workers (Solver Execution)
↓
Rust → WebAssembly Modules
↓
WebGPU / WebGL


---

## Key Components

### Frontend (JavaScript)

- Framework-free UI built with custom Web Components  
- Explicit application state management  
- Clear separation between UI, data, and compute orchestration  
- Communication with solver via Web Workers  

**Relevant directories:**

- `frontend/fea_service/components`
- `frontend/fea_service/store`
- `frontend/fea_service/workers`

---

### WASM Modules (Rust)

Each major subsystem is implemented as a separate Rust crate, compiled to WebAssembly:

- **Preprocessor** — domain model, constraints, materials  
- **Mesher** — mesh preparation  
- **Renderer** — data preparation for WebGL  
- **Solver** — iterative methods and linear algebra  
- **Actions Router** — typed command dispatch  

All crates are managed in a Cargo workspace and built with a dedicated script.

---

### GPU Compute (WebGPU)

Performance-critical operations are offloaded to the GPU using WebGPU compute shaders:

- Sparse Matrix–Vector Multiplication (CSR SpMV)  
- Dot products with multi-stage reductions  
- Vector operations (AXPY, scaling)  
- Block-Jacobi preconditioner (fixed 6×6 dense blocks)  
- Scalar update kernels for PCG (α, β)  

**Design characteristics:**

- One command submission per solver iteration  
- One scalar readback per iteration  
- Explicit memory ownership and buffer lifetimes  
- WGSL shaders written in a deterministic, debuggable style  

This part of the codebase closely resembles HPC GPU kernels, adapted to WebGPU constraints.

---

## Solver Details

- **Algorithm:** Preconditioned Conjugate Gradient (PCG)  
- **Matrix format:** CSR  
- **Preconditioner:** Block-Jacobi  
- **Fixed block size:** 6×6  
- **LU factorization:** performed on CPU  
- **Apply step:** executed entirely on GPU  
- **Convergence checks:** CPU-side using squared norms  

The solver is numerically consistent with its CPU reference implementation and is structured for further backend reuse.

---

## Execution Model

- All heavy computation runs inside Web Workers  
- GPU command encoding is performed inside the worker  
- UI thread remains fully responsive  
- CPU ↔ GPU synchronization is minimized and explicit  

---

## Tooling & Stack

- **Languages:** Rust, JavaScript, WGSL  
- **Compute:** WebAssembly, WebGPU  
- **Rendering:** WebGL  
- **Concurrency:** Web Workers  
- **Build:** Vite, Cargo  
- **Infrastructure:** Docker, Nginx (optional Traefik setup)  

---

## Why This Repo Exists

This repository serves as a focused technical portfolio showcasing:

- Advanced use of WebGPU beyond rendering  
- Practical numerical computing in the browser  
- Clean Rust ↔ JS ↔ GPU integration  
- Solver-oriented thinking applied to frontend technology  

It intentionally prioritizes **clarity, correctness, and architecture** over UI polish.
