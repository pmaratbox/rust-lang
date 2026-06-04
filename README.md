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

### Fundamentals & Control Flow
- [0001-hello-world](01-fundamentals/0001-hello-world/) — print "Hello, world!"
- [0002-variables-and-types](01-fundamentals/0002-variables-and-types/) — declare and print an int, float, string, and bool
- [0003-arithmetic-and-operators](01-fundamentals/0003-arithmetic-and-operators/) — sum, difference, product, quotient, modulo of two integers
- [0004-conditionals](01-fundamentals/0004-conditionals/) — compare to 10 and print less / equal / greater
- [0005-loops](01-fundamentals/0005-loops/) — print 1..5 with a for-loop
- [0020-pattern-matching](01-fundamentals/0020-pattern-matching/) — match a number against literal cases with a default; map 1, 2, 5 to one, two, many
- [0051-ternary-and-conditional-expressions](01-fundamentals/0051-ternary-and-conditional-expressions/) — classify 4 and 7 as even/odd
- [0058-labeled-break-and-continue](01-fundamentals/0058-labeled-break-and-continue/) — labeled continue/break over nested loops → 1,1 / 2,1 / stop at 2,2
- [0062-do-while-loop](01-fundamentals/0062-do-while-loop/) — run the body once before testing → 1 2 3
- [0095-guard-clauses](01-fundamentals/0095-guard-clauses/) — early-return classification → negative / zero / positive
- [0134-fizzbuzz](01-fundamentals/0134-fizzbuzz/) — print 1..15, replacing multiples of 3 with Fizz, of 5 with Buzz, and of both with FizzBuzz, one per line
- [0135-clamp](01-fundamentals/0135-clamp/) — clamp 15 and -3 into the range [0, 10], printing `10 0`
- [0136-sign-function](01-fundamentals/0136-sign-function/) — print the sign (-1, 0, or 1) of -5, 0, and 5: `-1 0 1`
- [0137-triangle-pattern](01-fundamentals/0137-triangle-pattern/) — print a left-aligned triangle of asterisks of height 3
- [0138-boolean-truth-table](01-fundamentals/0138-boolean-truth-table/) — for each (a,b) in (T,T),(T,F),(F,T),(F,F) print a, b, and a AND b, a OR b, a XOR b as lowercase booleans, one row per line

### Functions & Recursion
- [0006-functions](02-functions/0006-functions/) — define add(a, b), call it, print the result
- [0014-higher-order-functions](02-functions/0014-higher-order-functions/) — pass functions as values: apply inc and double via apply(f, x)
- [0018-closures](02-functions/0018-closures/) — return a counter function that captures private state; call it twice for 1 then 2
- [0019-recursion](02-functions/0019-recursion/) — define a recursive factorial(n); print factorial(5) = 120
- [0030-variadic-functions](02-functions/0030-variadic-functions/) — sum a variable number of arguments: sum(1, 2, 3) = 6
- [0034-default-and-named-arguments](02-functions/0034-default-and-named-arguments/) — a function with a default argument, called with and without it
- [0043-function-composition](02-functions/0043-function-composition/) — compose inc ∘ twice and apply to 3
- [0096-mutual-recursion](02-functions/0096-mutual-recursion/) — isEven/isOdd call each other → even / odd
- [0098-memoization](02-functions/0098-memoization/) — memoized fibonacci(10) → 55
- [0139-recursive-power](02-functions/0139-recursive-power/) — compute 2 raised to the 10th power recursively and print `1024`
- [0140-tower-of-hanoi](02-functions/0140-tower-of-hanoi/) — count the moves needed to solve Tower of Hanoi for 3 disks recursively and print `7`
- [0141-ackermann](02-functions/0141-ackermann/) — compute the Ackermann function A(2,3) recursively and print `9`
- [0142-countdown-recursion](02-functions/0142-countdown-recursion/) — recursively count down from 5 to 1, printing `5 4 3 2 1` on one line

