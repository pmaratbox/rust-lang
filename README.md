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
- [0333-even-odd-classify](01-fundamentals/0333-even-odd-classify/) — classify each of [1,2,3,4] as odd or even, printing `odd even odd even`
- [0334-count-digits](01-fundamentals/0334-count-digits/) — count the digits of 90210, printing `5`
- [0335-factorial-iterative](01-fundamentals/0335-factorial-iterative/) — compute 5! iteratively, printing `120`
- [0336-swap-values](01-fundamentals/0336-swap-values/) — swap a=1 and b=2 (without bitwise tricks) and print `2 1`
- [0337-multiplication-table](01-fundamentals/0337-multiplication-table/) — print the 3x3 multiplication table, each row space-separated

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
- [0338-min-max-return](02-functions/0338-min-max-return/) — return both the minimum and maximum of [4,1,7] from one function and print `1 7`
- [0339-recursive-sum](02-functions/0339-recursive-sum/) — recursively sum the list [1,2,3,4], printing `10`
- [0340-apply-twice](02-functions/0340-apply-twice/) — write applyTwice(f,x)=f(f(x)) and apply it with inc to 3, printing `5`
- [0341-recursive-list-reverse](02-functions/0341-recursive-list-reverse/) — reverse [1,2,3] using recursion (no built-in reverse), printing `3 2 1`

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
- [0342-tagged-union-eval](03-types/0342-tagged-union-eval/) — model an expression tagged union Num | Add and evaluate Add(Num 1, Num 2), printing `3`
- [0343-optional-chaining](03-types/0343-optional-chaining/) — read a deeply optional value a.b.c when present (5) and when absent (default 0), printing `5 0`
- [0344-enum-ordinal](03-types/0344-enum-ordinal/) — for the enum Direction(N,E,S,W) print the ordinal of S (2) and the name at ordinal 3 (W), printing `2 W`
- [0345-struct-defaults](03-types/0345-struct-defaults/) — create a Point with default fields (0,0), then one overriding x to 5, printing each on its own line `0 0` then `5 0`
- [0346-newtype-wrapper](03-types/0346-newtype-wrapper/) — wrap raw integers in distinct UserId and ProductId types so they cannot be confused, printing `user-1 prod-2`

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
- [0347-virtual-dispatch](04-oop/0347-virtual-dispatch/) — store a Circle, Square, and Triangle in one Shape collection and call describe() on each, printing `circle square triangle`
- [0348-interface-default-method](04-oop/0348-interface-default-method/) — define an interface with a default greet() returning "hi" and an implementer that overrides it to "hey", printing `hi hey`
- [0349-trait-composition](04-oop/0349-trait-composition/) — compose two capabilities (A printing "a", B printing "b") into one type and invoke both, printing `a b`
- [0350-static-factory](04-oop/0350-static-factory/) — use a static factory Color.fromHex("#ff0000") to build a color and print its components `255 0 0`
- [0351-instance-counter](04-oop/0351-instance-counter/) — track how many instances of a class have been created; after building three, print `3`

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
- [0352-rot13](05-strings/0352-rot13/) — apply ROT13 to "hello" (->"uryyb") and ROT13 again to recover "hello", printing `uryyb hello`
- [0353-char-frequency](05-strings/0353-char-frequency/) — count character frequencies in "aab" and print them in first-seen order `a:2 b:1`
- [0354-snake-to-camel](05-strings/0354-snake-to-camel/) — convert "hello_world" to camelCase `helloWorld`
- [0355-expand-char-range](05-strings/0355-expand-char-range/) — expand the range spec "a-e" into the characters `abcde`
- [0356-atoi-itoa](05-strings/0356-atoi-itoa/) — parse "-42" to the integer -42 and format it back to "-42", printing `-42 -42`

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
- [0357-zip-longest](06-collections/0357-zip-longest/) — zip [1,2,3] with ["a","b"], filling the missing slot with "-", printing `1a 2b 3-`
- [0358-adjacent-pairs](06-collections/0358-adjacent-pairs/) — produce the consecutive pairs of [1,2,3,4], printing `1,2 2,3 3,4`
- [0359-flatten-deep](06-collections/0359-flatten-deep/) — flatten the arbitrarily nested structure [1,[2,[3,4]],5] into `1 2 3 4 5`
- [0360-interleave-lists](06-collections/0360-interleave-lists/) — interleave [1,3,5] and [2,4,6] element by element, printing `1 2 3 4 5 6`
- [0361-frequency-sort](06-collections/0361-frequency-sort/) — sort [1,1,2,3,3,3] by descending frequency (ties keep first-seen order), printing `3 3 3 1 1 2`

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
- [0362-lru-cache](07-data-structures/0362-lru-cache/) — with a capacity-2 LRU cache: put(1,1),put(2,2),get(1),put(3,3) evicts key 2; then get(1)=1 and get(2)=-1, printing `1 -1`
- [0363-circular-queue](07-data-structures/0363-circular-queue/) — in a capacity-3 circular queue enqueue 1,2,3, dequeue once, enqueue 4, then print the contents `2 3 4`
- [0364-deque](07-data-structures/0364-deque/) — using a deque, push 1 and 2 to the back and 0 to the front, then print front-to-back `0 1 2`
- [0365-multiset-count](07-data-structures/0365-multiset-count/) — add 1,1,2 to a multiset; print the count of 1 (2), remove one 1, then print the count of 1 (1), as `2 1`
- [0366-sparse-matrix](07-data-structures/0366-sparse-matrix/) — store only nonzero entries; with (1,1)=5 set, read (1,1) (5) and (0,0) (0), printing `5 0`

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
- [0367-selection-sort](08-algorithms/0367-selection-sort/) — selection-sort [5,1,4,2] ascending, printing `1 2 4 5`
- [0368-shell-sort](08-algorithms/0368-shell-sort/) — shell-sort [5,2,8,1,9,3] ascending, printing `1 2 3 5 8 9`
- [0369-search-bounds](08-algorithms/0369-search-bounds/) — in the sorted array [1,3,5,5,7] find the lower bound (2) and upper bound (4) of 5, printing `2 4`
- [0370-quickselect](08-algorithms/0370-quickselect/) — find the 3rd smallest element of [7,10,4,3,20,15] with quickselect, printing `7`
- [0371-counting-sort](08-algorithms/0371-counting-sort/) — counting-sort [3,1,2,3,1] ascending, printing `1 1 2 3 3`
- [0372-dutch-national-flag](08-algorithms/0372-dutch-national-flag/) — three-way partition [2,0,2,1,1,0] of values 0/1/2 in one pass, printing `0 0 1 1 2 2`

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
- [0373-exception-hierarchy](09-systems/0373-exception-hierarchy/) — throw a specific error subtype and catch it through a base-type handler, printing `caught base`
- [0374-error-codes](09-systems/0374-error-codes/) — map the numeric error code 2 to its message "not found" and print `not found`
- [0375-retry-logic](09-systems/0375-retry-logic/) — retry an operation that fails on attempts 1 and 2 and succeeds on attempt 3, printing `ok after 3`
- [0376-feature-flag](09-systems/0376-feature-flag/) — branch on a boolean feature flag: on -> "enabled", off -> "disabled", printing `enabled disabled`
- [0377-permission-bits](09-systems/0377-permission-bits/) — decode the Unix permission bits 0b101 into the rwx string `r-x`

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
- [0378-barrier-sync](10-concurrency/0378-barrier-sync/) — have 3 threads each arrive at a barrier before any proceeds, then print `all reached: 3`
- [0379-future-chain](10-concurrency/0379-future-chain/) — chain asynchronous steps 5 -> (*2) -> (+1) and print the final value `11`
- [0380-compare-and-swap](10-concurrency/0380-compare-and-swap/) — increment a shared value to 100 using a CAS retry loop from multiple threads, printing `100`
- [0381-parallel-map](10-concurrency/0381-parallel-map/) — square 1,2,3,4 in parallel and collect the results in input order, printing `1 4 9 16`
- [0382-fork-join-sum](10-concurrency/0382-fork-join-sum/) — recursively fork the sum of [1..8] into halves and join the partial sums, printing `36`

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
- [0383-partition-runs](11-functional/0383-partition-runs/) — partition [1,1,2,3,3,3] into runs of equal adjacent elements, printing `1 1|2|3 3 3`
- [0384-scan-product](11-functional/0384-scan-product/) — produce the running products of [1,2,3,4], printing `1 2 6 24`
- [0385-unfold](11-functional/0385-unfold/) — unfold a sequence by doubling from 1, taking five terms, printing `1 2 4 8 16`
- [0386-iterate-take](11-functional/0386-iterate-take/) — iterate f(x)=x*3 from 1 and take the first four values, printing `1 3 9 27`

