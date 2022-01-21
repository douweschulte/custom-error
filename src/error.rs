use crate::colour::*;
use crate::context::Context;
use std::convert::From;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
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

    /// Because implementing From or Into did not work
    pub fn convert<O: From<T> + Debug>(self) -> CustomError<O> {
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
}

#[macro_export]
macro_rules! CustomError {
    ($kind:expr, $title:expr$(,)?) => {
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
            write!(f, "\n\t{}: {}", blue("help"), help)?;
        }
        Ok(())
    }
}

impl<T: Debug> Error for CustomError<T> {}

#[derive(Debug)]
pub struct CustomErrors<T: Debug> {
    errors: Vec<CustomError<T>>,
}

impl<T: Debug> CustomErrors<T> {
    pub fn new() -> Self {
        CustomErrors { errors: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Because implementing From or Into did not work
    pub fn convert<O: From<T> + Debug>(self) -> CustomErrors<O> {
        CustomErrors {
            errors: self.errors.into_iter().map(|e| e.convert()).collect(),
        }
    }
}

impl<T: Debug> Default for CustomErrors<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> std::ops::AddAssign<CustomError<T>> for CustomErrors<T> {
    fn add_assign(&mut self, rhs: CustomError<T>) {
        self.errors.push(rhs);
    }
}

impl<T: Debug> Display for CustomErrors<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut errors = 0;
        let mut warnings = 0;
        let mut infos = 0;
        for error in &self.errors {
            writeln!(f, "{}", error)?;
            if error.level == ErrorLevel::Error {
                errors += 1;
            }
            if error.level == ErrorLevel::Warning {
                warnings += 1;
            }
            if error.level == ErrorLevel::Info {
                infos += 1;
            }
        }
        if errors + warnings + infos == 0 {
            writeln!(f, "\n{}", green("no messages!"))?;
        } else {
            write!(f, "\nencountered: ")?;
            if errors > 0 {
                write!(f, "{} {}", errors, red("errors"))?;
            }
            if warnings > 0 {
                write!(f, "{} {}", warnings, yellow("warnings"))?;
            }
            if infos > 0 {
                write!(f, "{} {}", infos, blue("info messages"))?;
            }
        }

        Ok(())
    }
}

//custom_error::CustomError<SuperError>: std::convert::From<custom_error::CustomError<Type1>>

//impl<Start, End: From<Start> + Debug> Into<CustomError<End>> for CustomError<End> {
//    fn from(error: CustomError<Start>) -> CustomError<End> {
//        CustomError {
//            kind: error.kind.from(),
//            ..error
//        }
//    }
//}

//impl<Start: Debug, End: From<Start> + Debug> CustomError<End> {
//    fn from(error: CustomError<Start>) -> CustomError<End> {
//        CustomError {
//            kind: error.kind.from(),
//            ..error
//        }
//    }
//}
//
//struct Point<T> {
//    x: T,
//    y: T,
//}
//
//impl<Start, End: From<Start>> From<Point<Start>> for Point<End> {
//    fn from(point: Point<Start>) -> Point<End> {
//        Point {
//            x: point.x.into(),
//            y: point.y.into(),
//        }
//    }
//}
//
//impl From<Point<u16>> for Point<usize> {
//    fn from(point: Point<u16>) -> Point<usize> {
//        Point {
//            x: point.x.into(),
//            y: point.y.into(),
//        }
//    }
//}
//
//fn fun() {
//    let p: Point<u16> = Point { x: 0, y: 0 };
//    let p1: Point<usize> = p.into();
//}
