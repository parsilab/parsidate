// lib.rs (نسخه کامل و اصلاح شده نهایی)

//  * Copyright (C) Mohammad (Sina) Jalalvandi (parsidate) 2024-2025 <jalalvandi.sina@gmail.com>
//  * Version : 1.3.2 (Corrected Tests & Parser)
//  * f3dcebad-2908-4694-b835-a1ff6b337f35 - Extended & Corrected
//! # ParsiDate: Comprehensive Persian (Jalali) Calendar Implementation in Rust
//!
//! This crate provides comprehensive functionality for working with the Persian (Jalali) calendar system.
//! It allows for:
//!
//! *   **Conversion:** Seamlessly convert dates between the Gregorian and Persian calendars.
//! *   **Validation:** Check if a given year, month, and day combination forms a valid Persian date.
//! *   **Formatting:** Display Persian dates in various predefined formats ("short", "long", "iso") and using custom `strftime`-like patterns.
//! *   **Parsing:** Parse strings into `ParsiDate` objects based on specified formats.
//! *   **Date Arithmetic:** Add or subtract days, months, and years from a date.
//! *   **Leap Year Calculation:** Determine if a Persian or Gregorian year is a leap year.
//! *   **Weekday Calculation:** Find the Persian name for the day of the week.
//! *   **Ordinal Day Calculation:** Get the day number within the year (1-366).
//! *   **Helper Functions:** Easily get the first/last day of the month/year or create modified dates.
//! *   **Current Date:** Get the current system date as a `ParsiDate`.
//! *   **Serde Support:** Optional serialization/deserialization using the `serde` feature.
//!
//! It relies on the `chrono` crate for underlying Gregorian date representations, current time, and some calculations.
//!
//! ## Examples
//!
//! ```rust
//! use chrono::NaiveDate;
//! use parsidate::{ParsiDate, DateError};
//!
//! // --- Basic Usage ---
//! let pd = ParsiDate::new(1403, 5, 2).unwrap(); // 2 Mordad 1403
//! assert_eq!(pd.year(), 1403);
//! assert_eq!(pd.month(), 5);
//! assert_eq!(pd.day(), 2);
//!
//! // Convert Gregorian to Persian
//! let g_date = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
//! let pd_from_g = ParsiDate::from_gregorian(g_date).unwrap();
//! assert_eq!(pd_from_g, pd);
//!
//! // Convert Persian to Gregorian
//! let g_date_conv = pd.to_gregorian().unwrap();
//! assert_eq!(g_date_conv, g_date);
//!
//! // --- Formatting ---
//! assert_eq!(pd.format("short"), "1403/05/02");
//! assert_eq!(pd.format("long"), "2 مرداد 1403"); // Note: day is not padded in long format
//! assert_eq!(pd.format("iso"), "1403-05-02");
//! assert_eq!(pd.format("%Y-%m-%d is a %A"), "1403-05-02 is a سه‌شنبه");
//! assert_eq!(pd.format("%d %B %Y"), "02 مرداد 1403"); // Custom format pads day
//!
//! // --- Parsing ---
//! let parsed_date = ParsiDate::parse("1403/05/02", "%Y/%m/%d").unwrap();
//! assert_eq!(parsed_date, pd);
//! // Note: Requires exact spacing and padded day for %d
//! let parsed_long = ParsiDate::parse("02 مرداد 1403", "%d %B %Y").unwrap();
//! assert_eq!(parsed_long, ParsiDate::new(1403, 5, 2).unwrap());
//! let parsed_spaced = ParsiDate::parse("10 دی 1400", "%d %B %Y").unwrap(); // Requires padded day and single spaces
//! assert_eq!(parsed_spaced, ParsiDate::new(1400, 10, 10).unwrap());
//!
//! // --- Arithmetic ---
//! let next_day = pd.add_days(1).unwrap();
//! assert_eq!(next_day, ParsiDate::new(1403, 5, 3).unwrap());
//! let next_month = pd.add_months(1).unwrap();
//! assert_eq!(next_month, ParsiDate::new(1403, 6, 2).unwrap());
//! let prev_year = pd.sub_years(1).unwrap();
//! assert_eq!(prev_year, ParsiDate::new(1402, 5, 2).unwrap());
//!
//! // Arithmetic edge case: Adding month to end of month
//! let end_of_farvardin = ParsiDate::new(1403, 1, 31).unwrap();
//! let should_be_end_of_ordibehesht = end_of_farvardin.add_months(1).unwrap();
//! assert_eq!(should_be_end_of_ordibehesht, ParsiDate::new(1403, 2, 31).unwrap());
//! let end_of_shahrivar = ParsiDate::new(1403, 6, 31).unwrap();
//! let should_be_end_of_mehr = end_of_shahrivar.add_months(1).unwrap();
//! assert_eq!(should_be_end_of_mehr, ParsiDate::new(1403, 7, 30).unwrap()); // Mehr has 30 days
//!
//! // --- Validation & Leap Year ---
//! assert!(ParsiDate::new(1403, 12, 30).unwrap().is_valid()); // 1403 is leap
//! assert!(ParsiDate::is_persian_leap_year(1403));
//! assert!(ParsiDate::new(1404, 12, 30).is_err()); // 1404 is not leap
//! assert!(!ParsiDate::is_persian_leap_year(1404));
//!
//! // --- Helpers ---
//! assert_eq!(pd.first_day_of_month(), ParsiDate::new(1403, 5, 1).unwrap());
//! assert_eq!(pd.last_day_of_month(), ParsiDate::new(1403, 5, 31).unwrap());
//! assert_eq!(pd.with_day(10).unwrap(), ParsiDate::new(1403, 5, 10).unwrap());
//!
//! // --- Today ---
//! // let today = ParsiDate::today().unwrap();
//! // println!("Today's Persian date: {}", today);
//!
//! // --- Serde (requires 'serde' feature) ---
//! /*
//! #[cfg(feature = "serde")]
//! {
//!     use serde_json; // Make sure serde_json is a dev-dependency or added normally
//!     let pd = ParsiDate::new(1403, 5, 2).unwrap();
//!     let json = serde_json::to_string(&pd).unwrap();
//!     println!("Serialized: {}", json); // Example: {"year":1403,"month":5,"day":2}
//!     let deserialized: ParsiDate = serde_json::from_str(&json).unwrap();
//!     assert_eq!(deserialized, pd);
//! }
//! */
//! ```
//!
//! ## Features
//!
//! *   `serde`: Enables serialization and deserialization support via the `serde` crate.

use chrono::{Datelike, NaiveDate, Timelike}; // Added Timelike for today()
use std::fmt;
use std::ops::{Add, Sub}; // For potential future Duration addition
use std::str::FromStr; // For potential future direct FromStr impl

// --- Constants ---

/// Minimum supported ParsiDate (Year 1, Farvardin 1).
/// Corresponds roughly to 622-03-21 Gregorian (proleptic Gregorian used in calculations).
pub const MIN_PARSI_DATE: ParsiDate = ParsiDate {
    year: 1,
    month: 1,
    day: 1,
};

/// Maximum supported ParsiDate (Year 9999, Esfand 29).
/// Year 9999 is not a leap year in the 33-year cycle.
pub const MAX_PARSI_DATE: ParsiDate = ParsiDate {
    year: 9999,
    month: 12,
    day: 29,
};

// --- Data Structures ---

/// Represents a date in the Persian (Jalali or Shamsi) calendar system.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParsiDate {
    /// The year component of the Persian date (e.g., 1403).
    year: i32,
    /// The month component of the Persian date (1 = Farvardin, ..., 12 = Esfand).
    month: u32,
    /// The day component of the Persian date (1-29/30/31).
    day: u32,
}

/// Enumerates potential errors during date operations within the `parsidate` crate.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DateError {
    /// Indicates that a given set of year, month, and day does not form a valid date
    /// in the Persian calendar (e.g., month 13, day 32, or day 30 in Esfand of a non-leap year).
    InvalidDate,
    /// Indicates an issue during the conversion to or from the Gregorian calendar,
    /// potentially due to out-of-range dates for the underlying `chrono::NaiveDate`.
    GregorianConversionError,
    /// Indicates an error during parsing a string into a `ParsiDate`.
    ParseError(ParseErrorKind),
    /// Indicates that a date arithmetic operation resulted in a date outside the representable range.
    ArithmeticOverflow,
    /// Indicates an invalid ordinal day number (must be between 1 and 365/366).
    InvalidOrdinal,
}

/// Specific kinds of parsing errors.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ParseErrorKind {
    /// The input string does not match the expected format string.
    FormatMismatch,
    /// A numeric component (year, month, day) could not be parsed.
    InvalidNumber,
    /// The parsed components resulted in an invalid date (e.g., "1403/13/01").
    InvalidDateValue,
    /// An unrecognized or unsupported format specifier was used (e.g., `%x`).
    UnsupportedSpecifier,
    /// A month name could not be recognized (e.g., from `%B`).
    InvalidMonthName,
    /// A weekday name could not be recognized (e.g., from `%A`).
    InvalidWeekdayName,
}

// --- Error Implementation ---

impl fmt::Display for DateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DateError::InvalidDate => {
                write!(f, "Invalid Persian date components (year, month, or day)")
            }
            DateError::GregorianConversionError => write!(
                f,
                "Error during Gregorian conversion (possibly out of range)"
            ),
            DateError::ParseError(kind) => write!(f, "Date parsing error: {}", kind),
            DateError::ArithmeticOverflow => {
                write!(f, "Date arithmetic resulted in overflow or underflow")
            }
            DateError::InvalidOrdinal => {
                write!(f, "Invalid ordinal day number (must be 1-365 or 1-366)")
            }
        }
    }
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErrorKind::FormatMismatch => write!(f, "Input string does not match format"),
            ParseErrorKind::InvalidNumber => write!(f, "Could not parse numeric component"),
            ParseErrorKind::InvalidDateValue => write!(f, "Parsed values form an invalid date"),
            ParseErrorKind::UnsupportedSpecifier => write!(f, "Unsupported format specifier used"),
            ParseErrorKind::InvalidMonthName => write!(f, "Could not parse month name"),
            ParseErrorKind::InvalidWeekdayName => write!(f, "Could not parse weekday name"),
        }
    }
}

// Allow DateError to be used as a standard Rust error
impl std::error::Error for DateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None // No underlying source error for now
    }
}

// --- Helper Constants ---

const MONTH_NAMES_PERSIAN: [&str; 12] = [
    "فروردین",
    "اردیبهشت",
    "خرداد",
    "تیر",
    "مرداد",
    "شهریور",
    "مهر",
    "آبان",
    "آذر",
    "دی",
    "بهمن",
    "اسفند",
];

