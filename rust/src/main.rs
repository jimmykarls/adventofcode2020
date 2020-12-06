mod day1;
mod day2;
mod day3;

fn main() {
    println!("Hello, world!");
    let mut filename = "inputs/day1".to_string();
    day1::solve_day1(&filename);

    filename = "inputs/day2".to_string();
    day2::solve_day2(&filename);
    
    filename = "inputs/day3".to_string();
    day3::solve_day3(&filename);
    

}
