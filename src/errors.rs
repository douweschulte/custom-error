use crate::colour::*;
use crate::error::CustomError;
use std::convert::From;
use std::fmt::Debug;
use std::fmt::{Display, Formatter, Result};

/// A collection of errors of the given type with convenience methods for handling.
///
/// ```
/// use custom_error::*;
/// // This is the suggested way of creating error codes
/// #[derive(Debug, PartialEq, Eq)]
/// enum ErrorType{
///     Error1
/// }
/// let mut errors = CustomErrors::new();
/// errors += CustomError::new(ErrorType::Error1); // AddAssign is implemented but .push() does the same
/// if !errors.is_empty() {
///     println!("{}", errors);
/// }
/// for error in errors {
///     if *error.kind() == ErrorType::Error1 {
///         // Handle case    
///     }
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CustomErrors<T> {
    errors: Vec<CustomError<T>>,
}

impl<T> CustomErrors<T> {
    /// Create an empty set of errors.
    pub fn new() -> Self {
        CustomErrors { errors: Vec::new() }
    }

    /// Check if there are any errors, warnings, or information messages in this collection.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Check if there are any errors in this collection.
    pub fn any_errors(&self) -> bool {
        self.errors.iter().any(|e| e.is_error())
    }

    /// Check if there are any errors or warnings in this collection.
    pub fn any_errors_or_warnings(&self) -> bool {
        self.errors.iter().any(|e| e.is_error() || e.is_warning())
    }

    /// Add an extra error to the collection
    pub fn push(&mut self, error: CustomError<T>) {
        self.errors.push(error);
    }

    /// Convert one error type to another, useful if you aggregate multiple error sources together.
    /// It works in the same way as [CustomError::convert]
    ///
    /// Note: it is in the form of a method because implementing From or Into does not work. This
    /// is the case because `impl From<T> for T` is in the standard library, which clashes with any
    /// From implementation for generic types used.
    pub fn convert<O: From<T>>(self) -> CustomErrors<O> {
        CustomErrors {
            errors: self.errors.into_iter().map(|e| e.convert()).collect(),
        }
    }

    /// Create an iterator over references of the errors in this collection.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            iter: Box::new(self.errors.iter()),
        }
    }

    /// Create an iterator over mutable references of the errors in this collection.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            iter: Box::new(self.errors.iter_mut()),
        }
    }
}

impl<T> Default for CustomErrors<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> std::ops::AddAssign<CustomError<T>> for CustomErrors<T> {
    fn add_assign(&mut self, rhs: CustomError<T>) {
        self.errors.push(rhs);
    }
}

impl<T, C: Into<CustomError<T>>> std::iter::FromIterator<C> for CustomErrors<T> {
    fn from_iter<I: IntoIterator<Item = C>>(iter: I) -> Self {
        CustomErrors {
            errors: iter.into_iter().map(|i| i.into()).collect(),
        }
    }
}

impl<T, C: Into<CustomError<T>>> std::iter::Extend<C> for CustomErrors<T> {
    fn extend<I: IntoIterator<Item = C>>(&mut self, iter: I) {
        self.errors.extend(iter.into_iter().map(|item| item.into()))
    }
}

pub struct Iter<'a, T> {
    iter: Box<dyn Iterator<Item = &'a CustomError<T>> + 'a>,
}

impl<'a, T> std::iter::Iterator for Iter<'a, T> {
    type Item = &'a CustomError<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct IterMut<'a, T> {
    iter: Box<dyn Iterator<Item = &'a mut CustomError<T>> + 'a>,
}

impl<'a, T> std::iter::Iterator for IterMut<'a, T> {
    type Item = &'a mut CustomError<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<T> std::iter::IntoIterator for CustomErrors<T> {
    type Item = CustomError<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

impl<T: Debug> Display for CustomErrors<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut errors = 0;
        let mut warnings = 0;
        let mut infos = 0;
        for error in &self.errors {
            writeln!(f, "{}", error)?;
            if error.is_error() {
                errors += 1;
            }
            if error.is_warning() {
                warnings += 1;
            }
            if error.is_info() {
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

impl<T: Debug> std::error::Error for CustomErrors<T> {}
