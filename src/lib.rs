#![warn(missing_docs)]
//! # Custom error
//! A library to easily create nice end user facing errors, especially for custom parsing work.
//!
//! ## A simple example
//! ```
//! use custom_error::*;
//! enum ErrorType {
//!     NotANumber,
//!     CouldNotOpenFile,
//! }
//!
//! fn parse_num(input: &str) -> Result<isize, CustomError<ErrorType>> {
//!     match input.parse() {
//!         Ok(num) => Ok(num),
//!         Err(e) => Err(CustomError::new(ErrorType::NotANumber)
//!                     .message("The value provided was not a valid number")
//!                     .context(Context::new(e.to_string())))
//!     }
//! }
//! ```
//!
//! ## Errors can be aggregated
//! ```
//! use custom_error::*;
//! // This is the suggested way of creating error codes
//! #[derive(Debug, PartialEq, Eq)]
//! enum ErrorType{
//!     Error1
//! }
//! let mut errors = CustomErrors::new();
//! errors += CustomError::new(ErrorType::Error1); // AddAssign is implemented but .push() does the same
//! if !errors.is_empty() {
//!     println!("{}", errors);
//! }
//! for error in errors {
//!     if *error.kind() == ErrorType::Error1 {
//!         // Handle case    
//!     }
//! }
//! ```
mod colour;
mod context;
mod error;
mod errors;

pub use context::*;
pub use error::*;
pub use errors::CustomErrors;