### I/O & Serialization
- [0119-read-file-lines](12-io-serialization/0119-read-file-lines/) — write three lines to a file, read them back, and print `lines: 3`
- [0120-append-to-file](12-io-serialization/0120-append-to-file/) — write "a" to a file, append "b", then read both lines back and print `a b`
- [0121-csv-parse](12-io-serialization/0121-csv-parse/) — parse the two CSV rows `alice,30` and `bob,25` into name=value pairs and print `alice=30 bob=25`
- [0122-utf8-bytes](12-io-serialization/0122-utf8-bytes/) — print the UTF-8 byte values of "Hi": `72 105`
- [0123-base64-encode](12-io-serialization/0123-base64-encode/) — base64-encode the bytes of "hi" to get `aGk=`
- [0124-path-manipulation](12-io-serialization/0124-path-manipulation/) — join "/tmp" and "file.txt", then take the basename and extension, printing `/tmp/file.txt file.txt .txt`
- [0125-temp-file-roundtrip](12-io-serialization/0125-temp-file-roundtrip/) — write a string to a temporary file, read it back, confirm it matches, delete the file, and print `roundtrip: ok`
- [0387-json-stringify](12-io-serialization/0387-json-stringify/) — serialize the object {name:"Ada", age:36} to compact JSON `{"name":"Ada","age":36}`
- [0388-json-parse](12-io-serialization/0388-json-parse/) — parse the JSON object `{"x":1,"y":2}` and print its entries `x=1 y=2`
- [0389-hex-dump](12-io-serialization/0389-hex-dump/) — print the bytes of "Hi" as lowercase hex `48 69`
- [0390-binary-endian](12-io-serialization/0390-binary-endian/) — encode the integer 258 as two big-endian bytes (1, 2), decode them back to 258, printing `1 2 258`
- [0391-url-encode](12-io-serialization/0391-url-encode/) — percent-encode the string "a b&c" to `a%20b%26c`

