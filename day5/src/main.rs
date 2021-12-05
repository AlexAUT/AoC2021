#[derive(Debug, Copy, Clone)]
struct Point(i32, i32);

#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

fn main() {
    let input = include_str!("input.txt");

    let line_segments: Vec<LineSegment> = input
        .lines()
        .map(|x| {
            let (start_x, rest) = x.split_once(',').expect("No valid input!");
            let (start_y, rest) = rest.split_once("->").expect("No valid input!");
            let (end_x, end_y) = rest.split_once(',').expect("No valid input!");

            let start_x: i32 = start_x.trim().parse().unwrap();
            let start_y: i32 = start_y.trim().parse().unwrap();
            let end_x: i32 = end_x.trim().parse().unwrap();
            let end_y: i32 = end_y.trim().parse().unwrap();

            LineSegment {
                start: Point(start_x, start_y),
                end: Point(end_x, end_y),
            }
        })
        .collect();

    // Determine area size
    let mut area_size = (0, 0);
    for line_segment in &line_segments {
        area_size.0 = area_size.0.max(line_segment.start.0).max(line_segment.end.0) + 1;
        area_size.1 = area_size.1.max(line_segment.start.1).max(line_segment.end.1) + 1;
    }

    for task in 1..3 {
        let mut area: Vec<u32> = vec![0; area_size.0 as usize * area_size.1 as usize];

        // Mark area task 1
        for line_segment in &line_segments {
            // Ignore diagonals in task 1
            if task == 1 && (line_segment.start.0 != line_segment.end.0 && line_segment.start.1 != line_segment.end.1) {
                continue;
            }
            let x_deriv = line_segment.start.0 - line_segment.end.0;
            let y_deriv = line_segment.start.1 - line_segment.end.1;
            let iteration_count = x_deriv.abs().max(y_deriv.abs()) + 1;

            // Normalize derivate
            let x_deriv = if x_deriv != 0 { if x_deriv > 0 {1} else {-1} } else {x_deriv};
            let y_deriv = if y_deriv != 0 { if y_deriv > 0 {1} else {-1} } else {y_deriv};

            let mut cursor = line_segment.start;
            for _ in 0..iteration_count {
                area[(cursor.0 * area_size.1 + cursor.1) as usize] += 1;
                cursor.0 -= x_deriv;
                cursor.1 -= y_deriv;
            }
        }

        let number_of_overlaps = area.iter().filter(|&x| *x > 1).count();
        println!("[Task{}] Number of overlaps: {}", task, number_of_overlaps);
    }
}
