use crate::colour::*;
use crate::error::ErrorLevel;
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
///                         Context::line(line) // Create the context
///                         .linenumber(linenumber) // Add the linenumber
///                         // Add a highlight, with an offset and length
///                         .highlight(
///                             (line.len() - line.trim_start().len(), // Check how much whitespace there is before the number
///                             line.len() - line.trim_end().len())))) // Check how much whitespace there is after the number
///     }
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Context {
    lines: Vec<String>,
    linenumber: Option<usize>,
    highlights: Vec<Highlight>,
    file: Option<String>,
}

impl Context {
    /// Create a new Context with a single line
    pub fn line(line: impl Into<String>) -> Self {
        Context {
            lines: vec![line.into()],
            linenumber: None,
            highlights: Vec::new(),
            file: None,
        }
    }

    /// Create a new Context with multiple lines
    pub fn lines(lines: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Context {
            lines: lines.into_iter().map(|i| i.into()).collect(),
            linenumber: None,
            highlights: Vec::new(),
            file: None,
        }
    }

    /// Add a linenumber for the first line in the given lines
    pub fn linenumber(self, linenumber: usize) -> Self {
        Context {
            linenumber: Some(linenumber),
            ..self
        }
    }

    /// Add a single highlight to the context line
    pub fn highlight(mut self, highlight: impl Into<Highlight>) -> Self {
        self.highlights.push(highlight.into());
        self
    }

    /// Add highlights to the context line
    pub fn highlights(
        mut self,
        highlights: impl IntoIterator<Item = impl Into<Highlight>>,
    ) -> Self {
        self.highlights
            .extend(highlights.into_iter().map(|i| i.into()));
        self
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

/// A highlight in a context for an error.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Highlight {
    /// The line offset in the list of lines for a context
    line: usize,
    /// The column in the specified line
    column: usize,
    /// The length of the highlight
    length: usize,
    /// An optional note to display after the highlight
    note: Option<String>,
    level: ErrorLevel,
}

impl Highlight {
    /// Create a new highlight at the given position
    pub fn new(line: usize, column: usize, length: usize) -> Self {
        Self {
            line,
            column,
            length,
            note: None,
            level: ErrorLevel::Error,
        }
    }

    /// Add a note to the highlight
    pub fn note(self, note: impl Into<String>) -> Self {
        Self {
            note: Some(note.into()),
            ..self
        }
    }

    /// Make this error into a warning.
    pub fn warning(self) -> Self {
        Self {
            level: ErrorLevel::Warning,
            ..self
        }
    }

    /// Make this error into an information message
    pub fn info(self) -> Self {
        Self {
            level: ErrorLevel::Info,
            ..self
        }
    }
}

impl From<(usize, usize, usize)> for Highlight {
    fn from(tuple: (usize, usize, usize)) -> Self {
        Highlight::new(tuple.0, tuple.1, tuple.2)
    }
}

impl From<(usize, usize)> for Highlight {
    fn from(tuple: (usize, usize)) -> Self {
        Highlight::new(0, tuple.0, tuple.1)
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Determine how many chars are needed to display the biggest line number, default is 1 to have at least 1 character
        let linenumber_padding = ((self.linenumber.unwrap_or(1) + self.lines.len()) as f64)
            .log10()
            .ceil() as usize;

        if let Some(file) = &self.file {
            if self.highlights.len() == 1 {
                // Show the filename and location of the highlight (if there is only one)
                writeln!(
                    f,
                    "{:pad$} {}[{}{}]",
                    "",
                    blue("╭──"),
                    file,
                    self.linenumber // Show the linenumber followed by the column if the linenumber is known
                        .map(|l| {
                            let highlight = &self.highlights[0];
                            format!(":{}:{}", l + highlight.line, highlight.column)
                        })
                        .unwrap_or_else(|| "".to_string()),
                    pad = linenumber_padding
                )?;
            } else {
                // If there are no or multiple highlights only show the filename
                writeln!(
                    f,
                    "{:pad$} {}[{}]",
                    "",
                    blue("╭──"),
                    file,
                    pad = linenumber_padding
                )?;
            }
            // Extend the sideline so that it provides a single line of border between the file header and content
            writeln!(f, "{:pad$} {}", "", blue("│"), pad = linenumber_padding)?;
        } else {
            // If there is no file known just end the sideline nicely
            writeln!(f, "{:pad$} {}", "", blue("╷"), pad = linenumber_padding)?;
        }

        // Use offset numbers if there is no linenumber given
        let linenumber = self.linenumber.unwrap_or(0);
        for (index, line) in self.lines.iter().enumerate() {
            // Write the current line
            writeln!(
                f,
                "{:>pad$} {} {}",
                grey((linenumber + index).to_string()),
                blue("│"),
                line,
                pad = linenumber_padding,
            )?;
            // Determine if there needs to be a highlight
            for highlight in &self.highlights {
                if index == highlight.line {
                    writeln!(
                        f,
                        "{:>pad$} {} {}{}{}",
                        "",
                        blue("·"),
                        " ".repeat(highlight.column),
                        highlight.level.in_colour("─".repeat(highlight.length)),
                        highlight.level.in_colour(
                            highlight
                                .note
                                .as_ref()
                                .map(|n| " ".to_string() + n)
                                .unwrap_or_else(|| "".to_string())
                        ),
                        pad = linenumber_padding,
                    )?;
                }
            }
        }
        // Nicely end the sideline
        writeln!(f, "{:pad$} {}", "", blue("╵"), pad = linenumber_padding)?;
        Ok(())
    }
}
