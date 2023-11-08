// https://adventofcode.com/2015/

// toggle 461,550 through 564,900
// turn off 370,39 through 425,839

pub fn day6() -> u32 {
    // initialize grid
    let mut grid = [[0u32; 1000]; 1000];
    let input = include_str!("input/raw.txt");
    let (mut r1, mut c1, mut r2, mut c2);
    let mut num_idx;
    for line in input.lines() {
        let v: Vec<&str> = line.split_terminator(&[' ', ','][..]).collect();
        let f = match v[1] {
            "on" => {
                num_idx = 2;
                |x| x + 1
            }
            "off" => {
                num_idx = 2;
                |x| {
                    if x == 0 {
                        return 0
                    }
                    x - 1
                }
            }
            _ => { // toggle
                num_idx = 1;
                |x| x + 2
            }
        };
        (r1, c1, r2, c2) = (
            v[num_idx].parse().unwrap(),
            v[num_idx + 1].parse().unwrap(),
            v[num_idx + 3].parse().unwrap(),
            v[num_idx + 4].parse().unwrap(),
        );
        alter_grid(&mut grid, f, r1, c1, r2, c2);
    }
    // count 1s
    let mut count = 0;
    for r in 0..1000 {
        for c in 0..1000 {
            count += grid[r][c];
        }
    }
    count
}

pub fn alter_grid<F: Fn(u32) -> u32>(
    grid: &mut [[u32; 1000]; 1000],
    f: F,
    r1: usize,
    c1: usize,
    r2: usize,
    c2: usize,
) {
    for r in r1..=r2 {
        for c in c1..=c2 {
            grid[r][c] = f(grid[r][c])
        }
    }
}
