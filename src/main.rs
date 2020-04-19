use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

mod game;
mod game_rules;
mod io;

fn load_from_file(file: &String) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let translations: Vec<(String, String)> = serde_json::from_reader(reader)?;
    Ok(translations)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let rules = game_rules::Rules::new(args).unwrap();
    let words = load_from_file(&rules.file_name).unwrap();
    let mut io = io::IO::new().unwrap();
    let mut game = game::Game::new(rules, words, &mut io);
    game.run().unwrap();
}
