use std::hash::Hasher;
use std::io::Read;
use std::ops::{Index, IndexMut};
use ahash::{AHasher, HashMap, HashMapExt};
use std::hash::Hash;

struct Grid {
    g: Vec<u8>,
    dim: (usize, usize) // (width, height)
}

impl Index<usize> for Grid {
    type Output = [u8];
    fn index(&self, index: usize) -> &Self::Output {
        &self.g[index * self.dim.0..(index + 1) * self.dim.0]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.g[index * self.dim.0..(index + 1) * self.dim.0]
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in self.lines() {
            unsafe { write!(f, "{:?}\n", std::str::from_utf8_unchecked(l))? }
        }
        write!(f, "")
    }
}

impl Grid {
    fn from_input(b: &[u8]) -> Self {
        let mut g = Vec::new();
        let width = b.iter().position(|b| *b == b'\n').expect("");
        for line in b.split(|b| *b == b'\n') {
            g.extend_from_slice(line);
        }
        Grid{g, dim: (width, b.len() / width)}
    }

    fn lines(&self) -> impl DoubleEndedIterator + Iterator<Item=&[u8]> {
        self.g.chunks(self.dim.0)
    }

    fn hash(&self) -> u64 {
        let mut hasher = AHasher::default();
        self.g.as_slice().hash(&mut hasher);
        hasher.finish()
    }

    fn width(&self) -> usize { self.dim.0 }
    fn height(&self) -> usize { self.dim.1 }
}

fn main() {
    let mut input = vec![];
    let _ = std::fs::File::open("C:\\Users\\caleb\\Documents\\Projects\\advent_of_code_2023\\src\\bin\\day14\\input.txt").expect("").read_to_end(&mut input);

    let mut grid = Grid::from_input(&input);

    // test load = 153
    spin_cycle(&mut grid, 1000_000_000);
    println!("load {:?}", calculate_load(&grid));
}

fn roll_north(grid: &mut Grid) {
    let mut block = Vec::new();
    block.resize(grid.width(), 0);

    for yidx in 0..grid.height() {
        for ( xidx, block) in (0..grid.width()).zip(block.iter_mut()) {
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
    block.resize(grid.height(), 0);

    for xidx in 0..grid.width() {
        for (yidx, block) in (0..grid.height()).zip(block.iter_mut()) {
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
    block.resize(grid.height(), grid.width());

    for xidx in (0..grid.width()).rev() {
        for (yidx, block) in (0..grid.height()).zip(block.iter_mut()) {
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
    block.resize(grid.width(), grid.height());

    for yidx in (0..grid.height()).rev() {
        for ( xidx, block) in (0..grid.width()).zip(block.iter_mut()) {
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
    let mut states: HashMap<u64, usize> = HashMap::new();
    let mut idx = 0;
    let mut cycle = None;
    for _ in 0..n {
        if let Some(pidx) = states.insert(grid.hash(), idx) {
            cycle = Some(pidx);
            break;
        }
        roll_north(grid);
        roll_west(grid);
        roll_south(grid);
        roll_east(grid);
        idx += 1;
    }
    if let Some(c) = cycle {
        for _ in 0..((n - idx) % (idx - c)) {
            roll_north(grid);
            roll_west(grid);
            roll_south(grid);
            roll_east(grid);
        }
    }
}

fn calculate_load(grid: &Grid) -> usize {
    let mut load = 1;
    let mut sum = 0;
    for line in grid.lines().rev() {
        for b in line {
            if *b == b'O' { sum += load }
        }
        load += 1;
    }
    sum
}
