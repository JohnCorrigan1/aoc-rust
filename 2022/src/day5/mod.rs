pub mod day5 {

    use anyhow::{Error, Ok};
    use itertools::Itertools;
    use std::fs;

    pub fn run() -> Result<(), Error> {
        let contents = fs::read_to_string("./src/day5/input.txt")?;
        let contents: Vec<&str> = contents.lines().collect();

        let instructions: Vec<(usize, usize, usize)> = get_instructions(&contents).unwrap();
        let crates = get_crates(&contents);

        let order_q1 = q1(&instructions, &crates).unwrap();
        let order_q2 = q2(&instructions, &crates).unwrap();

        println!("");
        println!("Day 5");
        println!("-------------------");
        println!("Q1: {}", order_q1);
        println!("Q2: {}", order_q2);

        Ok(())
    }

    fn q1(
        instructions: &Vec<(usize, usize, usize)>,
        crates: &[Vec<char>; 9],
    ) -> Result<String, Error> {
        let mut crates = crates.clone();
        for order in instructions {
            let mut count = 0;
            loop {
                if count == order.0 {
                    break;
                }
                let val = crates[order.1].pop().unwrap();
                crates[order.2].push(val);
                count += 1;
            }
        }

        let mut answer: [char; 9] = ['a'; 9];
        let mut count: usize = 0;
        for mut col in crates {
            answer[count] = col.pop().unwrap();
            count += 1;
        }
        Ok(answer.iter().join(""))
    }

    fn q2(
        instructions: &Vec<(usize, usize, usize)>,
        crates: &[Vec<char>; 9],
    ) -> Result<String, Error> {
        let mut crates = crates.clone();

        for order in instructions {
            let mut temp: Vec<char> = vec![];
            let mut count: usize = 0;
            loop {
                if count == order.0 {
                    temp.reverse();
                    for val in temp {
                        crates[order.2].push(val);
                    }
                    break;
                }
                temp.push(crates[order.1].pop().unwrap());
                count += 1;
            }
        }
        let mut answer: [char; 9] = ['a'; 9];
        let mut count: usize = 0;
        for mut col in crates {
            answer[count] = col.pop().unwrap();
            count += 1;
        }
        Ok(answer.iter().join(""))
    }

    fn get_instructions(rows: &Vec<&str>) -> Result<Vec<(usize, usize, usize)>, Error> {
        let mut instructions: Vec<(usize, usize, usize)> = vec![];

        let mut count = 10;

        loop {
            if count == rows.len() {
                break;
            }
            let row: Vec<&str> = rows[count].trim().split(" ").collect();
            let order: (usize, usize, usize) = (
                row[1].parse()?,
                row[3].parse::<usize>()? - 1,
                row[5].parse::<usize>()? - 1,
            );
            instructions.push(order);
            count += 1;
        }
        Ok(instructions)
    }

    fn get_crates(rows: &Vec<&str>) -> [Vec<char>; 9] {
        let mut cols: [Vec<char>; 9] = Default::default();
        let mut count = 8;
        loop {
            count -= 1;
            let mut cols_count = 1;
            let row: Vec<char> = rows[count].trim().chars().collect();

            loop {
                if cols_count == 1 && row[cols_count].is_alphabetic() {
                    cols[0].push(row[cols_count]);
                } else if row[cols_count].is_alphabetic() {
                    cols[(cols_count - 1) / 4].push(row[cols_count]);
                }
                cols_count += 4;
                if cols_count == 37 {
                    break;
                }
            }
            if count == 0 {
                break;
            }
        }
        cols
    }
}
