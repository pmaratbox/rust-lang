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
- [0162-pad-left-right](05-strings/0162-pad-left-right/) — left-pad and right-pad "5" with spaces to width 3 and print both joined by a pipe: `  5|5  `

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
- [0163-take-while](06-collections/0163-take-while/) — from [1,2,3,4,1] take elements while they are < 3, printing `1 2`
- [0164-map-merge](06-collections/0164-map-merge/) — merge {a:1,b:2} and {b:3,c:4} with the right map winning on conflicts, printing `a:1 b:3 c:4`
- [0165-all-any](06-collections/0165-all-any/) — for [2,4,6] report whether all are even (yes) and whether any are odd (no), printing `yes no`
- [0166-cartesian-product](06-collections/0166-cartesian-product/) — print the cartesian product of [1,2] and ["a","b"] as `1a 1b 2a 2b`
- [0167-top-k-frequent](06-collections/0167-top-k-frequent/) — find the 2 most frequent elements of [a,b,a,c,b,a], printing `a b`
- [0168-list-difference](06-collections/0168-list-difference/) — compute [1,2,3,4] minus the elements of [2,4], printing `1 3`
- [0169-max-by-key](06-collections/0169-max-by-key/) — from ["a","bbb","cc"] pick the longest string, printing `bbb`
- [0170-index-by-key](06-collections/0170-index-by-key/) — index people [(1,alice),(2,bob)] by id into a map, look up id 2, and print `id 2: bob`

### Data Structures
- [0050-stacks](07-data-structures/0050-stacks/) — push 1 2 3 onto a stack, pop all → 3 2 1
- [0052-queues](07-data-structures/0052-queues/) — enqueue 1 2 3, dequeue all → 1 2 3
- [0056-linked-list](07-data-structures/0056-linked-list/) — build and traverse a singly-linked list → 1 -> 2 -> 3
- [0060-priority-queue](07-data-structures/0060-priority-queue/) — pop 3 1 2 in priority order → 1 2 3
- [0090-binary-search-tree](07-data-structures/0090-binary-search-tree/) — insert + in-order traversal → 1 3 4 5 8
- [0171-doubly-linked-list](07-data-structures/0171-doubly-linked-list/) — build a doubly linked list 1<->2<->3, traverse forward then backward, printing `1 2 3` then `3 2 1` on two lines
- [0172-ring-buffer](07-data-structures/0172-ring-buffer/) — push 1,2,3,4,5 into a fixed capacity-3 ring buffer (overwriting oldest) and print the final contents `3 4 5`
- [0173-trie](07-data-structures/0173-trie/) — insert "cat" and "car" into a trie, then search "car" (yes) and "can" (no), printing `yes no`
- [0174-graph-adjacency-list](07-data-structures/0174-graph-adjacency-list/) — build an undirected graph with adjacency 0:[1,2] 1:[0,3] 2:[0,3] 3:[1,2] and print the neighbors of node 0: `1 2`
- [0175-graph-bfs](07-data-structures/0175-graph-bfs/) — breadth-first traverse from node 0 of the graph 0:[1,2] 1:[0,3] 2:[0,3] 3:[1,2], printing visit order `0 1 2 3`
- [0176-graph-dfs](07-data-structures/0176-graph-dfs/) — depth-first traverse from node 0 of the graph 0:[1,2] 1:[0,3] 2:[0,3] 3:[1,2], printing visit order `0 1 3 2`
- [0177-min-heap-manual](07-data-structures/0177-min-heap-manual/) — implement a binary min-heap by hand, push 3,1,2, pop all, printing `1 2 3`
- [0178-stack-with-min](07-data-structures/0178-stack-with-min/) — build a stack that tracks its minimum in O(1); after pushing 3,1,2 print `min: 1`
- [0179-union-find](07-data-structures/0179-union-find/) — union (0,1) and (2,3), then query connectivity of (0,1)=yes and (0,2)=no, printing `yes no`

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
- [0180-bubble-sort](08-algorithms/0180-bubble-sort/) — bubble-sort [5,1,4,2,8] ascending, printing `1 2 4 5 8`
- [0181-insertion-sort](08-algorithms/0181-insertion-sort/) — insertion-sort [5,1,4,2,8] ascending, printing `1 2 4 5 8`
- [0182-is-prime](08-algorithms/0182-is-prime/) — test whether 7 (prime) and 9 (not) are prime, printing `yes no`
- [0183-prime-factorization](08-algorithms/0183-prime-factorization/) — factor 60 into primes, printing `2 2 3 5`
- [0184-lcm](08-algorithms/0184-lcm/) — compute lcm(4,6) via gcd and print `12`
- [0185-collatz-steps](08-algorithms/0185-collatz-steps/) — count Collatz steps to reach 1 starting from 6 and print `8`
- [0186-reverse-integer](08-algorithms/0186-reverse-integer/) — reverse the digits of 1234 and print `4321`
- [0187-count-set-bits](08-algorithms/0187-count-set-bits/) — count the set bits (popcount) of 13 and print `3`
- [0188-kadane-max-subarray](08-algorithms/0188-kadane-max-subarray/) — find the maximum subarray sum of [-2,1,-3,4,-1,2,1,-5,4] with Kadane and print `6`
- [0189-permutations](08-algorithms/0189-permutations/) — generate all permutations of [1,2,3] in lexicographic order, one per line space-separated
- [0190-subsets-powerset](08-algorithms/0190-subsets-powerset/) — generate the power set of [1,2,3] in bitmask order 0..7 (empty printed as `{}`), one subset per line space-separated
- [0191-pascal-triangle](08-algorithms/0191-pascal-triangle/) — print the first 4 rows of Pascal’s triangle, each row space-separated
- [0192-majority-element](08-algorithms/0192-majority-element/) — find the majority element of [2,2,1,2,3,2] using Boyer-Moore voting and print `2`
- [0193-happy-number](08-algorithms/0193-happy-number/) — determine whether 19 is a happy number and print `yes`

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
- [0194-try-catch-finally](09-systems/0194-try-catch-finally/) — throw and catch an error, printing `caught`, and always run a finally block printing `cleanup`, on two lines
- [0195-defer-lifo-order](09-systems/0195-defer-lifo-order/) — register three deferred actions printing 1, 2, 3 and show they run in last-in-first-out order `3 2 1`
- [0196-error-wrapping](09-systems/0196-error-wrapping/) — wrap an inner error "inner" inside an outer context and print the combined message `outer: inner`
- [0197-enum-iteration](09-systems/0197-enum-iteration/) — iterate over all values of a Color enum (RED, GREEN, BLUE) and print their names `RED GREEN BLUE`
- [0198-seeded-lcg-prng](09-systems/0198-seeded-lcg-prng/) — implement a linear congruential generator next=(5*x+3) mod 16 seeded at 1 and print its first 3 outputs `8 11 10`
- [0199-struct-field-names](09-systems/0199-struct-field-names/) — list the field names of a Point{x,y} struct/record and print `x y`
- [0200-precondition-assert](09-systems/0200-precondition-assert/) — check a precondition arg>0: for 5 print `ok`, and for -1 report the failure `error: must be positive`, on two lines

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

