use std::io::Read;
use std::ops::{Index, IndexMut};

struct Grid<T: Sized> {
    g: Vec<T>,
    dim: (usize, usize) // (width, height)
}

impl<T: Sized> Index<usize> for Grid<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        &self.g[index * self.dim.0..(index + 1) * self.dim.0]
    }
}

impl<T: Sized> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.g[index * self.dim.0..(index + 1) * self.dim.0]
    }
}

impl<T: Sized> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.g[index.1 * self.dim.0 + index.0]
    }
}

impl<T: Sized> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.g[index.1 * self.dim.0 + index.0]
    }
}

impl std::fmt::Debug for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in self.lines() {
            unsafe { write!(f, "{:?}\n", std::str::from_utf8_unchecked(l))? }
        }
        write!(f, "")
    }
}

impl<T: Sized> Grid<T> {
    fn lines(&self) -> impl DoubleEndedIterator + Iterator<Item=&[T]> {
        self.g.chunks(self.dim.0)
    }

    fn width(&self) -> usize { self.dim.0 }
    fn height(&self) -> usize { self.dim.1 }
}

impl<T: Sized + Clone> Grid<T> {
    fn with_dim(dim: (usize, usize), val: T) -> Self {
        let mut g = Vec::new();
        g.resize(dim.0 * dim.1, val);
        Grid{g, dim}
    }
}

impl Grid<u8> {
    fn from_input(b: &[u8]) -> Self {
        let mut g = Vec::new();
        let width = b.iter().position(|b| *b == b'\n').expect("");
        for line in b.split(|b| *b == b'\n') {
            g.extend_from_slice(line);
        }
        Grid{g, dim: (width, b.len() / width)}
    }

    // fn hash(&self) -> u64 {
    //     let mut hasher = AHasher::default();
    //     self.g.as_slice().hash(&mut hasher);
    //     hasher.finish()
    // }
}


fn main() {
    let mut input = vec![];
    let _ = std::fs::File::open("src\\bin\\day16\\input.txt").expect("").read_to_end(&mut input);

    let grid = Grid::from_input(&input);

    let mut max = 0;

    for x in 0..grid.width() {
        let mut visited = Grid::with_dim((grid.width(), grid.height()), 0u8);
        propagate(&grid, &mut visited, (x,0), Direction::South);
        let energization = count_visited(&visited);
        if energization > max {
            max = energization;
        }

        let mut visited = Grid::with_dim((grid.width(), grid.height()), 0u8);
        propagate(&grid, &mut visited, (x, grid.height() - 1), Direction::North);
        let energization = count_visited(&visited);
        if energization > max {
            max = energization;
        }
    }

    for y in 0..grid.height() {
        let mut visited = Grid::with_dim((grid.width(), grid.height()), 0u8);
        propagate(&grid, &mut visited, (0, y), Direction::East);
        let energization = count_visited(&visited);
        if energization > max {
            max = energization;
        }

        let mut visited = Grid::with_dim((grid.width(), grid.height()), 0u8);
        propagate(&grid, &mut visited, (grid.width() - 1, y), Direction::West);
        let energization = count_visited(&visited);
        if energization > max {
            max = energization;
        }
    }

    println!("{:?}", max);
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Direction {
    North = 0x1,
    East = 0x2,
    South = 0x4,
    West = 0x8,
}

fn count_visited(visited: &Grid<u8>) -> usize {
    visited.g.iter().fold(0usize, |accum, v| if *v > 0 { accum + 1} else { accum })
}

fn propagate(grid: &Grid<u8>, visited: &mut Grid<u8>, idx: (usize, usize), dir: Direction) {

    _propagate(grid, visited, (Some(idx), dir));

    fn _propagate(grid: &Grid<u8>, visited: &mut Grid<u8>, state: (Option<(usize, usize)>, Direction)) {

        fn get_idx(grid: &Grid<u8>, from: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
            match dir {
                Direction::East if from.0 < grid.width() - 1 => Some((from.0 + 1, from.1)),
                Direction::West if from.0 > 0 => Some((from.0 - 1, from.1)),
                Direction::North if from.1 > 0 => Some((from.0, from.1 - 1)),
                Direction::South if from.1 < grid.height() - 1 => Some((from.0, from.1 + 1)),
                _ => None
            }
        }

        let mut state_queue = std::collections::VecDeque::new();
        state_queue.push_back(state);

        while state_queue.len() > 0 {
            let (Some(idx), dir) = state_queue.pop_front().unwrap() else { continue; };
            if visited[idx] & dir as u8 != 0 { continue; }
            visited[idx] |= dir as u8;

            match (grid[idx], dir) {
                (b'|', Direction::East) | (b'|', Direction::West) => {
                    state_queue.push_back((get_idx(grid, idx, Direction::North), Direction::North));
                    state_queue.push_back((get_idx(grid, idx, Direction::South), Direction::South));
                }
                (b'-', Direction::North) |  (b'-', Direction::South) => {
                    state_queue.push_back((get_idx(grid, idx, Direction::East), Direction::East));
                    state_queue.push_back((get_idx(grid, idx, Direction::West), Direction::West));
                }
                (b'\\', Direction::North) => state_queue.push_back((get_idx(grid, idx, Direction::West), Direction::West)),
                (b'\\', Direction::South) => state_queue.push_back((get_idx(grid, idx, Direction::East), Direction::East)),
                (b'\\', Direction::East) => state_queue.push_back((get_idx(grid, idx, Direction::South), Direction::South)),
                (b'\\', Direction::West) => state_queue.push_back((get_idx(grid, idx, Direction::North), Direction::North)),
                (b'/', Direction::North) => state_queue.push_back((get_idx(grid, idx, Direction::East), Direction::East)),
                (b'/', Direction::South) => state_queue.push_back((get_idx(grid, idx, Direction::West), Direction::West)),
                (b'/', Direction::East) => state_queue.push_back((get_idx(grid, idx, Direction::North), Direction::North)),
                (b'/', Direction::West) => state_queue.push_back((get_idx(grid, idx, Direction::South), Direction::South)),
                _ => state_queue.push_back((get_idx(grid, idx, dir), dir))
            }
        }
    }


}
