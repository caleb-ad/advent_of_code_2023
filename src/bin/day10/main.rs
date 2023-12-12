use std::{io::Read, collections::VecDeque};

const LINE_LEN: usize = 141;
const LINE_NUM: usize = 140;
const OFFSETS: [i32; 4] = [-1, -1*LINE_LEN as i32, 1, LINE_LEN as i32];

fn main() {
    let mut input = String::from("\n");
    let _ = std::fs::File::open("src/bin/day10/input.txt")
        .expect("failed to open file")
        .read_to_string(&mut input)
        .expect("failed to read file");

    let locs: Vec<u8> = input.bytes().collect();


    let farthest = get_min_dist(&locs, locs.iter().position(|c| *c == b'S').unwrap())
        .iter()
        .fold(0, |max, n| if n.is_some() && n.unwrap() > max { n.unwrap() } else {max});

    println!("{:?}", farthest);
}

fn adjacents(pos: usize) -> impl Iterator<Item=Option<usize>> {
    OFFSETS.iter().map(move |o| {
        if let Some(a) = (pos as i32 + o).try_into().ok() {
            if a < LINE_LEN * LINE_NUM { return Some(a) }
        }
        None
    })
}

fn get_min_dist(map: &Vec<u8>, start: usize) -> Vec<Option<usize>> {
    let mut stack = VecDeque::from([start]);
    let mut dist = Vec::new();
    dist.resize_with(LINE_LEN * LINE_NUM, || None);
    dist[start] = Some(0);

    while stack.len() > 0 {
        let pos = stack.pop_front().unwrap();
        for (idx, oo) in adjacents(pos).enumerate() {
            if let Some(o) = oo {
                match (idx, map[o]) {
                    (0, b'F') | (0, b'L') | (0, b'-') |
                    (1, b'7') | (1, b'F') | (1, b'|') |
                    (2, b'J') | (2, b'7') | (2, b'-') |
                    (3, b'J') | (3, b'L') | (3, b'|') => {
                        if dist[o].is_none() || (dist[o].is_some() && dist[o].unwrap() > dist[pos].unwrap() + 1) {
                            dist[o] = Some(dist[pos].unwrap() + 1);
                            stack.push_back(o);
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    dist
}

// any tile on the edge which is not part of the loop must be outside the loop -> if we would index beyond the input return a "traversible" tile
// mark all the outside tiles, all the remaining unmarked tiles are inside
// squeezing between pipes,
