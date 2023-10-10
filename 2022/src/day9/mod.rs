pub mod day9 {

    use anyhow::{Error, Result};
    use std::collections::HashMap;
    use std::fs;

    struct Round {
        direction: Direction,
        moves: u32,
    }

    enum Direction {
        U,
        D,
        R,
        L,
    }

    impl From<&str> for Direction {
        fn from(value: &str) -> Self {
            match value {
                "U" => Direction::U,
                "D" => Direction::D,
                "R" => Direction::R,
                "L" => Direction::L,
                _ => panic!("Invalid input"),
            }
        }
    }

    pub fn run() -> Result<(), Error> {
        let contents = fs::read_to_string("src/day9/input.txt")?;
        //let contents = fs::read_to_string("src/day9/test.txt")?;

        let contents: Vec<&str> = contents.lines().collect();

        let mut game: Vec<Round> = vec![];

        for line in contents {
            //let round: Vec<char> = line.trim().chars().collect();
            let round: Vec<&str> = line.trim().split(" ").collect();

            if let Some(moves) = round[1].parse::<u32>().ok() {
                let direction: Direction = round[0].into();
                game.push(Round { direction, moves });
            }
        }

        println!("");
        println!("Day 9");
        println!("---------------------------");
        println!("Q1: {}", q1(&game));

        Ok(())
    }

    fn q1(game: &Vec<Round>) -> u32 {
        let mut visited: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        visited.insert((0, 0), (0, 0));

        let mut head_position: (i32, i32) = (0, 0);
        let mut tail_position: (i32, i32) = (0, 0);

        for round in game {
            for _i in 0..round.moves {
                move_me(
                    &mut head_position,
                    &mut tail_position,
                    &mut visited,
                    &round.direction,
                );
            }
        }
        // for (key, _value) in visited.iter() {
        //     println!("{}:{}", key.0, key.1);
        // }
        visited.len() as u32
    }

    fn move_me(
        head_position: &mut (i32, i32),
        tail_position: &mut (i32, i32),
        visited: &mut HashMap<(i32, i32), (i32, i32)>,
        direction: &Direction,
    ) -> () {
        match direction {
            Direction::U => head_position.0 -= 1,
            Direction::D => head_position.0 += 1,
            Direction::R => head_position.1 += 1,
            Direction::L => head_position.1 -= 1,
        }

        if needs_move(&head_position, &tail_position) {
            match direction {
                Direction::U => {
                    tail_position.0 = head_position.0 + 1;
                    tail_position.1 = head_position.1;
                }
                Direction::D => {
                    tail_position.0 = head_position.0 - 1;
                    tail_position.1 = head_position.1;
                }
                Direction::R => {
                    tail_position.1 = head_position.1 - 1;
                    tail_position.0 = head_position.0;
                }
                Direction::L => {
                    tail_position.1 = head_position.1 + 1;
                    tail_position.0 = head_position.0;
                }
            }
            insert_visited(visited, &tail_position);
        }
    }

    fn needs_move(head_position: &(i32, i32), tail_position: &(i32, i32)) -> bool {
        if head_position.0.abs_diff(tail_position.0) > 1
            || head_position.1.abs_diff(tail_position.1) > 1
        {
            true
        } else {
            false
        }
    }

    fn insert_visited(visited: &mut HashMap<(i32, i32), (i32, i32)>, position: &(i32, i32)) -> () {
        visited.entry(*position).or_insert(*position);
    }
}
