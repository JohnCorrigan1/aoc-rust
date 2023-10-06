pub mod day1 {
    use std::fs;
    pub fn run() {
        let contents =
            fs::read_to_string("./src/day1/input.txt").expect("Should have read the file");

        let split = contents.split("\n");

        let mut biggest = 0;
        let mut curr = 0;
        let mut top3: Vec<u32> = Vec::new();

        for line in split {
            if line.trim() != "" {
                let line: u32 = match line.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                curr += line;
            } else {
                top3.push(curr);
                if curr > biggest {
                    biggest = curr;
                }
                curr = 0;
            }
        }

        top3.sort_by(|a, b| b.cmp(a));
        let top3 = top3[0] + top3[1] + top3[2];
        println!("");
        println!("Day1");
        println!("--------------------");
        println!("Q1: {}", biggest);
        println!("Q2: {}", top3);
    }
}
