pub mod day8 {

    use anyhow::{Error, Ok};
    use std::fs;

    pub fn run() -> Result<(), Error> {
        let contents = fs::read_to_string("./src/day8/input.txt")?;
        let contents: Vec<&str> = contents.lines().collect();

        let mut trees: Vec<Vec<char>> = vec![];

        for line in contents {
            trees.push(line.trim().chars().collect())
        }

        let parsed_grid: Vec<Vec<u32>> = parse_grid(trees);

        let q1 = q1(&parsed_grid);
        let q2 = q2(&parsed_grid);

        println!("");
        println!("Day 8");
        println!("--------------------");
        println!("Q1: {}", q1);
        println!("Q2: {}", q2);

        Ok(())
    }

    fn q2(grid: &Vec<Vec<u32>>) -> u32 {
        let height = grid.len();
        let width = grid[0].len();

        let mut scenic_score: u32 = 0;

        for row in 0..height - 1 {
            for col in 0..width - 1 {
                let current_height = grid[row][col];
                let mut views: (u32, u32, u32, u32) = (0, 0, 0, 0);

                views.0 = grid[..row]
                    .iter()
                    .rev()
                    .take_while(|line| line[col] < current_height)
                    .count() as u32;

                if views.0 > 0 {
                    if views.0 < row as u32 {
                        views.0 += 1;
                    }
                } else {
                    views.0 = 1;
                }

                views.1 = grid[row + 1..]
                    .iter()
                    .take_while(|line| line[col] < current_height)
                    .count() as u32;

                if views.1 > 0 {
                    if let Some(_) = grid.get(row + views.1 as usize + 1) {
                        views.1 += 1;
                    }
                } else {
                    views.1 = 1;
                }

                views.2 = grid[row][col + 1..]
                    .iter()
                    .take_while(|column| **column < current_height)
                    .count() as u32;

                if views.2 > 0 {
                    if let Some(_) = grid[row].get(col + (views.2 as usize + 1)) {
                        views.2 += 1;
                    }
                } else {
                    views.2 = 1;
                }

                views.3 = grid[row][..col]
                    .iter()
                    .rev()
                    .take_while(|column| **column < current_height)
                    .count() as u32;
                if views.3 > 0 {
                    if views.3 < col as u32 {
                        views.3 += 1;
                    }
                } else {
                    views.3 = 1
                }

                let score = views.0 * views.1 * views.2 * views.3;

                if score > scenic_score {
                    scenic_score = score;
                }
            }
        }
        scenic_score
    }

    fn q1(grid: &Vec<Vec<u32>>) -> u32 {
        let height = grid.len();
        let width = grid[0].len();
        let mut visible_trees: u32 = height as u32 * 2 + width as u32 * 2 - 4;

        for row in 1..height - 1 {
            for col in 1..width - 1 {
                let current_height = grid[row][col];

                let up = grid[..row].iter().all(|line| line[col] < current_height);

                if up {
                    visible_trees += 1;
                    continue;
                }

                let down = grid[row + 1..]
                    .iter()
                    .all(|line| line[col] < current_height);

                if down {
                    visible_trees += 1;
                    continue;
                }

                let right = grid[row][col + 1..]
                    .iter()
                    .all(|column| *column < current_height);

                if right {
                    visible_trees += 1;
                    continue;
                }

                let left = grid[row][..col]
                    .iter()
                    .all(|column| *column < current_height);

                if left {
                    visible_trees += 1;
                    continue;
                }
            }
        }
        visible_trees
    }

    fn parse_grid(trees: Vec<Vec<char>>) -> Vec<Vec<u32>> {
        let mut grid: Vec<Vec<u32>> = vec![];

        for line in trees {
            let mut row: Vec<u32> = vec![];
            for char in line {
                row.push(char.to_digit(10).unwrap());
            }
            grid.push(row);
        }

        grid
    }
}
