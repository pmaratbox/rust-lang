# 0540 — Many (repetition)

Uses the `nom` parser-combinator library. The `many0` combinator repeatedly applies an inner parser (`char('a')`) zero or more times, collecting every match into a `Vec`; running it on the fixed input `"aaaa"` yields four matches, so the length is printed.

## Run

    cargo run
