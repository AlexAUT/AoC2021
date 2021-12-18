fn main() {
    let input = include_str!("input.txt");

    let mut field: Vec<u8> = input
        .lines()
        .flat_map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as u8))
        .collect();

    let mut flashes = 0;

    let mut found_task2 = false;

    for step in 0..100000 {
        let mut flash_queue: Vec<u8> = Vec::new();
        for element_index in 0..100 {
            if field[element_index] == 9 {
                flash_queue.push(element_index.try_into().unwrap());
            }
            field[element_index] += 1;
        }

        while !flash_queue.is_empty() {
            let flash_element = flash_queue.pop().unwrap();
            let col = flash_element % 10;
            let row = flash_element / 10;
            for x_off in -1 as i8..=1 {
                for y_off in -1 as i8..=1 {
                    if (x_off == 0 && y_off == 0)
                        || (x_off == -1 && col == 0)
                        || (x_off == 1 && col == 9)
                        || (y_off == -1 && row == 0)
                        || (y_off == 1 && row == 9)
                    {
                        continue;
                    }
                    let increase_index = (col as i8 + x_off + ((row as i8 + y_off) * 10)) as usize;
                    if field[increase_index] == 9 {
                        flash_queue.push(increase_index as u8);
                    }
                    if field[increase_index] <= 9 {
                        field[increase_index] += 1;
                    }
                }
            }
        }

        let mut flashes_this_turn = 0;
        for element in field.iter_mut() {
            if *element > 9 {
                *element = 0;
                flashes += 1;
                flashes_this_turn += 1;
            }
        }
        if flashes_this_turn == 100 {
            found_task2 = true;
            println!("[Task2] All flash at turn {}", step + 1);
            if step > 100 {
                break;
            }
        }

        if step == 100 {
            println!("[Task1] Flashes: {}", flashes);
            if found_task2 {
                break;
            }
        }
    }

}
