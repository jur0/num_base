use std::error;
use std::fmt;

use self::ErrorKind::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorKind {
    InvalidBase(u8),
    InvalidDigit(char),
    InvalidDigitBase(char, u8),
    NumberOverflow,
}

impl From<ErrorKind> for Error {
    fn from(e: ErrorKind) -> Self {
        Error { kind: e }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            InvalidBase(_base) => "invalid base",
            InvalidDigit(_digit) => "invalid digit",
            InvalidDigitBase(_base, _digit) => "invalid base for digit",
            NumberOverflow => "number overflow",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            InvalidBase(base) => write!(f, "Invalid base: {}.", base),
            InvalidDigit(digit) => write!(f, "Invalid digit: {}.", digit),
            InvalidDigitBase(base, digit) => {
                write!(f, "Invalid base: {} for digit: {}.", base, digit)
            }
            NumberOverflow => write!(f, "Number to convert is too big."),
        }
    }
}
