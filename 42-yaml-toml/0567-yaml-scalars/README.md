# 0567 — YAML scalars

Use the `serde_yaml` crate to parse the fixed YAML mapping
`name: Alice\nrole: admin\nage: 30\n` into a `BTreeMap<String, serde_yaml::Value>`.
The string scalars are read with `as_str` and the integer `age` with `as_i64`,
then the three values are printed space-joined: `Alice admin 30`.

## Run

    cargo run
