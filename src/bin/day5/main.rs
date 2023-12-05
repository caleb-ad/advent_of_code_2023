use std::io::Read;

fn main() {
    let mut input = String::from("\n");
    let _ = std::fs::File::open("src/bin/day5/input.txt")
        .expect("failed to open file")
        .read_to_string(&mut input)
        .expect("failed to read file");


    let mut lines = input.lines();
    lines.next();
    let mut seeds: Vec<u64> = lines.next().unwrap().split_whitespace().skip(1).map(|s| u64::from_str_radix(s, 10).expect("")).collect();

    while let Some(l) = lines.next() {
        if l.len() == 0 {continue;}
        println!("{:?} {:?}", l, seeds[0]);
        map_values(&mut lines, &mut seeds);
    }

    seeds.sort_unstable();
    println!("minimal location {:?}", seeds[0]);
}

/// vals must be sorted
/// fails if any map ranges overlap
fn map_values<'a>(map: &mut impl Iterator<Item = &'a str>, vals: &mut [u64]) {
    let og_vals = Vec::from(&(*vals));
    while let Some(range) = map.next() {
        if range.len() == 0 {return}
        let mut range_iter = range.split_whitespace().map(|a| u64::from_str_radix(a, 10).expect(""));
        let (start_dst, start_src, len) = (range_iter.next().unwrap(), range_iter.next().unwrap(), range_iter.next().unwrap());
        vals.iter_mut()
            .enumerate()
            // because 'start_src + len' may overflow we short circuit the comparison
            .filter(|(idx, _)| og_vals[*idx] >= start_src && og_vals[*idx] < start_src + len)
            .for_each(|(_, val)| *val = *val - start_src + start_dst);
    }
}