### Design Patterns
- [0201-factory-method](15-design-patterns/0201-factory-method/) — use a factory method to create shapes by name and print their kinds `circle square`
- [0202-abstract-factory](15-design-patterns/0202-abstract-factory/) — use a "dark" theme abstract factory to build a button and a checkbox, printing `dark-button dark-checkbox`
- [0203-prototype](15-design-patterns/0203-prototype/) — clone a prototype object holding 1, mutate the clone to 2, and show the original is unchanged, printing `1 2`
- [0204-adapter](15-design-patterns/0204-adapter/) — adapt a Celsius source (100) to a Fahrenheit interface, printing `212`
- [0205-decorator](15-design-patterns/0205-decorator/) — decorate a base coffee (cost 2) with milk (+1) and sugar (+1), printing the total cost `4`
- [0206-facade](15-design-patterns/0206-facade/) — expose a single facade call that starts three subsystems and reports `ready`
- [0207-composite](15-design-patterns/0207-composite/) — sum a composite tree of leaf values 1, 2, 3 through a uniform size() interface, printing `6`
- [0208-proxy](15-design-patterns/0208-proxy/) — use a lazy virtual proxy that loads the real subject only on first access, printing `loaded`
- [0209-observer](15-design-patterns/0209-observer/) — notify two observers of a new value 5; each prints its id and the value on its own line
- [0210-strategy](15-design-patterns/0210-strategy/) — select an "add" then a "mul" strategy and apply each to (3,4), printing `7 12`
- [0211-command](15-design-patterns/0211-command/) — execute an AddCommand that takes a counter from 0 to 5, then undo it back to 0, printing `5 0`
- [0212-template-method](15-design-patterns/0212-template-method/) — run an algorithm skeleton whose middle step is overridden, printing `start work end` on one line
- [0213-iterator-pattern](15-design-patterns/0213-iterator-pattern/) — implement a custom iterator over a range 1..3 and consume it, printing `1 2 3`
- [0214-visitor](15-design-patterns/0214-visitor/) — use a visitor to sum the values of a small node tree with leaves 1, 2, 3, printing `6`
- [0215-chain-of-responsibility](15-design-patterns/0215-chain-of-responsibility/) — pass a request of level 2 along a handler chain so the level-2 handler handles it, printing `handled by 2`
- [0216-mediator](15-design-patterns/0216-mediator/) — have colleague A send "hi" through a mediator to colleague B, which prints `B got: hi`
- [0217-memento](15-design-patterns/0217-memento/) — save state 1, change it to 2, then restore from the memento, printing current then restored `2 1`
- [0218-flyweight](15-design-patterns/0218-flyweight/) — share flyweight instances for repeated characters a, b, a and print the number of distinct objects created `2`
- [0219-bridge](15-design-patterns/0219-bridge/) — bridge a Circle abstraction to a Red color implementation, printing `red circle`
- [0220-null-object](15-design-patterns/0220-null-object/) — compare a no-op null logger with a real logger; only the real one records, so print the logged count `1`

