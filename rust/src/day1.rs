use std::io::Read;
use std::fs::File;

fn parse_file(filename: &str, out: &mut Vec<u32>) -> usize {
    out.clear();
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Ok(_size) => {
                    let iterator = content.lines()              // Split text by line
                        .map(|x| x.parse::<u32>().unwrap());    // Convert each line to a number
                                                                // unwrap. Assume no error. Returns the Result Ok value
                                                                // This will crash the program if a line
                                                                // cannot be parsed to a number
                    // Fill the output vector with the numbers
                    out.extend(iterator);
                },
                Err(error) => {
                    println!("Error reading contents {}", error);
                }
            }
        },
        Err(error) => {
            println!("Error opening file {}: {}", filename, error);
        },
    };
    out.len() // Return the size of the vector
}

fn test(filename: &str) -> std::result::Result<Vec<u32>, String> {
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Ok(_size) => {  // underscore prefix signals that 'size' will be unused
                    /* 
                        1) Split the content into single lines
                        2) Parse each line into a number with map
                            unwrap will cause a panic if we get a parse error
                        3) Map creates an iterator, and collect creates a vector 
                            with all elements in the iterator
                    */
                    let out: Vec<u32> = content.lines()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect();
                    return Ok(out);
                },
                Err(error) => {
                    return Err(format!("File empty {}", error));
                }
            }
        },
        Err(error) => {
            return Err(format!("Error opening file {}: {}", filename, error));
        },
    };
}

fn find_numbers_with_sum(sum: u32, numbers: &Vec<u32>) -> Result<(u32, u32), &str> {
    let end = numbers.len();
    for index in 0 .. end {
        for index2 in index .. end {
            if numbers[index] + numbers[index2] == sum {
                return Ok((numbers[index], numbers[index2]));
            }
        }
    }
    return Err("Could not be found");
}

fn find_numbers_3_with_sum(sum: u32, numbers: &Vec<u32>) -> Result<(u32, u32, u32), &str> {
    let end = numbers.len();
    for index in 0 .. end {
        for index2 in index .. end {
            for index3 in 0 .. end {
                if numbers[index] + numbers[index2] + numbers[index3] == sum {
                    return Ok((numbers[index], numbers[index2], numbers[index3]));
               }
            }
        }
    }
    return Err("Could not be found");
}

pub fn solve_day1(filename: &String) {
    let mut numbers: Vec<u32> = Vec::new();

    let size = parse_file(&filename, &mut numbers);

    match test(filename) {
        Ok(numbers) => {
            match find_numbers_with_sum(2020, &numbers) {
                Ok(tuple) => println!("Success! The product of the numbers is {}", tuple.0 * tuple.1),
                Err(error) => println!("Could not find numbers! {}", error),
            }
        },
        Err(error) => {
            println!("Got ERROR! {}!", error);
        }
    };

    match test(filename) {
        Ok(numbers) => {
            match find_numbers_3_with_sum(2020, &numbers) {
                Ok(tuple) => println!("Success! The product of the numbers is {}", tuple.0 * tuple.1 * tuple.2),
                Err(error) => println!("Could not find numbers! {}", error),
            }
        },
        Err(error) => {
            println!("Got ERROR! {}!", error);
        }
    }

    println!("Got {} numbers!", size);

}