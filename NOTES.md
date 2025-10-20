# Rust Learning Notes

## Cargo
Build and dependency management tool.

`cargo ...´

### Commands
- `cargo build`: build a debug executable
- `cargo build --release`: build a release executable
- ´cargo new foo`: create a new project with the standard structure

### Standard structure

```
src/
    main.rs <-- Hello world program
target/     <-- Build output
Cargo.toml
```

### Configuration
Configured in `Cargo.toml`

## Rust

### Main function
```
fn main() {
    ...
}
```

### Declare a variable
```
let foo: &str = "Foo";
```

### Print a string
Done using the `println` macro (NOT a function).

Print a variable:

```
println!("{}", foo);
```

### Prelude
A prelude is a collection of imports.
The ´std::prelude` is the standard prelude, which is automatically available to ALL Rust programs.

Certain modules may provide their own preludes (e.g., `std::io::prelude`), but those are NOT automatically loaded/available. They have to be imported explicitly. Those act a bit like `*` imports for Java packages/libraries.

Reference: https://doc.rust-lang.org/std/prelude/index.html

