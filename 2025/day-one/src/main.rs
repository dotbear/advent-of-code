use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "input.txt";
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    let mut current_value = 50;
    let mut result = 0;
    let mut iterator = 0;

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let mut chars = line.chars();
                let direction = chars.next();
                let ticks_string: String = chars.collect();
                let ticks = ticks_string.parse::<i32>().unwrap();
                let mut times_past_zero = ticks / 100;

                if iterator < 20 {
                    println!("the current value is {}", current_value);
                    println!(
                        "Reading direction {} and ticks {}",
                        direction.unwrap(),
                        ticks
                    );
                    println!("times past zero {}", times_past_zero);
                }

                match direction {
                    Some('L') => {
                        let freespin = current_value - (ticks % 100);
                        if freespin < 0 {
                            times_past_zero += 1;
                            if iterator < 20 {
                                println!("going past zero! {}", times_past_zero);
                            }
                            current_value = 100 + freespin;
                        } else {
                            current_value = freespin;
                        }
                    }
                    Some('R') => {
                        let freespin = current_value + (ticks % 100);
                        if freespin > 99 {
                            times_past_zero += 1;
                            if iterator < 20 {
                                println!("going past zero! {}", times_past_zero);
                            }
                            current_value = freespin - 100;
                        } else {
                            current_value = freespin;
                        }
                    }
                    _ => {
                        println!("direction mismatch");
                    }
                }

                // if current_value == 0 {
                //     times_past_zero += 1;
                // }

                result += times_past_zero;
                iterator += 1;
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }

    println!("The answer is {}", result);

    Ok(())
}