### Types & Data Modeling
- [0010-structs](03-types/0010-structs/) — define a Person type with name and age, print its fields
- [0012-enums](03-types/0012-enums/) — define a Color enum; print the integer position of two variants
- [0013-optional](03-types/0013-optional/) — hold a present and an absent value; print each with a fallback
- [0016-generics](03-types/0016-generics/) — define a generic first(list) returning the first element; call it on ints and strings
- [0025-tuples-and-multiple-return](03-types/0025-tuples-and-multiple-return/) — return min and max from one function; print min: 3 / max: 7
- [0031-type-conversion-and-parsing](03-types/0031-type-conversion-and-parsing/) — parse a string to int and float, and convert a number back to a string
- [0039-records-and-value-equality](03-types/0039-records-and-value-equality/) — two records with equal fields compare equal
- [0044-generic-constraints](03-types/0044-generic-constraints/) — a generic largest(a, b) bounded to ordered types; ints and strings
- [0047-immutable-update-copy-with](03-types/0047-immutable-update-copy-with/) — copy a record changing one field: (1,2) → (9,2)
- [0097-enums-with-associated-values](03-types/0097-enums-with-associated-values/) — tagged shapes → 6 / 16
- [0143-type-alias](03-types/0143-type-alias/) — define a semantic alias for an integer type (e.g. Meters), assign 5, and print `distance: 5`
- [0144-flags-enum](03-types/0144-flags-enum/) — combine bit flags READ(1) and WRITE(2) into 3, check that WRITE is set, and print `3 yes`
- [0145-nested-struct](03-types/0145-nested-struct/) — model a person whose address is a nested struct, and print the city `London`
- [0146-numeric-conversions](03-types/0146-numeric-conversions/) — truncate 3.9 to the int 3 and widen the int 3 to the float 3.0, printing `3 3.0`
- [0147-deep-equality](03-types/0147-deep-equality/) — compare two nested structures with equal contents for structural equality and print `equal: yes`
- [0148-parse-or-default](03-types/0148-parse-or-default/) — parse "42" to 42 and "x" (invalid) to a default 0, printing `42 0`

### OOP & Polymorphism
- [0015-interfaces](04-oop/0015-interfaces/) — define a Shape interface implemented by two types; call area() polymorphically
- [0036-inheritance-and-overriding](04-oop/0036-inheritance-and-overriding/) — a base type with a method, a subtype that overrides it
- [0037-operator-overloading](04-oop/0037-operator-overloading/) — define + for a Point type and add two points
- [0045-abstract-classes-and-methods](04-oop/0045-abstract-classes-and-methods/) — an abstract base with an abstract method, implemented by a subclass
- [0099-method-chaining](04-oop/0099-method-chaining/) — fluent calculator → 16
- [0100-static-methods](04-oop/0100-static-methods/) — class-level square(6) → 36
- [0149-encapsulation](04-oop/0149-encapsulation/) — keep an account balance private, deposit 50 onto an initial 100 through a method, and print `150`
- [0150-builder-pattern](04-oop/0150-builder-pattern/) — use a fluent builder to construct a Pizza of size M with cheese, and print `Pizza(M, cheese)`
- [0151-singleton](04-oop/0151-singleton/) — obtain a singleton instance twice and confirm both references are the same object, printing `same: yes`
- [0152-tostring-override](04-oop/0152-tostring-override/) — override the string conversion of a Point(1,2) so it prints `Point(1, 2)`
- [0153-comparable-sort](04-oop/0153-comparable-sort/) — sort people [(alice,30),(bob,25)] by age ascending and print their names `bob alice`
- [0154-polymorphic-area](04-oop/0154-polymorphic-area/) — sum the areas of a Rectangle(2,3)=6 and a Triangle(base=4,height=4)=8 through a common Shape interface, printing `total area: 14`

### Strings & Text
- [0008-strings](05-strings/0008-strings/) — greet, uppercase, and print the length of a string
- [0028-string-formatting](05-strings/0028-string-formatting/) — format a float to 2 decimals and zero-pad an int to width 5
- [0038-string-methods](05-strings/0038-string-methods/) — split, uppercase, and join: a,b,c → A-B-C
- [0061-reverse-a-string](05-strings/0061-reverse-a-string/) — reverse "abc" → cba
- [0072-palindrome-check](05-strings/0072-palindrome-check/) — is it a palindrome? → level: yes / hello: no
- [0073-caesar-cipher](05-strings/0073-caesar-cipher/) — shift "abc" by 1 → bcd
- [0074-run-length-encoding](05-strings/0074-run-length-encoding/) — encode "aaabbc" → a3b2c1
- [0075-anagram-check](05-strings/0075-anagram-check/) — are they anagrams? → listen/silent: yes
- [0076-count-vowels](05-strings/0076-count-vowels/) — count vowels in "hello" → 2
- [0077-title-case](05-strings/0077-title-case/) — title-case "hello world" → Hello World
- [0078-substring-search](05-strings/0078-substring-search/) — index of "lo" in "hello" → 3
- [0079-character-codes](05-strings/0079-character-codes/) — code of A and char 66 → 65 B
- [0088-string-builder](05-strings/0088-string-builder/) — build a string incrementally → 1-2-3
- [0155-reverse-words](05-strings/0155-reverse-words/) — reverse the order of words in "hello world", printing `world hello`
- [0156-count-substring](05-strings/0156-count-substring/) — count non-overlapping occurrences of "ab" in "ababab", printing `3`
- [0157-longest-common-prefix](05-strings/0157-longest-common-prefix/) — find the longest common prefix of ["flower","flow","flight"], printing `fl`
- [0158-string-rotation](05-strings/0158-string-rotation/) — check whether "cdab" is a rotation of "abcd", printing `yes`
- [0159-string-interpolation](05-strings/0159-string-interpolation/) — interpolate name="Ada" and age=36 into a sentence, printing `Ada is 36`
- [0160-most-frequent-char](05-strings/0160-most-frequent-char/) — find the most frequent character in "hello", printing `l`
- [0161-remove-duplicate-chars](05-strings/0161-remove-duplicate-chars/) — remove duplicate characters from "aabbcc" keeping first occurrence, printing `abc`

