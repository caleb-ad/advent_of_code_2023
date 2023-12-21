use std::io::Read;
use std::hash::Hasher;

struct ReindeerHash {
    current: usize
}

impl Hasher for ReindeerHash {
    fn write(&mut self, bytes: &[u8]) {
        // (a + b)%n = a%n + b%n ?
        // self.current = ((self.current + bytes.iter().map(|b| *b as usize).sum::<usize>()) * 17) % 256
        for b in bytes {
            self.current = ((self.current + *b as usize) * 17) % 256
        }
    }

    fn finish(&self) -> u64 { self.current as u64 }
}

impl Default for ReindeerHash {
    fn default() -> Self {
        ReindeerHash {current: 0}
    }
}

fn main() {
    let mut input = vec![];
    let _ = std::fs::File::open("src\\bin\\day15\\input.txt").expect("").read_to_end(&mut input);
    let input: Vec<u8> = input.into_iter().filter(|b| *b != b'\n').collect();

    let mut boxes: Vec<Vec<(&[u8], u8)>> = Vec::new();
    boxes.resize(256, Vec::new());

    for instr in input.split(|b| *b == b',') {
        let op_idx = instr.iter().position(|c| !c.is_ascii_alphabetic()).unwrap();

        let mut hash = ReindeerHash::default();
        hash.write(&instr[..op_idx]);
        let box_idx = hash.finish() as usize;

        if let Some(pos) = boxes[box_idx].iter().position(|(label, _)| *label == &instr[..op_idx]) {
            match instr[op_idx] {
                b'=' => {
                    boxes[box_idx][pos] = (&instr[..op_idx], instr[op_idx + 1] - 48);
                }
                b'-' => {
                    boxes[box_idx].remove(pos);
                }
                _ => {}
            }
        } else if instr[op_idx] == b'=' {
            boxes[box_idx].push((&instr[..op_idx], instr[op_idx + 1] - 48))
        }
    }

    let mut focus = 0;
    for (bidx, b) in boxes.iter().enumerate() {
        for (lidx, lens) in b.iter().enumerate() {
            focus += (bidx + 1) * (lidx + 1) * (lens.1 as usize);
        }
    }

    println!("focus {:?}", focus);
}

#[cfg(test)]
mod tests {
    use std::hash::Hasher;

    use super::ReindeerHash;

    #[test]
    fn test_reindeer_hash() {
        let mut h = ReindeerHash::default();
        h.write("rn=1".as_bytes());
        assert_eq!(h.finish(), 30);

        let mut h = ReindeerHash::default();
        h.write("cm-".as_bytes());
        assert_eq!(h.finish(), 253);

        let mut h = ReindeerHash::default();
        h.write("qp=3".as_bytes());
        assert_eq!(h.finish(), 97);

        let mut h = ReindeerHash::default();
        h.write("rn".as_bytes());
        assert_eq!(h.finish(), 0);
    }
}