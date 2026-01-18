# SLUB Allocator Write-up

## 1. Slab Allocator Overview
A **slab allocator** is a memory management mechanism that allocates memory in **fixed-size chunks** called *objects*.  
- Memory is organized into **slabs**, which are collections of objects.  
- Each slab typically contains objects of the **same type or size**, which reduces fragmentation and improves cache performance.  
- A **free list** tracks available objects within each slab.  
- Slab allocators are widely used in **operating systems** for efficient management of frequently allocated kernel objects.

Key benefits:
- Fast allocation and deallocation
- Low fragmentation
- Predictable memory layout

---

## 2. SLUB Allocator
**SLUB** (the “SLAB allocator, unqueued”) is the Linux kernel’s **default slab allocator**.  
- It simplifies the original SLAB design by **removing queues** for partial slabs.  
- It maintains **per-CPU caches** to minimize locking and improve concurrency.  
- SLUB uses **metadata embedded directly in slabs** to track free objects, reducing overhead.

Key SLUB components:
- `kmem_cache` — represents a cache of objects of the same type
- `slab` — a contiguous memory block containing multiple objects
- Free list — a linked list of free objects inside each slab
- Per-CPU caches — allow local allocation without locking

---

## 3. Free List Concept
The **free list** is a central concept in slab allocators:
- It stores the indices or pointers of **available objects** within a slab
- Allocation: pop an index/pointer from the free list
- Deallocation: push the index/pointer back to the free list
- This structure allows **constant-time allocation and free**, which is crucial for kernel performance

---

## 4. Common Vulnerabilities
Slab allocators are generally safe, but improper use can introduce vulnerabilities:

### 4.1 Use-After-Free
- Accessing an object after it has been freed can lead to **undefined behavior** or security issues.
- In kernel exploit development, use-after-free can allow **arbitrary memory read/write**.

### 4.2 Double Free
- Freeing the same object twice corrupts the free list.
- Can lead to **memory corruption**, crashes, or exploit primitives.

### 4.3 Out-of-Bounds Access
- Allocating or freeing pointers **not from the slab** can cause undefined behavior.
- Proper tracking and `unsafe` documentation are required in Rust or kernel code.

---

## 5. Summary
- A slab allocator efficiently manages **fixed-size objects** using slabs and free lists.
- SLUB is Linux’s simplified slab allocator that is **concurrent-friendly** and reduces metadata overhead.
- Understanding slab and SLUB internals is critical for:
  - Kernel programming
  - Memory management
  - Security/exploit exercises

---

**References**
- [Linux Kernel SLUB Allocator Documentation](https://www.kernel.org/doc/html/latest/core-api/slub.html)
- [Slab Allocator Concepts](https://lwn.net/Articles/175199/)
- [Slab allocator resources lwn](https://www.kernel.org/doc/gorman/html/understand/understand011.html)
- [The slab allocator, an object caching kernel memory allocator](https://people.eecs.berkeley.edu/~kubitron/courses/cs194-24-S13/hand-outs/bonwick_slab.pdf)
- [Kernel exploitation - Slab Allocators](https://www.youtube.com/watch?v=4UucTCiQLXE)