### Dates & Calendars
- [0126-leap-year](13-datetime/0126-leap-year/) — determine whether 2000, 1900, and 2024 are leap years, printing `yes no yes`
- [0127-day-of-week-zeller](13-datetime/0127-day-of-week-zeller/) — use Zeller’s congruence to compute the weekday of 2000-01-01 and print `Saturday`
- [0128-days-between-dates](13-datetime/0128-days-between-dates/) — compute the number of days from 2000-01-01 to 2000-12-31 and print `365`
- [0129-add-days](13-datetime/0129-add-days/) — add 40 days to 2000-01-01 and print the result as `2000-02-10`
- [0130-format-iso-date](13-datetime/0130-format-iso-date/) — format the date y=2026, m=6, d=4 as a zero-padded ISO string `2026-06-04`
- [0392-day-of-year](13-datetime/0392-day-of-year/) — compute the day-of-year of 2000-03-01 (a leap year), printing `61`
- [0393-weekday-after-days](13-datetime/0393-weekday-after-days/) — given 2000-01-01 is a Saturday, compute the weekday 3 days later, printing `Tuesday`
- [0394-time-add-minutes](13-datetime/0394-time-add-minutes/) — add 90 minutes to 10:45 and print the resulting 24-hour time `12:15`
- [0395-days-in-month](13-datetime/0395-days-in-month/) — print the number of days in February 2000 (29) and February 2001 (28) as `29 28`

### Tooling & Meta
- [0131-assertions](14-tooling/0131-assertions/) — assert several expected equalities (all true) and print `all passed`
- [0132-mini-test-runner](14-tooling/0132-mini-test-runner/) — run three named test functions that all pass and report `3 passed, 0 failed`
- [0133-debug-repr](14-tooling/0133-debug-repr/) — produce a debug/structured representation of a Point(1,2) and print `Point(x=1, y=2)`
- [0396-call-counter](14-tooling/0396-call-counter/) — wrap a function so each call increments a counter; after five calls print `calls: 5`
- [0397-log-levels](14-tooling/0397-log-levels/) — with a threshold of WARN, log messages at INFO, WARN, and ERROR but only emit WARN and ERROR, on two lines
- [0398-table-format](14-tooling/0398-table-format/) — format the rows (a,1),(bb,22) with the first column left-padded to the widest value, printing two aligned rows
- [0399-progress-bar](14-tooling/0399-progress-bar/) — render a width-10 progress bar at 40% completion, printing `[####------]`
- [0400-ascii-histogram](14-tooling/0400-ascii-histogram/) — render a histogram of [3,1,2] as rows of "#" characters, one bar per line

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
- [0289-climbing-stairs](21-dynamic-programming/0289-climbing-stairs/) — count the ways to climb 5 stairs taking 1 or 2 steps, printing `8`
- [0290-unique-paths](21-dynamic-programming/0290-unique-paths/) — count lattice paths from the top-left to bottom-right of a 3x3 grid moving right/down, printing `6`
- [0291-min-path-sum](21-dynamic-programming/0291-min-path-sum/) — find the minimum path sum through [[1,3,1],[1,5,1],[4,2,1]] moving right/down, printing `7`
- [0292-house-robber](21-dynamic-programming/0292-house-robber/) — maximize the sum of non-adjacent picks from [2,7,9,3,1], printing `12`
- [0293-longest-palindromic-subseq](21-dynamic-programming/0293-longest-palindromic-subseq/) — compute the longest palindromic subsequence length of "bbbab", printing `4`
- [0294-subset-sum](21-dynamic-programming/0294-subset-sum/) — decide whether some subset of [3,34,4,12,5,2] sums to 9, printing `yes`
- [0295-partition-equal-subset](21-dynamic-programming/0295-partition-equal-subset/) — decide whether [1,5,11,5] can split into two equal-sum subsets, printing `yes`
- [0296-matrix-chain-order](21-dynamic-programming/0296-matrix-chain-order/) — find the minimum scalar multiplications to multiply matrices with dimensions [10,20,30,40], printing `18000`

