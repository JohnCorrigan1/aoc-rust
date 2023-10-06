pub mod day3 {

    use anyhow::{Error, Ok};
    use std::collections::HashMap;
    use std::fs;

    pub fn run() -> Result<(), Error> {
        let contents = fs::read_to_string("./src/day3/input.txt")?;
        let contents: Vec<&str> = contents.lines().collect();

        let prios = get_priorities();
        let mut score_q1: u32 = 0;

        let mut score_q2: u32 = 0;
        let mut group: Vec<Vec<char>> = vec![];
        let mut count: u8 = 0;

        for line in contents {
            let line = line.trim();

            //q1
            let first_half = &line[0..line.len() / 2].chars().collect();
            let second_half = &line[line.len() / 2..line.len()].chars().collect();
            match compare_q1(&first_half, &second_half, &prios) {
                Some(value) => score_q1 += value,
                None => (),
            };

            //q2
            count += 1;
            group.push(line.chars().collect());
            if count == 3 {
                count = 0;
                match compare_q2(&group, &prios) {
                    Some(value) => score_q2 += value,
                    None => (),
                }
                group = vec![];
            }
        }

        println!("");
        println!("Day 3");
        println!("---------------------");
        println!("Q1: {}", score_q1);
        println!("Q2: {}", score_q2);

        Ok(())
    }

    fn compare_q2(group: &Vec<Vec<char>>, prios: &HashMap<char, u32>) -> Option<u32> {
        for letter in &group[0] {
            if group[1].contains(&letter) && group[2].contains(&letter) {
                return prios.get(&letter).copied();
            }
        }
        None
    }

    fn compare_q1(
        first_half: &Vec<char>,
        second_half: &Vec<char>,
        prios: &HashMap<char, u32>,
    ) -> Option<u32> {
        for letter in first_half {
            if second_half.contains(letter) {
                return prios.get(letter).copied();
            }
        }
        None
    }

    fn get_priorities() -> HashMap<char, u32> {
        let mut prios: HashMap<char, u32> = HashMap::new();

        let mut count = 0;

        for i in 97..123 {
            count += 1;
            let letter = char::from_u32(i).unwrap();
            prios.insert(letter, count);
        }

        for i in 65..91 {
            count += 1;
            let letter = char::from_u32(i).unwrap();
            prios.insert(letter, count);
        }
        prios
    }
}
