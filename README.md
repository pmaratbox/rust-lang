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
- [0014-higher-order-functions](0014-higher-order-functions/) — pass functions as values: apply inc and double via apply(f, x)
- [0015-interfaces](0015-interfaces/) — define a Shape interface implemented by two types; call area() polymorphically
- [0016-generics](0016-generics/) — define a generic first(list) returning the first element; call it on ints and strings
- [0017-iterators](0017-iterators/) — filter evens, double them, and sum 1..5 with map/filter/reduce
- [0018-closures](0018-closures/) — return a counter function that captures private state; call it twice for 1 then 2
- [0019-recursion](0019-recursion/) — define a recursive factorial(n); print factorial(5) = 120
- [0020-pattern-matching](0020-pattern-matching/) — match a number against literal cases with a default; map 1, 2, 5 to one, two, many
- [0021-mutability-and-references](0021-mutability-and-references/) — increment a value through a reference so the caller sees before: 1 / after: 2
- [0022-concurrency](0022-concurrency/) — run two tasks concurrently and combine their results into sum: 3
- [0023-modules-and-imports](0023-modules-and-imports/) — define square(n) in a second module, import it, and print square(8) = 64
- [0024-sorting-and-comparators](0024-sorting-and-comparators/) — sort 3 1 2 ascending, then descending with a custom comparator
- [0025-tuples-and-multiple-return](0025-tuples-and-multiple-return/) — return min and max from one function; print min: 3 / max: 7
- [0026-sets](0026-sets/) — build a set with a duplicate; print its size and membership of 2 and 5
- [0027-file-io](0027-file-io/) — write a line to a file, read it back, delete it, and print it
- [0028-string-formatting](0028-string-formatting/) — format a float to 2 decimals and zero-pad an int to width 5
- [0029-bitwise-operations](0029-bitwise-operations/) — AND, OR, XOR, and left-shift two integers
- [0030-variadic-functions](0030-variadic-functions/) — sum a variable number of arguments: sum(1, 2, 3) = 6
- [0031-type-conversion-and-parsing](0031-type-conversion-and-parsing/) — parse a string to int and float, and convert a number back to a string
- [0032-math-functions](0032-math-functions/) — sqrt, power, abs, and max on numbers
- [0033-custom-error-types](0033-custom-error-types/) — define a custom error, raise it, catch it, and print its message
- [0034-default-and-named-arguments](0034-default-and-named-arguments/) — a function with a default argument, called with and without it
- [0035-ranges-and-slicing](0035-ranges-and-slicing/) — extract a sub-sequence (indices 1..4) from a list
- [0036-inheritance-and-overriding](0036-inheritance-and-overriding/) — a base type with a method, a subtype that overrides it
- [0037-operator-overloading](0037-operator-overloading/) — define + for a Point type and add two points
- [0038-string-methods](0038-string-methods/) — split, uppercase, and join: a,b,c → A-B-C
- [0039-records-and-value-equality](0039-records-and-value-equality/) — two records with equal fields compare equal
- [0040-generators-and-lazy-sequences](0040-generators-and-lazy-sequences/) — a lazy sequence of squares; take the first three
- [0041-command-line-arguments](0041-command-line-arguments/) — read the first CLI argument and greet it
- [0042-environment-variables](0042-environment-variables/) — read an env var with a fallback default
- [0043-function-composition](0043-function-composition/) — compose inc ∘ twice and apply to 3
- [0044-generic-constraints](0044-generic-constraints/) — a generic largest(a, b) bounded to ordered types; ints and strings
- [0045-abstract-classes-and-methods](0045-abstract-classes-and-methods/) — an abstract base with an abstract method, implemented by a subclass
- [0046-number-bases-and-radix](0046-number-bases-and-radix/) — print 255 in hex, octal, and binary
- [0047-immutable-update-copy-with](0047-immutable-update-copy-with/) — copy a record changing one field: (1,2) → (9,2)
- [0048-maps-iterate-and-transform](0048-maps-iterate-and-transform/) — sum the values of a map → 6
- [0049-zip-and-enumerate](0049-zip-and-enumerate/) — pair letters with numbers → a=1 b=2 c=3
- [0050-stacks](0050-stacks/) — push 1 2 3 onto a stack, pop all → 3 2 1
- [0051-ternary-and-conditional-expressions](0051-ternary-and-conditional-expressions/) — classify 4 and 7 as even/odd

## How to run

See each lesson's `README.md` for the exact command.
