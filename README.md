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

## Example

Allocating past capacity returns `None` instead of panicking or reading out
of bounds, and freed slots are reused by later allocations:

```rust
use slab_allocator::slab::SlabAllocator;

let mut slab = SlabAllocator::new(32, 2);

let a = slab.alloc(); // Some(..)
let b = slab.alloc(); // Some(..)
let c = slab.alloc(); // None, pool exhausted (capacity = 2)

assert!(a.is_some() && b.is_some());
assert!(c.is_none());

unsafe { slab.free(a.unwrap()) };

let d = slab.alloc(); // Some(..), reuses the slot just freed
assert!(d.is_some());
```

---

## Build and Test

```bash
cargo check     # verify compilation
cargo test      # run all unit tests
```

Output on the current test suite:

```
running 4 tests
test slab::tests::alloc_test ... ok
test slab::tests::zero_capacity_test ... ok
test slab::tests::skeleton_test ... ok
test slab::tests::free_test ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

A GitHub Actions workflow (`.github/workflows/ci.yml`) runs `cargo build` and
`cargo test` on every push and pull request.

---

## Credits
- The implementation was inspired by [Linux Kernel SLUB allocator](https://www.kernel.org/doc/html/latest/core-api/slub.html)
- [64bit os tutorial](https://github.com/gmarino2048/64bit-os-tutorial)
- [Learning rust with entirely too many linked list](https://rust-unofficial.github.io/too-many-lists/)
- [Kernel exploitation - Slab Allocators](https://www.youtube.com/watch?v=4UucTCiQLXE)
- ChatGPT was used as an assistance tool for explanations, guidance, and documentation writing