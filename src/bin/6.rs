fn main() {
    const TO: u64 = 100;

    let sum: u64 = (1..=TO).map(|x| x.pow(2)).sum();
    let square: u64 = (TO * (TO + 1) / 2).pow(2);
    let result = square - sum;

    println!("{}", result);
}