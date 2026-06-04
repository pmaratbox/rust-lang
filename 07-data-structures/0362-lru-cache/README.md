# 0362 — LRU Cache

With a capacity-2 LRU cache: put(1,1),put(2,2),get(1),put(3,3) evicts key 2; then get(1)=1 and get(2)=-1, printing `1 -1`. A `HashMap` for values plus a recency `Vec` keeps the logic idiomatic.

## Run

    cargo run
