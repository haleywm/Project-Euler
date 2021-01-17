use std::io::stdin;

fn main() {
    // Reading one line of names in
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut names: Vec<&str> = input.split(',').map(|x| x.trim_matches(|x| x == '"' || x == '\n')).collect();

    names.sort();

    let total: u64 = names.into_iter().enumerate().map(|(i, name)| (i as u64 + 1) * string_pos_total(name)).sum();
    println!("{}", total);
}

fn string_pos_total(input: &str) -> u64 {
    // Gets the total of each value in the alphabet
    input.as_bytes().iter().map(|&x| x as u64 - 64).sum()
}