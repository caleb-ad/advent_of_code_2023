use std::io::Read;
use std::collections::HashMap;

const RANKING: &'static [u8] = &[b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'J', b'Q', b'K', b'A'];
const RBASE: u32 = RANKING.len() as u32;
const HAND_SIZE: u32 = 5;

#[derive(Debug, Clone, Copy)]
struct Hand<'a>{
    hand: &'a str,
    strength: u32,
    bid: u32,
}

impl<'a> Hand<'a> {
    fn new(hand: &'a str, bid: u32) -> Self {
        let mut digit = HAND_SIZE;
        let mut matches = [0u32; RBASE as usize];
        let mut strength: u32 = 0;
        for c in hand.bytes() {
            let cval = RANKING.iter().position(|cp| *cp == c).unwrap();
            //1 3 7 15 31
            //5 6 7 9 10 16 31
            matches[cval] += matches[cval] + 1;
            digit -= 1;
            strength += (cval as u32) << (digit * 4);
        }
        strength += matches.iter().sum::<u32>() << (HAND_SIZE * 4);
        Hand { hand, strength: strength, bid: bid}
    }
}

fn main() {
    let mut input = String::from("\n");
    let _ = std::fs::File::open("src/bin/day7/input.txt")
        .expect("failed to open file")
        .read_to_string(&mut input)
        .expect("failed to read file");

    let lines = input.lines();
    let mut hands = vec![];

    for line in lines {
        let mut hand_rank = line.split_whitespace();
        if let (Some(hand), Some(bid), None) = (hand_rank.next(), hand_rank.next(), hand_rank.next()) {
            hands.push(Hand::new(hand, u32::from_str_radix(bid, 10).expect("")))
        }
    }

    hands.sort_by_key(|h| h.strength);
    println!("{:#?}", hands);
    println!("score: {:?}", hands.iter().enumerate().fold(0, |score, (idx, h)| score + (idx as u32 + 1)*h.bid));
}

mod tests {
    use crate::Hand;


    #[test]
    fn test_hand_strengths() {
        let _h = Hand::new("AA8AA", 1);
        let _h = Hand::new("QQQQ2", 1);
    }
}
