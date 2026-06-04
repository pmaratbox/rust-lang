# 0299 — Floyd-Warshall

Run all-pairs shortest paths on 0->1(3),1->2(1),0->2(5) and print the distance from 0 to 2 `4`. A `Vec<Vec<i64>>` matrix with a capped infinity avoids overflow when summing two `inf` entries.

## Run

    cargo run
