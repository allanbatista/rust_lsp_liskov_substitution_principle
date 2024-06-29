# LSP (Liskov Substitution Principle) in Rust

This project demonstrates the Liskov Substitution Principle (LSP) in Rust. It contains two binaries: `lsp` and `not_lsp`.

## LSP

The `lsp` binary demonstrates the correct application of the LSP. It defines `IEmployee` and `IManager` traits and two structs (`Employee` and `Manager`) that implement these traits. The `main` function uses these traits and structs to demonstrate that an instance of a superclass (IEmployee) can be replaced with an instance of a subclass (Employee) without affecting the correctness of the program.

You can run this binary with `cargo run --bin lsp`.

## Not LSP

The `not_lsp` binary demonstrates a violation of the LSP. It defines an `Employee` struct with an `employee_type` field. The `add_employee`, `get_employees`, and `get_team_size` methods use a `match` statement to determine the behavior based on the `employee_type`.

This design violates the LSP because it requires type checking and different behavior based on the type.

You can run this binary with `cargo run --bin not_lsp`.

## Running the Project

To run the project, you will need Rust and Cargo installed on your machine. You can run the binaries with the following commands:

```bash
cargo run --bin lsp
cargo run --bin not_lsp