### Parsing & Interpreters
- [0221-tokenizer](16-parsing/0221-tokenizer/) — tokenize the expression "1 + 2" into token types, printing `NUM PLUS NUM`
- [0222-bracket-matching](16-parsing/0222-bracket-matching/) — check that mixed brackets are balanced: "([{}])" (yes) and "([)]" (no), printing `yes no`
- [0223-rpn-eval](16-parsing/0223-rpn-eval/) — evaluate the postfix expression "3 4 + 5 *" and print `35`
- [0224-shunting-yard](16-parsing/0224-shunting-yard/) — convert the infix expression "3 + 4 * 2" to postfix `3 4 2 * +`
- [0225-recursive-descent-calc](16-parsing/0225-recursive-descent-calc/) — parse and evaluate "2+3*4" honoring precedence, printing `14`
- [0226-roman-to-int](16-parsing/0226-roman-to-int/) — parse the Roman numeral "XIV" to its value `14`
- [0227-int-to-roman](16-parsing/0227-int-to-roman/) — format the integer 14 as the Roman numeral `XIV`
- [0228-csv-quoted-parse](16-parsing/0228-csv-quoted-parse/) — parse the CSV row `a,"b,c",d`, respecting the quoted comma, into three fields joined by pipes `a|b,c|d`
- [0229-ini-parse](16-parsing/0229-ini-parse/) — parse the INI text with section [s] and key k=v, printing the flattened entry `s.k=v`
- [0230-json-array-sum](16-parsing/0230-json-array-sum/) — parse the JSON array text "[1,2,3]" and print the sum of its elements `6`
- [0231-template-substitute](16-parsing/0231-template-substitute/) — substitute the variable in the template "hi {name}" with name="Ada", printing `hi Ada`
- [0232-wc-counts](16-parsing/0232-wc-counts/) — for the text "a b\\nc" report words, lines, and characters, printing `3 2 5`
- [0233-expression-ast](16-parsing/0233-expression-ast/) — build an AST for "1+2*3" and evaluate it to `7`
- [0234-number-base-parse](16-parsing/0234-number-base-parse/) — parse "ff" as hexadecimal (255) and "101" as binary (5), printing `255 5`
- [0235-brainfuck-cell](16-parsing/0235-brainfuck-cell/) — interpret the Brainfuck program "+++" on a single zeroed cell and print the cell value `3`

