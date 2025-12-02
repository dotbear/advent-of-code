use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "input.txt";
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    let mut current_value = 50;
    let mut result = 0;

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let mut chars = line.chars();
                let direction = chars.next();
                let ticks_string: String = chars.collect();
                let ticks = ticks_string.parse::<i32>().unwrap();
                let mut times_past_zero = ticks / 100;

                match direction {
                    Some('L') => {
                        let freespin = current_value - (ticks % 100);
                        if freespin < 0 {
                            if current_value != 0 {
                                times_past_zero += 1;
                            }
                            current_value = 100 + freespin;
                        } else {
                            current_value = freespin;
                        }
                    }
                    Some('R') => {
                        let freespin = current_value + (ticks % 100);
                        let new_pos = freespin - 100;
                        if freespin > 99 {
                            if current_value != 0 && new_pos != 0 {
                                times_past_zero += 1;
                            }
                            current_value = new_pos;
                        } else {
                            current_value = freespin;
                        }
                    }
                    _ => {
                        println!("direction mismatch");
                    }
                }

                if current_value == 0 {
                    times_past_zero += 1;
                }

                result += times_past_zero;
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