### Graph Algorithms
- [0297-dijkstra](22-graphs/0297-dijkstra/) — on the weighted digraph 0->1(4),0->2(1),2->1(2),1->3(1),2->3(5), print the shortest distances from node 0 `0 3 1 4`
- [0298-bellman-ford](22-graphs/0298-bellman-ford/) — on the digraph with a negative edge 0->1(1),1->2(-2),0->2(4), print the shortest distances from node 0 `0 1 -1`
- [0299-floyd-warshall](22-graphs/0299-floyd-warshall/) — run all-pairs shortest paths on 0->1(3),1->2(1),0->2(5) and print the distance from 0 to 2 `4`
- [0300-topological-sort](22-graphs/0300-topological-sort/) — topologically sort the DAG 0->1,0->2,1->3,2->3 using the Kahn algorithm (smallest index first), printing `0 1 2 3`
- [0301-cycle-detect-directed](22-graphs/0301-cycle-detect-directed/) — detect a cycle in the digraph 0->1,1->2,2->0, printing `cycle`
- [0302-kruskal-mst](22-graphs/0302-kruskal-mst/) — compute the MST total weight of edges (0,1,1),(1,2,2),(0,2,3) with union-find, printing `3`
- [0303-prim-mst](22-graphs/0303-prim-mst/) — compute the MST total weight of the chain (0,1,1),(1,2,2),(2,3,3) with Prim, printing `6`
- [0304-connected-components](22-graphs/0304-connected-components/) — count the connected components of the undirected graph with edges (0,1),(1,2),(3,4) over nodes 0..4, printing `2`
- [0305-bipartite-check](22-graphs/0305-bipartite-check/) — check bipartiteness of a 4-cycle 0-1-2-3-0 (yes) and a triangle 0-1-2-0 (no), printing `yes no`
- [0306-shortest-path-reconstruct](22-graphs/0306-shortest-path-reconstruct/) — on the Dijkstra graph 0->1(4),0->2(1),2->1(2),1->3(1),2->3(5), print the actual shortest path from 0 to 3 `0 2 1 3`
- [0307-astar-grid](22-graphs/0307-astar-grid/) — find the shortest path length from (0,0) to (2,2) on an obstacle-free 3x3 grid (4-directional) with the Manhattan heuristic, printing `4`
- [0308-scc-count](22-graphs/0308-scc-count/) — count the strongly connected components of 0->1,1->2,2->0,2->3, printing `2`

### Trees & Heaps
- [0309-validate-bst](23-trees-heaps/0309-validate-bst/) — check whether two trees are valid BSTs: a correct one (yes) and one with an out-of-place node (no), printing `yes no`
- [0310-bst-search](23-trees-heaps/0310-bst-search/) — insert 5,3,8,1,4 into a BST, then search for 4 (yes) and 6 (no), printing `yes no`
- [0311-bst-delete](23-trees-heaps/0311-bst-delete/) — insert 5,3,8,1,4 into a BST, delete 3, and print the in-order traversal `1 4 5 8`
- [0312-tree-height](23-trees-heaps/0312-tree-height/) — compute the height (in nodes) of the BST built from 5,3,8,1,4, printing `3`
- [0313-level-order-traversal](23-trees-heaps/0313-level-order-traversal/) — print the breadth-first (level-order) traversal of the BST built from 5,3,8,1,4 `5 3 8 1 4`
- [0314-preorder-iterative](23-trees-heaps/0314-preorder-iterative/) — print the iterative (stack-based) preorder traversal of the BST built from 5,3,8,1,4 `5 3 1 4 8`
- [0315-tree-diameter](23-trees-heaps/0315-tree-diameter/) — compute the diameter (longest path in edges) of a tree where root has children A and B, and A has children C and D, printing `3`
- [0316-max-heap-pop](23-trees-heaps/0316-max-heap-pop/) — build a max-heap from [3,1,4,1,5], pop the top three values, printing `5 4 3`
- [0317-heap-sort](23-trees-heaps/0317-heap-sort/) — heap-sort [5,3,8,1,4] ascending, printing `1 3 4 5 8`
- [0318-kth-largest](23-trees-heaps/0318-kth-largest/) — find the 2nd largest element of [3,2,1,5,6,4] using a heap, printing `5`
- [0319-segment-tree-sum](23-trees-heaps/0319-segment-tree-sum/) — build a segment tree over [1,2,3,4,5] and query the sum of indices 1..3 (inclusive), printing `9`
- [0320-fenwick-tree](23-trees-heaps/0320-fenwick-tree/) — build a Fenwick (BIT) tree over [1,2,3,4,5] and query the prefix sum of the first 4 elements, printing `10`

