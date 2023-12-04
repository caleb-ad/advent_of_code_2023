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

    let lines = input.lines();
    let mut cards = vec![1];

    for (card_no, line) in lines.enumerate() {
        if line.len() == 0 {continue;}
        if cards.len() <= card_no {cards.resize_with(card_no + 1, || 1)}
        let mut card_won = 1;
        let mut symbols = line.split_whitespace().skip(2).peekable();
        while symbols.peek() != Some(&"|") {
            let idx = usize::from_str_radix(symbols.next().unwrap(), 10).expect("");
            values[idx] = true;
            winners.push(idx);
        }
        symbols.next();
        while symbols.peek() != None  {
            if values[usize::from_str_radix(symbols.next().unwrap(), 10).expect("")] {
                if card_no + card_won >= cards.len() {cards.resize_with(card_no + card_won + 1, || 1)}
                cards[card_no + card_won] += cards[card_no];
                card_won += 1;
            }
        }
        while winners.len() > 0 {
            values[winners.pop().unwrap()] = false;
        }
        sum += cards[card_no];
    }
    println!("{:?}", sum);
}
