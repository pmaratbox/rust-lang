use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn h(p: (i32, i32), goal: (i32, i32)) -> i32 {
    (p.0 - goal.0).abs() + (p.1 - goal.1).abs()
}

fn main() {
    let n = 3;
    let start = (0, 0);
    let goal = (2, 2);
    let mut dist = vec![vec![i32::MAX; n]; n];
    dist[0][0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((h(start, goal), 0, start)));

    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut answer = -1;

    while let Some(Reverse((_f, g, (r, c)))) = heap.pop() {
        if (r, c) == goal {
            answer = g;
            break;
        }
        if g > dist[r as usize][c as usize] {
            continue;
        }
        for (dr, dc) in dirs {
            let nr = r + dr;
            let nc = c + dc;
            if nr < 0 || nr >= n as i32 || nc < 0 || nc >= n as i32 {
                continue;
            }
            let ng = g + 1;
            if ng < dist[nr as usize][nc as usize] {
                dist[nr as usize][nc as usize] = ng;
                heap.push(Reverse((ng + h((nr, nc), goal), ng, (nr, nc))));
            }
        }
    }

    println!("{}", answer);
}
