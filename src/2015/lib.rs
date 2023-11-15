// https://adventofcode.com/2015/

use std::mem;

pub fn day_18() {
    let mut grid = [[0u8; 100]; 100];

    let input = include_str!("input/raw.txt");
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                grid[i][j] = 1;
            }
        }
    }

    let mut out = [[0u8; 100]; 100];
    for _ in 0..100 {
        one_step(grid, &mut out);
        mem::swap(&mut grid, &mut out);
    }

    let mut sum: u16 = 0;
    for row in grid.iter() {
        for &value in row.iter() {
            sum += value as u16;
        }
    }
    println!("{}", sum);
}

fn one_step(grid: [[u8; 100]; 100], out: &mut [[u8; 100]; 100]) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let mut num_neighs = 0;
            let row_min = 0.max(i as i8 - 1) as usize;
            for row in row_min..=(grid.len() - 1).min(i + 1) {
                let col_min = 0.max(j as i8 - 1) as usize;
                for col in col_min..=(grid[0].len() - 1).min(j + 1) {
                    if i == row && col == j {
                        continue;
                    }
                    if grid[row][col] == 1 {
                        num_neighs += 1;
                    }
                }
            }
            out[i][j] = grid[i][j];
            if grid[i][j] == 1 && !(num_neighs == 2 || num_neighs == 3) {
                out[i][j] = 0;
            } else if grid[i][j] == 0 && num_neighs == 3 {
                out[i][j] = 1;
            }
        }
    }
    out[0][0] = 1;
    out[0][99] = 1;
    out[99][0] = 1;
    out[99][99] = 1;

}
