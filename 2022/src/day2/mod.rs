pub mod day2 {
    use anyhow::Error;
    use std::fs::read_to_string;
    pub enum OpponentChoice {
        A,
        B,
        C,
    }

    pub enum MyChoice {
        X,
        Y,
        Z,
    }

    impl From<char> for OpponentChoice {
        fn from(value: char) -> Self {
            match value {
                'A' => OpponentChoice::A,
                'B' => OpponentChoice::B,
                'C' => OpponentChoice::C,
                _ => panic!("Invalid input"),
            }
        }
    }

    impl OpponentChoice {
        fn round_q1(&self, me: &MyChoice) -> i32 {
            match self {
                OpponentChoice::A => match me {
                    MyChoice::X => 4,
                    MyChoice::Y => 8,
                    MyChoice::Z => 3,
                },
                OpponentChoice::B => match me {
                    MyChoice::X => 1,
                    MyChoice::Y => 5,
                    MyChoice::Z => 9,
                },
                OpponentChoice::C => match me {
                    MyChoice::X => 7,
                    MyChoice::Y => 2,
                    MyChoice::Z => 6,
                },
            }
        }

        fn round_q2(&self, me: &MyChoice) -> i32 {
            match self {
                OpponentChoice::A => match me {
                    MyChoice::X => 3,
                    MyChoice::Y => 4,
                    MyChoice::Z => 8,
                },
                OpponentChoice::B => match me {
                    MyChoice::X => 1,
                    MyChoice::Y => 5,
                    MyChoice::Z => 9,
                },
                OpponentChoice::C => match me {
                    MyChoice::X => 2,
                    MyChoice::Y => 6,
                    MyChoice::Z => 7,
                },
            }
        }
    }

    impl From<char> for MyChoice {
        fn from(value: char) -> Self {
            match value {
                'X' => MyChoice::X,
                'Y' => MyChoice::Y,
                'Z' => MyChoice::Z,
                _ => panic!("Invalid input"),
            }
        }
    }
    pub fn run() -> Result<(), Error> {
        let contents = read_to_string("./src/day2/input.txt")?;
        let contents: Vec<&str> = contents.lines().collect();

        let mut score_q1 = 0;
        let mut score_q2 = 0;

        for line in contents {
            let round_choices: Vec<char> = line.trim().chars().collect();
            let opp_choice: OpponentChoice = round_choices[0].into();
            let my_choice: MyChoice = round_choices[2].into();

            score_q1 += opp_choice.round_q1(&my_choice);
            score_q2 += opp_choice.round_q2(&my_choice);
        }

        println!("");
        println!("Day 2");
        println!("---------------");
        println!("Q1: {}", score_q1);
        println!("Q2: {}", score_q2);
        Ok(())
    }

    // fn round_q1(opp: char, me: char) -> Result<i32, Error> {
    //     match opp {
    //         'A' => match me {
    //             'X' => 4,
    //             'Y' => 8,
    //             'Z' => 3,
    //             _ => panic!("Not valid input"),
    //         },
    //         'B' => match me {
    //             'X' => 1,
    //             'Y' => 5,
    //             'Z' => 9,
    //             _ => panic!("Not valid input"),
    //         },
    //         'C' => match me {
    //             'X' => 7,
    //             'Y' => 2,
    //             'Z' => 6,
    //             _ => panic!("Not valid input"),
    //         },
    //         _ => panic!("Not valid input"),
    //     }
    // }

    // fn round_q2(opp: char, me: char) -> i32 {
    //     match opp {
    //         'A' => match me {
    //             'X' => 3,
    //             'Y' => 4,
    //             'Z' => 8,
    //             _ => panic!("Not valid input"),
    //         },
    //         'B' => match me {
    //             'X' => 1,
    //             'Y' => 5,
    //             'Z' => 9,
    //             _ => panic!("Not valid input"),
    //         },
    //         'C' => match me {
    //             'X' => 2,
    //             'Y' => 6,
    //             'Z' => 7,
    //             _ => panic!("Not valid input"),
    //         },
    //         _ => panic!("Not valid input"),
    //     }
    // }
}
