use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::convert::TryFrom;

// #[derive(Copy, Clone)]
struct Entry {
    min_times: i32,
    max_times: i32,
    password: String,
    c: String,
}



fn parse_file(filename: &String, out: &mut Vec<Entry>) {
    let re = Regex::new(r"([0-9]*)-([0-9]*)\s([a-z]):\s([a-z]+)").unwrap();

    match File::open(filename) {
        Ok(mut file) => {
            let mut content: String = String::new();
            file.read_to_string(&mut content).unwrap();
            for line in content.lines() {
                match re.captures(line) {
                    Some(groups) => {
                        let entry = Entry {
                            min_times: groups
                                .get(1)
                                .map_or(0, |x| x.as_str().parse::<i32>().unwrap()),
                            max_times: groups
                                .get(2)
                                .map_or(0, |x| x.as_str().parse::<i32>().unwrap()),
                            c: groups
                                .get(3)
                                .map_or("".to_string(), |x| x.as_str().to_string()),
                            password: groups
                                .get(4)
                                .map_or("".to_string(), |x| x.as_str().to_string()),
                        };
                        out.push(entry);
                    },
                    _ => println!("Ran into error on line {}", line),
                };
            }
        },
        Err(error) => println!("Error No file {}", error),
    }
}

fn is_entry_valid(entry: &Entry) -> bool {
    let c = i32::try_from(entry.password.matches(&entry.c).count()).unwrap();
    return entry.min_times <= c && c <= entry.max_times;
}

fn is_entry_valid2(entry: &Entry) -> bool {
    let index1 = usize::try_from(entry.min_times).unwrap() - 1;
    let index2 = usize::try_from(entry.max_times).unwrap() - 1;

    // Only convert the range of interest
    let v: Vec<char> = entry.password[0 .. index2 + 1].chars().collect();

    if v.len() < index1 || v.len() < index2 {
        return false;
    }

    let f = v[index1].to_string();
    let s = v[index2].to_string();

    let mut match_counter = 0;

    if f == entry.c {
        match_counter += 1;
    }

    if s == entry.c {
        match_counter += 1;
    }

    if match_counter == 1 {
        return true;
    } else {
        return false;
    }
}

pub fn solve_day2(filename: &String) {

    /* Problem 1 */
    let mut n_valid_entries = 0;
    let mut entries: Vec<Entry> = Vec::<Entry>::new();
    parse_file(&filename, &mut entries);
    let len = entries.len();

    /* & => borrow. Without it the for loop will cause a move */
    for entry in &entries {
        if is_entry_valid(&entry) {
            n_valid_entries += 1;
        }
    }
    println!("Number of valid entries are {}/{}", n_valid_entries, len);

    /* Problem 2 */
    n_valid_entries = 0;
    for entry in &entries {
        if is_entry_valid2(&entry) {
            n_valid_entries += 1;
        }
    }
    println!("Number of valid entries are {}/{}", n_valid_entries, len);
}