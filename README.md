# Minimal Slab Allocator

## Overview
This project implements a **minimal slab allocator** in Rust, using `no_std`.  
It provides fixed-size memory allocation and deallocation using a simple **free list**.

- The allocator is designed for educational purposes, to understand **slab allocation** and **SLUB**.
- Unsafe operations are thoroughly documented.

---

## Project Structure
slab_allocator/
├── Cargo.toml
├── src/
│ ├── lib.rs # crate root, exports slab module
│ └── slab.rs # slab allocator implementation
├── AUTHORS.md
└── README.md


---

## Features
- `SlabAllocator::new(object_size, capacity)` — create a new slab allocator
- `alloc()` — allocate one object
- `free(ptr)` — free a previously allocated object (unsafe)
- All fields and unsafe operations are documented
- Unit tests for allocation, deallocation, and skeleton

---

## Usage

```rust
use slab_allocator::slab::SlabAllocator;
use core::ptr::NonNull;

let mut slab = SlabAllocator::new(32, 4);

// allocate an object
let ptr: Option<NonNull<u8>> = slab.alloc();

// free the object
if let Some(p) = ptr {
    unsafe { slab.free(p) };
}
```
---

## Build and Test

```bash
cargo check     # verify compilation
cargo test      # run all unit tests
```
---

## Credits
- The implementation was inspired by [Linux Kernel SLUB allocator](https://www.kernel.org/doc/html/latest/core-api/slub.html)
- [64bit os tutorial](https://github.com/gmarino2048/64bit-os-tutorial)
- [Learning rust with entirely too many linked list](https://rust-unofficial.github.io/too-many-lists/)
- [Kernel exploitation - Slab Allocators](https://www.youtube.com/watch?v=4UucTCiQLXE)
- ChatGPT was used as an assistance tool for explanations, guidance, and documentation writing