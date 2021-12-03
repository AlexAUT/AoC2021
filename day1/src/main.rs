fn main() {
    println!("Hello, world!");
    let input = include_str!("inputA0.txt");

    // Task0
    {
        let mut iterators = input.lines();
        let last_probe = iterators.next().expect("File empty?");
        let mut last_probe = convert_probe(last_probe); 

        let mut larger_count = 0;

        for probe in iterators {
            let probe = convert_probe(probe); 

            if probe > last_probe {
                larger_count += 1;
            }
            last_probe = probe;
        }

        println!("Amount of increasing probes: {}", larger_count);
    }

    //Task 1
    {
        let mut probes: Vec<u32> = Vec::new();
        probes.reserve(input.lines().count());

        for probe in input.lines() {
            probes.push(convert_probe(probe));
        }

        let mut windows = probes.windows(3);
        let mut last_window: u32 = windows.next().expect("Less than 3 probes").iter().sum();

        let mut larger_count: u32 = 0;
 
        for window in windows {
            let window = window.iter().sum();

            if window > last_window {
                larger_count += 1;
            }
            last_window = window;
        }

        println!("Amount of increasing probes: {}", larger_count);
    }
}

fn convert_probe(probe: &str) -> u32 {
    return probe.parse().expect("Probe not a integer?"); 
}