### Advanced Functional
- [0321-lazy-take](24-functional-advanced/0321-lazy-take/) — build a lazy sequence of the natural numbers and take the first five, printing `1 2 3 4 5`
- [0322-lazy-filter-take](24-functional-advanced/0322-lazy-filter-take/) — from a lazy stream of naturals, filter the even ones and take three, printing `2 4 6`
- [0323-maybe-monad](24-functional-advanced/0323-maybe-monad/) — chain Maybe operations: Some(2) then +3 then *2 gives 10, and a None chain yields the fallback, printing `10 none`
- [0324-either-monad](24-functional-advanced/0324-either-monad/) — chain Either computations: a successful divide chain yields 2, and a divide-by-zero yields an error, printing `2 err`
- [0325-point-free](24-functional-advanced/0325-point-free/) — express "sum of squares" point-free (compose map-square with sum) and apply it to [1,2,3], printing `14`
- [0326-monoid-fold](24-functional-advanced/0326-monoid-fold/) — fold lists under two monoids: string concat ["a","b","c"]->"abc" and integer sum [1,2,3]->6, printing `abc 6`
- [0327-flatmap](24-functional-advanced/0327-flatmap/) — flatMap [1,2,3] with x -> [x, x*10] and print the flattened result `1 10 2 20 3 30`
- [0328-trampoline](24-functional-advanced/0328-trampoline/) — sum 1..100 with a trampolined recursion that avoids deep stacks, printing `5050`
- [0329-cps-factorial](24-functional-advanced/0329-cps-factorial/) — compute 5! in continuation-passing style, printing `120`
- [0330-lens-get-set](24-functional-advanced/0330-lens-get-set/) — use a lens over the nested value {a:{b:1}} to get b (1) and to set b to 2, printing `1 2`
- [0331-transducer-pipeline](24-functional-advanced/0331-transducer-pipeline/) — compose map(+1) with filter(even) and run it over [1,2,3,4], printing `2 4`
- [0332-y-combinator](24-functional-advanced/0332-y-combinator/) — define factorial via a fixed-point combinator (no named self-recursion) and compute 5!, printing `120`

### Reactive Programming
- [0401-observable-create](25-reactive/0401-observable-create/) — build a push-based Observable from scratch that emits 1, 2, 3 to its observer and then completes
- [0402-observer-next-error-complete](25-reactive/0402-observer-next-error-complete/) — demonstrate the observer contract next*-then-terminal: emit 1 and 2, complete, and show that a post-complete next is ignored
- [0403-subscribe-unsubscribe](25-reactive/0403-subscribe-unsubscribe/) — return a Subscription from subscribe() and use it to unsubscribe so later values are not delivered
- [0404-cold-vs-hot-observable](25-reactive/0404-cold-vs-hot-observable/) — contrast a cold observable (re-runs its producer per subscriber) with a hot one (shares a single execution, so late subscribers miss earlier values)
- [0405-map-operator](25-reactive/0405-map-operator/) — implement a map operator that transforms each emitted value, applying x => x*2 to a stream of 1, 2, 3, 4
- [0406-filter-operator](25-reactive/0406-filter-operator/) — implement a filter operator that forwards only values passing a predicate, keeping the even numbers of 1..6
- [0407-scan-accumulate](25-reactive/0407-scan-accumulate/) — implement a scan operator that emits the running accumulation; produce the running sums of 1, 2, 3, 4
- [0408-take-operator](25-reactive/0408-take-operator/) — implement take(n) over an unbounded source of the natural numbers, emitting the first 3 then completing (and unsubscribing the source)
- [0409-distinct-until-changed](25-reactive/0409-distinct-until-changed/) — implement distinctUntilChanged, dropping consecutive duplicate values from 1,1,2,2,2,3,1
- [0410-merge-streams](25-reactive/0410-merge-streams/) — implement merge of two timed streams using a virtual-time scheduler, interleaving them by emission time
- [0411-concat-streams](25-reactive/0411-concat-streams/) — implement concat: subscribe to the second source only after the first completes; concat [1,2] then [3,4]
- [0412-combine-latest](25-reactive/0412-combine-latest/) — implement combineLatest of two timed streams, emitting the pair of latest values whenever either source emits (once both have emitted)
- [0413-zip-streams](25-reactive/0413-zip-streams/) — implement zip that pairs values by index and combines them; zip [1,2,3] with [10,20,30] using a+b
- [0414-flatmap-streams](25-reactive/0414-flatmap-streams/) — implement flatMap/mergeMap: map each outer value to an inner timed stream and merge all inners concurrently (no cancellation)
- [0415-switch-map](25-reactive/0415-switch-map/) — implement switchMap: when a new outer value arrives, cancel the previous inner subscription before starting the new one
- [0416-subject-multicast](25-reactive/0416-subject-multicast/) — implement a Subject that multicasts each emission to all current observers; two observers both receive 1 then 2
- [0417-behavior-subject](25-reactive/0417-behavior-subject/) — implement a BehaviorSubject that holds a current value and replays it immediately to each new subscriber
- [0418-replay-subject](25-reactive/0418-replay-subject/) — implement a ReplaySubject with a buffer of the last 2 values, replayed to a late subscriber, which then also receives new values
- [0419-catch-error](25-reactive/0419-catch-error/) — implement catchError that, on an error from the source, switches to a fallback stream
- [0420-retry-on-error](25-reactive/0420-retry-on-error/) — implement retry(n) that resubscribes to the source on error up to n times; the source succeeds on the 3rd subscription
- [0421-debounce-virtual-time](25-reactive/0421-debounce-virtual-time/) — implement debounce(window) on a virtual-time scheduler: emit a value only after a quiet gap of `window` ticks with no newer value
- [0422-throttle-virtual-time](25-reactive/0422-throttle-virtual-time/) — implement throttle(window) (leading edge) on a virtual-time scheduler: emit a value, then suppress further values for `window` ticks
- [0423-event-emitter-pubsub](25-reactive/0423-event-emitter-pubsub/) — build a multi-topic EventEmitter with on(topic, handler), emit(topic, payload), and off(topic, handler)
- [0424-reactive-signal-computed](25-reactive/0424-reactive-signal-computed/) — implement fine-grained reactivity: a writable signal and a derived computed that recomputes when its dependency changes

