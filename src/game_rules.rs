use std::error::Error;
use std::fmt;

pub struct Rules {
    pub file_name: String,
    pub amount_of_rounds: u32,
}

#[derive(Debug)]
pub struct InvalidNumberOfArguments {
    pub expected: u32,
    pub provided: u32,
}

impl fmt::Display for InvalidNumberOfArguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Invalid number of arguments, expected: {}, provded: {}",
            self.expected, self.provided
        )
    }
}

impl Error for InvalidNumberOfArguments {}

fn verify_args(args: &Vec<String>) -> Result<(), InvalidNumberOfArguments> {
    let provided = args.len() as u32;
    let expected: u32 = 2;
    if provided != expected {
        return Err(InvalidNumberOfArguments { expected, provided });
    }
    Ok(())
}

impl Rules {
    pub fn new(args: Vec<String>) -> Result<Rules, Box<dyn Error>> {
        verify_args(&args)?;
        let result = Rules {
            file_name: args[0].clone(),
            amount_of_rounds: args[1].parse::<u32>()?,
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_number_of_arguments() {
        let rules = Rules::new(vec!["my_file".to_owned(), "2".to_owned()]).unwrap();
        assert_eq!(rules.file_name, "my_file".to_owned());
        assert_eq!(rules.amount_of_rounds, 2);
    }
    #[test]
    fn invalid_number_of_arguments() {
        let rules = Rules::new(vec!["my_file".to_owned()]);
        match rules {
            Err(ref e) => {
                let err = e.downcast_ref::<InvalidNumberOfArguments>().unwrap();
                assert_eq!(err.expected, 2);
                assert_eq!(err.provided, 1);
            }
            _ => panic!(),
        }
    }
}
