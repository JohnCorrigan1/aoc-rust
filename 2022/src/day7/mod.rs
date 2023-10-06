pub mod day7 {

    use anyhow::{Error, Ok};
    use std::fs;

    struct Dir {
        size: u32,
        name: String,
    }

    const MAX_SIZE: u32 = 100000;

    const TOTAL_SIZE: u32 = 70000000;
    const SIZE_NEEDED: u32 = 30000000;
    pub fn run() -> Result<(), Error> {
        let contents = fs::read_to_string("./src/day7/input.txt")?;
        let contents: Vec<&str> = contents.lines().collect();

        println!("");
        println!("Day 7");
        println!("--------------------");
        println!("Q1: {}", q1(&contents).unwrap());
        println!(
            "Q2: {}",
            q2(&contents, get_to_remove(&contents).unwrap()).unwrap()
        );
        Ok(())
    }

    fn q2(lines: &Vec<&str>, to_remove: u32) -> Result<u32, Error> {
        let mut all_dirs: Vec<Dir> = vec![Dir {
            size: 0,
            name: "/".to_string(),
        }];
        let mut pwd: Vec<&str> = vec![];

        for line in lines {
            let line: Vec<&str> = line.trim().split(" ").collect();
            match line[0] {
                "$" => match line[1] {
                    "cd" => match line[2] {
                        ".." => {
                            pwd.pop();
                            ()
                        }
                        "/" => (),
                        _ => {
                            pwd.push(line[2]);
                        }
                    },
                    _ => (),
                },
                "dir" => {
                    all_dirs.push(Dir {
                        size: 0,
                        name: "/".to_string() + &pwd.join("/") + "/" + line[1],
                    });
                    ()
                }
                _ => {
                    let size: u32 = line[0].parse()?;
                    let mut curr: String = Default::default();
                    for dir in pwd.clone() {
                        curr += &("/".to_string() + dir);
                        for maybe_dir in &mut all_dirs {
                            if maybe_dir.name == curr {
                                maybe_dir.size += size;
                            }
                        }
                    }
                    ()
                }
            }
        }

        let mut curr: u32 = 0;
        for dir in &all_dirs {
            if dir.size >= to_remove {
                if dir.size < curr {
                    curr = dir.size
                } else if curr == 0 {
                    curr = dir.size
                }
            }
        }
        Ok(curr)
    }

    fn q1(lines: &Vec<&str>) -> Result<u32, Error> {
        let mut answer: u32 = 0;
        let mut all_dirs: Vec<Dir> = vec![Dir {
            size: 0,
            name: "/".to_string(),
        }];
        let mut pwd: Vec<&str> = vec![];

        for line in lines {
            let line: Vec<&str> = line.trim().split(" ").collect();
            match line[0] {
                "$" => match line[1] {
                    "cd" => match line[2] {
                        ".." => {
                            pwd.pop();
                            ()
                        }
                        "/" => (),
                        _ => {
                            pwd.push(line[2]);
                        }
                    },
                    _ => (),
                },
                "dir" => {
                    all_dirs.push(Dir {
                        size: 0,
                        name: "/".to_string() + &pwd.join("/") + "/" + line[1],
                    });
                    ()
                }
                _ => {
                    let size: u32 = line[0].parse()?;
                    let mut curr: String = Default::default();
                    for dir in pwd.clone() {
                        curr += &("/".to_string() + dir);
                        for maybe_dir in &mut all_dirs {
                            if maybe_dir.name == curr {
                                maybe_dir.size += size;
                            }
                        }
                    }
                    ()
                }
            }
        }

        for dir in &all_dirs {
            if dir.size <= MAX_SIZE {
                answer += dir.size;
            }
        }
        Ok(answer)
    }

    fn get_to_remove(lines: &Vec<&str>) -> Result<u32, Error> {
        let mut total = 0;
        for line in lines {
            let line: Vec<&str> = line.split(" ").collect();
            match line[0] {
                "$" => (),
                "dir" => (),
                _ => {
                    total += line[0].parse::<u32>().unwrap();
                }
            }
        }

        let remaining = TOTAL_SIZE - total;

        Ok(SIZE_NEEDED - remaining)
    }
}
