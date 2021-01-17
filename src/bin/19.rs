fn main() {
    const START: i32 = 1901;
    const END: i32 = 2001;

    let sundays = (START..END).fold((0, 5), |(total, to_next), year| {
        let (total_add, to_next) = sundays_in(to_next, is_leap_year(year));
        (total + total_add, to_next)
    });

    println!("{}", sundays.0);
}

fn sundays_in(mut to_next: i32, leap_year: bool) -> (i32, i32) {
    // Returns (num sundays in year, days till next sunday after year)
    let mut total = 0;
    const MONTHS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for &month in &MONTHS {
        let month = if month == 28 && leap_year {
            29
        } else { month };

        if to_next == 0 {
            total += 1;
        }
        to_next = (((to_next - month) % 7) + 7) % 7;
    }
    (total, to_next)
}

fn is_leap_year(year: i32) -> bool {
    // Returns true if year is leap year
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}