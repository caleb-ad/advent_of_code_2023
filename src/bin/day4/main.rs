use std::io::Read;

fn main() {
    let mut input = String::from("\n");
    let _ = std::fs::File::open("src/bin/day4/input.txt")
        .expect("failed to open file")
        .read_to_string(&mut input)
        .expect("failed to read file");

    let mut values = [false; 100];
    let mut sum = 0;
    let mut winners = vec![];

    for line in input.lines() {
        if line.len() == 0 {continue;}
        let mut shift = None;
        let mut symbols = line.split_whitespace().skip(2).peekable();
        while symbols.peek() != Some(&"|") {
            let idx = usize::from_str_radix(symbols.next().unwrap(), 10).expect("");
            values[idx] = true;
            winners.push(idx);
        }
        symbols.next();
        while symbols.peek() != None  {
            if values[usize::from_str_radix(symbols.next().unwrap(), 10).expect("")] {
                shift = match shift {
                    None => Some(0),
                    Some(n) => Some(n+1)
                }
            }
        }
        while winners.len() > 0 {
            values[winners.pop().unwrap()] = false;
        }
        println!("{:?}", shift);
        sum += match shift {
            None => 0,
            Some(n) => 1 << n,
        }
    }
    println!("{:?}", sum);
}
