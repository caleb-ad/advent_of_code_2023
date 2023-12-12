use std::io::Read;


fn main() {
    let mut input = String::from("\n");
    let _ = std::fs::File::open("src/bin/day11/input.txt")
        .expect("failed to open file")
        .read_to_string(&mut input)
        .expect("failed to read file");


    let image: Vec<Vec<u8>> = input.lines().skip(1).map(|s| Vec::from(s.as_bytes())).collect();
    let row_voids = empty_rows(&image);
    let col_voids = empty_cols(&image);
    let galaxies = get_galaxies(&image);

    // println!("row {:?}", row_voids);
    // println!("col {:?}", col_voids);
    // println!("g {:?}", galaxies);

    // assume rectangular universe
    let mut sum = 0;
    for (idx, galaxy) in galaxies.iter().enumerate() {
        for other in galaxies[idx+1..].iter() {
            // println!("{:?} to {:?}: {:?}", galaxy, other, galactic_distance(galaxy, other, &row_voids, &col_voids));
            sum += galactic_distance(galaxy, other, &row_voids, &col_voids);
        }
    }
    println!("sum: {:?}", sum);
}

fn get_galaxies(image: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut g = vec![];
    for (y, row) in image.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != b'.' { g.push((x, y)) }
        }
    }
    g
}

fn galactic_distance(a: &(usize, usize), b: &(usize, usize), row_voids: &Vec<(usize, usize)>, col_voids: &Vec<(usize, usize)>) -> usize {
    row_voids.iter().filter(|range| belongs_to(range.0, (a.1, b.1))).fold(0, |sum, void| sum + void.1 * (999999)) +
    col_voids.iter().filter(|range| belongs_to(range.0, (a.0, b.0))).fold(0, |sum, void| sum + void.1 * (999999)) +
    range_len((a.1, b.1)) + range_len((a.0, b.0))
}

fn belongs_to(val: usize, interval: (usize, usize)) -> bool {
    (val > interval.0 && val < interval.1) || (val > interval.1 && val < interval.0)
}

fn range_len(r: (usize, usize)) -> usize {
    if r.0 > r.1 { r.0 - r.1 }
    else {r.1 - r.0 }
}

fn empty_rows(image: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut void = (0, 0);
    let mut empty = vec![];
    for (idx, row) in image.iter().enumerate() {
        if row.iter().find(|b| **b != b'.').is_none() {
            if void.1 == 0 { void.0 = idx }
            void.1 += 1;
        }
        else if void.1 > 0 {
            empty.push(void);
            void = (0, 0);
        }
    }
    // trailing voids don't matter, we don't have to push void to empty here
    empty
}

fn empty_cols(image: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut void = (0, 0);
    let mut empty = vec![];
    let cols = Columns::new(image, image[0].len());
    for (idx, mut col) in cols.enumerate() {
        if col.find(|b| **b != b'.').is_none() {
            if void.1 == 0 { void.0 = idx }
            void.1 += 1;
        }
        else if void.1 > 0 {
            empty.push(void);
            void = (0, 0);
        }
    }
    empty
}

struct Columns<'a, T> {
    data: &'a Vec<Vec<T>>,
    dim: usize, // num columns
    idx: usize,
}


impl<'a, T>  Columns<'a, T> {
    fn new<'b:'a>(v: &'b Vec<Vec<T>>, dim: usize) -> Self {
        Columns{data: v, dim, idx: 0}
    }
}

impl<'a, T> Iterator for Columns<'a, T> {
    type Item = SparseVec<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.dim {
            self.idx += 1;
            Some(SparseVec{data: self.data, len: self.data.len(), offset: self.idx - 1, idx: 0})
        } else { None }
    }
}

// not really a vec tbh
struct SparseVec<'a, T> {
    data: &'a Vec<Vec<T>>,
    len: usize,
    offset: usize,
    idx: usize,
}

impl<'a, T> Iterator for SparseVec<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.len {
            self.idx += 1;
            self.data[self.idx - 1].get(self.offset)
        } else { None }
    }
}

