use crate::colour::*;
use std::fmt::{Display, Formatter, Result};

/// The context for an error message. This can be created using builder style methods.
/// ```
/// use custom_error::*;
/// enum ErrorType {
///     NotANumber,
/// }
/// fn parse_num(line: &str, linenumber: usize) -> Result<isize, CustomError<ErrorType>> {
///     match line.parse() {
///         Ok(num) => Ok(num),
///         Err(e) => Err(CustomError::new(ErrorType::NotANumber)
///                     .message("The value provided was not a valid number")
///                     .context(
///                         Context::new(line) // Create the context
///                         .linenumber(linenumber) // Add the linenumber
///                         // Add a highlight, with an offset and length
///                         .highlight(
///                             line.len() - line.trim_start().len(), // Check how much whitespace there is before the number
///                             line.len() - line.trim_end().len()))) // Check how much whitespace there is after the number
///     }
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Context {
    line: String,
    linenumber: Option<usize>,
    context_before: Option<Vec<String>>,
    context_after: Option<Vec<String>>,
    highlight: Option<(usize, usize)>,
    file: Option<String>,
}

impl Context {
    /// Create a new Context.
    pub fn new(line: impl Into<String>) -> Self {
        Context {
            line: line.into(),
            linenumber: None,
            context_before: None,
            context_after: None,
            highlight: None,
            file: None,
        }
    }

    /// Add a linenumber for this context
    pub fn linenumber(self, linenumber: usize) -> Self {
        Context {
            linenumber: Some(linenumber),
            ..self
        }
    }

    /// Add lines to be displayed before the context line, if a linenumber is given the correct linenumbers will be generated.
    pub fn context_before(self, context: Vec<impl Into<String>>) -> Self {
        Context {
            context_before: Some(context.into_iter().map(|i| i.into()).collect()),
            ..self
        }
    }

    /// Add lines to be displayed after the context line, if a linenumber is given the correct linenumbers will be generated.
    pub fn context_after(self, context: Vec<impl Into<String>>) -> Self {
        Context {
            context_after: Some(context.into_iter().map(|i| i.into()).collect()),
            ..self
        }
    }

    /// Add a highlight to the context line
    pub fn highlight(self, offset: usize, length: usize) -> Self {
        Context {
            highlight: Some((offset, length)),
            ..self
        }
    }

    /// Add the name of the file where this context is located. It automatically adds linenumber information
    /// from the linenumber (if given) and column information from the highlight (if given). Which results in
    /// a location like this: `-->src/context.rs:81:53`.
    pub fn file(self, file: impl Into<String>) -> Self {
        Context {
            file: Some(file.into()),
            ..self
        }
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let linenumber_padding = (if let Some(linenumber) = self.linenumber {
            if let Some(context_after) = &self.context_after {
                linenumber + context_after.len()
            } else {
                linenumber
            }
        } else {
            0
        } as f64)
            .log10()
            .ceil() as usize;
        if let Some(file) = &self.file {
            writeln!(
                f,
                "{:pad$} {}[{}{}{}]",
                "",
                blue("╭──"),
                file,
                self.linenumber
                    .map(|l| ":".to_string() + &l.to_string())
                    .unwrap_or_default(),
                self.highlight
                    .map(|(column, _)| self
                        .linenumber // map over linenumber to make sure it only shows the column if the line is also known
                        .map(|_| ":".to_string() + &column.to_string()))
                    .flatten()
                    .unwrap_or_default(),
                pad = linenumber_padding
            )?;
            writeln!(f, "{:pad$} {}", "", blue("│"), pad = linenumber_padding)?;
        } else {
            writeln!(f, "{:pad$} {}", "", blue("╷"), pad = linenumber_padding)?;
        }
        if let Some(number) = self.linenumber {
            if let Some(before) = &self.context_before {
                let mut current_number = number - before.len();
                for line in before {
                    writeln!(
                        f,
                        "{:>pad$} {} {}",
                        grey(current_number.to_string()),
                        blue("│"),
                        line,
                        pad = linenumber_padding,
                    )?;
                    current_number += 1;
                }
            }
            writeln!(
                f,
                "{:>pad$} {} {}",
                grey(number.to_string()),
                blue("│"),
                self.line,
                pad = linenumber_padding,
            )?;
            if let Some((offset, length)) = self.highlight {
                writeln!(
                    f,
                    "{:>pad$} {} {}{}",
                    "",
                    blue("·"),
                    " ".repeat(offset),
                    red("─".repeat(length)),
                    pad = linenumber_padding,
                )?;
            }
            if let Some(after) = &self.context_after {
                let mut current_number = number + 1;
                for line in after {
                    writeln!(
                        f,
                        "{:>pad$} {} {}",
                        grey(current_number.to_string()),
                        blue("│"),
                        line,
                        pad = linenumber_padding,
                    )?;
                    current_number += 1;
                }
            }
        } else {
            if let Some(before) = &self.context_before {
                let mut current_number = -(before.len() as isize);
                for line in before {
                    writeln!(
                        f,
                        "{:>pad$} {} {}",
                        grey(current_number.to_string()),
                        blue("│"),
                        line,
                        pad = linenumber_padding,
                    )?;
                    current_number += 1;
                }
            }
            writeln!(
                f,
                "{:pad$} {} {}",
                "",
                blue("│"),
                self.line,
                pad = linenumber_padding,
            )?;
            if let Some(after) = &self.context_after {
                for (number, line) in after.iter().enumerate() {
                    writeln!(
                        f,
                        "{:+>pad$} {} {}",
                        grey((number + 1).to_string()),
                        blue("│"),
                        line,
                        pad = linenumber_padding,
                    )?;
                }
            }
        }
        writeln!(f, "{:pad$} {}", "", blue("╵"), pad = linenumber_padding)?;
        Ok(())
    }
}
