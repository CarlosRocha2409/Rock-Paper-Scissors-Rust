pub mod rock_paper_scissorss {
    use rand::Rng;
    use std::{fmt, io};

    fn gnerate_result() -> u8 {
        rand::thread_rng().gen_range(0..4)
    }
    fn get_users_pick() -> u8 {
        let guess: u8 = loop {
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read input");

            let guess: u8 = match guess.trim().parse() {
                Ok(num) => match num {
                    0..=3 => num,
                    _ => {
                        println!("Please input a valid choise");
                        continue;
                    }
                },
                Err(_) => {
                    println!("please input a a valid number");
                    continue;
                }
            };

            break guess;
        };

        guess
    }
    #[derive(Debug, Clone, Copy)]
    pub enum RpsChoice {
        Rock,
        Paper,
        Scissor,
    }
    impl fmt::Display for RpsChoice {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                RpsChoice::Rock => write!(f, "Rock"),
                RpsChoice::Paper => write!(f, "Paper"),
                RpsChoice::Scissor => write!(f, "Scissors"),
            }
        }
    }

    fn print_result(user_choice: &RpsChoice, computer_choice: &RpsChoice, result: &str) {
        println!(
            "Computer picked {} and user picked {}, {}",
            user_choice.to_string(),
            computer_choice.to_string(),
            result
        )
    }

    pub fn play_game() {
        use RpsChoice::{Paper, Rock, Scissor};

        let computer: RpsChoice = match gnerate_result() {
            0 => Rock,
            1 => Paper,
            2 => Scissor,
            _ => Rock,
        };
        let user: RpsChoice = match get_users_pick() {
            0 => Rock,
            1 => Paper,
            2 => Scissor,
            _ => Rock,
        };

        match computer {
            Rock => match user {
                Rock => {
                    print_result(&user, &computer, "Its a tie!");
                }
                Paper => {
                    print_result(&user, &computer, "user win!");
                }
                Scissor => {
                    print_result(&user, &computer, "computer win!");
                }
            },
            Paper => match user {
                Rock => {
                    print_result(&user, &computer, "computer win!");
                }
                Paper => {
                    print_result(&user, &computer, "Its a tie!");
                }
                Scissor => {
                    print_result(&user, &computer, "user win!");
                }
            },

            Scissor => match user {
                Rock => {
                    print_result(&user, &computer, "user win!");
                }
                Paper => {
                    print_result(&user, &computer, "computer win!");
                }
                Scissor => {
                    print_result(&user, &computer, "Its a tie!");
                }
            },
        }
    }
}
