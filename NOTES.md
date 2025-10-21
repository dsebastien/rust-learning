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

### Dependencies
Add a `[dependencies] section to `Cargo.toml`.

Each dependency version is set like this:

```
<name> = "<version>"
```

Dependencies are installed using `cargo update`, which is also done when using `cargo build`.

Where version may be `x.y.z` (semver). When installing dependencies, cargo will use the most recent compatible version and add a `Cargo.lock` file for build reproductibility.


## Rust

### Main function
```
fn main() {
    ...
}
```

### Variables

Declare a variable:
```
let foo: &str = "Foo";
let apples = 5; // This is a comment
```

Variables are immutable by default.

Declare a mutable variable:

```
let mut change_me = 5;
```

Create an empty string:

```
let mut my_name: String = String::new();
```

### Print a string
Done using the `println` macro (NOT a function).

Print a variable:

```
println!("{}", foo);
```

Print a variable and the result of an expression:

```
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

### Prelude
A prelude is a collection of imports.
The ´std::prelude` is the standard prelude, which is automatically available to ALL Rust programs.

Certain modules may provide their own preludes (e.g., `std::io::prelude`), but those are NOT automatically loaded/available. They have to be imported explicitly. Those act a bit like `*` imports for Java packages/libraries.

Reference: https://doc.rust-lang.org/std/prelude/index.html

### Imports
To import a "module":

```
use std::io;
...
io::stdin().read_line(&mut guess);
```

Just like in Java, imports are not mandatory. It's possible to use functions etc in modules by prefixing those with the complete "path":

```
std::io::stdin().read_line(&mut guess);
```

### Random numbers

Use the `rand` library.

Generate a random number: `let secret_number = rand::thread_rng().gen_range(1..=100);`
