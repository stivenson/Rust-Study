# Rust Study Repository

A comprehensive collection of Rust examples covering fundamental concepts to advanced patterns. This repository has been modernized to use Rust 2021 edition with current best practices and idioms.

## 🚀 Getting Started

### Prerequisites

- **Rust**: Version 1.75 or later (2021 edition)
- Install via [rustup](https://rustup.rs/): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Running Examples

Navigate to any example directory and run:

```bash
cd <example-folder>
cargo run
```

To build without running:
```bash
cargo build
```

---

## 📚 Learning Path

This repository is organized as a progressive learning curriculum. Follow the sections in order for the best learning experience.

### 🟢 Level 1: Fundamentals

#### 1. **hello_cargo**
Your first Rust program using Cargo.
- **Concepts**: Project structure, Cargo basics
- **Key Files**: `Cargo.toml`, `src/main.rs`
- **Run**: `cd hello_cargo && cargo run`

#### 2. **constants**
Understanding constants and their differences from variables.
- **Concepts**: `const` vs `let`, type annotations, numeric separators
- **Learn**: Constants must have explicit types and are immutable
- **Example**: `const MAX_POINTS: u32 = 100_000;`

#### 3. **data-types**
Comprehensive overview of Rust's type system.
- **Concepts**: Integers (u8, i32, u64, etc.), floats, characters, strings
- **Learn**: Type inference, explicit type annotations, literals
- **Example**: Parsing strings to numbers, working with different numeric types

#### 4. **basic-mutations**
Introduction to mutability in Rust.
- **Concepts**: `mut` keyword, mutable references
- **Learn**: Variables are immutable by default
- **Example**: Modifying values through mutable references

#### 5. **arrays-vectors**
Fixed-size arrays vs dynamic vectors.
- **Concepts**: Arrays `[T; N]`, Vectors `Vec<T>`
- **Learn**: Arrays are stack-allocated, vectors grow dynamically
- **Example**: Array initialization, vector operations (push, indexing)

---

### 🟡 Level 2: Ownership & Functions

#### 6. **mutations-functions**
Passing data between functions: by value, reference, or mutable reference.
- **Concepts**: Borrowing, ownership transfer, `&str` vs `&String`
- **Learn**: Ownership moves, borrowing rules
- **Example**: Functions that borrow vs consume values

#### 7. **match-some-err-options-results**
Error handling and optional values the Rust way.
- **Concepts**: `Option<T>`, `Result<T, E>`, pattern matching
- **Learn**: `match`, `if let`, `unwrap()`, `unwrap_or()`
- **Example**: Handling `None` cases, error propagation with `?`

---

### 🟠 Level 3: Intermediate Concepts

#### 8. **trait-struct-impl**
Defining types and behaviors with structs and traits.
- **Concepts**: Structs, traits (interfaces), implementations
- **Learn**: Methods vs associated functions, trait bounds
- **Example**: `Person` trait implemented for `Man` struct

#### 9. **implementables**
Implementing standard library traits for custom types.
- **Concepts**: Operator overloading with `AddAssign`, `DivAssign`
- **Learn**: How to make custom types work with operators like `+=`
- **Example**: Custom `Count` type with arithmetic operations

#### 10. **dereference**
Understanding the dereference operator and smart pointers.
- **Concepts**: `Deref`, `DerefMut` traits, the `*` operator
- **Learn**: Deref coercion, mutable dereferencing
- **Example**: Creating smart pointer-like types

#### 11. **visitor-pattern**
Design patterns in Rust: the Visitor pattern.
- **Concepts**: Design patterns, trait objects (`dyn Trait`)
- **Learn**: Separating data structures from operations
- **Example**: Animal structs with pluggable sound behaviors

---

### 🔴 Level 4: Advanced Features

#### 12. **closures-into-traits**
Closures as function arguments with generic trait bounds.
- **Concepts**: Closures, `Fn` trait, generics, caching/memoization
- **Learn**: Using closures for expensive operations, result caching
- **Example**: Search cacher with closure-based file operations

#### 13. **cashier-macro**
Declarative macros for code generation.
- **Concepts**: `macro_rules!`, repetition patterns, compile-time code generation
- **Learn**: When and how to write macros
- **Example**: Cashier calculator macro with variable arguments

---

### 🚀 Level 5: Web Development

#### 14. **parking-api**
RESTful API using Rocket web framework (v0.5, async/await).
- **Concepts**: HTTP routes, JSON serialization, async handlers
- **Learn**: Building REST APIs with Rocket
- **Example**: CRUD operations for parking management
- **Run**: `cd parking-api && cargo run`, then visit `http://127.0.0.1:8000`

#### 15. **graphql-juniper-mongo**
GraphQL API with MongoDB database integration (modern async).
- **Concepts**: GraphQL, async/await, database operations, error handling
- **Learn**: Modern async Rust, MongoDB driver, Juniper GraphQL
- **Dependencies**: MongoDB running on `localhost:27017`
- **Example**: Product CRUD via GraphQL
- **Run**: `cd graphql-juniper-mongo && cargo run`, then visit `http://127.0.0.1:3030`

---

## 🛠️ Key Rust Concepts Covered

| Concept | Examples |
|---------|----------|
| **Ownership & Borrowing** | mutations-functions, basic-mutations |
| **Error Handling** | match-some-err-options-results |
| **Traits & Generics** | trait-struct-impl, closures-into-traits |
| **Pattern Matching** | match-some-err-options-results, visitor-pattern |
| **Macros** | cashier-macro |
| **Async Programming** | parking-api, graphql-juniper-mongo |
| **Web Development** | parking-api, graphql-juniper-mongo |

---

## 📖 Additional Resources

- **The Rust Book**: [doc.rust-lang.org/book](https://doc.rust-lang.org/book/)
- **Rust by Example**: [doc.rust-lang.org/rust-by-example](https://doc.rust-lang.org/rust-by-example/)
- **Rust Standard Library**: [doc.rust-lang.org/std](https://doc.rust-lang.org/std/)
- **Cargo Book**: [doc.rust-lang.org/cargo](https://doc.rust-lang.org/cargo/)

---

## 🔧 Modern Updates (2024)

This repository was originally created in 2019 (Rust 2018 edition) and has been modernized:

- ✅ Updated to **Rust 2021 edition**
- ✅ Removed deprecated `extern crate` declarations
- ✅ Applied modern idioms and best practices
- ✅ Updated dependencies to latest stable versions
- ✅ Replaced `failure` with `anyhow`/`thiserror`
- ✅ Modernized web frameworks (Rocket 0.5, async MongoDB driver)
- ✅ Added comprehensive documentation

---

## 🤝 Contributing

Feel free to open issues or pull requests to improve examples or add new ones!

## 📝 License

Educational purposes - feel free to use and modify.

---

**Happy Learning! 🦀**
