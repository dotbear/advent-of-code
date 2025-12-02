use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut result = 0;
    let message: String = fs::read_to_string("input_example.txt")?;
    let ranges: Vec<&str> = message.trim().split(',').collect();

    for range in ranges {
        let parts: Vec<&str> = range.split('-').collect();
        let parsed_range = [parts[0].parse::<u64>(), parts[1].parse::<u64>()];

        match parsed_range {
            [Ok(start), Ok(end)] => {
                for i in start..=end {
                    let string: String = i.to_string();
                    let length = string.chars().count();

                    if length.is_multiple_of(2) {
                        let midpoint = length.div_ceil(2);
                        let halves = string.split_at(midpoint);
                        if halves.0 == halves.1 {
                            result += i;
                        }
                    }
                }
            }
            err => {
                println!("Error {:?}", parts);
                println!("Error! {:?}", err);
            }
        }
    }

    println!("Result is: {}", result);

    Ok(())
}
