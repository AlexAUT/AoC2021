use std::str::FromStr;

use strum_macros::EnumString;


#[derive(Debug, EnumString)]
enum Direction {
    #[strum(serialize="forward")]
    Forward,
    #[strum(serialize="down")]
    Down,
    #[strum(serialize="up")]
    Up,
}

fn main() {
    let input = include_str!("input.txt");

    // Task1
    {
        let mut horizontal_position: i32 = 0;
        let mut depth: i32 = 0;


        for line in input.lines() {
            let (command, amount) = line.split_once(' ').expect("No valid command?");
            let command = Direction::from_str(command).expect("Not a valid command?");
            let amount: i32 = amount.parse().expect("Not a valid direction?");

            match command {
                Direction::Forward => horizontal_position += amount,
                Direction::Down => depth += amount,
                Direction::Up => depth -= amount
            }
        }

        println!("Horizontal position {} depth {}, result: {}", horizontal_position, depth, horizontal_position * depth);
    }

    // Task2
    {
        let mut aim: i32 = 0;
        let mut horizontal_position: i32 = 0;
        let mut depth: i32 = 0;

        for line in input.lines() {
            let (command, amount) = line.split_once(' ').expect("No valid command?");
            let command = Direction::from_str(command).expect("Not a valid command?");
            let amount: i32 = amount.parse().expect("Not a valid direction?");

            match command {
                Direction::Forward => {
                    horizontal_position += amount;
                    depth += amount * aim;
                },
                Direction::Down => aim += amount,
                Direction::Up => aim -= amount
            }
        }

        println!("Horizontal position {} depth {}, result: {}", horizontal_position, depth, horizontal_position * depth);
    }
}
