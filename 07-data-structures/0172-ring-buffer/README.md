# 0172 — Ring Buffer

Push 1,2,3,4,5 into a fixed capacity-3 ring buffer (overwriting oldest) and print the final contents `3 4 5`. A fixed `[i32; 3]` array with modular head/len indices keeps the wrap-around allocation-free.

## Run

    cargo run
