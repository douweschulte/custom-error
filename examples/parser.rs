use custom_error::{Context, CustomError, CustomErrors};
use std::fs::File;
use std::io::{BufRead, BufReader};

mod simple {
    use super::*;

    #[derive(Debug)]
    pub enum ParseError {
        NotANumber,
        MissingHelp,
        IncorrectNumberOfArguments,
        InvalidLine,
    }

    pub fn parse(path: &str) -> Result<Vec<usize>, CustomErrors<ParseError>> {
        let mut output = Vec::new();
        let mut errors = CustomErrors::new();
        for (linenumber, line) in BufReader::new(File::open(path).unwrap())
            .lines()
            .enumerate()
        {
            if let Ok(line) = line {
                if line.starts_with('#') {
                    continue;
                }
                let pieces: Vec<_> = line.split_whitespace().collect();
                if pieces.len() != 2 {
                    errors += CustomError!(
                        ParseError::IncorrectNumberOfArguments,
                        "Incorrect number of arguments"
                    )
                    .context(Context::new(&line).linenumber(linenumber));
                    continue;
                }
                if pieces[0] != "help" {
                    errors += CustomError!(ParseError::MissingHelp, "Missing help")
                        .message("A line should always start with 'help'")
                        .context(Context::new(&line).linenumber(linenumber));
                }
                match pieces[1].parse::<usize>() {
                    Ok(n) => output.push(n),
                    Err(e) => {
                        errors += CustomError!(ParseError::NotANumber, "Not a valid number")
                            .message("After the 'help' a number should written")
                            .help(e.to_string())
                            .context(Context::new(&line).linenumber(linenumber))
                    }
                }
            } else {
                errors += CustomError!(ParseError::InvalidLine, "Invalid line");
            }
        }
        if errors.is_empty() {
            Ok(output)
        } else {
            Err(errors)
        }
    }
}

fn main() {
    match simple::parse("examples/example_input_file.txt") {
        Err(errors) => {
            println!("{}", errors);
        }
        Ok(nums) => {
            for num in nums {
                println!("{}", num);
            }
        }
    }
}
