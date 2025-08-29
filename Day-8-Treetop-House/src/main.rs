use std::fs;

use forest::Forest;

mod forest;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Could not read input file");
    let mut forest = Forest::new();

    for line in input.lines() {
        forest.add_row();
        for c in line.chars() {
            let tree = char_to_int(c);
            forest.add_tree(tree);
        }
    }

    let mut visible_trees = 0;
    let height = forest.height();
    let width = forest.width();

    let mut max_score = 0;

    for y in 0..height {
        for x in 0..width {
            if forest.is_visible(x, y) {
                visible_trees += 1;
            }
            let score = forest.get_scenic_score(x, y);
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("Visible trees: {}", visible_trees);
    println!("Max score: {}", max_score);
}

fn char_to_int(c: char) -> u8 {
    c.to_string().parse().expect("Could not parse char to int")
}
