struct DisplayInput<'a> {
    input: &'a str,
    output: &'a str,
}

fn main() {
    let input = include_str!("input.txt");
    let sequences: Vec<DisplayInput> = input
        .lines()
        .map(|x| {
            let (input, output) = x.split_once('|').unwrap();
            DisplayInput {
                input: input.trim(),
                output: output.trim(),
            }
        })
        .collect();

    // Task1
    {
        let task1: u32 = sequences
            .iter()
            .map(|x| {
                x.output
                    .split(' ')
                    .map(|y| match y.len() {
                        2 | 3 | 4 | 7 => 1,
                        _ => 0,
                    })
                    .sum::<u32>()
            })
            .sum::<u32>();
        println!("Number of 1,4,7,8 outputs: {}", task1);
    }

    // Task2
    {
        let mut task2_sum: u32 = 0;
        for sequence in sequences {
            // Store fixed inputs
            let mut lookups = vec![""; 10];
            for number_input in sequence.input.split(' ') {
                match number_input.chars().count() {
                    2 => {
                        lookups[1] = number_input.into();
                    }
                    3 => {
                        lookups[7] = number_input.into();
                    }
                    4 => {
                        lookups[4] = number_input.into();
                    }
                    _ => {}
                }
            }

            let mut output: String = String::new();
            for to_match in sequence.output.split(' ') {
                output = format!("{}{}", output, match to_match.chars().count() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    5 => {
                        let mut found = 2;
                        // Check if it contains a 1 => its 3, if contains (4 - 1) => 5 otherwise 2
                        if lookups[1].chars().all(|x| to_match.chars().any(|y| x == y)) {
                            found = 3;
                        } else if lookups[4].chars().all(|x| to_match.chars().any(|y| x == y) || lookups[1].chars().any(|y| x == y)) {
                            found =  5;
                        }
                        found
                    }
                    6 => {
                        // Check if it matches with 4
                        let mut found = 6;
                        // Check if it contains a 4 => its 9, if contains 1 => 0 otherwise 6
                        if lookups[4].chars().all(|x| to_match.chars().any(|y| x == y)) {
                            found = 9;
                        } else if lookups[1].chars().all(|x| to_match.chars().any(|y| x == y)) {
                            found =  0;
                        }
                        found
                    }
                    _ => panic!("Could not match!"),
                });
            }
            task2_sum += output.parse::<u32>().unwrap();
        }

        println!("[Task2]: {}", task2_sum);
    }
}
