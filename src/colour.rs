#[cfg(feature = "ansi_term")]
use ansi_term::Colour::{Blue, Green, Red, Yellow};

pub fn red(input: impl Into<String>) -> String {
    if cfg!(feature = "ansi_term") {
        Red.paint(input.into()).to_string()
    } else {
        input.into()
    }
}

pub fn yellow(input: impl Into<String>) -> String {
    if cfg!(feature = "ansi_term") {
        Yellow.paint(input.into()).to_string()
    } else {
        input.into()
    }
}

pub fn blue(input: impl Into<String>) -> String {
    if cfg!(feature = "ansi_term") {
        Blue.paint(input.into()).to_string()
    } else {
        input.into()
    }
}

pub fn green(input: impl Into<String>) -> String {
    if cfg!(feature = "ansi_term") {
        Green.paint(input.into()).to_string()
    } else {
        input.into()
    }
}
