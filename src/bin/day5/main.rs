use std::io::Read;

fn main() {
    let mut input = String::from("\n");
    let _ = std::fs::File::open("src/bin/day5/input.txt")
        .expect("failed to open file")
        .read_to_string(&mut input)
        .expect("failed to read file");


    let mut lines = input.lines();
    lines.next();
    let seed_p: Vec<u64> = lines.next().unwrap().split_whitespace().skip(1). map(|s| u64::from_str_radix(s, 10).expect("")).collect();
    let mut seeds: Vec<(u64, u64)> = seed_p.chunks(2).map(|c| (c[0], c[0] + c[1])).collect();

    while let Some(l) = lines.next() {
        if l.len() == 0 {continue;}
        map_values(&mut lines, &mut seeds);
    }

    seeds.sort_unstable_by_key(|a| a.0);
    println!("minimal location {:?}", seeds[0]);
}

/// vals must be sorted
/// fails if any map ranges overlap
fn map_values<'a>(map: &mut impl Iterator<Item = &'a str>, vals: &mut Vec<(u64, u64)>) {
    let mut og_vals = Vec::new();
    while let Some(range) = map.next() {
        if range.len() == 0 {break;}
        let mut range_iter = range.split_whitespace().map(|a| u64::from_str_radix(a, 10).expect(""));
        let (start_dst, start_src, len) = (range_iter.next().unwrap(), range_iter.next().unwrap(), range_iter.next().unwrap());

        let mut idx = 0;
        let mut slen = vals.len();
        while idx < slen {
            if vals[idx].0 > vals[idx].1 {panic!("{:?}", vals[idx])}
            match (vals[idx].1 >= start_src + len, vals[idx].1 >= start_src, vals[idx].0 < start_src, vals[idx].0 < start_src + len) {
                (true, _, true, _) => {
                    vals.push((start_src + len, vals[idx].1));
                    og_vals.push((start_dst, start_dst + len));
                    vals[idx] = (vals[idx].0, start_src);
                    idx += 1;
                }
                (true, _, false, true) => {
                    og_vals.push((vals[idx].0 - start_src + start_dst, start_dst + len));
                    vals[idx] = (start_src + len, vals[idx].1);
                    idx += 1;
                }
                (false, true, true, _) => {
                    og_vals.push((start_dst, vals[idx].1 - start_src + start_dst));
                    vals[idx] = (vals[idx].0, start_src);
                    idx += 1;
                }
                (false, _, false, _) => {
                    og_vals.push((vals[idx].0 - start_src + start_dst, vals[idx].1 - start_src + start_dst));
                    vals.remove(idx);
                    slen -= 1;
                }
                _ => idx += 1,
            }
        }
    }
    vals.extend(og_vals);
}
