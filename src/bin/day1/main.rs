use std::io::Read;

fn main() {
    let mut input = String::from("\n");
    let _ = std::fs::File::open("src/bin/day1/input.txt")
        .expect("failed to open file")
        .read_to_string(&mut input)
        .expect("failed to read file");

    let chars = input.chars().collect::<Vec<char>>();
    let mut rin = chars.as_slice();
    let mut next = (None, None);
    let mut sum = 0;
    while rin.len().clone() > 0 {
        match (next, rin[0].clone(), find_digit(&mut rin)) {
            ((Some(d2), Some(d1)), '\n', _) => {sum += 10*d2 + d1; next = (None, None)},
            ((Some(d1), None), '\n', _) => {sum += 10*d1 + d1; next = (None, None)},
            ((None, None), _, Some(val)) => {next = (Some(val), None)}
            ((Some(_), None), _, Some(val)) | ((Some(_), Some(_)), _, Some(val)) => {next.1 = Some(val)}
            (_, _, None) => (),
            ((None, Some(_)), _, _) => unreachable!(),
        }
    }
    println!("{:}", sum);
}

fn find_digit<'a, 'b: 'a>(s: &'a mut &'b [char]) -> Option<i32> {
    if s.len() == 0 {return None}
    match s {
        ['o', 'n', 'e', ..] => {*s = &s[3..]; Some(1)},
        ['t', 'w', 'o', ..] => {*s = &s[3..]; Some(2)},
        ['t', 'h', 'r', 'e', 'e', ..] => {*s = &s[5..]; Some(3)},
        ['f', 'o', 'u', 'r', ..] => {*s = &s[4..]; Some(4)},
        ['f', 'i', 'v', 'e', ..] => {*s = &s[4..]; Some(5)},
        ['s', 'i', 'x', ..] => {*s = &s[3..]; Some(6)},
        ['s', 'e', 'v', 'e', 'n', ..] => {*s = &s[5..]; Some(7)},
        ['e', 'i', 'g', 'h', 't', ..] => {*s = &s[5..]; Some(8)},
        ['n', 'i', 'n', 'e', ..] => {*s = &s[4..]; Some(9)},
        [f, ..] if f.is_numeric() => {*s = &s[1..]; Some(f.to_digit(10).unwrap().try_into().expect(""))},
        _ => {*s = &s[1..]; None},
    }
}
