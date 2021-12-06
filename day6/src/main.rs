fn main() {
    let input = include_str!("input.txt");

    // Task 1
    let lut_task1 = calculate_lut_for_growth(80);
    let number_of_fishes: u64 = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| lut_task1[x.parse::<usize>().unwrap()] as u64)
        .sum();
    println!("[Task2] Number of fishes: {}", number_of_fishes);

    let lut_task2 = calculate_lut_for_growth(256);
    let number_of_fishes: u128 = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| lut_task2[x.parse::<usize>().unwrap()] as u128)
        .sum();
    println!("[Task1] Number of fishes: {}", number_of_fishes);
}

fn calculate_lut_for_growth(number_of_days: u32) -> Vec<u64> {
    let mut fishes: Vec<u8> = Vec::new();
    fishes.reserve(1024);
    fishes.push(6);

    let mut lut: Vec<u64> = Vec::new();

    let mut buckets: Vec<u64> = vec![0;9];
    buckets[6] = 1;

    // Simulate 80 days for a single fish to generate a lookup table
    for day in 1..number_of_days + 7 {
        buckets.rotate_left(1);

        // Will rotate the 0 to 8 so the number in 8 gives us the new_count (also they should get
        // to 6 instead of 8
        buckets[6] += buckets[8];

        // Store the last 7 days
        if day >= number_of_days {
            lut.push(buckets.iter().sum());
        }
        println!("Day {}", day);
    }
    lut.reverse();
    lut
}
