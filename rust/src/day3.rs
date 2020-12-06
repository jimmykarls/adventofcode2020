use std::io::Read;
use std::fs::File;


fn parse_file(filename: &String, out: &mut Vec<String>) {

    let mut content: String = String::new();

    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut content).unwrap();

    let iter = content
        .lines()
        .map(|x| x.to_string());

    out.extend(iter);
}

fn count_trees(lines: &Vec<String>, x_steps: usize, y_steps: usize) -> usize {

    let mut n_trees = 0;
    let mut x = 0;
    let mut y = 0;

    let tree: String = "#".to_string();

    let width = lines[0].len();

    while y < lines.len() - 1 {
        y += y_steps;
        x = (x + x_steps) % width;
        let s: String = lines[y][x .. x + 1]
            .chars()
            .as_str()
            .to_string();

        if s == tree {
            n_trees += 1;
        }
    }
    return n_trees;
}

pub fn solve_day3(filename: &String) {

    /* PART 1 */
    let mut lines = Vec::<String>::new();
    parse_file(filename, &mut lines);

    let n_trees = count_trees(&lines, 3, 1);
    println!("Got {} hits!", n_trees);
    

    /* PART 2 */
    let x = count_trees(&lines, 1, 1) *
            count_trees(&lines, 3, 1) *
            count_trees(&lines, 5, 1) *
            count_trees(&lines, 7, 1) *
            count_trees(&lines, 1, 2);


    println!("Product is {}", x);

}