# 0023 — Modules & Imports

Define `square(n)` in a separate `mathutil` module and import it from the main program, printing `square(8) = 64` across the module boundary. `mod mathutil;` pulls in `src/mathutil.rs` as a submodule. Items are private by default, so `square` is marked `pub` to be callable from `main`; the path `mathutil::square` names it, or a `use` could bring it into scope.

## Run

    cargo run
