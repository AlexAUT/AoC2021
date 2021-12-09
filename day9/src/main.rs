use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
    let input = include_str!("input.txt");

    let cols = input.lines().next().unwrap().chars().count();

    let height_map: Vec<u8> = input
        .lines()
        .flat_map(|row| row.chars().map(|x| x.to_digit(10).unwrap() as u8))
        .collect();

    let rows = height_map.len() / cols;

    let slot = |col: usize, row: usize| col + row * cols;

    let mut risk: u32 = 0;

    let mut basins: Vec<(usize, usize)> = Vec::new();
    for col in 0..cols {
        for row in 0..rows {
            let v = height_map[slot(col, row)];
            let mut local_minima = true;

            if col != 0 && v >= height_map[slot(col - 1, row)] {
                local_minima = false;
            }
            if col != cols - 1 && v >= height_map[slot(col + 1, row)] {
                local_minima = false;
            }
            if row != 0 && v >= height_map[slot(col, row - 1)] {
                local_minima = false;
            }
            if row != rows - 1 && v >= height_map[slot(col, row + 1)] {
                local_minima = false;
            }

            if local_minima {
                risk += (v + 1) as u32;
                basins.push((col, row));
            }
        }
    }
    println!("[Task1] risk: {}", risk);

    let mut basin_sizes: Vec<u32> = Vec::new();
    basin_sizes.reserve(basins.len());
    // Calculate size of basins
    for (col, row) in basins {
        let mut search_stack: Vec<(usize, usize)> = vec![(col, row); 1];
        let mut visited_slots: HashSet<usize> = HashSet::new();
        let mut basin_size = 0;

        while !search_stack.is_empty() {
            let (col, row) = search_stack.pop().unwrap();
            let inserted = visited_slots.insert(slot(col, row));
            if !inserted {
                continue;
            }
            let v = height_map[slot(col, row)];
            if v == 9 { 
                continue;
            }
            basin_size += 1;

            if col != 0 && v < height_map[slot(col - 1, row)] {
                search_stack.push((col - 1, row));
            }
            if col != cols - 1 && v < height_map[slot(col + 1, row)] {
                search_stack.push((col + 1, row));
            }
            if row != 0 && v < height_map[slot(col, row - 1)] {
                search_stack.push((col, row - 1));
            }
            if row != rows - 1 && v < height_map[slot(col, row + 1)] {
                search_stack.push((col, row + 1));
            }
        }
        basin_sizes.push(basin_size);
    }

    basin_sizes.sort();
    let task2_output = basin_sizes.iter().rev().take(3).fold(1 as u64, |acc, &x| acc.checked_mul(x as u64).unwrap());

    println!("[Task2]: {}", task2_output);
}
