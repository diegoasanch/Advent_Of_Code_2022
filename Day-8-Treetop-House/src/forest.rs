use std::cmp;

#[derive(Debug)]
pub struct Forest {
    trees: Vec<Vec<u8>>,
}

#[derive(PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Forest {
    pub fn new() -> Self {
        Self { trees: Vec::new() }
    }

    pub fn add_tree(&mut self, tree_height: u8) {
        if self.trees.is_empty() {
            self.trees.push(vec![tree_height]);
        } else {
            let last_row = self.trees.last_mut().expect("Could not get last row");
            last_row.push(tree_height);
        }
    }

    pub fn add_row(&mut self) {
        self.trees.push(Vec::new());
    }

    pub fn width(&self) -> usize {
        self.trees[0].len()
    }

    pub fn height(&self) -> usize {
        self.trees.len()
    }

    pub fn is_visible(&self, x: usize, y: usize) -> bool {
        let directions = vec![
            Direction::Right,
            Direction::Left,
            Direction::Up,
            Direction::Down,
        ];

        directions.iter().any(|d| self.is_tree_taller(x, y, d))
    }

    fn is_tree_taller(&self, x: usize, y: usize, direction: &Direction) -> bool {
        let row = self.trees.get(y).expect("Could not get row");
        let tree = row.get(x).expect("Could not get tree");

        let mut is_taller = true;
        let trees_to_edge = self.get_trees_to_edge(x, y, direction);

        for t in trees_to_edge {
            if tree <= &t {
                is_taller = false;
                break;
            }
        }
        is_taller
    }

    fn get_trees_to_edge(&self, x: usize, y: usize, direction: &Direction) -> Vec<u8> {
        let width = self.width();
        let height = self.height();

        let max_safe_x = cmp::min(x + 1, width);
        let max_safe_y = cmp::min(y + 1, height);

        let mut result = match direction {
            Direction::Right => self.trees[y][max_safe_x..].to_vec(),
            Direction::Left => self.trees[y][0..x].to_vec(),
            Direction::Up => self.trees[0..y].iter().map(|row| row[x]).collect(),
            Direction::Down => self.trees[max_safe_y..].iter().map(|row| row[x]).collect(),
        };

        if direction == &Direction::Left || direction == &Direction::Up {
            result.reverse();
        }

        result
    }

    pub fn get_scenic_score(&self, x: usize, y: usize) -> u32 {
        let directions = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];

        let mut scores = Vec::new();

        for d in directions {
            let trees_to_edge = self.get_trees_to_edge(x, y, &d);
            let visible_trees = count_visible_trees(&trees_to_edge, self.trees[y][x]);
            scores.push(visible_trees);
        }

        multiply_vec(&scores)
    }
}

fn count_visible_trees(trees: &Vec<u8>, tree: u8) -> u32 {
    let mut count = 0;
    let mut max_height = 0;

    for t in trees {
        if t < &max_height {
            continue;
        }
        count += 1;
        if t > &max_height {
            max_height = *t;
        }
        if t > &tree {
            break;
        }
    }
    count
}

fn multiply_vec(vec: &Vec<u32>) -> u32 {
    if vec.is_empty() {
        return 0;
    }
    vec.iter().product()
}
