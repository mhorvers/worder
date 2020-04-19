use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::{stdin, stdout, Write};
use termion::clear;
use termion::color;
use termion::cursor;
use termion::event;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn load_from_file(file: &String) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let translations: Vec<(String, String)> = serde_json::from_reader(reader)?;
    Ok(translations)
}

struct Rules {
    pub file_name: String,
    pub amount_of_rounds: u32,
}

enum UserReply {
    Play,
    Exit,
}

fn parse_arguments(args: Vec<String>) -> Result<Rules, Box<dyn Error>> {
    let result = Rules {
        file_name: args[1].clone(),
        amount_of_rounds: args[2].parse::<u32>()?,
    };
    Ok(result)
}

fn get_user_input() -> Result<String, Box<dyn Error>> {
    let mut result = String::new();
    let mut stdout = stdout().into_raw_mode()?;
    for c in stdin().keys() {
        match c? {
            event::Key::Char('\n') => break,
            event::Key::Char(c) => {
                result.push(c);
                write!(stdout, "{}", c)?;
                stdout.flush()?;
            }
            event::Key::Esc => return Ok(String::new()),
            event::Key::Backspace => {
                result.pop();
                write!(stdout, "{}{}", cursor::Left(1), clear::AfterCursor,)?;
                stdout.flush()?;
            }
            _ => continue,
        }
    }
    Ok(result)
}

fn play_game(rules: Rules, words: Vec<(String, String)>) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout().into_raw_mode()?;
    for round in 0..rules.amount_of_rounds {
        let mut current_line = 1;
        write!(
            stdout,
            "{}{}Today's words are:",
            cursor::Goto(1, current_line),
            clear::All
        )?;
        stdout.flush()?;
        current_line += 1;
        for (word1, word2) in &words {
            write!(
                stdout,
                "{}{}{} - {}{}{}",
                cursor::Goto(1, current_line),
                color::Fg(color::AnsiValue::rgb(4, 0, 2)),
                word1,
                word2,
                cursor::Goto(1, current_line + 1),
                termion::cursor::BlinkingBlock
            )?;
            stdout.flush()?;
            current_line += 1;
        }
        for c in stdin().keys() {
            match c? {
                event::Key::Esc => return Ok(()),
                _ => break,
            }
        }
        write!(
            stdout,
            "{}{}{}Round {}!{}{}",
            clear::All,
            cursor::Goto(1, 1),
            color::Fg(color::AnsiValue::rgb(4, 0, 2)),
            round + 1,
            cursor::Goto(1, 2),
            termion::cursor::BlinkingBlock
        )?;
        stdout.flush()?;
        for c in stdin().keys() {
            match c? {
                event::Key::Esc => return Ok(()),
                _ => break,
            }
        }
        let mut shuffled = words.clone();
        shuffled.shuffle(&mut thread_rng());
        for (original_word, translation) in shuffled {
            current_line = 2;
            let (question, answer) = match thread_rng().gen_range(0, 2) {
                0 => (original_word, translation),
                1 => (translation, original_word),
                _ => panic!(),
            };
            write!(
                stdout,
                "{}{}{}{} - {}{}",
                cursor::Goto(1, current_line),
                clear::AfterCursor,
                color::Fg(color::AnsiValue::rgb(4, 0, 2)),
                question,
                cursor::Goto((question.len() + 4) as u16, current_line),
                termion::cursor::BlinkingBlock
            )?;
            stdout.flush().unwrap();
            current_line += 1;
            if get_user_input()? == answer {
                write!(
                    stdout,
                    "{}{}Correct!{}{}",
                    cursor::Goto(1, current_line),
                    color::Fg(color::AnsiValue::rgb(4, 0, 2)),
                    cursor::Goto(1, current_line + 1),
                    termion::cursor::BlinkingBlock
                )?;
                stdout.flush()?;
            } else {
                write!(
                    stdout,
                    "{}{}Wrong! Correct answer is \"{}\"{}{}",
                    cursor::Goto(1, current_line),
                    color::Fg(color::AnsiValue::rgb(4, 0, 2)),
                    answer,
                    cursor::Goto(1, current_line + 1),
                    termion::cursor::BlinkingBlock
                )?;
                stdout.flush()?;
            }
            for c in stdin().keys() {
                match c? {
                    event::Key::Esc => return Ok(()),
                    _ => break,
                }
            }
        }
    }
    Ok(())
}

fn print_greeting(
    rules: &Rules,
    words: &Vec<(String, String)>,
) -> Result<UserReply, Box<dyn Error>> {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}{}Greetings!{}{}",
        clear::All,
        cursor::Goto(1, 1),
        color::Fg(color::AnsiValue::rgb(4, 0, 2)),
        cursor::Goto(1, 2),
        termion::cursor::BlinkingBlock
    )?;
    stdout.flush()?;
    for c in stdin().keys() {
        match c? {
            _ => break,
        }
    }
    write!(
        stdout,
        "{}{}We are playing {} rounds. There are {} word pairs in file \"{}\"{}",
        cursor::Goto(1, 1),
        color::Fg(color::AnsiValue::rgb(4, 0, 2)),
        rules.amount_of_rounds,
        words.len(),
        rules.file_name,
        cursor::Goto(1, 2)
    )?;
    stdout.flush()?;
    write!(
        stdout,
        "{}Ready? Y/n{}",
        color::Fg(color::AnsiValue::rgb(4, 0, 2)),
        cursor::Goto(1, 3)
    )?;
    stdout.flush()?;
    for c in stdin().keys() {
        match c? {
            event::Key::Char('y') | event::Key::Char('Y') => return Ok(UserReply::Play),
            event::Key::Char('n') | event::Key::Char('N') => return Ok(UserReply::Exit),
            _ => continue,
        };
    }
    Ok(UserReply::Exit)
}

fn create_game(rules: Rules, words: Vec<(String, String)>) -> Result<(), Box<dyn Error>> {
    let user_reply = print_greeting(&rules, &words)?;
    match user_reply {
        UserReply::Play => Ok(play_game(rules, words)?),
        UserReply::Exit => Ok(()),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let rules = parse_arguments(args).unwrap();
    let words = load_from_file(&rules.file_name).unwrap();
    create_game(rules, words).unwrap();
}
