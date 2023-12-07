use std::io::Read;

const RANKING: &'static [u8] = &[b'J', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'Q', b'K', b'A'];
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
        let mut joker_count = 0;
        let mut joker_mask = 0;

        for c in hand.bytes() {
            let cval = RANKING.iter().position(|cp| *cp == c).unwrap();
            //1 3 7 15 31
            //5 6 7 9 10 16 31
            if c != b'J' { matches[cval] += matches[cval] + 1 }
            else {
                joker_count += 1;
                joker_mask = (joker_mask << 1) | 1;
            }
            digit -= 1;
            strength += (cval as u32) << (digit * 4);
        }

        // make best hand possible with jokers
        let mut matches_sum = 0;
        let mut max = 0;
        for dup in matches {
            if dup > max {
                matches_sum += max;
                max = dup;
            } else { matches_sum += dup; }
        }
        matches_sum = matches_sum + max*(1 << joker_count) + joker_mask;

        strength += matches_sum << (HAND_SIZE * 4);
        // println!("{:x}", strength);
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
        let _h = Hand::new("KJTJT", 1);
        let _h = Hand::new("JJ8J3", 1);
        let _h = Hand::new("JJ66J", 1);
        let _h = Hand::new("JJJJJ", 1);
        let _h = Hand::new("22222", 1);
    }
}
