# 0116 — Function Pipeline

Compose inc, double, and negate into a left-to-right pipeline and apply it to 3: ((3+1)*2) negated = `-8`. The pipeline `fold`s the input through a `Vec<Box<dyn Fn(i32) -> i32>>`, applying each boxed closure in order.

## Run

    cargo run
