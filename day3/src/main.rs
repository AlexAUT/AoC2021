fn main() {
    let input = include_str!("input.txt");

    // Check same length
    let length_of_first = input.lines().next().expect("File empty?").len();
    if !input.lines().all(|x| x.len() == length_of_first) {
        panic!("Input sizes differ!");
    }

    // Task 1
    {
        let mut byte_votings: Vec<i32> = vec![0; length_of_first];
        
        for line in input.lines() {
            for (index, char) in line.chars().enumerate() {
                match char {
                    '0' => byte_votings[index] -= 1,
                    '1' => byte_votings[index] += 1,
                    _ => panic!("Invalid input!")
                }
            }
        }

        let mut gamma: u32 = 0;
        let mut epsilon: u32 = 0;

        for (index, vote) in byte_votings.iter().enumerate() {
            match vote {
                v if *v > 0 => gamma |= 1 << (length_of_first - index - 1),
                v if *v < 0 => epsilon |= 1 << (length_of_first - index - 1),
                _ => panic!("Both bits same number of times")
            }
        }

        println!("Gamma: {}, Delta: {}, Results {}", gamma, epsilon, gamma as u64 * epsilon as u64);
    }

    // Task 2
    {
        // let mut oxy_candidates: Vec<&str> = input.lines().collect();
        let oxy = filter_candidates(input.lines().collect(), &|x, y| x == y);
        let co2 = filter_candidates(input.lines().collect(), &|x, y| x != y);
        println!("Oxy: {}, {}, {}", oxy, co2, u64::from_str_radix(oxy, 2).unwrap() * u64::from_str_radix(co2, 2).unwrap());
    }
    
}

fn filter_candidates<'a>(mut candidates: Vec<&'a str>, predicate: &dyn Fn(char, char) -> bool) -> &'a str {
    let mut bit_index: usize = 0;
    while candidates.len() > 1 { 
        let mut voting: i32 = 0;
        for candidate in &candidates {
            match candidate.chars().nth(bit_index).unwrap() {
                '1' => voting += 1,
                '0' => voting -= 1,
                _ => panic!("Invalid input!")
            }
        }
        let vote_result: char = match voting {
            v if v >= 0 => '1',
            _ => '0'
        };
        candidates.retain(|s| predicate(s.chars().nth(bit_index).unwrap(), vote_result));
        bit_index += 1;
        println!("Iteration {} size {}", bit_index, candidates.len());
    }
    return candidates[0];
}
