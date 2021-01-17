use std::collections::HashSet;

fn main() {
    // The total number of numbers in any range must equal 9
    // Using heaps algoirithm to test all possible combinations
    let mut numbers: Vec<u32> = (1..=9).collect();
    let mut fits: HashSet<u32> = HashSet::new();
    //find_pandigital(&vec![3, 9, 1, 8, 6, 7, 2, 5, 4], &mut fits);
    rec_arrange(numbers.len(), &mut numbers, &mut fits);

    println!("{:?}", fits.into_iter().sum::<u32>());
}

fn rec_arrange(k: usize, a: &mut Vec<u32>, numbers: &mut HashSet<u32>) {
    if k == 1 {
        // Base case, process this iteration of the array
        // Finding any values that multiply here
        find_pandigital(a, numbers);
    }
    else {
        // Generate perms with kth unaltered

        for i in 0..k - 1 {
            rec_arrange(k - 1, a, numbers);
            if k % 2 == 1 {
                a.swap(0, k - 1);
            }
            else {
                a.swap(i, k - 1);
            }
        }
        rec_arrange(k - 1, a, numbers);

    }
}

fn find_pandigital(vals: &Vec<u32>, to_add: &mut HashSet<u32>) {
    for point_a in 0..vals.len() - 2 {
        for point_b in point_a + 1..vals.len() {
            let mult_a = get_total(0, point_a, vals);
            let mult_b = get_total(point_a, point_b, vals);
            let total = get_total(point_b, vals.len(), vals);
            //println!("{} * {} = {}", mult_a, mult_b, total);
            if mult_a * mult_b == total {
                to_add.insert(total);
            }
        }
    }
}

fn get_total(start: usize, end: usize, vals: &Vec<u32>) -> u32 {
    let mut total = 0;
    for i in start..end {
        total = total * 10 + vals[i];
    }

    total
}