### Collections, Maps & Iteration
- [0007-collections](06-collections/0007-collections/) — build a list of 1..5; print its count, first, and last
- [0009-maps](06-collections/0009-maps/) — build a map, look up a key, print the value and size
- [0017-iterators](06-collections/0017-iterators/) — filter evens, double them, and sum 1..5 with map/filter/reduce
- [0026-sets](06-collections/0026-sets/) — build a set with a duplicate; print its size and membership of 2 and 5
- [0035-ranges-and-slicing](06-collections/0035-ranges-and-slicing/) — extract a sub-sequence (indices 1..4) from a list
- [0040-generators-and-lazy-sequences](06-collections/0040-generators-and-lazy-sequences/) — a lazy sequence of squares; take the first three
- [0048-maps-iterate-and-transform](06-collections/0048-maps-iterate-and-transform/) — sum the values of a map → 6
- [0049-zip-and-enumerate](06-collections/0049-zip-and-enumerate/) — pair letters with numbers → a=1 b=2 c=3
- [0055-frequency-count](06-collections/0055-frequency-count/) — count letters in "banana" → a:3 b:1 n:2
- [0059-group-by](06-collections/0059-group-by/) — group words by length → 3:[one,two] 5:[three]
- [0063-flatten-nested-list](06-collections/0063-flatten-nested-list/) — flatten [[1,2],[3,4]] → 1 2 3 4
- [0080-deduplicate](06-collections/0080-deduplicate/) — remove duplicates, keep first-seen order → 1 2 3
- [0081-set-operations](06-collections/0081-set-operations/) — union & intersection → 1 2 3 4 / 2 3
- [0082-chunk-list](06-collections/0082-chunk-list/) — split into chunks of 3 → 1 2 3 / 4 5 6 / 7
- [0083-sliding-window](06-collections/0083-sliding-window/) — windows of size 2 → 1 2 / 2 3 / 3 4
- [0084-partition](06-collections/0084-partition/) — split into evens and odds → evens: 2 4 6 / odds: 1 3 5
- [0085-rotate-list](06-collections/0085-rotate-list/) — rotate left by 2 → 3 4 5 1 2
- [0086-sort-map-by-value](06-collections/0086-sort-map-by-value/) — sort entries by value → b:1 c:2 a:3
- [0087-invert-map](06-collections/0087-invert-map/) — swap keys and values → 1:a 2:b 3:c

### Data Structures
- [0050-stacks](07-data-structures/0050-stacks/) — push 1 2 3 onto a stack, pop all → 3 2 1
- [0052-queues](07-data-structures/0052-queues/) — enqueue 1 2 3, dequeue all → 1 2 3
- [0056-linked-list](07-data-structures/0056-linked-list/) — build and traverse a singly-linked list → 1 -> 2 -> 3
- [0060-priority-queue](07-data-structures/0060-priority-queue/) — pop 3 1 2 in priority order → 1 2 3
- [0090-binary-search-tree](07-data-structures/0090-binary-search-tree/) — insert + in-order traversal → 1 3 4 5 8

