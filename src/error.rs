use crate::colour::*;
use crate::context::Context;
use std::convert::From;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::{Display, Formatter};

/// To define an error level, is only used internally in this file
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) enum ErrorLevel {
    Error,
    Warning,
    Info,
}

impl ErrorLevel {
    pub fn in_colour(self, text: impl Into<String>) -> String {
        match self {
            ErrorLevel::Error => red(text),
            ErrorLevel::Warning => yellow(text),
            ErrorLevel::Info => blue(text),
        }
    }
}

impl Display for ErrorLevel {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
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

/// An error which can be defined using builder style methods. It uses a generic
/// type parameter to generate codes (and docs rs links) for every error. It is
/// advised to use C style enums as the type.
///
/// ```
/// use custom_error::*;
/// enum ErrorType {
///     NotANumber,
///     CouldNotOpenFile,
/// }
///
/// fn parse_num(input: &str) -> Result<isize, CustomError<ErrorType>> {
///     match input.parse() {
///         Ok(num) => Ok(num),
///         Err(e) => Err(CustomError::new(ErrorType::NotANumber)
///                     .message("The value provided was not a valid number")
///                     .context(Context::line(e.to_string())))
///     }
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CustomError<T> {
    kind: T,
    level: ErrorLevel,
    title: Option<String>,
    message: Option<String>,
    help: Option<String>,
    url: Option<String>,
    context: Vec<Context>,
    location: Option<String>,
}

/// The functionality useful for creation of a CustomError
impl<T> CustomError<T> {
    /// Create a new error with the given type. It will be classified as an error
    /// (not a warning or info message).
    pub fn new(kind: T) -> Self {
        CustomError {
            kind,
            level: ErrorLevel::Error,
            title: None,
            message: None,
            help: None,
            url: None,
            context: Vec::new(),
            location: None,
        }
    }

    /// Set the title of the error message, this will be displayed before the error code.
    /// If you use descriptive error codes a title is not necessary.
    pub fn title(self, title: impl Into<String>) -> Self {
        CustomError {
            title: Some(title.into()),
            ..self
        }
    }

    /// Add a message to the error.
    pub fn message(self, message: impl Into<String>) -> Self {
        CustomError {
            message: Some(message.into()),
            ..self
        }
    }

    /// Add a message to the error which is flagged with 'help:' in front of it.
    pub fn help(self, help: impl Into<String>) -> Self {
        CustomError {
            help: Some(help.into()),
            ..self
        }
    }

    /// Add a url to extra documentation for this error, if you used the macro to generate
    /// a docs.rs url this will overwrite that url.
    pub fn url(self, url: impl Into<String>) -> Self {
        CustomError {
            url: Some(url.into()),
            ..self
        }
    }

    /// Give context for the error message, like the line where this error was encountered
    /// while reading in a file. Calling this multiple times adds all context
    pub fn context(mut self, context: Context) -> Self {
        self.context.push(context);
        self
    }

    /// Give multiple pieces of context for the error message, like the line where this error
    /// was encountered while reading in a file. With earlier/later pieces of code that made
    /// this error appear. Like setting a lint to deny in clippy, it show the deny line as well.
    pub fn multiple_context(mut self, context: impl IntoIterator<Item = Context>) -> Self {
        self.context.extend(context);
        self
    }

    /// Make this error into a warning.
    pub fn warning(self) -> Self {
        CustomError {
            level: ErrorLevel::Warning,
            ..self
        }
    }

    /// Make this error into an information message
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
    /// Convert one error type to another, useful if you aggregate multiple error sources together.
    /// ```
    /// use custom_error::*;
    /// #[derive(Debug)]
    /// enum Type1 {
    ///     Error1,
    /// }
    ///
    /// impl From<Type1> for SuperError {
    ///     fn from(t: Type1) -> SuperError {
    ///         SuperError::Type1(t)
    ///     }
    /// }
    ///
    /// #[derive(Debug)]
    /// enum Type2 {
    ///     Error1,
    /// }
    ///
    /// impl From<Type2> for SuperError {
    ///     fn from(t: Type2) -> SuperError {
    ///         SuperError::Type2(t)
    ///     }
    /// }
    ///
    /// #[derive(Debug)]
    /// enum SuperError {
    ///     Type1(Type1),
    ///     Type2(Type2),
    /// }
    ///
    /// fn fn1() -> CustomError<Type1> {
    ///     CustomError::new(Type1::Error1)
    /// }
    ///
    /// fn fn2() -> CustomError<Type2> {
    ///     CustomError::new(Type2::Error1)
    /// }
    ///
    /// fn fn3(one: bool) -> CustomError<SuperError> {
    ///     if one {
    ///         fn1().convert()
    ///     } else {
    ///         fn2().convert()
    ///     }
    /// }
    /// ```
    /// Note: it is in the form of a method because implementing From or Into does not work. This
    /// is the case because `impl From<T> for T` is in the standard library, which clashes with any
    /// From implementation for generic types used.
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

