pub mod day6 {
    use anyhow::{Error, Ok};
    use std::collections::VecDeque;
    use std::fs;

    pub fn run() -> Result<(), Error> {
        let contents = fs::read_to_string("./src/day6/input.txt")?;
        let contents: Vec<char> = contents.chars().collect();

        println!("");
        println!("Day 6");
        println!("--------------------");
        println!("Q1: {}", q1(&contents).unwrap());
        println!("Q2: {}", q2(&contents).unwrap());

        Ok(())
    }

    fn q1(line: &Vec<char>) -> Result<u32, Error> {
        let mut temp: VecDeque<char> = VecDeque::new();
        let mut count: u32 = 0;
        let mut answer: u32 = 0;

        for letter in line {
            count += 1;
            if temp.len() < 3 {
                temp.push_back(letter.clone());
            } else if !temp.contains(letter) && unique_letters(&temp) {
                answer = count;
                break;
            } else {
                temp.pop_front();
                temp.push_back(letter.clone());
            }
        }
        Ok(answer)
    }

    fn q2(line: &Vec<char>) -> Result<u32, Error> {
        let mut temp: VecDeque<char> = VecDeque::new();

        let mut count: u32 = 0;
        let mut answer: u32 = 0;

        for letter in line {
            count += 1;
            if temp.len() < 13 {
                temp.push_back(letter.clone());
            } else if !temp.contains(letter) && unique_letters(&temp) {
                answer = count;
                break;
            } else {
                temp.pop_front();
                temp.push_back(letter.clone());
            }
        }
        Ok(answer)
    }

    fn unique_letters(line: &VecDeque<char>) -> bool {
        let mut temp: Vec<char> = vec![];
        for letter in line {
            if temp.contains(letter) {
                return false;
            } else {
                temp.push(letter.clone());
            }
        }
        true
    }
}