### Numerics & Math
- [0236-bignum-add](17-numerics/0236-bignum-add/) — add the large numbers 999999999999 and 1 as digit strings, printing `1000000000000`
- [0237-bignum-multiply](17-numerics/0237-bignum-multiply/) — multiply 123 by 456 with schoolbook long multiplication on digit strings, printing `56088`
- [0238-fraction-add](17-numerics/0238-fraction-add/) — add the fractions 1/2 and 1/3 and print the reduced result `5/6`
- [0239-fraction-reduce](17-numerics/0239-fraction-reduce/) — reduce the fraction 6/8 to lowest terms `3/4`
- [0240-complex-multiply](17-numerics/0240-complex-multiply/) — multiply the complex numbers (1+2i) and (3+4i), printing the real and imaginary parts `-5 10`
- [0241-matrix-determinant](17-numerics/0241-matrix-determinant/) — compute the determinant of [[1,2],[3,4]], printing `-2`
- [0242-matrix-vector-mult](17-numerics/0242-matrix-vector-mult/) — multiply [[1,2],[3,4]] by the vector [5,6], printing `17 39`
- [0243-newton-sqrt](17-numerics/0243-newton-sqrt/) — approximate sqrt(2) with Newton iteration and print it to 4 decimals `1.4142`
- [0244-gcd-extended](17-numerics/0244-gcd-extended/) — compute extended gcd(30,12) returning g and Bezout coefficients, printing `6 1 -2` (since 30*1+12*(-2)=6)
- [0245-modular-exponentiation](17-numerics/0245-modular-exponentiation/) — compute 3^13 mod 7 by fast modular exponentiation, printing `3`
- [0246-perfect-number](17-numerics/0246-perfect-number/) — test whether 6 (yes) and 8 (no) are perfect numbers, printing `yes no`
- [0247-horner-eval](17-numerics/0247-horner-eval/) — evaluate 2x^2+3x+1 at x=2 using the Horner method, printing `15`
- [0248-catalan-number](17-numerics/0248-catalan-number/) — print the first five Catalan numbers `1 1 2 5 14`
- [0249-binomial-coefficient](17-numerics/0249-binomial-coefficient/) — compute C(5,2) without overflow and print `10`
- [0250-fast-power-iterative](17-numerics/0250-fast-power-iterative/) — compute 2^10 by iterative binary exponentiation, printing `1024`

### Bit Manipulation
- [0251-set-clear-toggle-bit](18-bit-manipulation/0251-set-clear-toggle-bit/) — on bit position 1: set it on 0 (->2), clear it on 2 (->0), toggle it on 0 (->2), printing `2 0 2`
- [0252-reverse-bits-byte](18-bit-manipulation/0252-reverse-bits-byte/) — reverse the 8 bits of the value 1 (00000001) to get 128 (10000000), printing `128`
- [0253-power-of-two-check](18-bit-manipulation/0253-power-of-two-check/) — test whether 16 (yes) and 18 (no) are powers of two, printing `yes no`
- [0254-swap-xor](18-bit-manipulation/0254-swap-xor/) — swap the values 3 and 5 using only XOR (no temporary), printing `5 3`
- [0255-lowest-set-bit](18-bit-manipulation/0255-lowest-set-bit/) — isolate the lowest set bit of 12 (1100), printing `4`
- [0256-hamming-distance](18-bit-manipulation/0256-hamming-distance/) — compute the bit Hamming distance between 1 and 4, printing `2`
- [0257-gray-code](18-bit-manipulation/0257-gray-code/) — print the Gray code (as decimals) for 0..3 `0 1 3 2`
- [0258-count-trailing-zeros](18-bit-manipulation/0258-count-trailing-zeros/) — count the trailing zero bits of 8 (1000), printing `3`
- [0259-bitmask-subsets](18-bit-manipulation/0259-bitmask-subsets/) — enumerate all submasks of the mask 5 (101) in descending order `5 4 1 0`
- [0260-pack-unpack-bits](18-bit-manipulation/0260-pack-unpack-bits/) — pack r=1,g=2,b=3 into one integer (8 bits each) then unpack them, printing `1 2 3`
- [0261-rotate-left](18-bit-manipulation/0261-rotate-left/) — rotate the 8-bit value 1 left by 1 (->2) and 128 left by 1 (->1), printing `2 1`
- [0262-parity-check](18-bit-manipulation/0262-parity-check/) — compute the bit parity (XOR of all bits) of 7 (->1) and 5 (->0), printing `1 0`

