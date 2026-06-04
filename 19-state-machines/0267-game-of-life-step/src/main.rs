fn main() {
    // vertical blinker on a 3x3 grid
    let grid = [
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
    ];

    let alive = |r: i32, c: i32| -> u8 {
        if (0..3).contains(&r) && (0..3).contains(&c) {
            grid[r as usize][c as usize]
        } else {
            0
        }
    };

    let mut next = [[0u8; 3]; 3];
    for r in 0..3 {
        for c in 0..3 {
            let mut neighbors = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    neighbors += alive(r as i32 + dr, c as i32 + dc);
                }
            }
            let live = grid[r][c] == 1;
            next[r][c] = if (live && (neighbors == 2 || neighbors == 3))
                || (!live && neighbors == 3)
            {
                1
            } else {
                0
            };
        }
    }

    for row in &next {
        let line: String = row.iter().map(|&c| if c == 1 { '#' } else { '.' }).collect();
        println!("{}", line);
    }
}
