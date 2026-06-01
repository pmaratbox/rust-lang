# 0032 — Math Functions

Take the square root of `16`, raise `2` to the 10th power, the absolute value of `-5`, and the larger of `3` and `9`, printing `sqrt: 4`, `pow: 1024`, `abs: 5`, and `max: 9`. Numeric methods hang off the types themselves: `f64::sqrt`, `i32::pow` (an exact integer power), `i32::abs`, and `Ord::max`. `sqrt` returns an `f64`, cast to `i32` with `as`.

## Run

    cargo run
