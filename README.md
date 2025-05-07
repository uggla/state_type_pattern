# 🦀 Type-State Pattern in Rust

This repository demonstrates the use of the **type-state pattern** in Rust
— a design pattern where an object’s state is encoded in its type. This
allows the compiler to enforce valid state transitions at compile time,
reducing bugs and improving correctness.

## 🎯 Main Goal

The project shows how to model different object states using distinct types
and make state transitions explicit through methods. For example, a method
like `send()` can only be called if the object is in a `Connected` state,
and attempting to call it while `Disconnected` simply won’t compile.

## 📚 Additional Learning Points

- ✅ **How to import and organize modules**  
  The code is structured across multiple files and modules. It’s a practical
  example of idiomatic module management and importing in Rust, with clear
  separation of concerns.

- ✅ **How to write functional tests**  
  Beyond unit tests, the repository includes **functional tests** under the
  `tests/` directory. These test higher-level behaviors and validate that
  state transitions work as intended across components.

## ✅ Why Use This Project

This repository is a useful reference for Rust developers who want to:

- Understand and apply the type-state pattern
- Learn proper project structure with multiple modules
- Write functional and integration-style tests
- Leverage Rust’s type system to enforce business rules at compile time

---

> This is a minimal but complete example of safe state management in Rust
> using types. Feel free to use it as a starting point for your own stateful
> abstractions.
