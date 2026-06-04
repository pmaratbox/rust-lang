# 0117 — Fold Left vs Right

Fold subtraction over [1,2,3] from 0 both ways: left ((((0-1)-2)-3)) = -6 and right (1-(2-(3-0))) = 2, printing `-6 2`. `fold` associates left and `rfold` walks the iterator from the back to associate right.

## Run

    cargo run
