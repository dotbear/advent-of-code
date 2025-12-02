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
                    let mut sequence: Vec<char> = vec![];
                    let mut test_sequence: Vec<char> = vec![];
                    let mut offset = 0;

                    println!("i: {}", i);

                    for (j, char) in i.to_string().chars().enumerate() {
                        if !sequence.is_empty()
                            && sequence[offset] == char
                            && test_sequence != sequence
                        {
                            offset = j - 1;
                            test_sequence.push(char);
                        } else {
                            sequence.push(char);
                        }

                        if (j + 1 == i.to_string().chars().count() && test_sequence == sequence) {
                            println!("Found sequence! {:?}", sequence);
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
