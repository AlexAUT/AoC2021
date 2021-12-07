fn main() {
    let input = include_str!("input.txt");

    let input: Vec<i32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    // Task1
    {
        let mut sorted = input.clone();
        sorted.sort();
        let median = sorted[sorted.len() / 2];
        println!("Sorted: {}", median);

        let fuel_consumption: i32 = sorted.iter().map(|x| (x - median).abs()).sum();
        println!("[Task1] Fuel consumption: {}", fuel_consumption);
    }

    // Task2
    {
        let sum: i64 = input.iter().map(|&x| x as i64).sum();
        let avg: i32 = (sum as f64 / input.len() as f64).floor() as i32;
        let fuel_consumption: i32 = input.iter().map(|x| sum_to_n((x - avg).abs())).sum();
        println!("[Task2] Fuel consumption: {}", fuel_consumption);
    }
}

fn sum_to_n(n: i32) -> i32 {
    (n * (n + 1)) / 2
}
