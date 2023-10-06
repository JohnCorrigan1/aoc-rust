pub mod day4 {

    use anyhow::{Error, Ok};
    use std::fs;

    struct Shelf {
        start: u8,
        end: u8,
    }

    pub fn run() -> Result<(), Error> {
        let contents = fs::read_to_string("./src/day4/input.txt")?;
        let contents: Vec<&str> = contents.lines().collect();

        let mut score_q1: u32 = 0;
        let mut score_q2: u32 = 0;

        for line in contents {
            let line = line.trim();
            let shelves = get_shelves(line).unwrap();

            score_q1 += match overlap_q1(&shelves[0], &shelves[1]) {
                true => 1,
                false => 0,
            };
            score_q2 += match overlap_q2(&shelves[0], &shelves[1]) {
                true => 1,
                false => 0,
            };
        }
        println!("");
        println!("Day 4");
        println!("--------------------");
        println!("Q1: {}", score_q1);
        println!("Q2: {}", score_q2);

        Ok(())
    }

    fn overlap_q1(shelf1: &Shelf, shelf2: &Shelf) -> bool {
        if shelf1.start <= shelf2.start && shelf1.end >= shelf2.end {
            true
        } else if shelf1.start >= shelf2.start && shelf1.end <= shelf2.end {
            true
        } else {
            false
        }
    }

    fn overlap_q2(shelf1: &Shelf, shelf2: &Shelf) -> bool {
        if shelf1.start <= shelf2.start && shelf1.end >= shelf2.start {
            true
        } else if shelf2.start <= shelf1.start && shelf2.end >= shelf1.start {
            true
        } else if shelf2.start >= shelf1.start && shelf2.end <= shelf1.end {
            true
        } else if overlap_q1(shelf1, shelf2) {
            true
        } else {
            false
        }
    }

    fn get_shelves(line: &str) -> Result<Vec<Shelf>, Error> {
        let shelves: Vec<&str> = line.split(",").collect();
        let split_shelves: Vec<Vec<&str>> = vec![
            shelves[0].split("-").collect(),
            shelves[1].split("-").collect(),
        ];
        let parsed_shelves: Vec<Shelf> = vec![
            Shelf {
                start: split_shelves[0][0].parse()?,
                end: split_shelves[0][1].parse()?,
            },
            Shelf {
                start: split_shelves[1][0].parse()?,
                end: split_shelves[1][1].parse()?,
            },
        ];

        Ok(parsed_shelves)
    }
}
