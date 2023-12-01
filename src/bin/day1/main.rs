use std::io::Read;

fn main() {
    let mut input = String::from("\n");
    let _ = std::fs::File::open("src/bin/day1/input.txt")
        .expect("failed to open file")
        .read_to_string(&mut input)
        .expect("failed to read file");

    let vals: Vec<char> = input.chars().filter(|c| c.is_numeric() || *c == '\n').collect();
    let mut output = String::new();
    for idx in 0..vals.len()-1 {
        match vals[idx..idx+2] {
            ['\n', c] => output.push(c),
            [c, '\n'] => {output.push(c); output.push('\n');},
            _ => (),
        }
    }
    let sum = output.split('\n').fold(0, |sum, val| sum + i32::from_str_radix(val, 10).unwrap_or(0));
    println!("{:}", sum);
}

fn find_digit<'a, 'b: 'a>(s: &'b mut &'a mut [char]) -> Option<i32> {
    if s.len() == 0 {return None}
    match s {
        _ if s[0].is_numeric() => {*s = &mut s[1..]; Some(s[0].to_digit(10).unwrap().try_into().expect(""))},
        ['o', 'n', 'e', ..] => {*s = &mut s[3..]; Some(1)},
        ['t', 'w', 'o', ..] => {*s = &mut s[3..]; Some(2)},
        ['t', 'h', 'r', 'e', 'e', ..] => {*s = &mut s[5..]; Some(3)},
        ['f', 'o', 'u', 'r', ..] => {*s = &mut s[4..]; Some(4)},
        ['f', 'i', 'v', 'e', ..] => {*s = &mut s[4..]; Some(5)},
        ['s', 'i', 'x', ..] => {*s = &mut s[3..]; Some(6)},
        ['s', 'e', 'v', 'e', 'n', ..] => {*s = &mut s[5..]; Some(7)},
        ['e', 'i', 'g', 'h', 't', ..] => {*s = &mut s[5..]; Some(8)},
        ['n', 'i', 'n', 'e', ..] => {*s = &mut s[4..]; Some(9)},
        _ => None,
    }
}
