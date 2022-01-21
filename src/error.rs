use crate::colour::*;
use crate::context::Context;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
enum ErrorLevel {
    Error,
    Warning,
    Info,
}

impl Display for ErrorLevel {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match self {
                ErrorLevel::Error => red("error"),
                ErrorLevel::Warning => yellow("warning"),
                ErrorLevel::Info => blue("info"),
            },
        )?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct CustomError<T: Debug> {
    kind: T,
    level: ErrorLevel,
    title: String,
    message: Option<String>,
    help: Option<String>,
    url: Option<String>,
    context: Option<Context>,
    location: Option<String>,
}

impl<T: Debug> CustomError<T> {
    pub fn new(kind: T, title: impl Into<String>) -> Self {
        CustomError {
            kind,
            level: ErrorLevel::Error,
            title: title.into(),
            message: None,
            help: None,
            url: None,
            context: None,
            location: None,
        }
    }

    pub fn message(self, message: impl Into<String>) -> Self {
        CustomError {
            message: Some(message.into()),
            ..self
        }
    }

    pub fn help(self, help: impl Into<String>) -> Self {
        CustomError {
            help: Some(help.into()),
            ..self
        }
    }

    pub fn url(self, url: impl Into<String>) -> Self {
        CustomError {
            url: Some(url.into()),
            ..self
        }
    }

    pub fn context(self, context: Context) -> Self {
        CustomError {
            context: Some(context),
            ..self
        }
    }

    pub fn warning(self) -> Self {
        CustomError {
            level: ErrorLevel::Warning,
            ..self
        }
    }

    pub fn info(self) -> Self {
        CustomError {
            level: ErrorLevel::Info,
            ..self
        }
    }

    pub fn location(self, location: String) -> Self {
        CustomError {
            location: Some(location),
            ..self
        }
    }
}

#[macro_export]
macro_rules! CustomError {
    ($kind:expr, $title:expr) => {
        CustomError::new($kind, $title).location(format!("{}:{}:{}", file!(), line!(), column!()))
    };
}

impl<T: Debug> Display for CustomError<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}: {} ({}::{:?})",
            self.level,
            self.title,
            std::any::type_name::<T>(),
            self.kind,
        )?;
        if let Some(url) = &self.url {
            write!(f, "\n{}: {}", blue("url"), blue(url))?;
        }
        if let Some(location) = &self.location {
            write!(f, "\n  {} {}", blue("-->"), location)?;
        }
        if let Some(context) = &self.context {
            write!(f, "\n\n{}", context)?;
        }
        if let Some(message) = &self.message {
            write!(f, "\n{}", message)?;
        }
        if let Some(help) = &self.help {
            write!(f, "\n{}: {}", blue("help"), help)?;
        }
        Ok(())
    }
}

impl<T: Debug> Error for CustomError<T> {}
