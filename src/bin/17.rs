fn main() {
    const HIGH: u32 = 1000;
    let result: u32 = (1..=HIGH).map(|x| num_to_letter_count(x)).sum();

    println!("{}", result);
}

fn num_to_letter_count(num: u32) -> u32 {
    // Takes a number, and returns the number of letters it's text representation would have in words
    // 5 -> 'five' -> 4
    // 123 -> 'one hundred and twenty three' -> 24
    let mut total = 0;
    // First, do hundreds and thousands, they're easy
    // 'thousand' -> 8 'hundred' = 7

    for &(div, add) in &[(1000, 8), (100, 7)] {
        let amount = num / div % 10;
        if amount > 0 {
            total += two_digits_count(amount) + add;
        }
    }
    // Add 'and' if there is something more and less than 100
    if (num > 100 && num % 100 != 0) || (num > 1000 && num % 1000 != 0) {
        total += 3;
    }

    // Then lastly last two digits
    total += two_digits_count(num % 100);

    total
}

fn two_digits_count(num: u32) -> u32 {
    assert!(num <= 100);
    let tens = num / 10;
    let ones = num % 10;
    /*
    one 3
    two 3
    three 5
    four 4
    five 4
    six 3
    seven 5
    eight 5
    nine 4
    ten 3
    eleven 6
    twelve 6
    thirteen 8
    fourteen 8
    fifteen 7
    sixteen 7
    seventeen 9
    eighteen 8
    nineteen 8
    twenty 6
    thirty 6
    forty 5
    fifty 5
    sixty 5
    seventy 7
    eighty 6
    ninety 6
    */
    let total = 
    if tens != 1 {
        // Not teens
        match ones {
            0 => 0,
            1 => 3,
            2 => 3,
            3 => 5,
            4 => 4,
            5 => 4,
            6 => 3,
            7 => 5,
            8 => 5,
            9 => 4,
            _ => panic!("What"),
        }
    }
    else {
        // Teens
        match ones {
            0 => 3,
            1 => 6,
            2 => 6,
            3 => 8,
            4 => 8,
            5 => 7,
            6 => 7,
            7 => 9,
            8 => 8,
            9 => 8,
            _ => panic!("What"),
        }
    } + match tens {
        0 => 0,
        1 => 0,
        2 => 6,
        3 => 6,
        4 => 5,
        5 => 5,
        6 => 5,
        7 => 7,
        8 => 6,
        9 => 6,
        _ => panic!("What"),
    };
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        assert_eq!(num_to_letter_count(1), 3);
        assert_eq!(num_to_letter_count(3), 5);
        assert_eq!(num_to_letter_count(5), 4);
        assert_eq!(num_to_letter_count(342), 23);
        assert_eq!(num_to_letter_count(115), 20);
        assert_eq!(num_to_letter_count(1000), 11);
    }
}