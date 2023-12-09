use std::io::Read;

fn main() {
    let mut input = String::from("\n");
    let _ = std::fs::File::open("src/bin/day9/input.txt")
        .expect("failed to open file")
        .read_to_string(&mut input)
        .expect("failed to read file");

    let mut histories = input.lines();
    histories.next();
    let mut sum = 0;
    for var in histories {
        let mut data: Vec<i64> = var.split_whitespace().map(|s| i64::from_str_radix(s, 10).expect("")).rev().collect();
        sum += extrapolate(&mut data);
    }
    println!("{:?}", sum);
}

fn extrapolate(d: &mut [i64]) -> i64 {
    for i in 1..d.len() {
        for j in (0..i).rev() {
            d[j] = d[j+1] - d[j]
        }
    }
    d.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::extrapolate;

    #[test]
    fn test_extrapolate() {
        assert_eq!(extrapolate(&mut[1,2]), 3);
        assert_eq!(extrapolate(&mut[1,2,5]), 10);
    }
}