### Algorithms & Math
- [0024-sorting-and-comparators](08-algorithms/0024-sorting-and-comparators/) — sort 3 1 2 ascending, then descending with a custom comparator
- [0029-bitwise-operations](08-algorithms/0029-bitwise-operations/) — AND, OR, XOR, and left-shift two integers
- [0032-math-functions](08-algorithms/0032-math-functions/) — sqrt, power, abs, and max on numbers
- [0046-number-bases-and-radix](08-algorithms/0046-number-bases-and-radix/) — print 255 in hex, octal, and binary
- [0053-binary-search](08-algorithms/0053-binary-search/) — find 7 in a sorted array → found 7 at index 3
- [0057-sieve-of-eratosthenes](08-algorithms/0057-sieve-of-eratosthenes/) — primes up to 10 → 2 3 5 7
- [0064-matrix-transpose](08-algorithms/0064-matrix-transpose/) — transpose a 2x3 matrix → 1 4 / 2 5 / 3 6
- [0065-merge-sort](08-algorithms/0065-merge-sort/) — merge sort 3 1 4 1 5 2 → 1 1 2 3 4 5
- [0067-prefix-sums](08-algorithms/0067-prefix-sums/) — running totals of 1 2 3 4 → 1 3 6 10
- [0068-gcd-euclid](08-algorithms/0068-gcd-euclid/) — gcd(48, 36) via Euclid → 12
- [0069-fibonacci-iterative](08-algorithms/0069-fibonacci-iterative/) — first 7 Fibonacci numbers → 0 1 1 2 3 5 8
- [0070-digit-sum](08-algorithms/0070-digit-sum/) — sum the digits of 1234 → 10
- [0071-celsius-to-fahrenheit](08-algorithms/0071-celsius-to-fahrenheit/) — convert 100C and 0C → 212F / 32F
- [0089-quicksort](08-algorithms/0089-quicksort/) — quicksort 3 1 4 1 5 2 → 1 1 2 3 4 5
- [0091-balanced-parentheses](08-algorithms/0091-balanced-parentheses/) — stack-based bracket check → yes / no
- [0092-two-sum](08-algorithms/0092-two-sum/) — find indices summing to 9 → 0 1
- [0093-matrix-multiply](08-algorithms/0093-matrix-multiply/) — multiply two 2x2 matrices → 19 22 / 43 50
- [0094-linear-search](08-algorithms/0094-linear-search/) — scan for 7 → found 7 at index 2

### Systems, Errors & Tooling
- [0011-error-handling](09-systems/0011-error-handling/) — divide with a divide-by-zero guard; print result or error
- [0021-mutability-and-references](09-systems/0021-mutability-and-references/) — increment a value through a reference so the caller sees before: 1 / after: 2
- [0022-concurrency](09-systems/0022-concurrency/) — run two tasks concurrently and combine their results into sum: 3
- [0023-modules-and-imports](09-systems/0023-modules-and-imports/) — define square(n) in a second module, import it, and print square(8) = 64
- [0027-file-io](09-systems/0027-file-io/) — write a line to a file, read it back, delete it, and print it
- [0033-custom-error-types](09-systems/0033-custom-error-types/) — define a custom error, raise it, catch it, and print its message
- [0041-command-line-arguments](09-systems/0041-command-line-arguments/) — read the first CLI argument and greet it
- [0042-environment-variables](09-systems/0042-environment-variables/) — read an env var with a fallback default
- [0054-resource-cleanup-and-defer](09-systems/0054-resource-cleanup-and-defer/) — acquire and auto-release a resource → open / use / close
- [0066-multiple-assignment-and-destructuring](09-systems/0066-multiple-assignment-and-destructuring/) — swap and unpack → 2 1 / 3 4

### Concurrency & Parallelism
- [0101-threads-spawn-join](10-concurrency/0101-threads-spawn-join/) — spawn 3 worker threads, wait for all of them to finish, then print `done: 3`
- [0102-thread-return-values](10-concurrency/0102-thread-return-values/) — run two threads that compute the squares of 3 and 4, join them, and print the sum of their results `25`
- [0103-mutex-shared-counter](10-concurrency/0103-mutex-shared-counter/) — have multiple threads each increment a shared counter under a mutex so the total is exactly `1000`
- [0104-atomic-counter](10-concurrency/0104-atomic-counter/) — increment a shared atomic counter from multiple threads 1000 times total without a lock, printing `1000`
- [0105-channels-message-passing](10-concurrency/0105-channels-message-passing/) — send the values 1, 2, 3 through a channel (or queue) from one thread and receive them in order, printing `1 2 3`
- [0106-producer-consumer](10-concurrency/0106-producer-consumer/) — a producer sends 1..5 to a consumer that sums them, printing `15`
- [0107-worker-pool](10-concurrency/0107-worker-pool/) — distribute squaring of 1..4 across a pool of workers, collect the results, and print them sorted ascending `1 4 9 16`
- [0108-parallel-tasks-combine](10-concurrency/0108-parallel-tasks-combine/) — run two independent tasks that produce 10 and 20 concurrently, then combine (sum) their results into `30`
- [0109-run-once-init](10-concurrency/0109-run-once-init/) — ensure an initializer runs exactly once even when several threads race to trigger it, printing `init count: 1`

