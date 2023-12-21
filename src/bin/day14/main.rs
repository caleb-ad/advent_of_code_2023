use std::{io::Read};

type Grid = Vec<Vec<u8>>;

fn main() {
    let mut input = vec![];
    let _ = std::fs::File::open("src/bin/day14/test.txt").expect("").read_to_end(&mut input);

    let mut grid: Grid = input.split(|b| *b == b'\n').map(|s| s.to_vec()).collect();

    // test load = 153
    spin_cycle(&mut grid, 1000_000_000);
    println!("load {:?}", calculate_load(&grid));
}

fn roll_north(grid: &mut Grid) {
    let mut block = Vec::new();
    block.resize(grid[0].len(), 0);

    for yidx in 0..grid.len() {
        for ( xidx, block) in (0..grid[yidx].len()).zip(block.iter_mut()) {
            let current = grid[yidx][xidx];
            if current == b'O' && *block < yidx {
                grid[*block][xidx] = b'O';
                grid[yidx][xidx] = b'.';
                *block += 1;
            }
            else if current == b'O' || current == b'#' { *block = yidx + 1}
        }
    }
}

fn roll_west(grid: &mut Grid) {
    let mut block = Vec::new();
    block.resize(grid.len(), 0);

    for xidx in 0..grid[0].len() {
        for (yidx, block) in (0..grid.len()).zip(block.iter_mut()) {
            let current = grid[yidx][xidx];
            if current == b'O' && *block < xidx {
                grid[yidx][*block] = b'O';
                grid[yidx][xidx] = b'.';
                *block += 1;
            }
            else if current == b'O' || current == b'#' { *block = xidx + 1}
        }
    }
}

fn roll_east(grid: &mut Grid) {
    let mut block = Vec::new();
    block.resize(grid.len(), grid[0].len());

    for xidx in (0..grid[0].len()).rev() {
        for (yidx, block) in (0..grid.len()).zip(block.iter_mut()) {
            let current = grid[yidx][xidx];
            if current == b'O' && *block > xidx + 1 {
                grid[yidx][*block - 1] = b'O';
                grid[yidx][xidx] = b'.';
                *block -= 1;
            }

            else if current == b'O' || current == b'#' { *block = xidx}
        }
    }
}

fn roll_south(grid: &mut Grid) {
    let mut block = Vec::new();
    block.resize(grid[0].len(), grid.len());

    for yidx in (0..grid.len()).rev() {
        for ( xidx, block) in (0..grid[yidx].len()).zip(block.iter_mut()) {
            let current = grid[yidx][xidx];
            if current == b'O' && *block > yidx + 1 {
                grid[*block - 1][xidx] = b'O';
                grid[yidx][xidx] = b'.';
                *block -= 1;
            }
            else if current == b'O' || current == b'#' { *block = yidx}
        }
    }
}

fn spin_cycle(grid: &mut Grid, n: usize) {
    for _ in 0..n {
        roll_north(grid);
        roll_west(grid);
        roll_south(grid);
        roll_east(grid);
    }
}

fn calculate_load(grid: &Grid) -> usize {
    let mut load = 1;
    let mut sum = 0;
    for line in grid.iter().rev() {
        for b in line {
            if *b == b'O' { sum += load }
        }
        load += 1;
    }
    sum
}

fn print_grid(grid: &Grid) {
    for line in grid {
        unsafe {
            println!("{:?}", std::str::from_utf8_unchecked(&line))
        }
    }
    println!("");
}
