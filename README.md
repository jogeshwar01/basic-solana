# Basics of Rust

#### Reference

https://doc.rust-lang.org/book/

### Concepts

1. Functions
2. Types, Flow Control Statements
3. Standard - Vectors, Maps, Option, Result
4. Ownership and Borrowing
5. Structs, impl, Associated Function vs Methods - static / &self
6. Enums, Option, match and if-let
7. Packages, Crates (Library/Binary) and Modules
8. Collections - Vectors, Strings and Hash Maps
9. Error handling - panic, unwrap, expect, ?, parse
10. Generics, Traits (Orphan Rule) and Lifetimes
11. Tests - assert (eq,ne), panic, cargo test commands, unit/integration tests
12. Minigrep - CLI program - fs module, TDD, env vars, eprintln!
13. Closures (Memoization Pattern, Capture Environment), Iterators (Adaptor and Consumer functions)
14. Crates, Release Profiles (dev/release), Crates.io (Doc Comments, pub use export), Cargo workspaces
15. Smart Pointers - Box, Deref, Drop, Reference Counting, Interior Mutability, Reference Cycles (Weak Pointers)
16. Fearless Concurrency - Threads, Message Passing (Channels - mpsc), Sharing state (Mutex, Arc)
17. Object oriented Programming - Objects, Encapsulation, Inheritance, Trait Objects, State Design Pattern
18. Patterns and Matching - Refutable/Irrefutable, match, destructure, ignoring with \_, range, match guards, @ bindings
19. Advanced Features -

- Unsafe Rust - Unsafe Blocks/Fns, Raw Pointers, Extern (no_mangle), Static variables (Global), Union
- Advanced Traits - Associated types, Default Generic Type Parameters and Operator Overloading, Same name trait methods/associated fns, Supertraits, Newtype pattern
- Advanced types - Type Aliases, Never type, Dynamically Sized Types and Sized Trait
- Advanced Functions and Closures - Function Pointers, Returning Closures
- Macros - Declarative and Procedural

20. Project - Web Server

- Single-Threaded HTTP Server - TCP, HTTP, TcpStream, TcpListener, buffer
- Multi-Threaded HTTP Server - Custom Thread Pools, Workers and Jobs
- Graceful Shutdown and Cleanup

### Setup

```
git clone https://github.com/jogeshwar01/rust-basics.git
cd rust-basics
cargo run
```
