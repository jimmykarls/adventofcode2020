use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn read_passports(path: &String, out: &mut Vec<HashMap<String, String>>) {
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut tmp: Vec<String> = Vec::new();

    for line in content.lines() {
        if line.len() > 0 {
            tmp.push(line.to_string());
        }
        else if line.len() == 0 {
            let mut passport: HashMap<String, String> = HashMap::new();
            while tmp.len() > 0 {
                let passport_string = tmp.pop().unwrap();
                let fields = passport_string.split_whitespace();
                for field in fields {
                    let field_pair: Vec<String> = field.split(":")
                        .map(|x| x.to_string())
                        .collect();
                    passport.insert(field_pair[0].clone(), field_pair[1].clone());
                }
            }
            out.push(passport);
        }
    }

    if tmp.len() > 0 {
        let mut passport: HashMap<String, String> = HashMap::new();
        while tmp.len() > 0 {
            let passport_string = tmp.pop().unwrap();
            let fields = passport_string.split_whitespace();
            for field in fields {
                let field_pair: Vec<String> = field.split(":")
                    .map(|x| x.to_string())
                    .collect();
                passport.insert(field_pair[0].clone(), field_pair[1].clone());
            }
        }
        out.push(passport);
    }
    
}

fn is_passport_valid1(passport: &HashMap<String, String>) -> bool {
    let n_fields = passport.len();
    n_fields == 8 ||        // all fields present OR
    (n_fields == 7 &&       // 7 fields and the missing field is the cid field
      passport.contains_key(&"cid".to_string()) == false)
}

fn is_number_field_valid(passport: &HashMap<String, String>,
                         field: &str,
                         n_digits: usize,
                         min_value: usize,
                         max_value: usize) -> bool {

    match get_value_of_length(&passport, &field.to_string(), n_digits) {
        Ok(value) => {
            match value.parse::<usize>() {
                Ok(value) => {
                    if min_value <= value && value <= max_value {
                        return true;
                    } else {
                        println!("wrong min/max size {} {}/{}/{}", field, min_value, value, max_value);
                        return false;
                    }
                },
                Err(error) => {
                    println!("Could not parse {} {}", value, error);
                    return false;
                },
            }
        }
        Err(error) => {
            println!("Error fetching value for key {} {}", field, error);
            return false;
        }
    }
}

fn get_value_of_length(passport: &HashMap<String, String>, key: &String, len: usize) -> Result<String, String> {
    match passport.get(key) {
        Some(value) => {
            if value.len() == len {
                Ok(value.clone())
            } else {
                Err(format!("Value too short/long {}/{}", value.len(), len))
            }
        },
        None => {
            Err(format!("Key '{}' not found", key))
        }
    }
}

fn is_eye_colour_valid(passport: &HashMap<String, String>) -> bool {
    match get_value_of_length(&passport, &String::from("ecl"), 3) {
        Ok(value) => {
            const VALID_EYE_COLOURS: [&str; 7] = [
                "amb",
                "blu",
                "brn",
                "gry",
                "grn",
                "hzl",
                "oth",
            ];

            if VALID_EYE_COLOURS.contains(&value.as_str()) {
                return true;
            } else {
                println!("Wrong eye colour {}", value);
                return false;
            }
        }
        Err(_error) => false,
    }
}

fn is_hair_colour_valid(passport: &HashMap<String, String>) -> bool {
    match get_value_of_length(&passport, &String::from("hcl"), 7) {
        Ok(value) => {
            if value.starts_with("#") {
                // Skip first char as it is #
                for c in value[1 ..].chars() {
                    if c.is_alphanumeric() {
                        continue;
                    } else {
                        println!("Hair colour NOT valid! {} '{}'", value, c);
                        return false
                    }
                }
                return true
            }
            else {
                println!("Hair colour NOT valid! does not start with #");
                return false
            }
        },
        Err(error) => {
            println!("Error getting hcl {}", error);
            return false;
        }
    }
}

fn is_height_valid(passport: &HashMap<String, String>) -> bool {
    let key = String::from("hgt");

    match passport.get(&key) {
        Some(value) => {
            if value.len() == 5 || value.len() == 4 {
                let mut height = value.clone();
                let unit = height.split_off(value.len() - 2);

                match height.parse::<usize>() {
                    Ok(height) => {
                        if unit == "in" {
                            if 59 <= height && height <= 76 {
                                return true;
                            } else {
                                println!("Wrong height 59 / {} / 76", height);
                                return false;
                            }
                        } else if unit == "cm" {
                            if  150 <= height && height <= 193 {
                                return true;
                            } else {
                                println!("Wrong height 150 / {} / 193", height);
                                return false;
                            }
                        } else {
                            println!("Got invalid unit {}", unit);
                            return false
                        }       
                    },
                    Err(error) => {
                        println!("Error Parsing height {} {}", height, error);
                        return false;
                    }
                }
            }
            else {
                println!("Error Parsing height wrong len {}", value.len());
                return false
            }
        },
        None => {
            println!("Error fetching {}", "hgt");
            return false;
        }
    }
}

fn is_passport_valid2(passport: &HashMap<String, String>) -> bool {
    is_number_field_valid(&passport, "byr", 4, 1920, 2002)   &&
    is_number_field_valid(&passport, "iyr", 4, 2010, 2020)   &&
    is_number_field_valid(&passport, "eyr", 4, 2020, 2030)   &&
    is_number_field_valid(&passport, "pid", 9, 0, 999999999) &&
    is_eye_colour_valid(&passport)                           &&
    is_height_valid(&passport)                               &&
    is_hair_colour_valid(&passport)                          &&
    is_passport_valid1(&passport)
}

pub fn solve_day4(path: &String) {
    let mut passport_maps = Vec::new();
    read_passports(path, &mut passport_maps);

    /* PART1 */
    let mut n_valid_passports = 0;
    for passport in &passport_maps {
        if is_passport_valid1(passport) {
            n_valid_passports += 1;
        }
    }
    println!("PART1: Got {}/{} valid passports", n_valid_passports, passport_maps.len());

    /* PART2 */
    n_valid_passports = 0;
    for passport in &passport_maps {
        if is_passport_valid2(&passport) {
            n_valid_passports += 1;
        } else {
            println!("Failed on {:?}", passport);
            println!("");
            println!("");
        }
    }
    println!("PART2: Got {}/{} valid passports", n_valid_passports, passport_maps.len());
}