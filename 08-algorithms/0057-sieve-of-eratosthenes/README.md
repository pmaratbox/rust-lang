# 0057 — Sieve of Eratosthenes

Use the Sieve of Eratosthenes to find every prime number up to `10` and print them: `2 3 5 7`. `vec![true; n + 1]` is the flag vector; the inner `while` strikes multiples from `i*i`, and a `(2..=n)` iterator with `filter`/`map` collects the primes.

## Run

    cargo run
