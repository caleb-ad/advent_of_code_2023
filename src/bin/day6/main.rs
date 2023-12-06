
#[derive(Debug, Copy, Clone)]
struct Race {
    time: u64,
    record: u64,
}

fn main() {
    // let input1 = [Race{time: 47, record: 400}, Race{time: 98, record: 1213}, Race{time: 66, record: 1011}, Race{time: 98, record: 1540}];
    let input = [Race{time: 47986698, record: 400121310111540}];


    // ht + d/v = rt
    // v = ht
    // d = ht(rt - ht)
    // record = ht*rt - ht^2
    // ht = (-rt +- sqrt(rt^2 - 4*record)) / -2
    let mut sum = 1;
    for race in input.iter() {
        let Race { time,record } = *race;
        let rt: f64 = time as f64;
        let d: f64 = record as f64;
        let ht = (rt / 2.0 - f64::sqrt(rt*rt - 4.0*d)/2.0, rt / 2.0 + f64::sqrt(rt*rt - 4.0*d)/2.0);
        println!("{:?} - {:?}", ht.0.ceil(), ht.1.floor());
        sum *= ht.1.floor() as u64 - ht.0.ceil() as u64 + 1;
    }
    println!("{:?}", sum)

}