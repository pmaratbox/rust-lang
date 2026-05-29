# rust-lang

Incremental lessons learning Rust.

## Prerequisites

- Rust toolchain (`rustc` + `cargo`)

### Install

The official installer is [rustup](https://rustup.rs):

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

macOS via Homebrew (alternative):

    brew install rustup-init && rustup-init

### Verify

    rustc --version && cargo --version

## Lessons

- [0001-hello-world](0001-hello-world/) — print "Hello, world!"
- [0002-variables-and-types](0002-variables-and-types/) — declare and print an int, float, string, and bool
- [0003-arithmetic-and-operators](0003-arithmetic-and-operators/) — sum, difference, product, quotient, modulo of two integers
- [0004-conditionals](0004-conditionals/) — compare to 10 and print less / equal / greater
- [0005-loops](0005-loops/) — print 1..5 with a for-loop
- [0006-functions](0006-functions/) — define add(a, b), call it, print the result
- [0007-collections](0007-collections/) — build a list of 1..5; print its count, first, and last
- [0008-strings](0008-strings/) — greet, uppercase, and print the length of a string
- [0009-maps](0009-maps/) — build a map, look up a key, print the value and size
- [0010-structs](0010-structs/) — define a Person type with name and age, print its fields
- [0011-error-handling](0011-error-handling/) — divide with a divide-by-zero guard; print result or error
- [0012-enums](0012-enums/) — define a Color enum; print the integer position of two variants
- [0013-optional](0013-optional/) — hold a present and an absent value; print each with a fallback

## How to run

See each lesson's `README.md` for the exact command.