### Databases (SQLite)
- [0437-connect-and-query](27-database-sqlite/0437-connect-and-query/) — open an in-memory SQLite database and run a single query
- [0438-create-table-insert](27-database-sqlite/0438-create-table-insert/) — create a table, insert rows, and select them back
- [0439-parameterized-query](27-database-sqlite/0439-parameterized-query/) — bind a parameter to query safely (no string concatenation)
- [0440-update-delete](27-database-sqlite/0440-update-delete/) — modify and remove rows, then read the result
- [0441-aggregate-functions](27-database-sqlite/0441-aggregate-functions/) — compute count, sum, min, and max
- [0442-group-by](27-database-sqlite/0442-group-by/) — aggregate amounts per group
- [0443-order-by-limit](27-database-sqlite/0443-order-by-limit/) — sort descending and take the top rows
- [0444-inner-join](27-database-sqlite/0444-inner-join/) — join two tables on a key
- [0445-transaction-commit-rollback](27-database-sqlite/0445-transaction-commit-rollback/) — commit one change and roll back another
- [0446-indexes](27-database-sqlite/0446-indexes/) — create an index and use it in a lookup
- [0447-batch-insert](27-database-sqlite/0447-batch-insert/) — insert many rows efficiently in one transaction
- [0448-upsert](27-database-sqlite/0448-upsert/) — insert or update on primary-key conflict

### ORM & Query Builders
- [0449-define-model-and-insert](28-orm/0449-define-model-and-insert/) — define an entity/model, insert rows, and read them back
- [0450-filter-where](28-orm/0450-filter-where/) — query rows matching a condition
- [0451-order-by](28-orm/0451-order-by/) — sort results by a column
- [0452-limit](28-orm/0452-limit/) — take only the top N rows
- [0453-update-row](28-orm/0453-update-row/) — modify a persisted entity
- [0454-delete-row](28-orm/0454-delete-row/) — remove a persisted entity
- [0455-count](28-orm/0455-count/) — count rows via the query API
- [0456-sum-aggregate](28-orm/0456-sum-aggregate/) — sum a column via the query API
- [0457-group-by](28-orm/0457-group-by/) — aggregate per group via the query API
- [0458-join](28-orm/0458-join/) — join two models/tables

### Serialization & JSON
- [0459-serialize-object](29-serialization/0459-serialize-object/) — serialize a typed object to a compact JSON string
- [0460-deserialize-object](29-serialization/0460-deserialize-object/) — parse a JSON string into a typed object
- [0461-nested-object](29-serialization/0461-nested-object/) — serialize an object containing another object
- [0462-array-of-objects](29-serialization/0462-array-of-objects/) — serialize a list of objects to a JSON array
- [0463-array-of-primitives](29-serialization/0463-array-of-primitives/) — serialize a list of integers
- [0464-optional-default](29-serialization/0464-optional-default/) — deserialize JSON missing a field, using a default
- [0465-round-trip](29-serialization/0465-round-trip/) — serialize then deserialize the same object
- [0466-parse-nested-access](29-serialization/0466-parse-nested-access/) — parse JSON and read a deeply nested value
- [0467-field-rename](29-serialization/0467-field-rename/) — map a code field name to a different JSON key
- [0468-scalar-types](29-serialization/0468-scalar-types/) — serialize bool, int, and string fields