### State Machines & Automata
- [0263-traffic-light](19-state-machines/0263-traffic-light/) — starting at red, advance a traffic light through four transitions (red->green->yellow->red->green), printing each new state `green yellow red green`
- [0264-turnstile-fsm](19-state-machines/0264-turnstile-fsm/) — drive a turnstile (locked/unlocked) with events coin, push, push and print the resulting states `unlocked locked locked`
- [0265-vending-machine](19-state-machines/0265-vending-machine/) — insert coins 10, 10, 5 to reach the price 25 and dispense, printing `dispensed`
- [0266-dfa-divisible-by-3](19-state-machines/0266-dfa-divisible-by-3/) — use a 3-state DFA over binary input to test whether "110" (6, yes) and "100" (4, no) are divisible by 3, printing `yes no`
- [0267-game-of-life-step](19-state-machines/0267-game-of-life-step/) — advance one step of a vertical blinker on a 3x3 grid (it becomes horizontal); print the grid using . and #
- [0268-rule90-step](19-state-machines/0268-rule90-step/) — apply one Rule 90 step (new = left XOR right) to the row "00100" with zero boundaries, printing `01010`
- [0269-markov-deterministic](19-state-machines/0269-markov-deterministic/) — walk a deterministic transition map A->B->C->A for three steps from A, printing the visited states `B C A`
- [0270-elevator-fsm](19-state-machines/0270-elevator-fsm/) — move an elevator from floor 0 up to floor 2 then back to 0, printing every floor it stops at `0 1 2 1 0`
- [0271-string-match-dfa](19-state-machines/0271-string-match-dfa/) — build a DFA for the pattern "ab" and report the first match index in "aab", printing `1`
- [0272-glob-star-match](19-state-machines/0272-glob-star-match/) — match the glob "a*b" (* = any run) against "aaab" (yes) and "aac" (no), printing `yes no`

### String Algorithms
- [0273-kmp-search](20-text-algorithms/0273-kmp-search/) — use Knuth-Morris-Pratt to find all start indices of "ab" in "ababab", printing `0 2 4`
- [0274-levenshtein](20-text-algorithms/0274-levenshtein/) — compute the edit distance between "kitten" and "sitting", printing `3`
- [0275-lcs-length](20-text-algorithms/0275-lcs-length/) — compute the LCS length of "ABCBDAB" and "BDCAB", printing `4`
- [0276-longest-common-substring](20-text-algorithms/0276-longest-common-substring/) — find the longest common contiguous substring of "abcde" and "xbcdy", printing `bcd`
- [0277-longest-palindrome-substring](20-text-algorithms/0277-longest-palindrome-substring/) — find the longest palindromic substring of "babad" (first of maximal length), printing `bab`
- [0278-suffix-array](20-text-algorithms/0278-suffix-array/) — build the suffix array of "banana" (sorted suffix start indices), printing `5 3 1 0 4 2`
- [0279-min-window-substring](20-text-algorithms/0279-min-window-substring/) — find the smallest window in "ADOBECODEBANC" containing all of "ABC", printing `BANC`
- [0280-group-anagrams](20-text-algorithms/0280-group-anagrams/) — group ["eat","tea","tan","ate","nat"] into anagram classes and print the number of groups `2`
- [0281-z-function](20-text-algorithms/0281-z-function/) — compute the Z-array of "aaaa" (excluding z[0]) and print `3 2 1`
- [0282-boyer-moore-search](20-text-algorithms/0282-boyer-moore-search/) — use the bad-character rule to find "abc" in "zzabc", printing the index `2`
- [0283-rabin-karp](20-text-algorithms/0283-rabin-karp/) — use rolling-hash Rabin-Karp to find all start indices of "ab" in "xabxab", printing `1 4`
- [0284-trie-autocomplete](20-text-algorithms/0284-trie-autocomplete/) — insert "car","card","dog" into a trie and autocomplete the prefix "car", printing `car card`

### Dynamic Programming
- [0285-coin-change-min](21-dynamic-programming/0285-coin-change-min/) — find the fewest coins from {1,2,5} summing to 11, printing `3`
- [0286-coin-change-count](21-dynamic-programming/0286-coin-change-count/) — count the ways to make 5 from {1,2,5}, printing `4`
- [0287-knapsack-01](21-dynamic-programming/0287-knapsack-01/) — maximize value within capacity 5 for items (w,v)=(2,3),(3,4),(4,5), printing `7`
- [0288-lis-length](21-dynamic-programming/0288-lis-length/) — compute the LIS length of [10,9,2,5,3,7,101,18], printing `4`

## How to run

Lessons are grouped into topic folders (e.g. `05-strings/0072-palindrome-check/`).

See each lesson's `README.md` for the exact command.
