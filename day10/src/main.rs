use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let mut error_score: u64 = 0;

    let symbol_pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let error_scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let autocomplete_score = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let check_error = |expect: char, found: char| match found {
        f if f == expect => 0,
        _ => *error_scores.get(&found).unwrap(),
    };

    let mut autocomplete_scores: Vec<u64> = Vec::new();

    for line in input.lines() {
        let mut open_stack: Vec<char> = Vec::new();
        let mut incomplete: bool = true;
        for symbol in line.chars() {
            match symbol {
                s if symbol_pairs.contains_key(&s) => {
                    open_stack.push(*symbol_pairs.get(&s).unwrap())
                }
                _ => match check_error(open_stack.pop().unwrap_or('\0'), symbol) {
                    score if score > 0 => {
                        incomplete = false;
                        error_score += score;
                        break;
                    }
                    _ => {}
                },
            }
        }
        if incomplete {
            let mut completion_score = 0;
            for symbol in open_stack.iter().rev() {
                completion_score = completion_score * 5 + autocomplete_score.get(symbol).unwrap();
            }
            autocomplete_scores.push(completion_score);
        }
    }
    println!("[Task1] Error Score: {}", error_score);
    autocomplete_scores.sort();
    println!(
        "[Task2] Middle autocompletion score: {}",
        autocomplete_scores[autocomplete_scores.len() / 2]
    );
}
