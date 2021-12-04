#[derive(Debug)]
struct Board {
    elements: [[u32; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl Board {
    fn new() -> Board {
        Board {
            elements: [[0; 5]; 5],
            marked: [[false; 5]; 5],
        }
    }

    fn mark_elements(&mut self, number: u32) {
        for (row_index, column) in self.elements.iter().enumerate() {
            for (col_index, entry) in column.iter().enumerate() {
                if *entry == number {
                    self.marked[row_index][col_index] = true;
                }
            }
        }
    }

    fn is_winning(&self) -> bool {
        // Check columsn
        self.marked
            .iter()
            .any(|&col| col.iter().all(|marked| *marked))
            ||
            // Check rows
        self
            .marked
            .iter()
            .enumerate()
            .any(|(index, _)| self.marked.iter().all(|col| col[index]))
    }

    fn score(&self) -> u64 {
        let mut score: u64 = 0;
        for (row_index, column) in self.elements.iter().enumerate() {
            for (col_index, number) in column.iter().enumerate() {
                if !self.marked[row_index][col_index] {
                    score += *number as u64;
                }
            }
        }
        score
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let draw_queue: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let boards_data: Vec<&str> = lines.collect();
    let board_chunks = boards_data.chunks(6);

    let mut boards: Vec<Board> = Vec::new();
    for board_data in board_chunks {
        let mut board = Board::new();
        for (row_index, row) in board_data.iter().enumerate() {
            match row_index {
                0 => assert!(row.is_empty() == true),
                1..=5 => {
                    for (col_index, element) in row
                        .trim()
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .enumerate()
                    {
                        board.elements[row_index - 1][col_index] = element;
                    }
                }
                _ => panic!("Window size wrong?"),
            }
        }
        boards.push(board);
    }

    let mut finished_boards: usize = 0;
    let number_of_boards = boards.len();

    'drawing_cards:
    for drawn_number in draw_queue {
        for board in boards.iter_mut() {
            if board.is_winning() {
                continue;
            }

            board.mark_elements(drawn_number);

            if board.is_winning() {
                finished_boards += 1;
                match finished_boards {
                    1 => println!("First Board won: {:?}", drawn_number as u64 * board.score()),
                    x if x >= number_of_boards => {
                        println!("Last Board won: {:?}", drawn_number as u64 * board.score());
                        break 'drawing_cards
                    }
                    _ => {}
                }
            }
        }
    }
}
