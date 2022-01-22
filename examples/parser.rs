use custom_error::{Context, CustomError, CustomErrors};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod simple {
    use super::*;

    #[derive(Debug)]
    pub enum ParseError {
        /// Where the value provided is not a valid number. Note there are some characters which look like numbers like l and I which look like 1.
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
                let pieces: Vec<_> = split_with_offset(&line);
                if pieces.len() != 2 {
                    errors += CustomError::new(ParseError::IncorrectNumberOfArguments)
                        .context(Context::new(&line).linenumber(linenumber).file(path));
                    continue;
                }
                if pieces[0].0 != "help" {
                    errors += CustomError::new(ParseError::MissingHelp)
                        .message("A line should always start with 'help'")
                        .context(
                            Context::new(&line)
                                .linenumber(linenumber)
                                .file(path)
                                .highlight(pieces[0].1, pieces[0].2),
                        );
                }
                match pieces[1].0.parse::<usize>() {
                    Ok(n) => output.push(n),
                    Err(e) => {
                        errors += CustomError::new(ParseError::NotANumber)
                            .message("After the 'help' a number should written")
                            .help(e.to_string())
                            .context(
                                Context::new(&line)
                                    .linenumber(linenumber)
                                    .file(path)
                                    .highlight(pieces[1].1, pieces[1].2),
                            )
                    }
                }
            } else {
                errors += CustomError!(ParseError::InvalidLine);
            }
        }
        if errors.is_empty() {
            Ok(output)
        } else {
            Err(errors)
        }
    }
}

fn split_with_offset(input: &str) -> Vec<(&str, usize, usize)> {
    let mut output = Vec::new();
    let mut start = usize::MAX;

    for (n, char) in input.as_bytes().iter().enumerate() {
        if char.is_ascii_whitespace() {
            if start != usize::MAX && start < n {
                output.push((&input[start..n], start, n - start));
                start = usize::MAX;
            }
        } else if start == usize::MAX {
            start = n;
        }
    }
    if start != usize::MAX {
        output.push((&input[start..], start, input.len() - start));
    }
    output
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