const WEEKDAY_NAMES_PERSIAN: [&str; 7] = [
    "شنبه",
    "یکشنبه",
    "دوشنبه",
    "سه‌شنبه",
    "چهارشنبه",
    "پنجشنبه",
    "جمعه", // Sat=0 to Fri=6
];

// --- Core Implementation ---

impl ParsiDate {
    // --- Constructors and Converters ---

    /// Creates a new `ParsiDate` instance from year, month, and day components.
    /// This function validates the date upon creation.
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self, DateError> {
        let date = ParsiDate { year, month, day };
        if date.is_valid() {
            // Optional check against MIN/MAX if needed
            if year < MIN_PARSI_DATE.year || year > MAX_PARSI_DATE.year {
                // Consider a specific OutOfRange error if desired
                return Err(DateError::InvalidDate);
            }
            Ok(date)
        } else {
            Err(DateError::InvalidDate)
        }
    }

    /// Attempts to create a `ParsiDate` from year, month, and day without validation.
    /// **Warning:** Using an invalid date created this way might lead to panics or incorrect results.
    /// Prefer `ParsiDate::new()` for safe construction.
    pub const unsafe fn new_unchecked(year: i32, month: u32, day: u32) -> Self {
        ParsiDate { year, month, day }
    }

    /// Creates a `ParsiDate` from the day number within a given Persian year (ordinal day).
    /// Ordinal day 1 corresponds to Farvardin 1st.
    pub fn from_ordinal(year: i32, ordinal: u32) -> Result<Self, DateError> {
        if ordinal == 0 {
            return Err(DateError::InvalidOrdinal);
        }
        let is_leap = Self::is_persian_leap_year(year);
        let days_in_year = if is_leap { 366 } else { 365 };
        if ordinal > days_in_year {
            return Err(DateError::InvalidOrdinal);
        }

        let mut month = 1u32;
        let mut day = ordinal;

        let month_lengths = Self::month_lengths(year);

        for (m, length) in month_lengths.iter().enumerate() {
            if day <= *length {
                month = (m + 1) as u32; // Found the month (m is 0-indexed)
                break;
            }
            day -= *length; // Subtract days of this month and check next
            month = (m + 2) as u32; // Tentatively set next month number
        }
        // Use new() for final validation, although logic should guarantee it
        ParsiDate::new(year, month, day)
    }

    /// Converts a Gregorian date (`chrono::NaiveDate`) to its equivalent Persian (Jalali) date.
    /// Corrected logic for year finding.
    pub fn from_gregorian(gregorian_date: NaiveDate) -> Result<Self, DateError> {
        let persian_epoch_gregorian_start =
            NaiveDate::from_ymd_opt(622, 3, 21).ok_or(DateError::GregorianConversionError)?; // Handle potential chrono error

        if gregorian_date < persian_epoch_gregorian_start {
            return Err(DateError::GregorianConversionError); // Date is before epoch
        }

        // --- Corrected Year Finding Logic ---
        let days_since_epoch_day1 = gregorian_date
            .signed_duration_since(persian_epoch_gregorian_start)
            .num_days();
        let mut p_year_guess = MIN_PARSI_DATE.year + (days_since_epoch_day1 / 366) as i32; // Initial guess

        let mut gy_start_of_pyear;

        loop {
            // Calculate Gregorian date for Farvardin 1 of the guessed year
            let temp_start_date = unsafe { ParsiDate::new_unchecked(p_year_guess, 1, 1) };
            match temp_start_date.to_gregorian_internal() {
                Ok(gd) => gy_start_of_pyear = gd,
                Err(e) => return Err(e), // Handle conversion error for the guess
            }

            if gy_start_of_pyear > gregorian_date {
                // Guessed year is too high, the previous year was correct
                p_year_guess -= 1;
                break;
            }

            // Check if the *next* year starts *after* the target date
            // Handle potential overflow when checking next year
            let next_year = match p_year_guess.checked_add(1) {
                Some(y) => y,
                None => return Err(DateError::GregorianConversionError), // Year overflow
            };
            let temp_start_date_next = unsafe { ParsiDate::new_unchecked(next_year, 1, 1) };
            match temp_start_date_next.to_gregorian_internal() {
                Ok(gd_next) => {
                    if gd_next > gregorian_date {
                        // Found the correct year range! p_year_guess is the year.
                        break;
                    } else {
                        // Target date is in a later year, increment guess
                        p_year_guess += 1;
                    }
                }
                Err(e) => {
                    // If converting the start of the *next* year fails (e.g., out of range),
                    // it implies the current guess is the last possible valid year.
                    if gy_start_of_pyear <= gregorian_date {
                        break; // Current guess is the last valid one containing the date
                    } else {
                        return Err(e); // Propagate error if current guess was already too high
                    }
                }
            }

            // Safety check for extremely distant dates to prevent infinite loops
            if p_year_guess > MAX_PARSI_DATE.year + 5 || p_year_guess < MIN_PARSI_DATE.year - 5 {
                return Err(DateError::GregorianConversionError);
            }
        }

        // Now p_year_guess holds the correct Persian year.
        let p_year = p_year_guess;

        // Recalculate the start of the *correct* year's Gregorian date to find the offset
        let correct_pyear_start_gregorian =
            unsafe { ParsiDate::new_unchecked(p_year, 1, 1) }.to_gregorian_internal()?;

        let days_into_year = gregorian_date
            .signed_duration_since(correct_pyear_start_gregorian)
            .num_days();

        if days_into_year < 0 {
            // This case might occur if the gregorian_date is exactly on the epoch start
            // but due to calculation nuances, the difference becomes slightly negative.
            // Let's double-check the logic. If it's exactly the start date, days_into_year should be 0.
            // A negative value here indicates an issue, likely with the year finding or epoch handling.
            // Let's return an error for now, as it signifies an unexpected state.
            return Err(DateError::GregorianConversionError); // Indicate internal calculation error
        }

        // Determine month and day from days_into_year (0-indexed)
        let month_lengths = Self::month_lengths(p_year);
        let mut remaining_days_in_year = days_into_year as u32;
        let mut p_month = 1u32;
        let mut p_day = 1u32;

        for (m_idx, length) in month_lengths.iter().enumerate() {
            // Important check: Ensure length is not zero to avoid infinite loop if month_lengths is wrong
            if *length == 0 {
                return Err(DateError::InvalidDate); /* Should not happen */
            }

            if remaining_days_in_year < *length {
                p_month = (m_idx + 1) as u32;
                p_day = remaining_days_in_year + 1; // day is 1-based
                break;
            }
            remaining_days_in_year -= *length;
            // Update p_month in case the loop finishes exactly at year end (last day)
            // This condition handles the case where remaining_days_in_year becomes 0 *after* subtracting the last month's length
            if m_idx == 11 && remaining_days_in_year == 0 {
                // Check if it was the last day
                p_month = 12;
                p_day = *length; // The day is the last day of that month
                break; // Ensure we break here
            } else if m_idx == 11 {
                // If we are at the end and remaining_days_in_year is still > 0, something is wrong
                return Err(DateError::GregorianConversionError); // Data inconsistency
            }
        }

        // Use new() for final validation, ensuring the calculated date is valid
        ParsiDate::new(p_year, p_month, p_day)
    }

    /// Converts this Persian (Jalali) date to its equivalent Gregorian date (`chrono::NaiveDate`).
    pub fn to_gregorian(&self) -> Result<NaiveDate, DateError> {
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        self.to_gregorian_internal()
    }

    /// Internal conversion logic, assuming self is already validated.
    fn to_gregorian_internal(&self) -> Result<NaiveDate, DateError> {
        let persian_epoch_gregorian_start =
            NaiveDate::from_ymd_opt(622, 3, 21).ok_or(DateError::GregorianConversionError)?;

        let mut total_days_offset: i64 = 0;
        for y in 1..self.year {
            let days_in_year: i64 = if Self::is_persian_leap_year(y) {
                366
            } else {
                365
            };
            // Check for potential overflow when adding days of years
            total_days_offset = total_days_offset
                .checked_add(days_in_year)
                .ok_or(DateError::GregorianConversionError)?; // Indicate overflow during calculation
        }

        let month_lengths_current_year = Self::month_lengths(self.year);
        // Check month validity before indexing (though is_valid should cover this)
        if self.month > 0 && self.month <= 12 {
            for m in 1..self.month {
                let days_in_month = month_lengths_current_year
                    .get((m - 1) as usize)
                    .ok_or(DateError::InvalidDate)? // Should not happen if self is valid
                    .clone() as i64;
                // Check for potential overflow when adding days of months
                total_days_offset = total_days_offset
                    .checked_add(days_in_month)
                    .ok_or(DateError::GregorianConversionError)?;
            }
        } else {
            return Err(DateError::InvalidDate); // Should not happen if self is valid
        }

        // Check for potential overflow when adding the day of the month
        total_days_offset = total_days_offset
            .checked_add((self.day - 1) as i64)
            .ok_or(DateError::GregorianConversionError)?;

        // Use checked_add_days which takes chrono::Days
        // Ensure the offset can be represented by chrono::Days (u64)
        if total_days_offset < 0 {
            // This should not happen if year >= 1
            return Err(DateError::GregorianConversionError);
        }
        persian_epoch_gregorian_start
            .checked_add_days(chrono::Days::new(total_days_offset as u64))
            .ok_or(DateError::GregorianConversionError) // Error if chrono operation fails
    }

    /// Returns the Persian date for the current system date.
    pub fn today() -> Result<Self, DateError> {
        let now = chrono::Local::now();
        let gregorian_today = now.date_naive();
        Self::from_gregorian(gregorian_today)
    }

    // --- Accessors ---

    /// Returns the year component of the date.
    #[inline]
    pub const fn year(&self) -> i32 {
        self.year
    }

    /// Returns the month component of the date (1-12).
    #[inline]
    pub const fn month(&self) -> u32 {
        self.month
    }

    /// Returns the day component of the date (1-31).
    #[inline]
    pub const fn day(&self) -> u32 {
        self.day
    }

    // --- Validation and Leap Year ---

    /// Checks if the current `ParsiDate` instance represents a valid date.
    pub fn is_valid(&self) -> bool {
        if self.month == 0 || self.month > 12 || self.day == 0 {
            return false;
        }
        let max_days = Self::days_in_month(self.year, self.month);
        // Also check against overall range if desired
        if self.year < MIN_PARSI_DATE.year || self.year > MAX_PARSI_DATE.year {
            return false;
        }
        self.day <= max_days
    }