### CLI Argument Parsing
- [0469-positional-argument](30-cli-args/0469-positional-argument/) — parse a positional argument
- [0470-boolean-flag](30-cli-args/0470-boolean-flag/) — parse a boolean flag
- [0471-option-with-value](30-cli-args/0471-option-with-value/) — parse an option that takes a value
- [0472-default-value](30-cli-args/0472-default-value/) — use an option's default when absent
- [0473-multiple-values](30-cli-args/0473-multiple-values/) — collect a repeated option into a list
- [0474-subcommand](30-cli-args/0474-subcommand/) — dispatch to a subcommand
- [0475-required-option](30-cli-args/0475-required-option/) — declare an option as required
- [0476-choice-option](30-cli-args/0476-choice-option/) — restrict an option to a set of choices

### Template Engines
- [0477-variable-substitution](31-template-engines/0477-variable-substitution/) — render a template with a variable
- [0478-conditional](31-template-engines/0478-conditional/) — render based on a condition
- [0479-loop](31-template-engines/0479-loop/) — iterate a list in a template
- [0480-nested-access](31-template-engines/0480-nested-access/) — access a nested field in a template
- [0481-loop-objects](31-template-engines/0481-loop-objects/) — iterate a list of objects
- [0482-uppercase-filter](31-template-engines/0482-uppercase-filter/) — apply an uppercase filter/helper
- [0483-default-value](31-template-engines/0483-default-value/) — provide a default for a missing value
- [0484-list-length](31-template-engines/0484-list-length/) — render the length of a list

### Validation & Schemas
- [0485-valid-input](32-validation/0485-valid-input/) — a valid object passes validation
- [0486-required-field](32-validation/0486-required-field/) — a required field is missing
- [0487-min-length](32-validation/0487-min-length/) — a string is too short
- [0488-number-range](32-validation/0488-number-range/) — a number is out of range
- [0489-max-length](32-validation/0489-max-length/) — a string is too long
- [0490-email-format](32-validation/0490-email-format/) — an email is malformed
- [0491-multiple-errors](32-validation/0491-multiple-errors/) — collect several errors, sorted
- [0492-custom-rule](32-validation/0492-custom-rule/) — a custom validation rule fails

### Web Frameworks
- [0493-hello-route](33-web-frameworks/0493-hello-route/) — define a GET route returning text
- [0494-json-response](33-web-frameworks/0494-json-response/) — return a JSON body
- [0495-path-param](33-web-frameworks/0495-path-param/) — capture a path parameter
- [0496-query-param](33-web-frameworks/0496-query-param/) — read a query-string parameter
- [0497-post-json-body](33-web-frameworks/0497-post-json-body/) — parse a JSON request body
- [0498-status-404](33-web-frameworks/0498-status-404/) — unknown route returns 404
- [0499-custom-status](33-web-frameworks/0499-custom-status/) — return a custom status code
- [0500-request-header](33-web-frameworks/0500-request-header/) — read a request header
- [0501-middleware](33-web-frameworks/0501-middleware/) — apply middleware that transforms the response
- [0502-multiple-routes](33-web-frameworks/0502-multiple-routes/) — register and hit two routes
- [0503-method-routing](33-web-frameworks/0503-method-routing/) — same path, different methods
- [0504-error-handler](33-web-frameworks/0504-error-handler/) — handle a thrown error with a 500

### GraphQL
- [0505-hello-query](34-graphql/0505-hello-query/) — execute a simple query
- [0506-field-argument](34-graphql/0506-field-argument/) — pass an argument to a field
- [0507-object-field](34-graphql/0507-object-field/) — return an object type and select a field
- [0508-list-field](34-graphql/0508-list-field/) — return a list of scalars
- [0509-nested-object](34-graphql/0509-nested-object/) — select a field through nested object types
- [0510-multiple-fields](34-graphql/0510-multiple-fields/) — select multiple fields of an object
- [0511-mutation](34-graphql/0511-mutation/) — execute a mutation
- [0512-variables](34-graphql/0512-variables/) — execute a query with variables

### Property-Based Testing
- [0513-first-property](35-property-testing/0513-first-property/) — a property over generated lists
- [0514-integer-property](35-property-testing/0514-integer-property/) — a property over generated integers
- [0515-string-property](35-property-testing/0515-string-property/) — a property over generated strings
- [0516-sort-idempotent](35-property-testing/0516-sort-idempotent/) — sorting twice equals sorting once
- [0517-custom-generator](35-property-testing/0517-custom-generator/) — build a generator and test a property
- [0518-multiple-args](35-property-testing/0518-multiple-args/) — a property with two generated arguments
- [0519-precondition](35-property-testing/0519-precondition/) — constrain inputs with a precondition
- [0520-finds-counterexample](35-property-testing/0520-finds-counterexample/) — a false property is detected