### Functional Programming
- [0110-currying](11-functional/0110-currying/) — curry a two-argument add into a chain of one-argument functions and call it as `add(2)(3)`, printing `5`
- [0111-partial-application](11-functional/0111-partial-application/) — partially apply a two-argument add by fixing the first argument to 10, then call the result with 3 to print `13`
- [0112-predicate-combinators](11-functional/0112-predicate-combinators/) — combine predicates with AND/OR/NOT: test `isEven AND isPositive` on 4 (yes) and -4 (no), printing `yes no`
- [0113-result-type](11-functional/0113-result-type/) — model success and failure with a Result type: safeDiv(10,2) prints `ok: 5` and safeDiv(1,0) prints `err: divide by zero`
- [0114-zip-with](11-functional/0114-zip-with/) — combine [1,2,3] and [4,5,6] element-wise with addition, printing `5 7 9`
- [0115-dispatch-table](11-functional/0115-dispatch-table/) — store functions in a map keyed by name, then apply "add" and "mul" to (3,4), printing `7 12`
- [0116-function-pipeline](11-functional/0116-function-pipeline/) — compose inc, double, and negate into a left-to-right pipeline and apply it to 3: ((3+1)*2) negated = `-8`
- [0117-fold-left-vs-right](11-functional/0117-fold-left-vs-right/) — fold subtraction over [1,2,3] from 0 both ways: left ((((0-1)-2)-3)) = -6 and right (1-(2-(3-0))) = 2, printing `-6 2`
- [0118-option-map-chain](11-functional/0118-option-map-chain/) — map a function over a present optional (10 -> 12) and an absent one (-> fallback), printing `12 none`

### I/O & Serialization
- [0119-read-file-lines](12-io-serialization/0119-read-file-lines/) — write three lines to a file, read them back, and print `lines: 3`
- [0120-append-to-file](12-io-serialization/0120-append-to-file/) — write "a" to a file, append "b", then read both lines back and print `a b`
- [0121-csv-parse](12-io-serialization/0121-csv-parse/) — parse the two CSV rows `alice,30` and `bob,25` into name=value pairs and print `alice=30 bob=25`
- [0122-utf8-bytes](12-io-serialization/0122-utf8-bytes/) — print the UTF-8 byte values of "Hi": `72 105`
- [0123-base64-encode](12-io-serialization/0123-base64-encode/) — base64-encode the bytes of "hi" to get `aGk=`
- [0124-path-manipulation](12-io-serialization/0124-path-manipulation/) — join "/tmp" and "file.txt", then take the basename and extension, printing `/tmp/file.txt file.txt .txt`
- [0125-temp-file-roundtrip](12-io-serialization/0125-temp-file-roundtrip/) — write a string to a temporary file, read it back, confirm it matches, delete the file, and print `roundtrip: ok`

### Dates & Calendars
- [0126-leap-year](13-datetime/0126-leap-year/) — determine whether 2000, 1900, and 2024 are leap years, printing `yes no yes`
- [0127-day-of-week-zeller](13-datetime/0127-day-of-week-zeller/) — use Zeller’s congruence to compute the weekday of 2000-01-01 and print `Saturday`
- [0128-days-between-dates](13-datetime/0128-days-between-dates/) — compute the number of days from 2000-01-01 to 2000-12-31 and print `365`
- [0129-add-days](13-datetime/0129-add-days/) — add 40 days to 2000-01-01 and print the result as `2000-02-10`
- [0130-format-iso-date](13-datetime/0130-format-iso-date/) — format the date y=2026, m=6, d=4 as a zero-padded ISO string `2026-06-04`

### Tooling & Meta
- [0131-assertions](14-tooling/0131-assertions/) — assert several expected equalities (all true) and print `all passed`
- [0132-mini-test-runner](14-tooling/0132-mini-test-runner/) — run three named test functions that all pass and report `3 passed, 0 failed`
- [0133-debug-repr](14-tooling/0133-debug-repr/) — produce a debug/structured representation of a Point(1,2) and print `Point(x=1, y=2)`

## How to run

Lessons are grouped into topic folders (e.g. `05-strings/0072-palindrome-check/`).

See each lesson's `README.md` for the exact command.
