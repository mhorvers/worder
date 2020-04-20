use crate::game_rules::Rules;
use crate::io::Reply;
use crate::io::IO;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::error::Error;

pub struct Game<'a> {
    rules: Rules,
    words: Vec<(String, String)>,
    io: &'a mut IO,
}

fn generate_question(data: (String, String)) -> (String, String) {
    match thread_rng().gen_range(0, 2) {
        0 => (data.0, data.1),
        1 => (data.1, data.0),
        _ => panic!(),
    }
}

impl<'a> Game<'a> {
    pub fn new(rules: Rules, words: Vec<(String, String)>, io: &mut IO) -> Game {
        Game { rules, words, io }
    }

    fn play_game(&mut self) -> Result<(), Box<dyn Error>> {
        for round in 0..self.rules.amount_of_rounds {
            self.io
                .clear()
                .and_then(|io| io.put_string("Today's words are"))?;
            for (word1, word2) in &self.words {
                self.io
                    .put_string(format!("{} - {}", word1, word2).as_str())?;
            }
            self.io
                .request_any_key()
                .and_then(|io| io.clear())
                .and_then(|io| io.put_string(format!("Round {}!", round + 1).as_str()))
                .and_then(|io| io.request_any_key())?;

            let mut shuffled = self.words.clone();
            shuffled.shuffle(&mut thread_rng());
            for data in shuffled {
                let (question, answer) = generate_question(data);
                let user_answer = self
                    .io
                    .request_user_input(Some(format!("{} - ", question).as_str()))?;
                if user_answer.0 == answer {
                    self.io.put_string("Correct!")?;
                } else {
                    self.io
                        .put_string(format!("Wrong! Correct answer is \"{}\"", answer).as_str())?;
                }
            }
        }
        Ok(())
    }

    fn print_greeting(&mut self) -> Result<(), Box<dyn Error>> {
        let amount_of_rounds = self.rules.amount_of_rounds;
        let amount_of_word_pairs = self.words.len();
        let file_name = self.rules.file_name.as_str();
        let reply = self
            .io
            .clear()
            .and_then(|io| io.put_string("Greetings!"))
            .and_then(|io| {
                io.put_string(
                    format!(
                        "We are playing {} rounds. There are {} word pairs in file \"{}\"",
                        amount_of_rounds, amount_of_word_pairs, file_name
                    )
                    .as_str(),
                )
            })
            .and_then(|io| io.request_confirmation("Start playing?"))?;
        match reply {
            Reply::Yes(_) => self.play_game(),
            Reply::No(_) => Ok(()),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.print_greeting()?;
        Ok(())
    }
}
