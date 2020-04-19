use std::error::Error;
use std::io::{stdin, stdout, Stdout, Write};
use termion::clear;
use termion::cursor;
use termion::event;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub enum Reply<'a> {
    Yes(&'a mut IO),
    No(&'a mut IO),
}

pub struct UserInput<'a>(pub String, pub &'a mut IO);

pub struct IO {
    stdout: termion::raw::RawTerminal<Stdout>,
    current_x: u16,
    current_y: u16,
}

impl IO {
    pub fn new() -> Result<IO, Box<dyn Error>> {
        Ok(IO {
            stdout: stdout().into_raw_mode()?,
            current_x: 1,
            current_y: 1,
        })
    }

    fn new_line(&mut self) -> Result<&mut IO, Box<dyn Error>> {
        self.current_x = 1;
        self.current_y += 1;
        write!(
            self.stdout,
            "{}",
            cursor::Goto(self.current_x, self.current_y)
        )?;
        self.stdout.flush()?;
        Ok(self)
    }

    fn write_string(&mut self, string: &str) -> Result<&mut IO, Box<dyn Error>> {
        write!(self.stdout, "{}", string)?;
        self.stdout.flush()?;
        self.current_x += string.len() as u16;
        Ok(self)
    }

    fn move_cursor_left(&mut self, steps: u16) -> Result<&mut IO, Box<dyn Error>> {
        if self.current_x > 1 {
            self.current_x -= 1;
            write!(self.stdout, "{}{}", cursor::Left(steps), clear::AfterCursor,)?;
            self.stdout.flush()?;
        } // TODO error handling
        Ok(self)
    }

    pub fn clear(&mut self) -> Result<&mut IO, Box<dyn Error>> {
        self.current_x = 1;
        self.current_y = 1;
        write!(
            self.stdout,
            "{}{}",
            clear::All,
            cursor::Goto(self.current_x, self.current_y)
        )?;
        self.stdout.flush()?;
        Ok(self)
    }

    pub fn put_string(&mut self, string: &str) -> Result<&mut IO, Box<dyn Error>> {
        self.write_string(string)?;
        self.new_line()?;
        Ok(self)
    }

    pub fn request_confirmation(&mut self, question: &str) -> Result<Reply, Box<dyn Error>> {
        self.put_string(question)?;
        self.put_string("Y/n")?;
        for c in stdin().keys() {
            match c? {
                event::Key::Char('y') | event::Key::Char('Y') => return Ok(Reply::Yes(self)),
                event::Key::Char('n') | event::Key::Char('N') => return Ok(Reply::No(self)),
                _ => continue,
            };
        }
        Ok(Reply::No(self))
    }

    pub fn request_any_key(&mut self) -> Result<&mut IO, Box<dyn Error>> {
        self.put_string("Press any key")?;
        for c in stdin().keys() {
            match c? {
                _ => break,
            }
        }
        Ok(self)
    }

    pub fn request_user_input(
        &mut self,
        string: Option<&str>,
    ) -> Result<UserInput, Box<dyn Error>> {
        if let Some(request) = string {
            self.write_string(request)?;
        }
        let mut result = String::new();
        for c in stdin().keys() {
            match c? {
                event::Key::Char('\n') => break,
                event::Key::Char(c) => {
                    self.write_string(&c.to_string())?;
                    result.push(c);
                }
                event::Key::Backspace => {
                    result.pop();
                    self.move_cursor_left(1)?;
                }
                _ => continue,
            }
        }
        self.new_line()?;
        Ok(UserInput(result, self))
    }
}