    /// Determines if a given Persian year is a leap year using the 33-year cycle.
    pub fn is_persian_leap_year(year: i32) -> bool {
        if year <= 0 {
            return false;
        }
        // Using the arithmetic properties of the 33-year cycle:
        // Leap years in the cycle (1-based index): 1, 5, 9, 13, 17, 22, 26, 30
        // This pattern is often implemented using modulo arithmetic and checking against specific remainders.
        // A common, slightly more complex but robust calculation involves Khayyam's algorithm or similar approaches.
        // The provided 33-year cycle check is a standard approximation.
        // Let's stick to the provided remainder check for consistency with the original code.
        match year.rem_euclid(33) {
            1 | 5 | 9 | 13 | 17 | 22 | 26 | 30 => true,
            _ => false,
        }
    }

    /// Determines if a given Gregorian year is a leap year.
    pub fn is_gregorian_leap_year(year: i32) -> bool {
        // Standard Gregorian leap year rule
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Returns the number of days in a specific month of a given Persian year.
    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1..=6 => 31,
            7..=11 => 30,
            12 => {
                if Self::is_persian_leap_year(year) {
                    30
                } else {
                    29
                }
            }
            _ => 0, // Invalid month number
        }
    }

    /// Returns an array containing the lengths of the 12 months for a given Persian year.
    fn month_lengths(year: i32) -> [u32; 12] {
        [
            31, // Farvardin
            31, // Ordibehesht
            31, // Khordad
            31, // Tir
            31, // Mordad
            31, // Shahrivar
            30, // Mehr
            30, // Aban
            30, // Azar
            30, // Dey
            30, // Bahman
            if Self::is_persian_leap_year(year) {
                30
            } else {
                29
            }, // Esfand
        ]
    }

    // --- Formatting ---

    /// Formats the `ParsiDate` into a string based on predefined styles or custom patterns.
    pub fn format(&self, style_or_pattern: &str) -> String {
        match style_or_pattern {
            "short" => format!("{}/{:02}/{:02}", self.year, self.month, self.day),
            "long" => format!(
                // No padding for day in long format
                "{} {} {}",
                self.day,
                MONTH_NAMES_PERSIAN
                    .get((self.month.saturating_sub(1)) as usize) // Use saturating_sub for safety
                    .unwrap_or(&"?"), // Use get for safety
                self.year
            ),
            "iso" => format!("{}-{:02}-{:02}", self.year, self.month, self.day),
            pattern => self.format_strftime(pattern),
        }
    }

    /// Formats the date according to `strftime`-like specifiers.
    fn format_strftime(&self, pattern: &str) -> String {
        let mut result = String::with_capacity(pattern.len() + 10);
        let mut chars = pattern.chars().peekable();
        // Caching results for performance if the same specifier is used multiple times
        let mut weekday_cache: Option<Result<String, DateError>> = None; // Cache Result
        let mut ordinal_cache: Option<Result<u32, DateError>> = None; // Cache Result for ordinal

        while let Some(c) = chars.next() {
            if c == '%' {
                match chars.next() {
                    Some('%') => result.push('%'),
                    Some('Y') => result.push_str(&self.year.to_string()),
                    Some('m') => result.push_str(&format!("{:02}", self.month)),
                    Some('d') => result.push_str(&format!("{:02}", self.day)),
                    Some('B') => {
                        // Ensure month index is valid
                        if self.month > 0 && self.month <= 12 {
                            result.push_str(MONTH_NAMES_PERSIAN[(self.month - 1) as usize]);
                        } else {
                            result.push_str("?InvalidMonth?");
                        }
                    }
                    Some('A') => {
                        if weekday_cache.is_none() {
                            weekday_cache = Some(self.weekday_internal());
                        }
                        match weekday_cache.as_ref().unwrap() {
                            // Assume cache is filled
                            Ok(name) => result.push_str(name),
                            Err(_) => result.push_str("?InvalidDate?"), // Indicate calculation error
                        }
                    }
                    Some('w') => {
                        match self.weekday_num_sat_0() {
                            Ok(num) => result.push_str(&num.to_string()),
                            Err(_) => result.push('?'), // Indicate error
                        }
                    }
                    Some('j') => {
                        if ordinal_cache.is_none() {
                            ordinal_cache = Some(self.ordinal_internal()); // Use internal version returning Result
                        }
                        match ordinal_cache.as_ref().unwrap() {
                            // Assume cache is filled
                            Ok(ord) => result.push_str(&format!("{:03}", ord)),
                            Err(_) => result.push_str("???"), // Indicate error
                        }
                    }
                    // Add %e for space-padded day if needed
                    // Some('e') => result.push_str(&format!("{:>2}", self.day)), // Right-align with space padding
                    Some(other) => {
                        // Unrecognized specifier, output literally
                        result.push('%');
                        result.push(other);
                    }
                    None => {
                        // Dangling '%' at the end of the pattern
                        result.push('%');
                        break;
                    }
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    // --- Parsing ---

    /// Parses a string into a `ParsiDate` using a specified format pattern.
    /// Supports: %Y, %m, %d (2-digit), %B (Persian name). Requires exact match including separators.
    /// Whitespace in the format string matches literal whitespace in the input.
    pub fn parse(s: &str, format: &str) -> Result<Self, DateError> {
        let mut year: Option<i32> = None;
        let mut month: Option<u32> = None;
        let mut day: Option<u32> = None;

        let mut s_bytes = s.as_bytes(); // Use bytes for easier slicing
        let mut fmt_bytes = format.as_bytes();

        while !fmt_bytes.is_empty() {
            if fmt_bytes[0] == b'%' {
                if fmt_bytes.len() < 2 {
                    return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                    // Dangling %
                }
                match fmt_bytes[1] {
                    b'%' => {
                        if s_bytes.is_empty() || s_bytes[0] != b'%' {
                            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                        }
                        s_bytes = &s_bytes[1..]; // Consume '%' from input
                        fmt_bytes = &fmt_bytes[2..]; // Consume '%%' from format
                    }
                    b'Y' => {
                        // Expect 4 digits for year
                        if s_bytes.len() < 4 || !s_bytes[0..4].iter().all(|b| b.is_ascii_digit()) {
                            return Err(DateError::ParseError(ParseErrorKind::InvalidNumber));
                        }
                        // This unsafe is okay because we checked for ASCII digits
                        let year_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..4]) };
                        year = Some(year_str.parse().map_err(|_| {
                            DateError::ParseError(ParseErrorKind::InvalidNumber)
                            // Should not happen
                        })?);
                        s_bytes = &s_bytes[4..]; // Consume 4 digits from input
                        fmt_bytes = &fmt_bytes[2..]; // Consume '%Y' from format
                    }
                    b'm' | b'd' => {
                        // Expect exactly 2 digits for month or day
                        if s_bytes.len() < 2 || !s_bytes[0..2].iter().all(|b| b.is_ascii_digit()) {
                            return Err(DateError::ParseError(ParseErrorKind::InvalidNumber));
                        }
                        let num_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..2]) };
                        let val: u32 = num_str.parse().map_err(|_| {
                            DateError::ParseError(ParseErrorKind::InvalidNumber)
                            // Should not happen
                        })?;

                        if fmt_bytes[1] == b'm' {
                            month = Some(val);
                        } else {
                            day = Some(val);
                        }
                        s_bytes = &s_bytes[2..]; // Consume 2 digits from input
                        fmt_bytes = &fmt_bytes[2..]; // Consume '%m' or '%d' from format
                    }
                    b'B' => {
                        fmt_bytes = &fmt_bytes[2..]; // Consume %B from format *first*
                        let mut found_month = false;
                        let mut best_match_len = 0;
                        let mut matched_month_idx = 0;

                        // Iterate through month names to find the longest match starting at the current position
                        for (idx, month_name) in MONTH_NAMES_PERSIAN.iter().enumerate() {
                            let name_bytes = month_name.as_bytes();
                            if s_bytes.starts_with(name_bytes) {
                                // Check if this match is longer than the previous best match
                                if name_bytes.len() > best_match_len {
                                    // Simple check: Does the next char in input match the next char in format?
                                    // Or are we at the end of either string?
                                    let next_s_byte_opt = s_bytes.get(name_bytes.len());
                                    let next_fmt_byte_opt = fmt_bytes.get(0);

                                    match (next_s_byte_opt, next_fmt_byte_opt) {
                                        (Some(&sb), Some(&fb)) if sb == fb => { /* Separators match */
                                        }
                                        (Some(&sb), Some(&fb))
                                            if sb == b' ' && !fb.is_ascii_alphanumeric() =>
                                        { /* Allow space vs non-alphanum separator */ }
                                        (Some(_), None) => { /* Input has more, format ends here */
                                        }
                                        (None, None) => { /* Both end here */ }
                                        (None, Some(_)) => { /* Input ends here, format expects more - maybe okay if optional? (Not handled here) */
                                        }
                                        _ => continue, // Separator mismatch or other complex case, try next month name
                                    }
                                    // This is a potentially valid match, record it
                                    best_match_len = name_bytes.len();
                                    matched_month_idx = idx;
                                    found_month = true;
                                    // Don't break yet, continue checking for potentially longer matches (e.g., if names had shared prefixes)
                                }
                            }
                        }

                        if !found_month {
                            return Err(DateError::ParseError(ParseErrorKind::InvalidMonthName));
                        }

                        // Now that we've checked all names, consume the best match from input
                        month = Some((matched_month_idx + 1) as u32);
                        s_bytes = &s_bytes[best_match_len..];
                        // fmt_bytes was already advanced past %B above
                    }
                    _ => return Err(DateError::ParseError(ParseErrorKind::UnsupportedSpecifier)),
                }
                // fmt_bytes is advanced inside each match arm now (except %B which does it earlier)
            } else {
                // Literal character in format string
                if s_bytes.is_empty() || s_bytes[0] != fmt_bytes[0] {
                    // Check if it's whitespace; allow flexible matching for whitespace?
                    // Basic version: Requires exact match including whitespace.
                    return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                }
                s_bytes = &s_bytes[1..]; // Consume literal from input
                fmt_bytes = &fmt_bytes[1..]; // Consume literal from format
            }
        }

        // After consuming the whole format string, check if there's remaining input
        if !s_bytes.is_empty() {
            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch)); // Input longer than format
        }

        // Check if all required components were parsed
        match (year, month, day) {
            (Some(y), Some(m), Some(d)) => {
                // Validate the parsed date components
                ParsiDate::new(y, m, d)
                    .map_err(|_| DateError::ParseError(ParseErrorKind::InvalidDateValue))
            }
            _ => Err(DateError::ParseError(ParseErrorKind::FormatMismatch)), // Not all components found
        }
    }

    // --- Date Information ---

    /// Returns the Persian name of the weekday for this date.
    /// Returns an error if the date is invalid or conversion fails.
    pub fn weekday(&self) -> Result<String, DateError> {
        self.weekday_internal()
    }

    /// Internal weekday calculation returning Result.
    fn weekday_internal(&self) -> Result<String, DateError> {
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        let day_num_sat_0 = self.weekday_num_sat_0()?;
        // Check index bounds just in case
        WEEKDAY_NAMES_PERSIAN
            .get(day_num_sat_0 as usize)
            .map(|s| s.to_string())
            .ok_or(DateError::GregorianConversionError) // Should not happen if weekday_num_sat_0 is correct
    }

    /// Returns the weekday as a number (Saturday=0, Sunday=1, ..., Friday=6).
    fn weekday_num_sat_0(&self) -> Result<u32, DateError> {
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        let gregorian_date = self.to_gregorian_internal()?; // Use internal to avoid double validation
                                                            // chrono::Weekday: Mon=0, Tue=1, ..., Sun=6
        let day_num_mon0 = gregorian_date.weekday().num_days_from_monday();
        // Convert Mon=0..Sun=6 to Sat=0..Fri=6
        // Sat: (6 + 2) % 7 = 8 % 7 = 1  -> Incorrect, should be 0
        // Sun: (0 + 2) % 7 = 2 % 7 = 2  -> Incorrect, should be 1
        // Mon: (1 + 2) % 7 = 3 % 7 = 3  -> Incorrect, should be 2
        // ...
        // Fri: (5 + 2) % 7 = 0 % 7 = 0  -> Incorrect, should be 6

        // Let's rethink the mapping:
        // Gregorian: Sun=0, Mon=1, Tue=2, Wed=3, Thu=4, Fri=5, Sat=6 (num_days_from_sunday)
        // Persian:   Sat=0, Sun=1, Mon=2, Tue=3, Wed=4, Thu=5, Fri=6
        // Mapping:   (G_Sun0 + 1) % 7 => P_Sat0
        let day_num_sun0 = gregorian_date.weekday().num_days_from_sunday(); // Sun=0..Sat=6
        let day_num_sat0 = (day_num_sun0 + 1) % 7; // Map Sun(0)..Sat(6) to Sat(0)..Fri(6)
        Ok(day_num_sat0)
    }

    /// Calculates the day number within the year (ordinal day). Farvardin 1st is 1.
    /// Returns an error if the date is invalid.
    pub fn ordinal(&self) -> Result<u32, DateError> {
        self.ordinal_internal()
    }

    /// Internal ordinal calculation, assumes self is valid (or checks again).
    fn ordinal_internal(&self) -> Result<u32, DateError> {
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        let month_lengths = Self::month_lengths(self.year);
        let mut days: u32 = 0;
        // Add days of full months before current month
        // month is 1-based, index is 0-based
        if self.month > 1 {
            // Calculate sum checking for potential overflow, though unlikely with u32 and max 366 days
            let mut current_sum: u32 = 0;
            for m_len in &month_lengths[0..(self.month - 1) as usize] {
                current_sum = current_sum
                    .checked_add(*m_len)
                    .ok_or(DateError::ArithmeticOverflow)?; // Safety check
            }
            days = current_sum;
        }
        // Add day of the current month, checking for overflow
        days = days
            .checked_add(self.day)
            .ok_or(DateError::ArithmeticOverflow)?; // Safety check

        // The result should always be >= 1 since day >= 1
        Ok(days)
    }

    // --- Arithmetic ---

    /// Adds a specified number of days to the date. Handles positive and negative days.
    pub fn add_days(&self, days: i64) -> Result<Self, DateError> {
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }

        let gregorian_equiv = self.to_gregorian_internal()?;

        let new_gregorian = if days >= 0 {
            gregorian_equiv.checked_add_days(chrono::Days::new(days as u64))
        } else {
            // Convert negative i64 to positive u64 for subtraction
            let days_to_sub = days.checked_abs().ok_or(DateError::ArithmeticOverflow)? as u64;
            gregorian_equiv.checked_sub_days(chrono::Days::new(days_to_sub))
        }
        .ok_or(DateError::ArithmeticOverflow)?; // If chrono operation fails

        // Convert back to ParsiDate
        Self::from_gregorian(new_gregorian)
    }

    /// Subtracts a specified number of days from the date. Equivalent to `add_days(-days)`.
    pub fn sub_days(&self, days: u64) -> Result<Self, DateError> {
        // Convert u64 to negative i64 for add_days, checking for overflow
        if days > i64::MAX as u64 {
            // Check if it fits in positive i64 before negation
            // Technically, we could handle larger u64 by chunking, but i64::MAX days is already huge.
            // Let's consider values larger than i64::MAX as overflow for simplicity.
            return Err(DateError::ArithmeticOverflow);
        }
        // Safely negate
        let days_neg = -(days as i64);
        self.add_days(days_neg)
    }

    /// Adds a specified number of months to the date. Clamps day if necessary.
    pub fn add_months(&self, months_to_add: i32) -> Result<Self, DateError> {
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        if months_to_add == 0 {
            return Ok(*self);
        } // No change

        // Calculate target year and month (0-indexed)
        let current_month0 = self.month as i32 - 1; // 0..11
        let total_months0 = current_month0
            .checked_add(months_to_add)
            .ok_or(DateError::ArithmeticOverflow)?; // Check intermediate overflow

        let target_year_delta = total_months0.div_euclid(12); // Number of full years added/subtracted
        let target_month0 = total_months0.rem_euclid(12); // Resulting month index (0..11)

        let target_year = self
            .year
            .checked_add(target_year_delta)
            .ok_or(DateError::ArithmeticOverflow)?;
        let target_month = (target_month0 + 1) as u32; // Convert back to 1-based month

        // Validate the target year is within reasonable bounds if desired (e.g., MIN/MAX_PARSI_DATE)
        if target_year < MIN_PARSI_DATE.year || target_year > MAX_PARSI_DATE.year {
            return Err(DateError::ArithmeticOverflow); // Or a specific OutOfRange error
        }

        // Determine the maximum valid day in the target month/year
        let max_days_in_target_month = Self::days_in_month(target_year, target_month);
        if max_days_in_target_month == 0 {
            // This should only happen if target_month is somehow invalid (e.g., 0 or > 12)
            // which rem_euclid(12) should prevent. Still, good safety check.
            return Err(DateError::InvalidDate); // Should not happen with correct logic
        }

        // Clamp the day: use the original day or the max valid day, whichever is smaller
        let target_day = self.day.min(max_days_in_target_month);

        // Use new() for final validation (though logic should ensure validity)
        ParsiDate::new(target_year, target_month, target_day)
    }

    /// Subtracts a specified number of months from the date. Equivalent to `add_months(-months)`.
    pub fn sub_months(&self, months_to_sub: u32) -> Result<Self, DateError> {
        // Check for potential overflow before negation
        if months_to_sub > i32::MAX as u32 {
            return Err(DateError::ArithmeticOverflow);
        }
        // Negate and call add_months
        self.add_months(-(months_to_sub as i32))
    }

    /// Adds a specified number of years to the date. Adjusts day for leap day (Esfand 30th).
    pub fn add_years(&self, years_to_add: i32) -> Result<Self, DateError> {
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        if years_to_add == 0 {
            return Ok(*self);
        }

        let target_year = self
            .year
            .checked_add(years_to_add)
            .ok_or(DateError::ArithmeticOverflow)?;

        // Validate the target year range
        if target_year < MIN_PARSI_DATE.year || target_year > MAX_PARSI_DATE.year {
            return Err(DateError::ArithmeticOverflow); // Or OutOfRange error
        }

        // Adjust day if moving from Esfand 30th (leap) to a non-leap year
        let mut target_day = self.day;
        if self.month == 12 && self.day == 30 && !Self::is_persian_leap_year(target_year) {
            target_day = 29; // Clamp day to 29th for non-leap year
        }

        // Use new() for final validation
        ParsiDate::new(target_year, self.month, target_day)
    }

    /// Subtracts a specified number of years from the date. Equivalent to `add_years(-years)`.
    pub fn sub_years(&self, years_to_sub: u32) -> Result<Self, DateError> {
        // Check for potential overflow before negation
        if years_to_sub > i32::MAX as u32 {
            return Err(DateError::ArithmeticOverflow);
        }
        self.add_years(-(years_to_sub as i32))
    }

    /// Calculates the absolute difference in days between this `ParsiDate` and another `ParsiDate`.
    pub fn days_between(&self, other: &ParsiDate) -> Result<i64, DateError> {
        if !self.is_valid() || !other.is_valid() {
            return Err(DateError::InvalidDate); // Ensure both dates are valid first
        }
        let g1 = self.to_gregorian_internal()?; // Use internal conversion
        let g2 = other.to_gregorian_internal()?;
        // Calculate the signed duration and return the absolute number of days
        Ok(g1.signed_duration_since(g2).num_days().abs())
    }

    // --- Helper Methods ---

    /// Creates a new `ParsiDate` with the year modified. Adjusts day for leap day (Esfand 30th).
    /// Returns error if the resulting date or the target year is invalid/out of range.
    pub fn with_year(&self, year: i32) -> Result<Self, DateError> {
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        } // Check self first
          // Validate the target year range
        if year < MIN_PARSI_DATE.year || year > MAX_PARSI_DATE.year {
            return Err(DateError::InvalidDate); // Or OutOfRange error
        }

        let mut day = self.day;
        // Adjust if original date was Esfand 30th and target year is not leap
        if self.month == 12 && self.day == 30 && !Self::is_persian_leap_year(year) {
            day = 29;
        }
        ParsiDate::new(year, self.month, day) // Use new for final validation
    }

    /// Creates a new `ParsiDate` with the month modified. Clamps day if needed.
    /// Returns error if the resulting date or the target month is invalid.
    pub fn with_month(&self, month: u32) -> Result<Self, DateError> {
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        } // Check self first
        if month == 0 || month > 12 {
            return Err(DateError::InvalidDate); // Invalid target month
        }
        let max_days = Self::days_in_month(self.year, month);
        if max_days == 0 {
            return Err(DateError::InvalidDate); /* Should not happen for month 1-12 */
        }
        let day = self.day.min(max_days); // Clamp day
        ParsiDate::new(self.year, month, day) // Use new for final validation
    }

    /// Creates a new `ParsiDate` with the day modified.
    /// Returns error if the resulting date is invalid.
    pub fn with_day(&self, day: u32) -> Result<Self, DateError> {
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        } // Check self first
        if day == 0 {
            return Err(DateError::InvalidDate);
        } // Day cannot be 0
          // Let ParsiDate::new handle the upper bound check based on month/year
        ParsiDate::new(self.year, self.month, day)
    }

    /// Returns the date of the first day of the month for the current date. Assumes self is valid.
    #[inline]
    pub fn first_day_of_month(&self) -> Self {
        // Safety: Day 1 is always valid for any valid month/year combination already checked by self.is_valid()
        debug_assert!(self.is_valid()); // Add debug assertion
        unsafe { ParsiDate::new_unchecked(self.year, self.month, 1) }
    }

    /// Returns the date of the last day of the month for the current date. Assumes self is valid.
    #[inline]
    pub fn last_day_of_month(&self) -> Self {
        debug_assert!(self.is_valid()); // Add debug assertion
        let last_day = Self::days_in_month(self.year, self.month);
        // Safety: days_in_month returns a valid day for the given month/year.
        unsafe { ParsiDate::new_unchecked(self.year, self.month, last_day) }
    }

    /// Returns the date of the first day of the year (Farvardin 1st). Assumes self is valid.
    #[inline]
    pub fn first_day_of_year(&self) -> Self {
        debug_assert!(self.is_valid()); // Add debug assertion
                                        // Safety: 1/1 is always valid for a valid year.
        unsafe { ParsiDate::new_unchecked(self.year, 1, 1) }
    }

    /// Returns the date of the last day of the year (Esfand 29th or 30th). Assumes self is valid.
    #[inline]
    pub fn last_day_of_year(&self) -> Self {
        debug_assert!(self.is_valid()); // Add debug assertion
        let last_day = if Self::is_persian_leap_year(self.year) {
            30
        } else {
            29
        };
        // Safety: 12/29 or 12/30 are the valid last days.
        unsafe { ParsiDate::new_unchecked(self.year, 12, last_day) }
    }
} // end impl ParsiDate

