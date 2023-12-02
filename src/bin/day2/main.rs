use std::io::Read;

fn main() {
    let mut input = String::from("\n");
    let _ = std::fs::File::open("src/bin/day2/input.txt")
        .expect("failed to open file")
        .read_to_string(&mut input)
        .expect("failed to read file");

    const MAX_COUNTS: (u32, u32, u32) = (12, 13, 14);
    let mut sum = 0;

    for line in input.split(|c| c == '\n') {
        let mut tokens = line.split(|c| c == ' ');
        let Some(str_id) = tokens.nth(1) else { continue; };
        let id = u32::from_str_radix(str_id.strip_suffix(':').unwrap(), 10).expect("");
        let mut current = (0, 0, 0);
        let mut valid = 1;

        while let Some(amount) = tokens.next() {
            let (cnt, check) = get_color(&mut tokens, &mut current);
            *cnt += u32::from_str_radix(amount, 10).expect("");
            if check {
                if current.0 > MAX_COUNTS.0 || current.1 > MAX_COUNTS.1 || current.2 > MAX_COUNTS.2 {
                    valid = 0;
                    break;
                }
                current = (0, 0, 0);
            }
        }
        sum += id * valid;
    }
    println!("{:?}", sum)
}

fn get_color<'a, 'b>(tokens: &mut impl Iterator<Item=&'a str>, amnt: &'b mut (u32, u32, u32)) -> (&'b mut u32, bool) {
    let code = tokens.next().unwrap();
    (match &code[0..1] {
        "r" => &mut amnt.0,
        "g" => &mut amnt.1,
        "b" => &mut amnt.2,
        a => panic!("unexpected color {:?}", a)
    },
    match &code[code.len()-1..] {
        "," => false,
        _ => true,
    })
}