### Date & Time Libraries
- [0521-parse-and-format](36-datetime/0521-parse-and-format/) — parse an ISO date and format it back
- [0522-add-days](36-datetime/0522-add-days/) — add a number of days to a date
- [0523-difference-days](36-datetime/0523-difference-days/) — days between two dates
- [0524-components](36-datetime/0524-components/) — extract year, month, day
- [0525-weekday](36-datetime/0525-weekday/) — iSO weekday number of a date
- [0526-add-time](36-datetime/0526-add-time/) — add minutes to a datetime
- [0527-timezone-offset](36-datetime/0527-timezone-offset/) — convert a UTC instant to a fixed offset
- [0528-unix-timestamp](36-datetime/0528-unix-timestamp/) — epoch seconds of a UTC instant

### Immutable Collections
- [0529-immutable-list](37-immutable-collections/0529-immutable-list/) — adding returns a new list; original unchanged
- [0530-immutable-map](37-immutable-collections/0530-immutable-map/) — setting a key returns a new map
- [0531-immutable-set](37-immutable-collections/0531-immutable-set/) — adding to a set returns a new set
- [0532-update-element](37-immutable-collections/0532-update-element/) — updating an index returns a new list
- [0533-map-filter](37-immutable-collections/0533-map-filter/) — transform via map/filter returning new collections
- [0534-set-union](37-immutable-collections/0534-set-union/) — union of two immutable sets
- [0535-prepend-append](37-immutable-collections/0535-prepend-append/) — add to both ends
- [0536-nested-update](37-immutable-collections/0536-nested-update/) — update a nested value immutably

### Parser Combinators
- [0537-parse-integer](38-parser-combinators/0537-parse-integer/) — parse a run of digits into an int
- [0538-sequence](38-parser-combinators/0538-sequence/) — parse two parsers in sequence
- [0539-alternative](38-parser-combinators/0539-alternative/) — choice between two parsers
- [0540-many](38-parser-combinators/0540-many/) — repeat a parser zero+ times
- [0541-separated-list](38-parser-combinators/0541-separated-list/) — parse a comma-separated list
- [0542-map-transform](38-parser-combinators/0542-map-transform/) — transform a parsed value
- [0543-whitespace](38-parser-combinators/0543-whitespace/) — skip surrounding whitespace
- [0544-expression](38-parser-combinators/0544-expression/) — parse and evaluate a sum expression

### State Machines
- [0545-basic-transition](39-state-machines/0545-basic-transition/) — one event changes state
- [0546-toggle](39-state-machines/0546-toggle/) — an event that flips between two states
- [0547-traffic-light](39-state-machines/0547-traffic-light/) — cycle through states
- [0548-guarded-transition](39-state-machines/0548-guarded-transition/) — a transition only valid from a specific state
- [0549-invalid-transition](39-state-machines/0549-invalid-transition/) — an event with no transition leaves state unchanged
- [0550-transition-count](39-state-machines/0550-transition-count/) — an action runs on each transition
- [0551-reset](39-state-machines/0551-reset/) — a reset event returns to the initial state
- [0552-workflow](39-state-machines/0552-workflow/) — a multi-step approval workflow

### Numeric & Arrays
- [0553-vector-sum](40-numeric-arrays/0553-vector-sum/) — sum the elements of a vector
- [0554-dot-product](40-numeric-arrays/0554-dot-product/) — dot product of two vectors
- [0555-elementwise-add](40-numeric-arrays/0555-elementwise-add/) — add two vectors element-wise
- [0556-matrix-multiply](40-numeric-arrays/0556-matrix-multiply/) — multiply two 2x2 matrices
- [0557-transpose](40-numeric-arrays/0557-transpose/) — transpose a matrix
- [0558-scalar-multiply](40-numeric-arrays/0558-scalar-multiply/) — multiply a matrix by a scalar

### Cryptography & Hashing
- [0559-sha256](41-crypto-hashing/0559-sha256/) — sHA-256 hash of a string
- [0560-sha512](41-crypto-hashing/0560-sha512/) — sHA-512 hash of a string
- [0561-sha1](41-crypto-hashing/0561-sha1/) — sHA-1 hash of a string
- [0562-md5](41-crypto-hashing/0562-md5/) — mD5 hash of a string
- [0563-hmac-sha256](41-crypto-hashing/0563-hmac-sha256/) — hMAC-SHA256 with a key
- [0564-pbkdf2](41-crypto-hashing/0564-pbkdf2/) — derive a key with PBKDF2
- [0565-base64-encode](41-crypto-hashing/0565-base64-encode/) — base64-encode bytes
- [0566-incremental-hash](41-crypto-hashing/0566-incremental-hash/) — hash data in two update calls

### YAML & TOML
- [0567-yaml-scalars](42-yaml-toml/0567-yaml-scalars/) — parse a YAML mapping and read scalar fields
- [0568-yaml-nested](42-yaml-toml/0568-yaml-nested/) — read a value from a nested YAML mapping
- [0569-yaml-sequence](42-yaml-toml/0569-yaml-sequence/) — parse a YAML sequence (list)

## How to run

Lessons are grouped into topic folders (e.g. `05-strings/0072-palindrome-check/`).

See each lesson's `README.md` for the exact command.