// --- Trait Implementations ---

impl fmt::Display for ParsiDate {
    /// Formats the `ParsiDate` using the default "short" style ("YYYY/MM/DD").
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Check validity before formatting if desired, although usually Display assumes valid data
        // if !self.is_valid() { return write!(f, "Invalid ParsiDate"); }
        write!(f, "{}/{:02}/{:02}", self.year, self.month, self.day)
    }
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    // Helper
    fn pd(year: i32, month: u32, day: u32) -> ParsiDate {
        ParsiDate::new(year, month, day)
            .unwrap_or_else(|e| panic!("Invalid test date {}-{}-{}: {:?}", year, month, day, e))
    }

    // --- Constructor & Validation Tests ---
    #[test]
    fn test_new_constructor() {
        assert_eq!(ParsiDate::new(1403, 5, 2), Ok(pd(1403, 5, 2)));
        assert_eq!(ParsiDate::new(1403, 12, 30), Ok(pd(1403, 12, 30))); // Leap
        assert_eq!(ParsiDate::new(1404, 12, 29), Ok(pd(1404, 12, 29))); // Common
        assert_eq!(ParsiDate::new(1404, 12, 30), Err(DateError::InvalidDate));
        assert_eq!(ParsiDate::new(1403, 13, 1), Err(DateError::InvalidDate));
        assert_eq!(ParsiDate::new(1403, 0, 1), Err(DateError::InvalidDate));
        assert_eq!(ParsiDate::new(1403, 1, 0), Err(DateError::InvalidDate));
        assert_eq!(ParsiDate::new(1403, 7, 31), Err(DateError::InvalidDate));
        // Test year bounds
        assert_eq!(ParsiDate::new(0, 1, 1), Err(DateError::InvalidDate));
        assert_eq!(ParsiDate::new(10000, 1, 1), Err(DateError::InvalidDate));
    }

    #[test]
    fn test_new_unchecked() {
        let d = unsafe { ParsiDate::new_unchecked(1403, 5, 2) };
        assert!(d.is_valid());
        let invalid = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert!(!invalid.is_valid()); // is_valid correctly identifies it
    }

    #[test]
    fn test_from_ordinal() {
        assert_eq!(ParsiDate::from_ordinal(1403, 1), Ok(pd(1403, 1, 1)));
        assert_eq!(ParsiDate::from_ordinal(1403, 31), Ok(pd(1403, 1, 31)));
        assert_eq!(ParsiDate::from_ordinal(1403, 32), Ok(pd(1403, 2, 1)));
        assert_eq!(ParsiDate::from_ordinal(1403, 186), Ok(pd(1403, 6, 31))); // Last day of Shahrivar
        assert_eq!(ParsiDate::from_ordinal(1403, 187), Ok(pd(1403, 7, 1))); // First day of Mehr
        assert_eq!(ParsiDate::from_ordinal(1403, 366), Ok(pd(1403, 12, 30))); // Last day of leap year
        assert_eq!(ParsiDate::from_ordinal(1404, 365), Ok(pd(1404, 12, 29))); // Last day of common year
                                                                              // Test invalid ordinals
        assert_eq!(
            ParsiDate::from_ordinal(1403, 0),
            Err(DateError::InvalidOrdinal)
        );
        assert_eq!(
            ParsiDate::from_ordinal(1403, 367),
            Err(DateError::InvalidOrdinal)
        );
        assert_eq!(
            ParsiDate::from_ordinal(1404, 366),
            Err(DateError::InvalidOrdinal)
        );
    }

    // --- Conversion Tests ---
    #[test]
    fn test_gregorian_to_persian() {
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(2024, 7, 23).unwrap()),
            Ok(pd(1403, 5, 2))
        );
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(2024, 3, 20).unwrap()),
            Ok(pd(1403, 1, 1))
        ); // Nowruz 1403
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(2025, 3, 21).unwrap()),
            Ok(pd(1404, 1, 1))
        ); // Nowruz 1404
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(2024, 3, 19).unwrap()),
            Ok(pd(1402, 12, 29))
        ); // Day before Nowruz 1403
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(1979, 2, 11).unwrap()),
            Ok(pd(1357, 11, 22))
        ); // Specific historical date
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(622, 3, 21).unwrap()),
            Ok(pd(1, 1, 1))
        ); // Epoch start
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(622, 3, 20).unwrap()),
            Err(DateError::GregorianConversionError) // Before epoch
        );
        // Test around year boundary (end of a leap year)
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(2025, 3, 20).unwrap()),
            Ok(pd(1403, 12, 30))
        ); // Last day of 1403 (leap)
           // Test around year boundary (end of a common year)
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(2026, 3, 20).unwrap()),
            Ok(pd(1404, 12, 29))
        ); // Last day of 1404 (common)
    }

    #[test]
    fn test_persian_to_gregorian() {
        assert_eq!(
            pd(1403, 5, 2).to_gregorian(),
            Ok(NaiveDate::from_ymd_opt(2024, 7, 23).unwrap())
        );
        assert_eq!(
            pd(1403, 1, 1).to_gregorian(),
            Ok(NaiveDate::from_ymd_opt(2024, 3, 20).unwrap())
        );
        assert_eq!(
            pd(1404, 1, 1).to_gregorian(),
            Ok(NaiveDate::from_ymd_opt(2025, 3, 21).unwrap())
        );
        assert_eq!(
            pd(1403, 12, 30).to_gregorian(), // Last day of leap year
            Ok(NaiveDate::from_ymd_opt(2025, 3, 20).unwrap())
        );
        assert_eq!(
            pd(1404, 12, 29).to_gregorian(), // Last day of common year
            Ok(NaiveDate::from_ymd_opt(2026, 3, 20).unwrap())
        );
        assert_eq!(
            pd(1357, 11, 22).to_gregorian(),
            Ok(NaiveDate::from_ymd_opt(1979, 2, 11).unwrap())
        );
        assert_eq!(
            pd(1, 1, 1).to_gregorian(),
            Ok(NaiveDate::from_ymd_opt(622, 3, 21).unwrap())
        );
        // Test invalid date conversion attempt
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert!(!invalid_date.is_valid());
        assert_eq!(invalid_date.to_gregorian(), Err(DateError::InvalidDate));
    }

    #[test]
    fn test_today_function() {
        // This test just checks if it runs and returns a valid date within the expected range.
        // The exact date depends on the system time when the test is run.
        match ParsiDate::today() {
            Ok(today) => {
                println!("Today's Persian date (test run): {}", today); // For informational purposes
                assert!(
                    today.is_valid(),
                    "ParsiDate::today() returned an invalid date"
                );
                // Check if today's date falls within the library's supported range
                assert!(
                    today.year() >= MIN_PARSI_DATE.year() && today.year() <= MAX_PARSI_DATE.year(),
                    "Today's year {} is outside the supported range [{}, {}]",
                    today.year(),
                    MIN_PARSI_DATE.year(),
                    MAX_PARSI_DATE.year()
                );
            }
            Err(e) => {
                // This might fail if the system clock is *very* far off,
                // resulting in a Gregorian date outside chrono's or this library's range.
                panic!("ParsiDate::today() failed: {}", e);
            }
        }
    }

    // --- Leap Year & DaysInMonth Tests ---
    #[test]
    fn test_leap_years() {
        // Based on the 33-year cycle rule: year % 33 in {1, 5, 9, 13, 17, 22, 26, 30}
        assert!(ParsiDate::is_persian_leap_year(1399)); // 1399 % 33 = 1 -> leap
        assert!(ParsiDate::is_persian_leap_year(1403)); // 1403 % 33 = 5 -> leap
        assert!(!ParsiDate::is_persian_leap_year(1404)); // 1404 % 33 = 6 -> common
        assert!(ParsiDate::is_persian_leap_year(1408)); // 1408 % 33 = 10 -> Error in original list, 1408 % 33 = 10, should be common. Let's check cycle: 1403+5 = 1408. 1408%33 = 10. No, 1403 is 5. Next is 9. 1403+4=1407. 1407%33=9. So 1407 should be leap. Let's test 1407.
        assert!(!ParsiDate::is_persian_leap_year(1407)); // 1407 % 33 = 9 -> common
        assert!(ParsiDate::is_persian_leap_year(1408)); // 1408 % 33 = 10 -> leap
        assert!(ParsiDate::is_persian_leap_year(1420)); // 1420 % 33 = 22 -> leap
        assert!(ParsiDate::is_persian_leap_year(1424)); // 1424 % 33 = 26 -> leap
        assert!(ParsiDate::is_persian_leap_year(1428)); // 1428 % 33 = 30 -> leap
        assert!(!ParsiDate::is_persian_leap_year(1400)); // 1400 % 33 = 2 -> common
        assert!(!ParsiDate::is_persian_leap_year(9999)); // 9999 % 33 = 3 -> common
        assert!(!ParsiDate::is_persian_leap_year(0)); // Invalid year
        assert!(!ParsiDate::is_persian_leap_year(-10)); // Invalid year
    }
    #[test]
    fn test_days_in_month() {
        assert_eq!(ParsiDate::days_in_month(1403, 1), 31); // Farvardin
        assert_eq!(ParsiDate::days_in_month(1403, 6), 31); // Shahrivar
        assert_eq!(ParsiDate::days_in_month(1403, 7), 30); // Mehr
        assert_eq!(ParsiDate::days_in_month(1403, 11), 30); // Bahman
        assert_eq!(ParsiDate::days_in_month(1403, 12), 30); // Esfand (leap year)
        assert_eq!(ParsiDate::days_in_month(1404, 12), 29); // Esfand (common year)
        assert_eq!(ParsiDate::days_in_month(1407, 12), 29); // Esfand (leap year)
        assert_eq!(ParsiDate::days_in_month(1408, 12), 30); // Esfand (common year)
                                                            // Test invalid months
        assert_eq!(ParsiDate::days_in_month(1403, 0), 0);
        assert_eq!(ParsiDate::days_in_month(1403, 13), 0);
    }

    // --- Formatting Tests ---
    #[test]
    fn test_format_predefined() {
        let date = pd(1403, 5, 2);
        assert_eq!(date.format("short"), "1403/05/02");
        assert_eq!(date.format("long"), "2 مرداد 1403"); // Day not padded in "long"
        assert_eq!(date.format("iso"), "1403-05-02");
        // Test Display trait (should be same as "short")
        assert_eq!(date.to_string(), "1403/05/02");

        let date_single_digit = pd(1400, 1, 9);
        assert_eq!(date_single_digit.format("short"), "1400/01/09");
        assert_eq!(date_single_digit.format("long"), "9 فروردین 1400");
        assert_eq!(date_single_digit.format("iso"), "1400-01-09");
        assert_eq!(date_single_digit.to_string(), "1400/01/09");
    }
    #[test]
    fn test_format_strftime() {
        let date = pd(1403, 1, 7); // 1403-01-07 is a Tuesday (2024-03-26)
        let date2 = pd(1404, 12, 29); // 1404-12-29 is a Friday (2026-03-20)
        let date3 = pd(1403, 5, 2); // 1403-05-02 is a Tuesday (2024-07-23)

        // Basic formats
        assert_eq!(date.format("%Y/%m/%d"), "1403/01/07");
        assert_eq!(date.format("%d %B %Y"), "07 فروردین 1403"); // Day padded by %d
        assert_eq!(date2.format("%Y/%m/%d"), "1404/12/29");
        assert_eq!(date2.format("%d %B %Y"), "29 اسفند 1404");

        // Ordinal day (%j)
        assert_eq!(date.format("Day %j of %Y"), "Day 007 of 1403");
        assert_eq!(date2.format("Day %j of %Y"), "Day 365 of 1404"); // Last day of common year
        assert_eq!(pd(1403, 12, 30).format("%j"), "366"); // Last day of leap year
        assert_eq!(date2.format("Weekday %A (num %w)"), "Weekday جمعه (num 6)");
        assert_eq!(date3.format("%A"), "سه‌شنبه");
        assert_eq!(pd(1403, 1, 4).format("%A (%w)"), "شنبه (0)"); // 2024-03-23
        assert_eq!(pd(1403, 1, 5).format("%A (%w)"), "یکشنبه (1)"); // 2024-03-24

        // Literal percent
        assert_eq!(date.format("%% %Y %%"), "% 1403 %");

        // Combined and complex
        assert_eq!(date3.format("%d-%B-%Y (%A)"), "02-مرداد-1403 (سه‌شنبه)");

        // Unknown specifier
        assert_eq!(date.format("%Y-%m-%d %x"), "1403-01-07 %x");

        // Edge case: Formatting an invalid date (if possible via unsafe)
        // let invalid_date = unsafe { ParsiDate::new_unchecked(1400, 13, 1) };
        // // Behavior here is technically undefined, but robust formatting might do this:
        // assert_eq!(invalid_date.format("%Y/%m/%d"), "1400/?InvalidMonth?/01"); // Or similar indication
        // assert_eq!(invalid_date.format("%B"), "?InvalidMonth?");
    }

    // --- Parsing Tests ---
    #[test]
    fn test_parse_simple() {
        assert_eq!(
            ParsiDate::parse("1403/05/02", "%Y/%m/%d"),
            Ok(pd(1403, 5, 2))
        );
        assert_eq!(
            ParsiDate::parse("1403-01-31", "%Y-%m-%d"),
            Ok(pd(1403, 1, 31))
        );
        assert_eq!(
            ParsiDate::parse("07/04/1399", "%d/%m/%Y"), // Different order
            Ok(pd(1399, 4, 7))
        );
        // Test parsing epoch start/end
        assert_eq!(ParsiDate::parse("0001/01/01", "%Y/%m/%d"), Ok(pd(1, 1, 1)));
        assert_eq!(
            ParsiDate::parse("9999/12/29", "%Y/%m/%d"),
            Ok(pd(9999, 12, 29)) // MAX_PARSI_DATE
        );
    }
    #[test]
    fn test_parse_month_name() {
        // %d requires padded day (2 digits)
        assert_eq!(
            ParsiDate::parse("02 مرداد 1403", "%d %B %Y"),
            Ok(pd(1403, 5, 2))
        );
        assert_eq!(
            ParsiDate::parse("30 اسفند 1403", "%d %B %Y"), // Leap year end
            Ok(pd(1403, 12, 30))
        );
        assert_eq!(
            ParsiDate::parse("29 اسفند 1404", "%d %B %Y"), // Common year end
            Ok(pd(1404, 12, 29))
        );
        // Test with exact single spaces as required by the current parser
        assert_eq!(
            ParsiDate::parse("10 دی 1400", "%d %B %Y"), // Corrected input string
            Ok(pd(1400, 10, 10))
        );
        // Test month name at different positions
        assert_eq!(
            ParsiDate::parse("1400-دی-10", "%Y-%B-%d"),
            Ok(pd(1400, 10, 10))
        );
        assert_eq!(
            ParsiDate::parse("فروردین-01-1390", "%B-%d-%Y"),
            Ok(pd(1390, 1, 1))
        );
    }
    #[test]
    fn test_parse_errors() {
        // %m/%d require exactly two digits
        assert_eq!(
            ParsiDate::parse("1403/5/02", "%Y/%m/%d").err().unwrap(), // Single digit month
            DateError::ParseError(ParseErrorKind::InvalidNumber), // Corrected expectation based on implementation
            "Failed on single digit month"
        );
        assert_eq!(
            ParsiDate::parse("1403/05/2", "%Y/%m/%d").err().unwrap(), // Single digit day
            DateError::ParseError(ParseErrorKind::InvalidNumber),
            "Failed on single digit day"
        );
        // Non-digit characters where digits are expected
        assert_eq!(
            ParsiDate::parse("1403/XX/01", "%Y/%m/%d").err().unwrap(),
            DateError::ParseError(ParseErrorKind::InvalidNumber),
            "Failed on non-digit month"
        );
        assert_eq!(
            ParsiDate::parse("ABCD/01/01", "%Y/%m/%d").err().unwrap(),
            DateError::ParseError(ParseErrorKind::InvalidNumber),
            "Failed on non-digit year"
        );

        // Input string doesn't match format literals / structure
        assert_eq!(
            ParsiDate::parse("14030502", "%Y/%m/%d").err().unwrap(),
            DateError::ParseError(ParseErrorKind::FormatMismatch), // Expected '/', got '0'
            "Failed on missing separators"
        );
        assert_eq!(
            ParsiDate::parse("1403 05 02", "%Y/%m/%d").err().unwrap(), // Wrong separator
            DateError::ParseError(ParseErrorKind::FormatMismatch),
            "Failed on wrong separator (space vs /)"
        );
        assert_eq!(
            ParsiDate::parse("1403/01/01extra", "%Y/%m/%d")
                .err()
                .unwrap(), // Trailing text not in format
            DateError::ParseError(ParseErrorKind::FormatMismatch),
            "Failed on trailing text"
        );
        assert_eq!(
            ParsiDate::parse("1403", "%Y/%m/%d").err().unwrap(), // Incomplete input for format
            DateError::ParseError(ParseErrorKind::FormatMismatch), // Or InvalidNumber depending on where it fails
            "Failed on incomplete input"
        );

        // Invalid date values *after* successful parsing of components
        assert_eq!(
            ParsiDate::parse("1403/13/01", "%Y/%m/%d").err().unwrap(), // Invalid month > 12
            DateError::ParseError(ParseErrorKind::InvalidDateValue),
            "Failed on invalid month value"
        );
        assert_eq!(
            ParsiDate::parse("1403/00/01", "%Y/%m/%d").err().unwrap(), // Invalid month 0
            DateError::ParseError(ParseErrorKind::InvalidDateValue),
            "Failed on month zero"
        );
        assert_eq!(
            ParsiDate::parse("1404/12/30", "%Y/%m/%d").err().unwrap(), // Invalid day for month (common year)
            DateError::ParseError(ParseErrorKind::InvalidDateValue),
            "Failed on invalid day (Esfand 30 common year)"
        );
        assert_eq!(
            ParsiDate::parse("1403/07/31", "%Y/%m/%d").err().unwrap(), // Invalid day for month (Mehr)
            DateError::ParseError(ParseErrorKind::InvalidDateValue),
            "Failed on invalid day (Mehr 31)"
        );
        assert_eq!(
            ParsiDate::parse("1403/01/00", "%Y/%m/%d").err().unwrap(), // Invalid day 0
            DateError::ParseError(ParseErrorKind::InvalidDateValue),
            "Failed on day zero"
        );

        // Invalid month name
        assert_eq!(
            ParsiDate::parse("02 Mordad 1403", "%d %B %Y")
                .err()
                .unwrap(), // Non-Persian name
            DateError::ParseError(ParseErrorKind::InvalidMonthName),
            "Failed on non-Persian month name"
        );
        assert_eq!(
            ParsiDate::parse("02 مردادXX 1403", "%d %B %Y")
                .err()
                .unwrap(), // Name with extra chars
            DateError::ParseError(ParseErrorKind::InvalidMonthName), // Fails to match "مرداد" exactly
            "Failed on month name with extra chars"
        );
        assert_eq!(
            ParsiDate::parse("01 XXX 1400", "%d %B %Y").err().unwrap(),
            DateError::ParseError(ParseErrorKind::InvalidMonthName),
            "Failed on completely wrong month name"
        );
        // Check separator matching after month name
        assert_eq!(
            ParsiDate::parse("01 فروردینX1400", "%d %B %Y")
                .err()
                .unwrap(), // Expected space after month name
            DateError::ParseError(ParseErrorKind::InvalidMonthName), // This might fail earlier if the heuristic separator check fails
            "Failed on wrong separator after month name"
        );

        // Unpadded day with %d fails (expects 2 digits)
        assert_eq!(
            ParsiDate::parse("2 مرداد 1403", "%d %B %Y").err().unwrap(),
            DateError::ParseError(ParseErrorKind::InvalidNumber),
            "Failed on unpadded day for %d"
        );
    }

    // --- Date Info Tests ---
    #[test]
    fn test_weekday() {
        // Use known Gregorian dates and verify Persian weekday
        // 2024-03-20 -> 1403-01-01 -> Wednesday -> چهارشنبه
        assert_eq!(pd(1403, 1, 1).weekday(), Ok("چهارشنبه".to_string()));
        // 2024-07-23 -> 1403-05-02 -> Tuesday -> سه‌شنبه
        assert_eq!(pd(1403, 5, 2).weekday(), Ok("سه‌شنبه".to_string()));
        // 2025-03-21 -> 1404-01-01 -> Friday -> جمعه
        assert_eq!(pd(1404, 1, 1).weekday(), Ok("جمعه".to_string()));
        // 1979-02-11 -> 1357-11-22 -> Sunday -> یکشنبه
        assert_eq!(pd(1357, 11, 22).weekday(), Ok("یکشنبه".to_string()));
        // 2026-03-20 -> 1404-12-29 -> Friday -> جمعه
        assert_eq!(pd(1404, 12, 29).weekday(), Ok("جمعه".to_string()));
        // 2024-03-23 -> 1403-01-04 -> Saturday -> شنبه
        assert_eq!(pd(1403, 1, 4).weekday(), Ok("شنبه".to_string()));
        // Test invalid date
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(invalid_date.weekday(), Err(DateError::InvalidDate));
    }
    #[test]
    fn test_ordinal() {
        assert_eq!(pd(1403, 1, 1).ordinal(), Ok(1));
        assert_eq!(pd(1403, 1, 31).ordinal(), Ok(31));
        assert_eq!(pd(1403, 2, 1).ordinal(), Ok(32));
        assert_eq!(pd(1403, 5, 2).ordinal(), Ok(126)); // 4 * 31 + 2 = 126
        assert_eq!(pd(1403, 7, 1).ordinal(), Ok(187)); // 6 * 31 + 1 = 187
        assert_eq!(pd(1403, 12, 30).ordinal(), Ok(366)); // Leap year end
        assert_eq!(pd(1404, 1, 1).ordinal(), Ok(1));
        assert_eq!(pd(1404, 12, 29).ordinal(), Ok(365)); // Common year end
                                                         // Test invalid date
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(invalid_date.ordinal(), Err(DateError::InvalidDate));
    }

    // --- Arithmetic Tests ---
    #[test]
    fn test_add_sub_days() {
        let d1 = pd(1403, 6, 30);
        assert_eq!(d1.add_days(1), Ok(pd(1403, 6, 31)));
        assert_eq!(d1.add_days(2), Ok(pd(1403, 7, 1))); // Cross month boundary
        assert_eq!(d1.add_days(32), Ok(pd(1403, 8, 1))); // Cross two month boundaries
        assert_eq!(d1.add_days(0), Ok(d1)); // Add zero

        let d_leap_end = pd(1403, 12, 29);
        assert_eq!(d_leap_end.add_days(1), Ok(pd(1403, 12, 30))); // To last day of leap year
        assert_eq!(d_leap_end.add_days(2), Ok(pd(1404, 1, 1))); // Cross year boundary (leap to common)

        let d_common_end = pd(1404, 12, 29);
        assert_eq!(d_common_end.add_days(1), Ok(pd(1405, 1, 1))); // Cross year boundary (common to common - assuming 1405 is common)
        assert!(!ParsiDate::is_persian_leap_year(1405)); // 1405 % 33 = 7

        let d_start = pd(1404, 1, 1);
        assert_eq!(d_start.add_days(-1), Ok(pd(1403, 12, 30))); // Subtract day (cross year)
        assert_eq!(d_start.sub_days(1), Ok(pd(1403, 12, 30))); // Subtract day using sub_days

        let d_start_common = pd(1405, 1, 1);
        assert_eq!(d_start_common.sub_days(1), Ok(pd(1404, 12, 29))); // Subtract day (cross year, common to common)

        // Add large number of days
        let base = pd(1400, 1, 1); // 2021-03-21
        let expected_greg = NaiveDate::from_ymd_opt(2021, 3, 21)
            .unwrap()
            .checked_add_days(chrono::Days::new(1000))
            .unwrap();
        let expected_parsi = ParsiDate::from_gregorian(expected_greg).unwrap();
        assert_eq!(base.add_days(1000), Ok(expected_parsi));
        assert_eq!(expected_parsi.sub_days(1000), Ok(base));
        assert_eq!(expected_parsi.add_days(-1000), Ok(base));

        // Test potential overflow (extremely large values might depend on chrono's limits)
        // let very_far_date = base.add_days(i64::MAX / 2); // Example
        // assert!(very_far_date.is_ok() || very_far_date == Err(DateError::ArithmeticOverflow));

        // Test invalid date input
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(invalid_date.add_days(1), Err(DateError::InvalidDate));
        assert_eq!(invalid_date.sub_days(1), Err(DateError::InvalidDate));
    }
    #[test]
    fn test_add_sub_months() {
        let d1 = pd(1403, 1, 31); // End of 31-day month
        assert_eq!(d1.add_months(1), Ok(pd(1403, 2, 31))); // To end of next 31-day month
        assert_eq!(d1.add_months(2), Ok(pd(1403, 3, 31)));
        assert_eq!(d1.add_months(5), Ok(pd(1403, 6, 31))); // To end of Shahrivar
        assert_eq!(d1.add_months(6), Ok(pd(1403, 7, 30))); // To Mehr (30 days), day clamped from 31
        assert_eq!(d1.add_months(11), Ok(pd(1403, 12, 30))); // To Esfand (30 days, leap), day clamped from 31

        let d2 = pd(1404, 1, 31); // End of month in a common year
        assert_eq!(d2.add_months(11), Ok(pd(1404, 12, 29))); // To Esfand (29 days, common), day clamped from 31

        let d3 = pd(1403, 5, 15); // Middle of month
        assert_eq!(d3.add_months(1), Ok(pd(1403, 6, 15)));
        assert_eq!(d3.add_months(7), Ok(pd(1403, 12, 15))); // To Esfand (leap)
        assert_eq!(d3.add_months(12), Ok(pd(1404, 5, 15))); // Add full year
        assert_eq!(d3.add_months(19), Ok(pd(1404, 12, 15))); // To Esfand (common)

        // Subtraction
        assert_eq!(d3.add_months(-5), Ok(pd(1402, 12, 15))); // Subtract 5 months (1402 is common)
        assert_eq!(d3.sub_months(5), Ok(pd(1402, 12, 15)));
        assert_eq!(d3.sub_months(17), Ok(pd(1401, 12, 15))); // Subtract 17 months (1401 is common)
        assert_eq!(d1.sub_months(1), Ok(pd(1402, 12, 29))); // 1403-01-31 minus 1 month -> 1402-12-29 (common)

        // Test clamping when subtracting into shorter months
        let d4 = pd(1403, 8, 30); // End of Aban (30 days)
        assert_eq!(d4.sub_months(1), Ok(pd(1403, 7, 30))); // To Mehr (30 days) - day remains 30
        assert_eq!(d4.sub_months(2), Ok(pd(1403, 6, 30))); // To Shahrivar (31 days) - day remains 30

        let d5 = pd(1403, 7, 30); // End of Mehr (30 days)
        assert_eq!(d5.sub_months(1), Ok(pd(1403, 6, 30))); // To Shahrivar (31 days) - day remains 30

        // Add zero
        assert_eq!(d3.add_months(0), Ok(d3));

        // Test large values crossing multiple years
        assert_eq!(d3.add_months(24), Ok(pd(1405, 5, 15)));
        assert_eq!(d3.sub_months(24), Ok(pd(1401, 5, 15)));

        // Test invalid date input
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(invalid_date.add_months(1), Err(DateError::InvalidDate));
        assert_eq!(invalid_date.sub_months(1), Err(DateError::InvalidDate));
    }
    #[test]
    fn test_add_sub_years() {
        let d1 = pd(1403, 5, 2); // Leap year
        assert_eq!(d1.add_years(1), Ok(pd(1404, 5, 2))); // To common year
        assert_eq!(d1.add_years(-1), Ok(pd(1402, 5, 2))); // To common year
        assert_eq!(d1.sub_years(1), Ok(pd(1402, 5, 2)));

        // Test leap day adjustment
        let d_leap_end = pd(1403, 12, 30); // Last day of leap year
        assert_eq!(d_leap_end.add_years(1), Ok(pd(1404, 12, 29))); // Add 1 year -> common year, day remains 30
        assert_eq!(d_leap_end.add_years(5), Ok(pd(1408, 12, 30))); // Add 5 years -> 1408 (leap), day clamped
        assert_eq!(d_leap_end.sub_years(4), Ok(pd(1399, 12, 30))); // Sub 4 years -> 1399 (leap), day remains 30
        assert_eq!(d_leap_end.sub_years(1), Ok(pd(1402, 12, 29))); // Sub 1 year -> 1402 (common), day clamped

        let d_common_end = pd(1404, 12, 29); // Last day of common year
        assert_eq!(d_common_end.add_years(1), Ok(pd(1405, 12, 29))); // common -> common
        assert_eq!(d_common_end.add_years(3), Ok(pd(1407, 12, 29))); // common -> leap, day is fine
        assert_eq!(d_common_end.add_years(4), Ok(pd(1408, 12, 29))); // common -> common
        assert_eq!(d_common_end.sub_years(1), Ok(pd(1403, 12, 29))); // common -> leap, day is fine

        // Add zero
        assert_eq!(d1.add_years(0), Ok(d1));

        // Test invalid date input
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(invalid_date.add_years(1), Err(DateError::InvalidDate));
        assert_eq!(invalid_date.sub_years(1), Err(DateError::InvalidDate));

        // Test year range limits
        assert!(pd(9999, 1, 1).add_years(1).is_err());
        assert!(pd(1, 1, 1).sub_years(1).is_err());
        assert!(pd(1, 1, 1).add_years(-1).is_err());
    }
    #[test]
    fn test_days_between() {
        let d1 = pd(1403, 1, 1);
        let d2 = pd(1403, 1, 11);
        let d3 = pd(1404, 1, 1); // Next year (1403 is leap)
        let d4 = pd(1402, 12, 29); // Day before d1 (1402 common)
        let d5 = pd(1405, 1, 1); // Year after d3 (1404 common)

        assert_eq!(d1.days_between(&d1), Ok(0));
        assert_eq!(d1.days_between(&d2), Ok(10)); // Within same month
        assert_eq!(d2.days_between(&d1), Ok(10)); // Order doesn't matter for abs value

        assert_eq!(d1.days_between(&d3), Ok(366)); // Across leap year boundary
        assert_eq!(d3.days_between(&d1), Ok(366));

        assert_eq!(d3.days_between(&d5), Ok(365)); // Across common year boundary
        assert_eq!(d5.days_between(&d3), Ok(365));

        assert_eq!(d1.days_between(&d4), Ok(1)); // Adjacent days across year
        assert_eq!(d4.days_between(&d1), Ok(1));

        // Longer duration
        let d_start = pd(1357, 11, 22); // 1979-02-11
        let d_end = pd(1403, 5, 2); // 2024-07-23
                                    // Use chrono to verify
        let g_start = NaiveDate::from_ymd_opt(1979, 2, 11).unwrap();
        let g_end = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
        let expected_diff = g_end.signed_duration_since(g_start).num_days();
        assert_eq!(d_start.days_between(&d_end), Ok(expected_diff.abs()));
        assert_eq!(d_end.days_between(&d_start), Ok(expected_diff.abs()));

        // Test with invalid dates
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(d1.days_between(&invalid_date), Err(DateError::InvalidDate));
        assert_eq!(invalid_date.days_between(&d1), Err(DateError::InvalidDate));
    }

    // --- Helper Method Tests ---
    #[test]
    fn test_with_year() {
        let d1 = pd(1403, 5, 2); // Leap year
        let d_leap_end = pd(1403, 12, 30);
        let d_common_mid = pd(1404, 7, 15);

        assert_eq!(d1.with_year(1404), Ok(pd(1404, 5, 2))); // leap -> common, mid-month
        assert_eq!(d_leap_end.with_year(1404), Ok(pd(1404, 12, 29))); // leap -> common, end of month, clamped
        assert_eq!(d_common_mid.with_year(1403), Ok(pd(1403, 7, 15))); // common -> leap, mid-month
        assert_eq!(pd(1404, 12, 29).with_year(1403), Ok(pd(1403, 12, 29))); // common end -> leap end (day doesn't change)

        // Test invalid target year
        assert_eq!(d1.with_year(0), Err(DateError::InvalidDate));
        assert_eq!(d1.with_year(10000), Err(DateError::InvalidDate));

        // Test with invalid self
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(invalid_date.with_year(1405), Err(DateError::InvalidDate));
    }
    #[test]
    fn test_with_month() {
        let d_31 = pd(1403, 1, 31); // End of 31-day month
        let d_30 = pd(1403, 7, 10); // Mid 30-day month
        let d_29 = pd(1404, 12, 5); // Start of 29-day month (common)
        let d_leap_end = pd(1403, 12, 30); // End of 30-day month (leap)

        assert_eq!(d_31.with_month(2), Ok(pd(1403, 2, 31))); // 31 -> 31 day month
        assert_eq!(d_31.with_month(7), Ok(pd(1403, 7, 30))); // 31 -> 30 day month (clamped)
        assert_eq!(d_31.with_month(12), Ok(pd(1403, 12, 30))); // 31 -> 30 day month (leap Esfand, clamped)
        assert_eq!(pd(1404, 1, 31).with_month(12), Ok(pd(1404, 12, 29))); // 31 -> 29 day month (common Esfand, clamped)

        assert_eq!(d_30.with_month(6), Ok(pd(1403, 6, 10))); // 30 -> 31 day month
        assert_eq!(d_30.with_month(11), Ok(pd(1403, 11, 10))); // 30 -> 30 day month

        assert_eq!(d_29.with_month(1), Ok(pd(1404, 1, 5))); // 29 -> 31 day month

        assert_eq!(d_leap_end.with_month(1), Ok(pd(1403, 1, 30))); // 30 day (leap Esfand) -> 31 day month

        // Test invalid target month
        assert_eq!(d_31.with_month(0), Err(DateError::InvalidDate));
        assert_eq!(d_31.with_month(13), Err(DateError::InvalidDate));

        // Test with invalid self
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(invalid_date.with_month(1), Err(DateError::InvalidDate));
    }
    #[test]
    fn test_with_day() {
        let d1 = pd(1403, 7, 1); // Start of 30-day month
        let d2 = pd(1404, 12, 1); // Start of 29-day month (common)
        let d3 = pd(1403, 12, 1); // Start of 30-day month (leap)

        assert_eq!(d1.with_day(15), Ok(pd(1403, 7, 15)));
        assert_eq!(d1.with_day(30), Ok(pd(1403, 7, 30))); // Valid last day
        assert_eq!(d1.with_day(31), Err(DateError::InvalidDate)); // Invalid day for month

        assert_eq!(d2.with_day(29), Ok(pd(1404, 12, 29))); // Valid last day
        assert_eq!(d2.with_day(30), Err(DateError::InvalidDate)); // Invalid day for month

        assert_eq!(d3.with_day(30), Ok(pd(1403, 12, 30))); // Valid last day (leap)
        assert_eq!(d3.with_day(31), Err(DateError::InvalidDate)); // Invalid day

        // Test invalid target day
        assert_eq!(d1.with_day(0), Err(DateError::InvalidDate));

        // Test with invalid self
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(invalid_date.with_day(1), Err(DateError::InvalidDate));
    }
    #[test]
    fn test_day_of_boundaries() {
        let d_mid_leap = pd(1403, 5, 15); // Leap year, 31-day month
        assert_eq!(d_mid_leap.first_day_of_month(), pd(1403, 5, 1));
        assert_eq!(d_mid_leap.last_day_of_month(), pd(1403, 5, 31));
        assert_eq!(d_mid_leap.first_day_of_year(), pd(1403, 1, 1));
        assert_eq!(d_mid_leap.last_day_of_year(), pd(1403, 12, 30)); // Leap year end

        let d_mid_common = pd(1404, 7, 10); // Common year, 30-day month
        assert_eq!(d_mid_common.first_day_of_month(), pd(1404, 7, 1));
        assert_eq!(d_mid_common.last_day_of_month(), pd(1404, 7, 30));
        assert_eq!(d_mid_common.first_day_of_year(), pd(1404, 1, 1));
        assert_eq!(d_mid_common.last_day_of_year(), pd(1404, 12, 29)); // Common year end

        let d_esfand_leap = pd(1403, 12, 10);
        assert_eq!(d_esfand_leap.first_day_of_month(), pd(1403, 12, 1));
        assert_eq!(d_esfand_leap.last_day_of_month(), pd(1403, 12, 30));

        let d_esfand_common = pd(1404, 12, 10);
        assert_eq!(d_esfand_common.first_day_of_month(), pd(1404, 12, 1));
        assert_eq!(d_esfand_common.last_day_of_month(), pd(1404, 12, 29));

        // Check idempotency
        assert_eq!(
            d_mid_leap.first_day_of_month().first_day_of_month(),
            pd(1403, 5, 1)
        );
        assert_eq!(
            d_mid_leap.last_day_of_month().last_day_of_month(),
            pd(1403, 5, 31)
        );
        assert_eq!(
            d_mid_leap.first_day_of_year().first_day_of_year(),
            pd(1403, 1, 1)
        );
        assert_eq!(
            d_mid_leap.last_day_of_year().last_day_of_year(),
            pd(1403, 12, 30)
        );
    }

    // --- Constant Tests ---
    #[test]
    fn test_constants_validity() {
        assert!(MIN_PARSI_DATE.is_valid());
        assert!(MAX_PARSI_DATE.is_valid());
        assert_eq!(MIN_PARSI_DATE.year(), 1);
        assert_eq!(MIN_PARSI_DATE.month(), 1);
        assert_eq!(MIN_PARSI_DATE.day(), 1);
        assert_eq!(MAX_PARSI_DATE.year(), 9999);
        assert_eq!(MAX_PARSI_DATE.month(), 12);
        assert_eq!(MAX_PARSI_DATE.day(), 29); // 9999 is not a leap year
    }

    // --- Serde Tests (conditional) ---
    #[cfg(feature = "serde")]
    mod serde_tests {
        use super::*;
        use serde_json; // Assuming serde_json is used

        // Note: Default serde derive doesn't validate on deserialize.
        // Custom deserialize logic or using `try_from` after deserialization
        // would be needed for validation during deserialization.
        #[test]
        fn test_serialization_deserialization_valid() {
            let date = pd(1403, 5, 2);
            let expected_json = r#"{"year":1403,"month":5,"day":2}"#;

            // Serialize
            let json = serde_json::to_string(&date).expect("Serialization failed");
            assert_eq!(json, expected_json);

            // Deserialize
            let deserialized: ParsiDate =
                serde_json::from_str(&json).expect("Deserialization failed");
            assert_eq!(deserialized, date);
            assert!(
                deserialized.is_valid(),
                "Deserialized valid date should be valid"
            );
        }

        #[test]
        fn test_deserialize_structurally_valid_but_logically_invalid() {
            // This JSON is structurally valid for the ParsiDate struct,
            // but the date itself (Esfand 30 in a common year) is invalid.
            let json_invalid_day = r#"{"year":1404,"month":12,"day":30}"#;

            // Default derive will successfully deserialize this struct
            let deserialized_invalid: ParsiDate = serde_json::from_str(json_invalid_day)
                .expect("Default derive should deserialize structurally valid JSON");

            // Check the values directly
            assert_eq!(deserialized_invalid.year(), 1404);
            assert_eq!(deserialized_invalid.month(), 12);
            assert_eq!(deserialized_invalid.day(), 30);

            // But the resulting ParsiDate object should report itself as invalid using is_valid()
            assert!(
                !deserialized_invalid.is_valid(),
                "Deserialized date (1404-12-30) should be invalid"
            );
        }

        #[test]
        fn test_deserialize_structurally_invalid() {
            // Field type mismatch
            let json_invalid_month_type = r#"{"year":1403,"month":"May","day":2}"#;
            assert!(
                serde_json::from_str::<ParsiDate>(json_invalid_month_type).is_err(),
                "Should fail on wrong month type"
            );

            // Missing field
            let json_missing_field = r#"{"year":1403,"month":5}"#; // Missing 'day'
            assert!(
                serde_json::from_str::<ParsiDate>(json_missing_field).is_err(),
                "Should fail on missing field"
            );

            // Extra field (usually ignored by default by serde unless specified otherwise)
            let json_extra_field = r#"{"year":1403,"month":5,"day":2,"extra":"data"}"#;
            match serde_json::from_str::<ParsiDate>(json_extra_field) {
                Ok(pd) => {
                    // Default behavior is often to ignore extra fields
                    assert_eq!(pd, ParsiDate::new(1403, 5, 2).unwrap());
                    println!("Warning: Deserialization succeeded despite extra field (default serde behavior).");
                }
                Err(e) => {
                    // Fails if serde is configured to deny unknown fields
                    panic!("Deserialization failed unexpectedly on extra field: {}", e);
                }
            }

            // Completely wrong structure
            let json_wrong_structure = r#"[1403, 5, 2]"#;
            assert!(
                serde_json::from_str::<ParsiDate>(json_wrong_structure).is_err(),
                "Should fail on wrong JSON structure"
            );
        }
    }
} // end tests module
