use crate::colour::*;
use crate::context::Context;
use std::convert::From;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CustomError<T> {
    kind: T,
    level: ErrorLevel,
    title: String,
    message: Option<String>,
    help: Option<String>,
    url: Option<String>,
    context: Option<Context>,
    location: Option<String>,
}

/// The functionality useful for creation of a CustomError
impl<T> CustomError<T> {
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

    /// Should not be used by end users, use the macro instead [CustomError!]
    #[doc(hidden)]
    pub fn location(self, location: String) -> Self {
        CustomError {
            location: Some(location),
            ..self
        }
    }
}

impl<T: Debug> CustomError<T> {
    /// Should not be used by end users, use the macro instead [CustomError!]
    #[doc(hidden)]
    pub fn docs_link(self, module_path: &str, version: &str) -> Self {
        let module_path = module_path.split("::").collect::<Vec<_>>();
        let ty = std::any::type_name::<T>().split("::");
        CustomError {
            url: Some(format!(
                "https://docs.rs/{crate}/{version}/{crate}/{path}enum.{name}.html#variant.{variant:?}",
                crate = module_path[0],
                version = version,
                path = module_path
                    .iter()
                    .skip(1)
                    .fold("".to_string(), |acc, item| acc + item + "/"),
                name = ty.last().unwrap(),
                variant = self.kind
            )),
            ..self
        }
    }
}

/// The functionality useful for manipulation/introspection after creation
impl<T> CustomError<T> {
    /// Because implementing From or Into did not work
    pub fn convert<O: From<T>>(self) -> CustomError<O> {
        CustomError {
            kind: self.kind.into(),
            level: self.level,
            title: self.title,
            message: self.message,
            help: self.help,
            url: self.url,
            context: self.context,
            location: self.location,
        }
    }

    pub fn kind(&self) -> &T {
        &self.kind
    }

    pub fn is_error(&self) -> bool {
        self.level == ErrorLevel::Error
    }

    pub fn is_warning(&self) -> bool {
        self.level == ErrorLevel::Warning
    }

    pub fn is_info(&self) -> bool {
        self.level == ErrorLevel::Info
    }
}

#[macro_export]
macro_rules! CustomError {
    // Create a [CustomError] with the location of the code generating this error
    ($kind:expr, $title:expr$(,)?) => {
        CustomError::new($kind, $title).location(format!("{}:{}:{}", file!(), line!(), column!()))
    };
    // Create a [CustomError] with the location of the code generating this error and a link to the docs.rs page for this error (assuming it has one)
    ($kind:expr, $title:expr, doc) => {
        CustomError::new($kind, $title)
            .location(format!("{}:{}:{}", file!(), line!(), column!()))
            .docs_link(module_path!(), env!("CARGO_PKG_VERSION"))
    };
}

impl<T: Debug> Display for CustomError<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(
            f,
            "{}: {} ({}::{:?})",
            self.level,
            self.title,
            std::any::type_name::<T>(),
            self.kind,
        )?;
        if let Some(url) = &self.url {
            writeln!(f, "{}: {}", blue("url"), blue(url))?;
        }
        if let Some(location) = &self.location {
            writeln!(f, "  {} {}", blue("-->"), location)?;
        }
        if let Some(context) = &self.context {
            write!(f, "{}", context)?;
        }
        if let Some(message) = &self.message {
            writeln!(f, "{}", message)?;
        }
        if let Some(help) = &self.help {
            writeln!(f, "  {}: {}", blue("help"), help)?;
        }
        Ok(())
    }
}

impl<T: Debug> Error for CustomError<T> {}
