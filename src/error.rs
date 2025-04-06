//  * src/error.rs
//
//  * Copyright (C) Mohammad (Sina) Jalalvandi (parsidate) 2024-2025 <jalalvandi.sina@gmail.com>
//  * Version : 1.4.0
//  * eb1f0cae-a178-41e5-b109-47f208e77913
//
//! Defines the error types used within the parsidate library.

use std::fmt;

// --- Data Structures ---

/// Enumerates potential errors during date operations within the `parsidate` crate.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DateError {
    /// Indicates that a given set of year, month, and day does not form a valid date
    /// in the Persian calendar (e.g., month 13, day 32, day 30 in Esfand of a non-leap year,
    /// or year outside the supported 1-9999 range).
    InvalidDate,
    /// Indicates an issue during the conversion to or from the Gregorian calendar.
    /// This can happen if:
    /// * The target Gregorian date is outside the range supported by `chrono::NaiveDate`.
    /// * An internal arithmetic operation during conversion resulted in an overflow.
    /// * The Gregorian date provided is before the Persian epoch start (approx. 622-03-21).
    GregorianConversionError,
    /// Indicates an error during parsing a string into a `ParsiDate`.
    /// Contains a `ParseErrorKind` detailing the specific reason for the failure.
    ParseError(ParseErrorKind),
    /// Indicates that a date arithmetic operation (e.g., adding/subtracting days, months, years)
    /// resulted in a date outside the representable range (Years 1-9999) or caused an
    /// internal integer overflow during calculation.
    ArithmeticOverflow,
    /// Indicates an invalid ordinal day number was provided (e.g., to `from_ordinal`).
    /// The ordinal day must be between 1 and 365 (for common years) or 1 and 366 (for leap years).
    InvalidOrdinal,
    /// Indicates that a given hour, minute, or second does not form a valid time
    /// (e.g., hour 24, minute 60).
    InvalidTime,
}

/// Specific kinds of parsing errors encountered by `ParsiDate::parse`.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ParseErrorKind {
    /// The input string's structure or literal characters do not match the expected format string.
    FormatMismatch,
    /// A numeric component (year using `%Y`, month using `%m`, day using `%d`)
    /// contained non-digit characters or could not be parsed into a number.
    /// Note: `%m` and `%d` require exactly two digits.
    InvalidNumber,
    /// The numeric components were successfully parsed from the string according to the format,
    /// but they form an invalid date logically (e.g., "1403/13/01" parsed with "%Y/%m/%d").
    InvalidDateValue,
    /// An unrecognized or unsupported format specifier was used in the format string
    /// (e.g., `%x`). Supported specifiers are `%Y`, `%m`, `%d`, `%B`, `%%`.
    UnsupportedSpecifier,
    /// A Persian month name expected by `%B` could not be recognized in the input string
    /// at the current position.
    InvalidMonthName,
    /// Reserved for future use if weekday parsing (`%A`) is implemented. Currently unused.
    InvalidWeekdayName,
    /// A time component (hour using `%H`, minute using `%M`, second using `%S`)
    /// was parsed successfully but forms an invalid time logically (e.g., "25:00:00").
    InvalidTimeValue,
}

// --- Error Implementation ---

impl fmt::Display for DateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DateError::InvalidDate => write!(
                f,
                "Invalid Persian date components (year [1-9999], month [1-12], or day [1-29.30.31])"
            ),
            DateError::GregorianConversionError => write!(
                f,
                "Error during Gregorian conversion (date out of range or calculation issue)"
            ),
            DateError::ParseError(kind) => write!(f, "Date parsing error: {}", kind),
            DateError::ArithmeticOverflow => write!(
                f,
                "Date arithmetic resulted in overflow or date outside supported range [1-9999]"
            ),
            DateError::InvalidOrdinal => write!(
                f,
                "Invalid ordinal day number (must be 1-365 or 1-366 based on leap year)"
            ),
            DateError::InvalidTime => write!(f, "Invalid time components (hour [0-23], minute [0-59], or second [0-59])"),
        }
    }
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErrorKind::FormatMismatch => {
                write!(f, "Input string does not match the format string structure")
            }
            ParseErrorKind::InvalidNumber => write!(
                f,
                "Could not parse numeric component (year, month, or day) or incorrect digit count"
            ),
            ParseErrorKind::InvalidDateValue => {
                write!(f, "Parsed year, month, and day form an invalid date")
            }
            ParseErrorKind::UnsupportedSpecifier => {
                write!(f, "Unsupported format specifier used in format string")
            }
            ParseErrorKind::InvalidMonthName => {
                write!(f, "Could not parse or recognize Persian month name for %B")
            }
            ParseErrorKind::InvalidWeekdayName => write!(
                f,
                "Could not parse or recognize Persian weekday name (currently unused)"
            ),
            ParseErrorKind::InvalidTimeValue => write!(f, "Parsed hour, minute, and second form an invalid time"),
        }
    }
}

// Allow DateError to be used as a standard Rust error
impl std::error::Error for DateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Currently, DateError doesn't wrap other errors directly.
        // If it were to wrap (e.g., a chrono error), it could be returned here.
        None
    }
}
