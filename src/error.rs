// ~/src/error.rs
//
//  * Copyright (C) ParsiCore (parsidate) 2024-2025 <parsicore.dev@gmail.com>
//  * Package : parsidate
//  * License : Apache-2.0
//  * Version : 1.7.1
//  * URL     : https://github.com/parsicore/parsidate
//  * Sign: parsidate-20250607-fea13e856dcd-459c6e73c83e49e10162ee28b26ac7cd
//
//! # Centralized Error Handling for `parsidate`
//!
//! This module defines the error types used throughout the `parsidate` library. The primary
//! error type is [`DateError`], which encapsulates all possible failures that can occur during
//! date and time operations.
//!
//! For parsing operations, `DateError` contains a more granular [`ParseErrorKind`] to provide
//! specific details about why a string could not be parsed into a `ParsiDate` or `ParsiDateTime`.
//!
//! Both error types implement the standard `std::error::Error` and `std::fmt::Display` traits,
//! ensuring they integrate seamlessly into the Rust ecosystem for error handling and reporting.

use std::fmt;

// --- Data Structures ---

/// The primary error enum for all operations in the `parsidate` library.
///
/// This enum covers all failure modes, from invalid date construction and parsing
/// to arithmetic overflows and conversion issues.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DateError {
    /// Indicates that a given combination of year, month, and day is not a valid
    /// date in the Persian calendar.
    ///
    /// This error occurs when:
    /// - The year is outside the supported range of `1-9999`.
    /// - The month is not between `1` and `12`.
    /// - The day is `0` or greater than the number of days in the given month and year
    ///   (e.g., day `31` in Mehr, or day `30` in Esfand of a common year).
    ///
    /// Returned by: [`ParsiDate::new`](crate::date::ParsiDate::new), [`ParsiDateTime::new`](crate::datetime::ParsiDateTime::new), and various arithmetic methods.
    InvalidDate,

    /// Indicates that a provided hour, minute, or second is outside its valid range.
    ///
    /// This error occurs when:
    /// - `hour` is greater than `23`.
    /// - `minute` is greater than `59`.
    /// - `second` is greater than `59`.
    ///
    /// Returned by: [`ParsiDateTime::new`](crate::datetime::ParsiDateTime::new) and `with_*` methods for time components.
    InvalidTime,

    /// An error occurred during conversion to or from the Gregorian calendar.
    ///
    /// This can happen if:
    /// - The Gregorian date is before the Persian epoch (approximately March 21, 622 CE).
    /// - The resulting date would fall outside the range supported by `chrono::NaiveDate`.
    /// - An internal calculation error or overflow occurred during the conversion process.
    ///
    /// Returned by: [`ParsiDate::from_gregorian`](crate::date::ParsiDate::from_gregorian), [`ParsiDate::to_gregorian`](crate::date::ParsiDate::to_gregorian), and methods that rely on them.
    GregorianConversionError,

    /// A string could not be parsed into a `ParsiDate` or `ParsiDateTime`.
    ///
    /// This variant wraps a [`ParseErrorKind`] which provides specific details about the
    /// nature of the parsing failure.
    ParseError(ParseErrorKind),

    /// A date arithmetic operation resulted in an overflow or an out-of-range date.
    ///
    /// This error occurs when adding or subtracting durations, months, or years results in
    /// a Persian year outside the supported `1-9999` range, or if an internal integer
    /// overflow happens during the calculation.
    ArithmeticOverflow,

    /// An invalid ordinal day number was provided.
    ///
    /// The ordinal day must be between `1` and `365` for a common year, or `1` and `366`
    /// for a leap year.
    ///
    /// Returned by: [`ParsiDate::from_ordinal`](crate::date::ParsiDate::from_ordinal).
    InvalidOrdinal,
}

/// Provides specific reasons for a parsing failure.
///
/// This enum is wrapped by [`DateError::ParseError`] to give detailed feedback when
/// parsing a string into a date or date-time fails.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ParseErrorKind {
    /// The input string's structure or literal characters did not match the format string.
    /// For example, expecting a `/` but finding a `-`, or the input string has trailing characters.
    FormatMismatch,

    /// A numeric component (e.g., `%Y`, `%m`, `%d`, `%H`) contained non-digit characters,
    /// or did not have the required number of digits.
    InvalidNumber,

    /// The components were parsed successfully but form a logically invalid date.
    /// For example, parsing `"1404/12/30"` with `"%Y/%m/%d"`, where 1404 is not a leap year.
    InvalidDateValue,

    /// The components were parsed successfully but form a logically invalid time.
    /// For example, parsing `"25:10:00"` with `"%H:%M:%S"`.
    InvalidTimeValue,

    /// The format string contained an unrecognized or unsupported specifier for parsing.
    /// For example, using `%A` (weekday name) or `%j` (ordinal day), which are for formatting only.
    UnsupportedSpecifier,

    /// A Persian month name required by the `%B` specifier was not found or recognized in the input.
    InvalidMonthName,

    /// Reserved for future use if weekday parsing is implemented. Currently not returned.
    InvalidWeekdayName,
}

// --- Trait Implementations ---

impl fmt::Display for DateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DateError::InvalidDate => write!(
                f,
                "Invalid Persian date: year, month, or day is out of range or inconsistent"
            ),
            DateError::InvalidTime => {
                write!(f, "Invalid time: hour, minute, or second is out of range")
            }
            DateError::GregorianConversionError => {
                write!(f, "Failed to convert to or from the Gregorian calendar")
            }
            DateError::ParseError(kind) => write!(f, "Parsing error: {}", kind),
            DateError::ArithmeticOverflow => write!(
                f,
                "Date arithmetic resulted in an overflow or a date outside the supported range"
            ),
            DateError::InvalidOrdinal => {
                write!(f, "Invalid ordinal day: must be between 1 and 365/366")
            }
        }
    }
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErrorKind::FormatMismatch => write!(f, "input string does not match the format string's structure"),
            ParseErrorKind::InvalidNumber => write!(f, "a numeric component could not be parsed or had an incorrect number of digits"),
            ParseErrorKind::InvalidDateValue => write!(f, "the parsed components form a logically invalid date (e.g., day 30 in Esfand of a common year)"),
            ParseErrorKind::InvalidTimeValue => write!(f, "the parsed components form a logically invalid time (e.g., hour 24)"),
            ParseErrorKind::UnsupportedSpecifier => write!(f, "the format string contains a specifier that is not supported for parsing"),
            ParseErrorKind::InvalidMonthName => write!(f, "could not recognize a valid Persian month name for the '%B' specifier"),
            ParseErrorKind::InvalidWeekdayName => write!(f, "could not recognize a valid Persian weekday name (currently unused)"),
        }
    }
}

/// Implements the standard `Error` trait for `DateError`.
///
/// This allows `DateError` to be used with standard Rust error handling mechanisms,
/// such as the `?` operator and error-handling libraries like `anyhow` or `thiserror`.
impl std::error::Error for DateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // This implementation does not wrap other errors, so `source` returns `None`.
        // If, in the future, DateError were to wrap an error from `chrono` or another
        // library, that underlying error could be returned here.
        None
    }
}