    /// Get the kind of this error, which is the type that was provided in the creation of this error.
    /// See [CustomError::new].
    pub fn kind(&self) -> &T {
        &self.kind
    }

    /// Test if this error is flagged as an error.
    pub fn is_error(&self) -> bool {
        self.level == ErrorLevel::Error
    }

    /// Test if this error is flagged as a warning.
    pub fn is_warning(&self) -> bool {
        self.level == ErrorLevel::Warning
    }

    /// Test if this error is flagged as an informational message.
    pub fn is_info(&self) -> bool {
        self.level == ErrorLevel::Info
    }
}

#[macro_export]
/// Create a CustomError with the location where it is generated annotated (in the source file).
/// It can also create a CustomError with a link to the docs.rs page, assuming the crate is published
/// and the used type is an enum.
macro_rules! CustomError {
    // Create a [CustomError] with the location of the code generating this error
    ($kind:expr$(,)?) => {
        CustomError::new($kind).location(format!("{}:{}:{}", file!(), line!(), column!()))
    };
    // Create a [CustomError] with the location of the code generating this error and a link to the docs.rs page for this error (assuming it has one)
    ($kind:expr, doc) => {
        CustomError::new($kind)
            .location(format!("{}:{}:{}", file!(), line!(), column!()))
            .docs_link(module_path!(), env!("CARGO_PKG_VERSION"))
    };
}

impl<T: Debug> Display for CustomError<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if let Some(title) = &self.title {
            writeln!(
                f,
                "{}: {} ({}::{:?})",
                self.level,
                title,
                std::any::type_name::<T>(),
                self.kind,
            )?;
        } else {
            writeln!(
                f,
                "{}: {}::{:?}",
                self.level,
                std::any::type_name::<T>(),
                self.kind,
            )?;
        }
        if let Some(url) = &self.url {
            writeln!(f, "{}: {}", blue("url"), blue(url))?;
        } //┅┅┅┅ ┉┉┉┉┉┉ ┗━━━━━━┛ ╍╍╍╍╍╍ ══════════ ╰────╯╭
        if let Some(location) = &self.location {
            writeln!(f, "  {} generated at: {}", blue("-->"), location)?;
        }
        for context in &self.context {
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

/// A trait to help with creating Custom Errors from structs that are normally used with .unwrap().
pub trait CustomErrorUnwrap<T> {
    /// Use this to create a new error message based on a type normally unwrapped.
    /// ```
    /// use custom_error::*;
    /// enum ErrorType{
    ///     NotANumber
    /// }
    /// fn test() -> Result<(), CustomError<ErrorType>> {
    ///     let a: Result<usize, _> = "12e".parse();
    ///     a.unwrap_or_error(CustomError::new(ErrorType::NotANumber))?; // Use '?' to propagate the error
    ///     Ok(())
    /// }
    /// ```
    fn unwrap_or_error<E>(self, error: CustomError<E>) -> Result<T, CustomError<E>>;
}

/// A trait to help with creating Custom Errors from structs that are normally used with .unwrap().
pub trait CustomErrorFnUnwrap<T, R> {
    /// Use this to create a new error message based on a type normally unwrapped.
    /// ```
    /// use custom_error::*;
    /// enum ErrorType{
    ///     NotANumber
    /// }
    /// fn test() -> Result<(), CustomError<ErrorType>> {
    ///     let a: Result<usize, _> = "12e".parse();
    ///     a.unwrap_or_error_fn(|e| CustomError::new(ErrorType::NotANumber).message(e.to_string()))?; // Use '?' to propagate the error
    ///     Ok(())
    /// }
    /// ```
    fn unwrap_or_error_fn<E, F: Fn(R) -> CustomError<E>>(
        self,
        error_fn: F,
    ) -> Result<T, CustomError<E>>;
}

impl<T, R> CustomErrorUnwrap<T> for Result<T, R> {
    fn unwrap_or_error<E>(self, error: CustomError<E>) -> Result<T, CustomError<E>> {
        if let Ok(o) = self {
            Ok(o)
        } else {
            Err(error)
        }
    }
}

impl<T, R> CustomErrorFnUnwrap<T, R> for Result<T, R> {
    fn unwrap_or_error_fn<E, F: Fn(R) -> CustomError<E>>(
        self,
        error_fn: F,
    ) -> Result<T, CustomError<E>> {
        match self {
            Ok(o) => Ok(o),
            Err(e) => Err(error_fn(e)),
        }
    }
}

impl<T> CustomErrorUnwrap<T> for Option<T> {
    fn unwrap_or_error<E>(self, error: CustomError<E>) -> Result<T, CustomError<E>> {
        if let Some(o) = self {
            Ok(o)
        } else {
            Err(error)
        }
    }
}
