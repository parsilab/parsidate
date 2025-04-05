//  * Copyright (C) Mohammad (Sina) Jalalvandi (parsidate) 2024-2025 <jalalvandi.sina@gmail.com>
//  * Version : 1.3.2
//  * f3dcebad-2908-4694-b835-a1ff6b337f35 - Extended & Corrected
//
//! # ParsiDate: Comprehensive Persian (Jalali) Calendar Implementation in Rust
//!
//! This crate provides comprehensive functionality for working with the Persian (Jalali or Shamsi) calendar system.
//! It allows for:
//!
//! *   **Conversion:** Seamlessly convert dates between the Gregorian and Persian calendars.
//! *   **Validation:** Check if a given year, month, and day combination forms a valid Persian date.
//! *   **Formatting:** Display Persian dates in various predefined formats ("short", "long", "iso") and using custom `strftime`-like patterns.
//! *   **Parsing:** Parse strings into `ParsiDate` objects based on specified formats.
//! *   **Date Arithmetic:** Add or subtract days, months, and years from a date.
//! *   **Leap Year Calculation:** Determine if a Persian or Gregorian year is a leap year (using a common 33-year cycle approximation for Persian).
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
//!     // Note: Default serde derive does not validate logic on deserialize.
//!     // The struct fields will be populated, but may form an invalid date.
//!     // Use `is_valid()` after deserialization if validation is needed.
//!     let deserialized: ParsiDate = serde_json::from_str(&json).unwrap();
//!     assert_eq!(deserialized, pd);
//!     assert!(deserialized.is_valid());
//!
//!     // Example of deserializing potentially invalid data
//!     let json_invalid = r#"{"year":1404,"month":12,"day":30}"#; // 1404 is not leap
//!     let deserialized_invalid: ParsiDate = serde_json::from_str(json_invalid).unwrap();
//!     assert!(!deserialized_invalid.is_valid());
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

/// Minimum supported ParsiDate: Year 1, Month 1 (Farvardin), Day 1.
///
/// This corresponds approximately to the Gregorian date 622-03-21 (using proleptic Gregorian calendar for calculations),
/// representing the epoch start of the Persian calendar.
pub const MIN_PARSI_DATE: ParsiDate = ParsiDate {
    year: 1,
    month: 1,
    day: 1,
};

/// Maximum supported ParsiDate: Year 9999, Month 12 (Esfand), Day 29.
///
/// The year 9999 is chosen as a practical upper limit. According to the 33-year cycle approximation
/// used in this library (`9999 % 33 == 3`), it is *not* a leap year, hence the last day is the 29th.
pub const MAX_PARSI_DATE: ParsiDate = ParsiDate {
    year: 9999,
    month: 12,
    day: 29,
};

// --- Data Structures ---

/// Represents a date in the Persian (Jalali or Shamsi) calendar system.
///
/// Stores the year, month (1-12), and day (1-31) components.
/// Provides methods for validation, conversion, formatting, parsing, and arithmetic.
///
/// Note on Range: Supports years from 1 up to 9999.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParsiDate {
    /// The year component of the Persian date (e.g., 1403). Must be between 1 and 9999 inclusive.
    year: i32,
    /// The month component of the Persian date (1 = Farvardin, ..., 12 = Esfand). Must be between 1 and 12 inclusive.
    month: u32,
    /// The day component of the Persian date (1-29/30/31). Must be valid for the given month and year.
    day: u32,
}

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
}

// --- Error Implementation ---

impl fmt::Display for DateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DateError::InvalidDate => write!(
                f,
                "Invalid Persian date components (year [1-9999], month [1-12], or day)"
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
                "Invalid ordinal day number (must be 1-365 or 1-366 based on year)"
            ),
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

// --- Internal Helper Constants ---

/// Persian month names (index 0 = Farvardin, ..., 11 = Esfand).
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

/// Persian weekday names (index 0 = Shanbeh/Saturday, ..., 6 = Jomeh/Friday).
const WEEKDAY_NAMES_PERSIAN: [&str; 7] = [
    "شنبه",     // 0
    "یکشنبه",   // 1
    "دوشنبه",   // 2
    "سه‌شنبه",   // 3
    "چهارشنبه", // 4
    "پنجشنبه",  // 5
    "جمعه",     // 6
];

// --- Core Implementation ---

impl ParsiDate {
    // --- Constructors and Converters ---

    /// Creates a new `ParsiDate` instance from year, month, and day components.
    ///
    /// This function validates the date upon creation. The year must be between 1 and 9999,
    /// the month between 1 and 12, and the day must be valid for the given month and year
    /// (considering leap years for Esfand).
    ///
    /// # Arguments
    ///
    /// * `year`: The Persian year (1-9999).
    /// * `month`: The Persian month (1-12).
    /// * `day`: The Persian day (1-31, depending on month and leap year).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the provided components do not form a valid
    /// Persian date within the supported range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// assert!(ParsiDate::new(1403, 5, 2).is_ok());
    /// assert_eq!(ParsiDate::new(1403, 13, 1), Err(DateError::InvalidDate)); // Invalid month
    /// assert_eq!(ParsiDate::new(1404, 12, 30), Err(DateError::InvalidDate)); // Invalid day (1404 not leap)
    /// assert_eq!(ParsiDate::new(0, 1, 1), Err(DateError::InvalidDate)); // Invalid year
    /// ```
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self, DateError> {
        // Initial struct creation
        let date = ParsiDate { year, month, day };
        // Validate the components
        if date.is_valid() {
            Ok(date)
        } else {
            Err(DateError::InvalidDate)
        }
    }

    /// Creates a `ParsiDate` from year, month, and day without validation.
    ///
    /// **Warning:** This function is `unsafe` because it bypasses the validation checks
    /// performed by `ParsiDate::new`. Creating a `ParsiDate` with invalid components
    /// (e.g., month 13, day 32, year 0) using this function can lead to undefined behavior,
    /// incorrect results, or panics when other methods are called on the invalid date object.
    ///
    /// This should only be used in performance-critical situations where the date components
    /// are already known to be valid through external means. Prefer `ParsiDate::new()`
    /// for safe construction in most cases.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided `year`, `month`, and `day` combination
    /// represents a logically valid Persian date according to the calendar rules and
    /// within the supported year range (1-9999).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// // Assume year, month, day are validated elsewhere
    /// let year = 1403;
    /// let month = 5;
    /// let day = 2;
    ///
    /// if year >= 1 && year <= 9999 && month >= 1 && month <= 12 && day >= 1 && day <= ParsiDate::days_in_month(year, month) {
    ///     let date = unsafe { ParsiDate::new_unchecked(year, month, day) };
    ///     assert_eq!(date.year(), 1403);
    /// } else {
    ///     // Handle invalid input case
    /// }
    /// ```
    pub const unsafe fn new_unchecked(year: i32, month: u32, day: u32) -> Self {
        ParsiDate { year, month, day }
    }

    /// Creates a `ParsiDate` from the day number within a given Persian year (ordinal day).
    ///
    /// The ordinal day is 1-based, where 1 corresponds to Farvardin 1st.
    /// The valid range for `ordinal` is 1 to 365 for common years, and 1 to 366 for leap years.
    ///
    /// # Arguments
    ///
    /// * `year`: The Persian year (1-9999).
    /// * `ordinal`: The day number within the year (1-365 or 1-366).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidOrdinal)` if the `ordinal` value is 0 or greater than
    /// the number of days in the specified `year`.
    /// Returns `Err(DateError::InvalidDate)` if the `year` is outside the supported range (1-9999),
    /// although this check happens during the final `ParsiDate::new` call.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// assert_eq!(ParsiDate::from_ordinal(1403, 1), Ok(ParsiDate::new(1403, 1, 1).unwrap())); // Farvardin 1st
    /// assert_eq!(ParsiDate::from_ordinal(1403, 366), Ok(ParsiDate::new(1403, 12, 30).unwrap())); // Last day of leap year 1403
    /// assert_eq!(ParsiDate::from_ordinal(1404, 365), Ok(ParsiDate::new(1404, 12, 29).unwrap())); // Last day of common year 1404
    /// assert_eq!(ParsiDate::from_ordinal(1404, 366), Err(DateError::InvalidOrdinal)); // Too large for common year
    /// assert_eq!(ParsiDate::from_ordinal(1403, 0), Err(DateError::InvalidOrdinal)); // Zero is invalid
    /// ```
    pub fn from_ordinal(year: i32, ordinal: u32) -> Result<Self, DateError> {
        // Basic validation of ordinal
        if ordinal == 0 {
            return Err(DateError::InvalidOrdinal);
        }
        // Determine days in the target year
        let is_leap = Self::is_persian_leap_year(year);
        let days_in_year = if is_leap { 366 } else { 365 };

        // Validate ordinal against year length
        if ordinal > days_in_year {
            return Err(DateError::InvalidOrdinal);
        }

        // Iterate through months to find the correct month and day
        let mut month = 1u32;
        let mut day = ordinal;
        let month_lengths = Self::month_lengths(year);

        for (m_idx, length) in month_lengths.iter().enumerate() {
            if day <= *length {
                month = (m_idx + 1) as u32; // Found the month (m_idx is 0-based)
                break; // Exit loop once month is found
            }
            // Subtract days of the current month and move to the next
            day -= *length;
            // Update month number for the next iteration (or if loop ends)
            // This ensures month is correct even if `day` becomes exactly 0 after subtraction
            month = (m_idx + 2) as u32;
        }

        // Use new() for final validation (including year range check)
        // The logic above should guarantee month/day are valid if ordinal was valid,
        // but `new` provides an extra safety layer and handles the year check.
        ParsiDate::new(year, month, day)
    }

    /// Converts a Gregorian date (`chrono::NaiveDate`) to its equivalent Persian (Jalali) date.
    ///
    /// This function implements the conversion algorithm, determining the corresponding
    /// Persian year, month, and day for the given Gregorian date.
    ///
    /// # Arguments
    ///
    /// * `gregorian_date`: The Gregorian date to convert.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::GregorianConversionError)` if:
    /// * The `gregorian_date` is before the Persian epoch start (approximately 622-03-21).
    /// * The calculation results in a Persian year outside the supported range (1-9999).
    /// * An internal `chrono` operation fails (e.g., creating the epoch date).
    /// * An internal inconsistency is detected during calculation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// let g_date = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
    /// assert_eq!(ParsiDate::from_gregorian(g_date), Ok(ParsiDate::new(1403, 5, 2).unwrap()));
    ///
    /// let epoch_gregorian = NaiveDate::from_ymd_opt(622, 3, 21).unwrap();
    /// assert_eq!(ParsiDate::from_gregorian(epoch_gregorian), Ok(ParsiDate::new(1, 1, 1).unwrap()));
    ///
    /// let before_epoch = NaiveDate::from_ymd_opt(622, 3, 20).unwrap();
    /// assert_eq!(ParsiDate::from_gregorian(before_epoch), Err(DateError::GregorianConversionError));
    /// ```
    pub fn from_gregorian(gregorian_date: NaiveDate) -> Result<Self, DateError> {
        // Define the start of the Persian epoch in Gregorian terms
        let persian_epoch_gregorian_start =
            NaiveDate::from_ymd_opt(622, 3, 21).ok_or(DateError::GregorianConversionError)?;

        // Check if the date is before the epoch
        if gregorian_date < persian_epoch_gregorian_start {
            return Err(DateError::GregorianConversionError); // Date is before epoch start
        }

        // --- Calculate Persian Year ---
        // Estimate the number of days passed since the epoch start
        let days_since_epoch_day1 = gregorian_date
            .signed_duration_since(persian_epoch_gregorian_start)
            .num_days();

        // Make an initial guess for the Persian year.
        // Average ~365.25 days/year. Dividing by 366 provides a conservative initial guess.
        let mut p_year_guess = MIN_PARSI_DATE.year + (days_since_epoch_day1 / 366) as i32;
        if p_year_guess < MIN_PARSI_DATE.year {
            p_year_guess = MIN_PARSI_DATE.year; // Ensure guess is at least 1
        }

        // Loop to find the correct Persian year by checking the Gregorian date of Farvardin 1st
        // for the guessed year and the next year.
        let p_year = loop {
            // Calculate Gregorian date for Farvardin 1 of the guessed year
            // Use unsafe new_unchecked + internal conversion for performance inside the loop
            let temp_start_date = unsafe { ParsiDate::new_unchecked(p_year_guess, 1, 1) };
            let gy_start_of_pyear = match temp_start_date.to_gregorian_internal() {
                Ok(gd) => gd,
                // If conversion fails for the guess (e.g., year too high), we need to adjust down.
                // However, the logic should generally converge before hitting extreme limits.
                // If it *does* fail, it implies an issue, likely out of range.
                Err(e) => return Err(e),
            };

            // If Farvardin 1st of the guess is *after* the target date, the guess is too high.
            if gy_start_of_pyear > gregorian_date {
                p_year_guess -= 1; // Adjust guess down
                                   // If the adjusted guess is now the correct year, break.
                                   // We need to re-check the start date for the new guess if we continue looping,
                                   // but if `gy_start_of_pyear` was only slightly too high, `p_year_guess - 1` is likely correct.
                                   // Re-evaluating in the next loop iteration is safer. Let's refine this.

                // Let's test the *new* guess immediately.
                let temp_prev_start_date = unsafe { ParsiDate::new_unchecked(p_year_guess, 1, 1) };
                match temp_prev_start_date.to_gregorian_internal() {
                    Ok(gd_prev) => {
                        if gd_prev <= gregorian_date {
                            // The previous year starts on or before the target date.
                            // And we know the original guess year started *after*.
                            // So, the correct year is `p_year_guess`.
                            break p_year_guess;
                        } else {
                            // Still too high? Continue loop to decrement further. Should be rare.
                            continue;
                        }
                    }
                    Err(e) => return Err(e), // Error converting decremented year start
                }
            }

            // If Farvardin 1st of the guess is on or before the target date,
            // check if Farvardin 1st of the *next* year is *after* the target date.
            let next_year = match p_year_guess.checked_add(1) {
                Some(y) => y,
                None => return Err(DateError::GregorianConversionError), // Year overflow
            };
            let temp_start_date_next = unsafe { ParsiDate::new_unchecked(next_year, 1, 1) };
            match temp_start_date_next.to_gregorian_internal() {
                Ok(gd_next) => {
                    if gd_next > gregorian_date {
                        // Found the correct year range! `p_year_guess` is the year.
                        break p_year_guess;
                    } else {
                        // Target date is in a later year, increment guess and loop again.
                        p_year_guess += 1;
                    }
                }
                Err(_) => {
                    // If converting the start of the *next* year fails (e.g., out of range like year 10000+),
                    // it implies the current guess (`p_year_guess`) might be the last possible valid year
                    // containing the date.
                    if gy_start_of_pyear <= gregorian_date {
                        // The current guess starts on/before the target, and the next year is invalid/too far.
                        break p_year_guess;
                    } else {
                        // This case (current guess starts *after* target AND next year fails) seems unlikely
                        // given the earlier check. If it happens, return error.
                        return Err(DateError::GregorianConversionError);
                    }
                }
            }

            // Safety break to prevent potential infinite loops with very large dates or logic errors.
            if p_year_guess > MAX_PARSI_DATE.year + 5 || p_year_guess < MIN_PARSI_DATE.year {
                return Err(DateError::GregorianConversionError); // Likely out of range or issue
            }
        }; // End of year-finding loop

        // --- Calculate Persian Month and Day ---
        // Now `p_year` holds the correct Persian year.
        // Find the Gregorian date corresponding to Farvardin 1st of this correct year.
        let correct_pyear_start_gregorian =
            unsafe { ParsiDate::new_unchecked(p_year, 1, 1) }.to_gregorian_internal()?;

        // Calculate how many days into the Persian year the target Gregorian date falls (0-based index).
        let days_into_year = gregorian_date
            .signed_duration_since(correct_pyear_start_gregorian)
            .num_days();

        // This should not be negative if the year-finding logic is correct.
        if days_into_year < 0 {
            return Err(DateError::GregorianConversionError); // Internal calculation error state
        }

        // Determine month and day from the 0-based `days_into_year`.
        let month_lengths = Self::month_lengths(p_year);
        let mut remaining_days_in_year = days_into_year as u32; // Now 0-indexed day number within year
        let mut p_month = 1u32;
        let mut p_day = 1u32; // Will be overwritten

        for (m_idx, length) in month_lengths.iter().enumerate() {
            // Ensure length is not zero to avoid infinite loop (shouldn't happen)
            if *length == 0 {
                return Err(DateError::InvalidDate); // Should not happen with valid month_lengths
            }
            // Check if the day falls within the current month (length)
            if remaining_days_in_year < *length {
                p_month = (m_idx + 1) as u32; // Month is 1-based index + 1
                p_day = remaining_days_in_year + 1; // Day is 1-based remaining days + 1
                break; // Found the month and day
            }
            // Subtract the days of this month and continue to the next
            remaining_days_in_year -= *length;

            // Handle case where the date is the very last day of the year
            if m_idx == 11 && remaining_days_in_year == 0 {
                // This occurs *after* subtracting the last month's length.
                // It means the target day was the last day of Esfand.
                p_month = 12;
                p_day = *length; // Day is the length of Esfand
                break;
            }
        }

        // Use new() for final validation of the calculated date (year, month, day).
        // This ensures consistency and catches potential edge cases in the logic above.
        ParsiDate::new(p_year, p_month, p_day)
    }

    /// Converts this Persian (Jalali) date to its equivalent Gregorian date (`chrono::NaiveDate`).
    ///
    /// Performs validation before attempting the conversion.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the `ParsiDate` instance itself holds invalid data
    /// (e.g., created via `unsafe fn new_unchecked` with bad values).
    /// Returns `Err(DateError::GregorianConversionError)` if the conversion results in a Gregorian
    /// date outside the range supported by `chrono::NaiveDate` or if an internal arithmetic
    /// overflow occurs during calculation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use parsidate::ParsiDate;
    ///
    /// let pd = ParsiDate::new(1403, 5, 2).unwrap();
    /// let expected_gregorian = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
    /// assert_eq!(pd.to_gregorian(), Ok(expected_gregorian));
    ///
    /// let pd_epoch = ParsiDate::new(1, 1, 1).unwrap();
    /// let expected_epoch_gregorian = NaiveDate::from_ymd_opt(622, 3, 21).unwrap();
    /// assert_eq!(pd_epoch.to_gregorian(), Ok(expected_epoch_gregorian));
    /// ```
    pub fn to_gregorian(&self) -> Result<NaiveDate, DateError> {
        // Ensure the ParsiDate object itself is valid before converting.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Call the internal conversion logic.
        self.to_gregorian_internal()
    }

    /// Internal conversion logic: Persian to Gregorian.
    /// Assumes `self` represents a valid ParsiDate.
    /// Calculates days since the Persian epoch and adds them to the Gregorian epoch start date.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::GregorianConversionError)` if chrono fails to create the epoch date,
    /// if integer overflow occurs during day summation, or if adding the final day offset
    /// to the Gregorian epoch date fails (e.g., results in a date out of chrono's range).
    fn to_gregorian_internal(&self) -> Result<NaiveDate, DateError> {
        // Gregorian start date corresponding to Persian epoch (1/1/1).
        let persian_epoch_gregorian_start =
            NaiveDate::from_ymd_opt(622, 3, 21).ok_or(DateError::GregorianConversionError)?;

        // Calculate the total number of days from Persian epoch start (1/1/1) up to the day *before* self.
        // Sum days in full years prior to self.year.
        let mut total_days_offset: i64 = 0;
        // Loop from year 1 up to (but not including) self.year.
        for y in MIN_PARSI_DATE.year..self.year {
            let days_in_year: i64 = if Self::is_persian_leap_year(y) {
                366
            } else {
                365
            };
            // Add days, checking for potential integer overflow.
            total_days_offset = total_days_offset
                .checked_add(days_in_year)
                .ok_or(DateError::GregorianConversionError)?;
        }

        // Sum days in full months prior to self.month within self.year.
        let month_lengths_current_year = Self::month_lengths(self.year);
        // month is 1-based, loop from month 1 up to (but not including) self.month.
        if self.month > 1 {
            // self.month is guaranteed to be <= 12 because to_gregorian checks is_valid first.
            for m in 1..self.month {
                // Get month length (m-1 is the 0-based index).
                // This indexing is safe due to the is_valid check.
                let days_in_month = month_lengths_current_year[(m - 1) as usize] as i64;
                // Add days, checking for potential integer overflow.
                total_days_offset = total_days_offset
                    .checked_add(days_in_month)
                    .ok_or(DateError::GregorianConversionError)?;
            }
        }
        // If self.month is 1, this loop doesn't run, which is correct.

        // Add the day of the month (minus 1, since we want offset from the start of the month).
        // self.day is guaranteed to be >= 1.
        total_days_offset = total_days_offset
            .checked_add((self.day - 1) as i64)
            .ok_or(DateError::GregorianConversionError)?;

        // The total_days_offset now represents the number of days elapsed since 1/1/1.
        // Add this offset to the Gregorian date corresponding to 1/1/1.
        if total_days_offset < 0 {
            // This should not happen if year >= 1 and day >= 1.
            return Err(DateError::GregorianConversionError); // Indicates an internal logic error
        }

        // Use chrono's checked_add_days for safe addition.
        persian_epoch_gregorian_start
            .checked_add_days(chrono::Days::new(total_days_offset as u64))
            .ok_or(DateError::GregorianConversionError) // Return error if chrono addition fails (e.g., out of range)
    }

    /// Returns the Persian date for the current system date based on the local timezone.
    ///
    /// Obtains the current Gregorian date from the system and converts it to `ParsiDate`.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::GregorianConversionError)` if the conversion from the current
    /// Gregorian date fails. This could happen if the system clock is set to a date
    /// outside the range supported by this library or `chrono`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// match ParsiDate::today() {
    ///     Ok(today) => println!("Today in Persian calendar is: {}", today),
    ///     Err(e) => eprintln!("Failed to get today's Persian date: {}", e),
    /// }
    /// ```
    pub fn today() -> Result<Self, DateError> {
        // Get current local time.
        let now = chrono::Local::now();
        // Extract the naive date part (ignoring time and timezone offset after getting local date).
        let gregorian_today = now.date_naive();
        // Convert the Gregorian date to ParsiDate.
        Self::from_gregorian(gregorian_today)
    }

    // --- Accessors ---

    /// Returns the year component of the date.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// assert_eq!(date.year(), 1403);
    /// ```
    #[inline]
    pub const fn year(&self) -> i32 {
        self.year
    }

    /// Returns the month component of the date (1 = Farvardin, ..., 12 = Esfand).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// assert_eq!(date.month(), 5); // 5 corresponds to Mordad
    /// ```
    #[inline]
    pub const fn month(&self) -> u32 {
        self.month
    }

    /// Returns the day component of the date (1-31).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// assert_eq!(date.day(), 2);
    /// ```
    #[inline]
    pub const fn day(&self) -> u32 {
        self.day
    }

    // --- Validation and Leap Year ---

    /// Checks if the current `ParsiDate` instance represents a valid date according to the
    /// Persian calendar rules and the supported range of this library.
    ///
    /// Validation checks include:
    /// * Year is within the range [1, 9999].
    /// * Month is within the range [1, 12].
    /// * Day is within the range [1, days_in_month(year, month)].
    ///
    /// # Returns
    ///
    /// `true` if the date is valid, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let valid_date = ParsiDate::new(1403, 12, 30).unwrap(); // 1403 is leap
    /// assert!(valid_date.is_valid());
    ///
    /// let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) }; // 1404 not leap
    /// assert!(!invalid_date.is_valid());
    ///
    /// let invalid_month = unsafe { ParsiDate::new_unchecked(1403, 13, 1) };
    /// assert!(!invalid_month.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        // Check year range
        if self.year < MIN_PARSI_DATE.year || self.year > MAX_PARSI_DATE.year {
            return false;
        }
        // Check month range
        if self.month == 0 || self.month > 12 {
            return false;
        }
        // Check day range (day must be at least 1)
        if self.day == 0 {
            return false;
        }
        // Check day against the maximum days for the given month and year
        let max_days = Self::days_in_month(self.year, self.month);
        // Note: days_in_month returns 0 for invalid months, handled by the month check above.
        // If max_days were 0 here, it would imply an invalid month passed the earlier check,
        // which shouldn't happen. The final check ensures day <= max_days.
        self.day <= max_days
    }

    /// Determines if a given Persian year is a leap year.
    ///
    /// This implementation uses a common algorithmic approximation based on a 33-year cycle.
    /// A year `y` is considered leap if `y % 33` results in one of the specific remainders:
    /// 1, 5, 9, 13, 17, 22, 26, or 30.
    ///
    /// Note: Astronomical calculations provide the most accurate determination, but this
    /// 33-year cycle is widely used and accurate for a very long period around the present.
    ///
    /// Years less than 1 are considered non-leap.
    ///
    /// # Arguments
    ///
    /// * `year`: The Persian year to check.
    ///
    /// # Returns
    ///
    /// `true` if the year is a leap year according to the 33-year cycle, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// assert!(ParsiDate::is_persian_leap_year(1403)); // 1403 % 33 = 5
    /// assert!(!ParsiDate::is_persian_leap_year(1404)); // 1404 % 33 = 6
    /// assert!(ParsiDate::is_persian_leap_year(1399)); // 1399 % 33 = 30
    /// assert!(!ParsiDate::is_persian_leap_year(1400)); // 1400 % 33 = 1
    /// assert!(ParsiDate::is_persian_leap_year(1432)); // 1432 % 33 = 13
    /// assert!(!ParsiDate::is_persian_leap_year(0));
    /// ```
    pub fn is_persian_leap_year(year: i32) -> bool {
        // Years <= 0 are not valid Persian years in this context.
        if year <= 0 {
            return false;
        }
        // Check the remainder when the year is divided by 33 using Euclidean remainder.
        match year.rem_euclid(33) {
            // These remainders correspond to leap years in the cycle.
            1 | 5 | 9 | 13 | 17 | 22 | 26 | 30 => true,
            // All other remainders correspond to common years.
            _ => false,
        }
    }

    /// Determines if a given Gregorian year is a leap year.
    ///
    /// Uses the standard Gregorian calendar rules:
    /// * Divisible by 4, but not by 100, unless also divisible by 400.
    ///
    /// # Arguments
    ///
    /// * `year`: The Gregorian year to check.
    ///
    /// # Returns
    ///
    /// `true` if the year is a leap year, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// assert!(ParsiDate::is_gregorian_leap_year(2000)); // Divisible by 400
    /// assert!(ParsiDate::is_gregorian_leap_year(2024)); // Divisible by 4, not by 100
    /// assert!(!ParsiDate::is_gregorian_leap_year(1900)); // Divisible by 100, not by 400
    /// assert!(!ParsiDate::is_gregorian_leap_year(2023)); // Not divisible by 4
    /// ```
    pub fn is_gregorian_leap_year(year: i32) -> bool {
        // Standard Gregorian leap year rule implementation.
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Returns the number of days in a specific month of a given Persian year.
    ///
    /// Takes into account whether the year is a leap year for the month of Esfand (12).
    /// * Months 1-6 (Farvardin to Shahrivar) have 31 days.
    /// * Months 7-11 (Mehr to Bahman) have 30 days.
    /// * Month 12 (Esfand) has 30 days in a leap year, and 29 days in a common year.
    ///
    /// # Arguments
    ///
    /// * `year`: The Persian year (used to check for leap year if month is 12).
    /// * `month`: The Persian month (1-12).
    ///
    /// # Returns
    ///
    /// The number of days (29, 30, or 31) in the specified month and year.
    /// Returns 0 if the `month` number is invalid (outside 1-12).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// assert_eq!(ParsiDate::days_in_month(1403, 1), 31); // Farvardin
    /// assert_eq!(ParsiDate::days_in_month(1403, 7), 30); // Mehr
    /// assert_eq!(ParsiDate::days_in_month(1403, 12), 30); // Esfand (1403 is leap)
    /// assert_eq!(ParsiDate::days_in_month(1404, 12), 29); // Esfand (1404 is common)
    /// assert_eq!(ParsiDate::days_in_month(1403, 13), 0); // Invalid month
    /// ```
    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1..=6 => 31,  // Farvardin to Shahrivar
            7..=11 => 30, // Mehr to Bahman
            12 => {
                // Esfand: depends on leap year status
                if Self::is_persian_leap_year(year) {
                    30
                } else {
                    29
                }
            }
            // Invalid month number specified
            _ => 0,
        }
    }

    /// Returns an array containing the lengths of the 12 months for a given Persian year.
    ///
    /// This is an internal helper function used by other methods like `from_ordinal`
    /// and `to_gregorian_internal`. The length of the 12th month (Esfand) depends
    /// on whether the given `year` is a leap year.
    ///
    /// # Arguments
    ///
    /// * `year`: The Persian year.
    ///
    /// # Returns
    ///
    /// An array `[u32; 12]` where index 0 is the length of Farvardin, ..., index 11 is the length of Esfand.
    fn month_lengths(year: i32) -> [u32; 12] {
        [
            31, // 1: Farvardin
            31, // 2: Ordibehesht
            31, // 3: Khordad
            31, // 4: Tir
            31, // 5: Mordad
            31, // 6: Shahrivar
            30, // 7: Mehr
            30, // 8: Aban
            30, // 9: Azar
            30, // 10: Dey
            30, // 11: Bahman
            // 12: Esfand - length depends on whether the year is leap
            if Self::is_persian_leap_year(year) {
                30
            } else {
                29
            },
        ]
    }

    // --- Formatting ---

    /// Formats the `ParsiDate` into a string based on predefined styles or a custom pattern.
    ///
    /// # Arguments
    ///
    /// * `style_or_pattern`: A string specifying the desired format. Can be one of:
    ///     * `"short"`: Formats as "YYYY/MM/DD" (e.g., "1403/05/02"). This is the default for `Display`.
    ///     * `"long"`: Formats as "D Month YYYY" (e.g., "2 مرداد 1403"). Note: Day is *not* zero-padded.
    ///     * `"iso"`: Formats as "YYYY-MM-DD" (e.g., "1403-05-02").
    ///     * Custom `strftime`-like pattern: Any other string is treated as a format pattern.
    ///       See [`format_strftime`](#method.format_strftime) for supported specifiers.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted date. If the `ParsiDate` instance is invalid
    /// (e.g., created via `unsafe`), the output for some format specifiers might indicate
    /// an error (like "?InvalidMonth?").
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// assert_eq!(date.format("short"), "1403/05/02");
    /// assert_eq!(date.format("long"), "2 مرداد 1403");
    /// assert_eq!(date.format("iso"), "1403-05-02");
    /// assert_eq!(date.format("%Y, %B %d"), "1403, مرداد 02"); // Custom pattern
    /// assert_eq!(date.to_string(), "1403/05/02"); // Display trait uses "short"
    /// ```
    pub fn format(&self, style_or_pattern: &str) -> String {
        match style_or_pattern {
            "short" => format!("{}/{:02}/{:02}", self.year, self.month, self.day),
            "long" => format!(
                // Note: Day 'D' is NOT zero-padded in the "long" format specification.
                "{} {} {}",
                self.day,
                // Get month name safely, using saturating_sub and get to handle potential invalid month values gracefully.
                MONTH_NAMES_PERSIAN
                    .get((self.month.saturating_sub(1)) as usize)
                    .unwrap_or(&"?InvalidMonth?"), // Provide fallback for invalid index
                self.year
            ),
            "iso" => format!("{}-{:02}-{:02}", self.year, self.month, self.day),
            // Any other string is treated as a custom format pattern.
            pattern => self.format_strftime(pattern),
        }
    }

    /// Formats the date according to `strftime`-like specifiers.
    ///
    /// This method is called internally by `format` when a custom pattern is provided.
    ///
    /// # Supported Format Specifiers:
    ///
    /// *   `%Y`: The full Persian year (e.g., 1403).
    /// *   `%m`: The Persian month number, zero-padded (01-12).
    /// *   `%d`: The Persian day of the month, zero-padded (01-31).
    /// *   `%B`: The full Persian month name (e.g., "فروردین", "مرداد").
    /// *   `%A`: The full Persian weekday name (e.g., "شنبه", "سه‌شنبه").
    /// *   `%w`: The weekday number (Saturday=0, Sunday=1, ..., Friday=6).
    /// *   `%j`: The day of the year (ordinal day), zero-padded (001-366).
    /// *   `%%`: A literal percent sign (`%`).
    ///
    /// Any characters in the pattern that are not part of a recognized specifier are included literally
    /// in the output string. Unrecognized specifiers (e.g., `%x`) are also output literally.
    ///
    /// # Arguments
    ///
    /// * `pattern`: The format string containing literal characters and format specifiers.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted date according to the pattern.
    /// If the `ParsiDate` instance is invalid, or if calculations like weekday/ordinal fail,
    /// placeholders like "?InvalidMonth?", "?", "???" may appear in the output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 1, 7).unwrap(); // A Tuesday
    /// assert_eq!(date.format_strftime("%Y-%m-%d is a %A (day %j)"), "1403-01-07 is a سه‌شنبه (day 007)");
    /// assert_eq!(date.format_strftime("Date: %d %B, %Y %%"), "Date: 07 فروردین, 1403 %");
    /// ```
    pub fn format_strftime(&self, pattern: &str) -> String {
        // Preallocate string with a reasonable estimate capacity.
        let mut result = String::with_capacity(pattern.len() + 10);
        // Use iterator over characters for Unicode safety.
        let mut chars = pattern.chars().peekable();

        // Caching results for performance if the same specifier is used multiple times in one pattern.
        // Cache the Result to handle potential errors during calculation once.
        let mut weekday_cache: Option<Result<String, DateError>> = None;
        let mut ordinal_cache: Option<Result<u32, DateError>> = None;
        let mut weekday_num_cache: Option<Result<u32, DateError>> = None;

        while let Some(c) = chars.next() {
            if c == '%' {
                // Check the character following the '%'
                match chars.next() {
                    // Literal '%'
                    Some('%') => result.push('%'),
                    // Year
                    Some('Y') => result.push_str(&self.year.to_string()),
                    // Month (zero-padded)
                    Some('m') => result.push_str(&format!("{:02}", self.month)),
                    // Day (zero-padded)
                    Some('d') => result.push_str(&format!("{:02}", self.day)),
                    // Month name
                    Some('B') => {
                        // Access month name safely using get() with 0-based index.
                        if let Some(name) =
                            MONTH_NAMES_PERSIAN.get((self.month.saturating_sub(1)) as usize)
                        {
                            result.push_str(name);
                        } else {
                            result.push_str("?InvalidMonth?"); // Handle invalid month value in date
                        }
                    }
                    // Weekday name
                    Some('A') => {
                        // Compute (or retrieve from cache) the weekday name.
                        if weekday_cache.is_none() {
                            weekday_cache = Some(self.weekday_internal()); // Use internal fn returning Result
                        }
                        // Handle the cached Result.
                        match weekday_cache.as_ref().unwrap() {
                            // Safe unwrap as we just set it if None
                            Ok(name) => result.push_str(name),
                            Err(_) => result.push_str("?WeekdayError?"), // Indicate calculation error
                        }
                    }
                    // Weekday number (Sat=0..Fri=6)
                    Some('w') => {
                        if weekday_num_cache.is_none() {
                            weekday_num_cache = Some(self.weekday_num_sat_0()); // Calculate if not cached
                        }
                        match weekday_num_cache.as_ref().unwrap() {
                            Ok(num) => result.push_str(&num.to_string()),
                            Err(_) => result.push('?'), // Indicate calculation error
                        }
                    }
                    // Ordinal day (zero-padded)
                    Some('j') => {
                        if ordinal_cache.is_none() {
                            ordinal_cache = Some(self.ordinal_internal()); // Calculate if not cached
                        }
                        match ordinal_cache.as_ref().unwrap() {
                            Ok(ord) => result.push_str(&format!("{:03}", ord)),
                            Err(_) => result.push_str("???"), // Indicate calculation error
                        }
                    }
                    // Optional: Add %e for space-padded day (common in some strftime implementations)
                    // Some('e') => result.push_str(&format!("{:>2}", self.day)), // Right-align with space padding

                    // Unrecognized specifier - output literally
                    Some(other) => {
                        result.push('%');
                        result.push(other);
                    }
                    // Dangling '%' at the end of the pattern string
                    None => {
                        result.push('%');
                        break; // End of pattern reached unexpectedly after %
                    }
                }
            } else {
                // Literal character, push directly to result
                result.push(c);
            }
        }
        result
    }

    // --- Parsing ---

    /// Parses a string into a `ParsiDate` using a specified format pattern.
    ///
    /// This function attempts to match the input string `s` against the `format` pattern.
    /// It requires an exact match, including separators and padding as specified.
    /// Whitespace in the format string matches literal whitespace in the input.
    ///
    /// # Supported Format Specifiers for Parsing:
    ///
    /// *   `%Y`: Parses a 4-digit Persian year.
    /// *   `%m`: Parses a 2-digit Persian month (01-12).
    /// *   `%d`: Parses a 2-digit Persian day (01-31).
    /// *   `%B`: Parses a full Persian month name (e.g., "مرداد", "اسفند"). Case-sensitive.
    /// *   `%%`: Matches a literal percent sign (`%`) in the input.
    ///
    /// # Arguments
    ///
    /// * `s`: The input string slice to parse.
    /// * `format`: The format string containing literal characters and supported specifiers.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::ParseError(kind))` where `kind` indicates the failure:
    /// * `ParseErrorKind::FormatMismatch`: Input string doesn't match the format structure,
    ///   separators, has trailing characters, or is shorter than expected.
    /// * `ParseErrorKind::InvalidNumber`: Failed to parse `%Y`, `%m`, or `%d` as a number,
    ///   or they didn't have the required number of digits (4 for `%Y`, 2 for `%m`/`%d`).
    /// * `ParseErrorKind::InvalidMonthName`: Failed to match a known Persian month name for `%B`.
    /// * `ParseErrorKind::UnsupportedSpecifier`: An unsupported specifier (like `%A` or `%j`)
    ///   was used in the `format` string.
    /// * `ParseErrorKind::InvalidDateValue`: The components were parsed successfully but formed
    ///   an invalid date (e.g., month 13, day 31 in Mehr, day 30 in Esfand of a common year).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, DateError, ParseErrorKind};
    ///
    /// // Simple parsing
    /// assert_eq!(ParsiDate::parse("1403/05/02", "%Y/%m/%d"), Ok(ParsiDate::new(1403, 5, 2).unwrap()));
    /// assert_eq!(ParsiDate::parse("1399-12-30", "%Y-%m-%d"), Ok(ParsiDate::new(1399, 12, 30).unwrap()));
    ///
    /// // Parsing with month name (%B requires exact match)
    /// assert_eq!(ParsiDate::parse("02 مرداد 1403", "%d %B %Y"), Ok(ParsiDate::new(1403, 5, 2).unwrap()));
    /// assert_eq!(ParsiDate::parse("10 دی 1400", "%d %B %Y"), Ok(ParsiDate::new(1400, 10, 10).unwrap()));
    ///
    /// // --- Error Cases ---
    /// // Wrong format (separator)
    /// assert_eq!(ParsiDate::parse("1403 05 02", "%Y/%m/%d"), Err(DateError::ParseError(ParseErrorKind::FormatMismatch)));
    /// // Invalid number (single digit day for %d)
    /// assert_eq!(ParsiDate::parse("1403/05/2", "%Y/%m/%d"), Err(DateError::ParseError(ParseErrorKind::InvalidNumber)));
    /// // Invalid month name
    /// assert_eq!(ParsiDate::parse("02 Tirr 1403", "%d %B %Y"), Err(DateError::ParseError(ParseErrorKind::InvalidMonthName)));
    /// // Invalid date value (Esfand 30 in common year)
    /// assert_eq!(ParsiDate::parse("1404/12/30", "%Y/%m/%d"), Err(DateError::ParseError(ParseErrorKind::InvalidDateValue)));
    /// // Unsupported specifier
    /// assert_eq!(ParsiDate::parse("Tuesday 1403", "%A %Y"), Err(DateError::ParseError(ParseErrorKind::UnsupportedSpecifier)));
    /// ```
    pub fn parse(s: &str, format: &str) -> Result<Self, DateError> {
        let mut parsed_year: Option<i32> = None;
        let mut parsed_month: Option<u32> = None;
        let mut parsed_day: Option<u32> = None;

        // Use byte slices for efficient processing, assuming ASCII for format specifiers and digits.
        // Input string `s` can contain UTF-8 (for %B), handled specifically.
        let mut s_bytes = s.as_bytes();
        let mut fmt_bytes = format.as_bytes();

        // Iterate through the format string
        while !fmt_bytes.is_empty() {
            // Check if current format char is '%'
            if fmt_bytes[0] == b'%' {
                // Check for specifier character after '%'
                if fmt_bytes.len() < 2 {
                    // Dangling '%' at end of format string
                    return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                }

                // Match the specifier byte
                match fmt_bytes[1] {
                    // Literal '%%'
                    b'%' => {
                        // Input must also start with '%'
                        if s_bytes.is_empty() || s_bytes[0] != b'%' {
                            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                        }
                        // Consume '%' from both input and format
                        s_bytes = &s_bytes[1..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    // Year '%Y' (expect 4 digits)
                    b'Y' => {
                        if s_bytes.len() < 4 || !s_bytes[0..4].iter().all(|b| b.is_ascii_digit()) {
                            // Not enough chars or not all digits
                            return Err(DateError::ParseError(ParseErrorKind::InvalidNumber));
                        }
                        // Parse the 4 digits as year (unsafe from_utf8_unchecked is safe due to ASCII digit check)
                        let year_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..4]) };
                        parsed_year = Some(year_str.parse().map_err(|_| {
                            // This parse should not fail if the digits were validated, but handle defensively.
                            DateError::ParseError(ParseErrorKind::InvalidNumber)
                        })?);
                        // Consume 4 digits from input and '%Y' from format
                        s_bytes = &s_bytes[4..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    // Month '%m' or Day '%d' (expect exactly 2 digits)
                    b'm' | b'd' => {
                        if s_bytes.len() < 2 || !s_bytes[0..2].iter().all(|b| b.is_ascii_digit()) {
                            // Not enough chars or not exactly 2 digits
                            return Err(DateError::ParseError(ParseErrorKind::InvalidNumber));
                        }
                        // Parse the 2 digits (unsafe from_utf8_unchecked is safe)
                        let num_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..2]) };
                        let val: u32 = num_str
                            .parse()
                            .map_err(|_| DateError::ParseError(ParseErrorKind::InvalidNumber))?;

                        // Store the value in the correct Option
                        if fmt_bytes[1] == b'm' {
                            parsed_month = Some(val);
                        } else {
                            parsed_day = Some(val);
                        }
                        // Consume 2 digits from input and '%m' or '%d' from format
                        s_bytes = &s_bytes[2..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    // Month Name '%B' (expects Persian name)
                    b'B' => {
                        // Consume '%B' from format first
                        fmt_bytes = &fmt_bytes[2..];
                        let mut found_month = false;
                        let mut best_match_len = 0; // Length of the matched month name in bytes
                        let mut matched_month_idx = 0; // 0-based index of the matched month

                        // Need to work with the original string slice for UTF-8 month names
                        let current_s = unsafe { std::str::from_utf8_unchecked(s_bytes) };

                        // Iterate through known Persian month names
                        for (idx, month_name) in MONTH_NAMES_PERSIAN.iter().enumerate() {
                            if current_s.starts_with(month_name) {
                                let name_bytes_len = month_name.as_bytes().len();
                                // Found a potential match. Check if it's the longest one so far.
                                // This helps disambiguate if names had shared prefixes (not the case here, but robust).
                                if name_bytes_len > best_match_len {
                                    // Simple heuristic: Check if the character *after* the matched name in the input
                                    // matches the *next* character in the format string (often a separator),
                                    // or if we are at the end of either input or format.
                                    let next_s_char_opt =
                                        current_s.chars().nth(month_name.chars().count()); // Get char after name
                                    let next_fmt_byte_opt = fmt_bytes.get(0); // Get next byte in format

                                    let separator_match = match (next_s_char_opt, next_fmt_byte_opt)
                                    {
                                        (Some(sc), Some(&fb)) => sc as u8 == fb, // Compare char and byte
                                        (None, None) => true, // Both input and format end after month name
                                        (Some(_), None) => true, // Input continues, format ends (allow trailing chars?) - current logic requires exact match
                                        (None, Some(_)) => true, // Input ends, format continues (e.g. "%B%Y") - let next loop handle fmt
                                    };

                                    // This check is basic. A more robust parser might handle whitespace flexibility.
                                    // For now, it requires the character immediately following the month name
                                    // to match the next format character, or for one/both to end.

                                    // Store this match as the current best.
                                    best_match_len = name_bytes_len;
                                    matched_month_idx = idx;
                                    found_month = true;
                                    // Continue checking in case a longer name matches later (e.g., hypothetical "FarvardinLong" vs "Farvardin")
                                }
                            }
                        }

                        // If no month name was matched at the current position
                        if !found_month {
                            return Err(DateError::ParseError(ParseErrorKind::InvalidMonthName));
                        }

                        // Consume the matched month name (best_match_len bytes) from input `s_bytes`.
                        parsed_month = Some((matched_month_idx + 1) as u32); // Store 1-based month index
                        s_bytes = &s_bytes[best_match_len..];
                        // `fmt_bytes` was already advanced past '%B' above.
                    }
                    // Any other specifier after '%' is unsupported for parsing
                    _ => return Err(DateError::ParseError(ParseErrorKind::UnsupportedSpecifier)),
                }
            } else {
                // Literal character in format string
                // Input must match this literal character
                if s_bytes.is_empty() || s_bytes[0] != fmt_bytes[0] {
                    // Mismatch between input and literal format character
                    return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                }
                // Consume the matched literal character from both input and format
                s_bytes = &s_bytes[1..];
                fmt_bytes = &fmt_bytes[1..];
            }
        } // End while loop through format string

        // After processing the entire format string, check if there's any remaining input.
        // If yes, the input string was longer than the format expected.
        if !s_bytes.is_empty() {
            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
        }

        // Check if all required components (year, month, day) were successfully parsed
        match (parsed_year, parsed_month, parsed_day) {
            (Some(y), Some(m), Some(d)) => {
                // All components parsed, now validate the resulting date logically.
                // Use ParsiDate::new() for this final validation step.
                ParsiDate::new(y, m, d).map_err(|_| {
                    // If ParsiDate::new fails, it means the parsed values were logically invalid.
                    DateError::ParseError(ParseErrorKind::InvalidDateValue)
                })
            }
            // If any component is missing, the input string didn't fully match the format.
            _ => Err(DateError::ParseError(ParseErrorKind::FormatMismatch)),
        }
    }

    // --- Date Information ---

    /// Returns the Persian name of the weekday for this date (e.g., "شنبه", "یکشنبه").
    ///
    /// Calculates the weekday based on the Gregorian equivalent date.
    /// Saturday is considered the first day of the week in the Persian calendar.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the `ParsiDate` instance holds invalid data.
    /// Returns `Err(DateError::GregorianConversionError)` if the conversion to Gregorian fails
    /// during the calculation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// // 2024-07-23 is a Tuesday (سه‌شنبه) -> 1403-05-02
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// assert_eq!(date.weekday(), Ok("سه‌شنبه".to_string()));
    ///
    /// // 2024-03-23 is a Saturday (شنبه) -> 1403-01-04
    /// let date_sat = ParsiDate::new(1403, 1, 4).unwrap();
    /// assert_eq!(date_sat.weekday(), Ok("شنبه".to_string()));
    /// ```
    pub fn weekday(&self) -> Result<String, DateError> {
        self.weekday_internal() // Call the internal implementation
    }

    /// Internal helper for weekday calculation, returns Result.
    /// Assumes self might be invalid, performs check.
    fn weekday_internal(&self) -> Result<String, DateError> {
        // Ensure the date is valid before proceeding.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Get the numerical weekday (Sat=0..Fri=6).
        let day_num_sat_0 = self.weekday_num_sat_0()?;
        // Get the corresponding name from the constant array.
        // The index should be valid (0-6) if weekday_num_sat_0 is correct.
        WEEKDAY_NAMES_PERSIAN
            .get(day_num_sat_0 as usize)
            .map(|s| s.to_string())
            // If get fails (shouldn't happen), map it to a conversion error.
            .ok_or(DateError::GregorianConversionError)
    }

    /// Returns the weekday as a number, where Saturday=0, Sunday=1, ..., Friday=6.
    /// Internal helper function.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if `self` is invalid.
    /// Returns `Err(DateError::GregorianConversionError)` if `to_gregorian_internal` fails.
    fn weekday_num_sat_0(&self) -> Result<u32, DateError> {
        // Ensure the date is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Convert to Gregorian to use chrono's weekday calculation. Use internal to avoid double validation.
        let gregorian_date = self.to_gregorian_internal()?;

        // chrono::Weekday provides Sunday=0, ..., Saturday=6 via num_days_from_sunday().
        let day_num_sun0 = gregorian_date.weekday().num_days_from_sunday(); // 0=Sun, 1=Mon, ..., 6=Sat

        // Map Sunday=0..Saturday=6 to Persian Saturday=0..Friday=6.
        // Sun (0) -> Ekshanbe (1)  => (0 + 1) % 7 = 1
        // Mon (1) -> Doshanbe (2)  => (1 + 1) % 7 = 2
        // Tue (2) -> Seshanbe (3)  => (2 + 1) % 7 = 3
        // Wed (3) -> Chaharshanbe(4)=> (3 + 1) % 7 = 4
        // Thu (4) -> Panjshanbe(5) => (4 + 1) % 7 = 5
        // Fri (5) -> Jomeh (6)     => (5 + 1) % 7 = 6
        // Sat (6) -> Shanbeh (0)   => (6 + 1) % 7 = 0
        let day_num_sat0 = (day_num_sun0 + 1) % 7;

        Ok(day_num_sat0)
    }

    /// Calculates the day number within the year (ordinal day).
    ///
    /// Farvardin 1st is day 1. The result ranges from 1 to 365 (common year)
    /// or 1 to 366 (leap year).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the `ParsiDate` instance holds invalid data.
    /// Returns `Err(DateError::ArithmeticOverflow)` if an internal overflow occurs during summation
    /// (very unlikely with u32 for days in a year).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// assert_eq!(ParsiDate::new(1403, 1, 1).unwrap().ordinal(), Ok(1));
    /// assert_eq!(ParsiDate::new(1403, 2, 1).unwrap().ordinal(), Ok(32)); // After 31 days in Farvardin
    /// assert_eq!(ParsiDate::new(1403, 12, 30).unwrap().ordinal(), Ok(366)); // Last day of leap year
    /// assert_eq!(ParsiDate::new(1404, 12, 29).unwrap().ordinal(), Ok(365)); // Last day of common year
    /// ```
    pub fn ordinal(&self) -> Result<u32, DateError> {
        self.ordinal_internal() // Call the internal implementation
    }

    /// Internal helper for ordinal calculation, returns Result.
    /// Assumes self might be invalid, performs check.
    fn ordinal_internal(&self) -> Result<u32, DateError> {
        // Ensure the date is valid before calculating.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }

        // Get lengths of months for the current year.
        let month_lengths = Self::month_lengths(self.year);
        let mut days: u32 = 0;

        // Sum the lengths of all full months preceding the current month.
        // month is 1-based, loop goes from 0 up to month-2 (inclusive index).
        if self.month > 1 {
            let mut current_sum: u32 = 0;
            // Slice includes months from index 0 up to self.month - 2.
            for m_len in &month_lengths[0..(self.month - 1) as usize] {
                // Use checked_add for safety against potential overflow (unlikely here).
                current_sum = current_sum
                    .checked_add(*m_len)
                    .ok_or(DateError::ArithmeticOverflow)?;
            }
            days = current_sum;
        }

        // Add the day of the current month.
        // day is 1-based, so this gives the correct total ordinal day.
        days = days
            .checked_add(self.day)
            .ok_or(DateError::ArithmeticOverflow)?; // Safety check

        // Result should always be >= 1 since self.day >= 1.
        Ok(days)
    }

    // --- Arithmetic ---

    /// Adds a specified number of days to the date. Handles positive and negative `days`.
    ///
    /// This operation is performed by converting the `ParsiDate` to its Gregorian equivalent,
    /// adding the days using `chrono`, and then converting back to `ParsiDate`.
    ///
    /// # Arguments
    ///
    /// * `days`: The number of days to add. Can be positive to move forward in time,
    ///           or negative to move backward.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid.
    /// Returns `Err(DateError::ArithmeticOverflow)` if the addition/subtraction results in a
    /// Gregorian date outside `chrono`'s representable range, or if the resulting date, when
    /// converted back to Persian, falls outside the supported year range (1-9999).
    /// Returns `Err(DateError::GregorianConversionError)` if the initial conversion to Gregorian
    /// or the final conversion back to Persian fails for reasons other than overflow (e.g., epoch issues).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 12, 28).unwrap(); // Leap year
    /// assert_eq!(date.add_days(1), Ok(ParsiDate::new(1403, 12, 29).unwrap()));
    /// assert_eq!(date.add_days(2), Ok(ParsiDate::new(1403, 12, 30).unwrap()));
    /// assert_eq!(date.add_days(3), Ok(ParsiDate::new(1404, 1, 1).unwrap())); // Cross year boundary
    ///
    /// let date2 = ParsiDate::new(1404, 1, 1).unwrap();
    /// assert_eq!(date2.add_days(-1), Ok(ParsiDate::new(1403, 12, 30).unwrap())); // Subtract day
    /// assert_eq!(date2.add_days(-366), Ok(ParsiDate::new(1403, 1, 1).unwrap())); // Subtract leap year days
    /// ```
    pub fn add_days(&self, days: i64) -> Result<Self, DateError> {
        // Ensure the starting date is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }

        // Convert to Gregorian to perform arithmetic.
        let gregorian_equiv = self.to_gregorian_internal()?;

        // Use chrono's checked arithmetic for adding/subtracting days.
        let new_gregorian = if days >= 0 {
            // Add positive number of days.
            gregorian_equiv.checked_add_days(chrono::Days::new(days as u64))
        } else {
            // Subtract days (add negative). Convert negative i64 to positive u64 for subtraction.
            // Use checked_abs to handle potential i64::MIN overflow if needed, although days=i64::MIN is extreme.
            let days_to_sub = days.checked_abs().ok_or(DateError::ArithmeticOverflow)? as u64;
            gregorian_equiv.checked_sub_days(chrono::Days::new(days_to_sub))
        }
        .ok_or(DateError::ArithmeticOverflow)?; // Map chrono's None result (overflow/invalid) to our error type.

        // Convert the resulting Gregorian date back to ParsiDate.
        // This also handles checks for the supported Persian year range.
        Self::from_gregorian(new_gregorian)
    }

    /// Subtracts a specified number of days from the date.
    ///
    /// Equivalent to `add_days(-days)`. `days` must be non-negative.
    ///
    /// # Arguments
    ///
    /// * `days`: The non-negative number of days to subtract.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid.
    /// Returns `Err(DateError::ArithmeticOverflow)` if `days` is too large to be represented
    /// as a negative `i64`, or if the subtraction results in a date outside the representable range.
    /// Returns `Err(DateError::GregorianConversionError)` if conversion issues occur.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1404, 1, 3).unwrap();
    /// assert_eq!(date.sub_days(1), Ok(ParsiDate::new(1404, 1, 2).unwrap()));
    /// assert_eq!(date.sub_days(2), Ok(ParsiDate::new(1404, 1, 1).unwrap()));
    /// assert_eq!(date.sub_days(3), Ok(ParsiDate::new(1403, 12, 30).unwrap())); // Cross year boundary (1403 leap)
    /// ```
    pub fn sub_days(&self, days: u64) -> Result<Self, DateError> {
        // Convert u64 to negative i64 for add_days.
        // Check if the u64 value fits within the positive range of i64 before negating.
        // Negating i64::MIN is undefined behavior, but u64 can represent i64::MAX + 1 up to u64::MAX.
        if days > i64::MAX as u64 {
            // A number of days larger than i64::MAX is practically astronomical and likely leads to overflow anyway.
            return Err(DateError::ArithmeticOverflow);
        }
        // Safely negate the value (which is now known to be <= i64::MAX).
        let days_neg = -(days as i64);
        // Call add_days with the negative value.
        self.add_days(days_neg)
    }

    /// Adds a specified number of months to the date. Handles positive and negative `months_to_add`.
    ///
    /// If the resulting month has fewer days than the original day component,
    /// the day is clamped to the last day of the target month.
    ///
    /// # Arguments
    ///
    /// * `months_to_add`: The number of months to add. Can be positive or negative.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid.
    /// Returns `Err(DateError::ArithmeticOverflow)` if the calculation results in a year
    /// outside the supported range (1-9999) or causes internal integer overflow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 1, 31).unwrap(); // End of Farvardin (31 days)
    /// // Add 1 month -> Ordibehesht (31 days) -> 1403/02/31
    /// assert_eq!(date.add_months(1), Ok(ParsiDate::new(1403, 2, 31).unwrap()));
    /// // Add 6 months -> Mehr (30 days) -> Day clamped from 31 to 30 -> 1403/07/30
    /// assert_eq!(date.add_months(6), Ok(ParsiDate::new(1403, 7, 30).unwrap()));
    /// // Add 12 months -> Farvardin next year -> 1404/01/31
    /// assert_eq!(date.add_months(12), Ok(ParsiDate::new(1404, 1, 31).unwrap()));
    ///
    /// let date2 = ParsiDate::new(1404, 1, 1).unwrap();
    /// // Subtract 1 month -> Esfand previous year (1403 is leap, 30 days) -> 1403/12/01
    /// assert_eq!(date2.add_months(-1), Ok(ParsiDate::new(1403, 12, 1).unwrap()));
    ///
    /// let date3 = ParsiDate::new(1403, 12, 30).unwrap(); // End of Esfand (leap)
    /// // Subtract 1 month -> Bahman (30 days) -> Day remains 30 -> 1403/11/30
    /// assert_eq!(date3.sub_months(1), Ok(ParsiDate::new(1403, 11, 30).unwrap()));
    /// ```
    pub fn add_months(&self, months_to_add: i32) -> Result<Self, DateError> {
        // Ensure the starting date is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // If adding zero months, return the original date.
        if months_to_add == 0 {
            return Ok(*self);
        }

        // Calculate the total number of months from the start of the era (or a relative baseline).
        // Work with 0-indexed months (0=Farvardin..11=Esfand) for easier modulo arithmetic.
        let current_year = self.year;
        let current_month0 = self.month as i32 - 1; // 0..11

        // Calculate the total months as an offset from the beginning of year 0 conceptually.
        // total_months0 = (current_year * 12 + current_month0) + months_to_add;
        // To avoid large intermediate numbers and potential overflow with year * 12,
        // calculate the target month index and year delta separately.

        // Calculate the absolute month index if we flattened the calendar from year 0.
        let total_months_abs =
            (current_year as i64 * 12) + current_month0 as i64 + months_to_add as i64;
        // Check if this absolute month count could lead to year overflow (e.g., year > 9999 or < 1).
        // Target Year = floor(total_months_abs / 12)
        // Target Month Index = total_months_abs % 12
        let target_year_abs = total_months_abs.div_euclid(12);
        let target_month0 = total_months_abs.rem_euclid(12); // Resulting month index (0..11)

        // Check if target year is within our i32 and supported range.
        if target_year_abs < MIN_PARSI_DATE.year as i64
            || target_year_abs > MAX_PARSI_DATE.year as i64
        {
            return Err(DateError::ArithmeticOverflow);
        }
        let target_year = target_year_abs as i32;
        let target_month = (target_month0 + 1) as u32; // Convert back to 1-based month (1..12)

        // Determine the maximum valid day in the target month and year.
        let max_days_in_target_month = Self::days_in_month(target_year, target_month);
        // This check should be redundant if target_month calculation is correct (always 1-12).
        if max_days_in_target_month == 0 {
            return Err(DateError::InvalidDate); // Should not happen
        }

        // Clamp the day: use the original day or the max valid day, whichever is smaller.
        let target_day = self.day.min(max_days_in_target_month);

        // Use new() for final validation (primarily year range, month/day should be valid by logic).
        ParsiDate::new(target_year, target_month, target_day)
    }

    /// Subtracts a specified number of months from the date.
    ///
    /// Equivalent to `add_months(-months)`. `months_to_sub` must be non-negative.
    /// Clamps day if necessary.
    ///
    /// # Arguments
    ///
    /// * `months_to_sub`: The non-negative number of months to subtract.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid.
    /// Returns `Err(DateError::ArithmeticOverflow)` if `months_to_sub` is too large (exceeds `i32::MAX`)
    /// or if the calculation results in a year outside the supported range (1-9999).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 7, 30).unwrap(); // End of Mehr (30 days)
    /// // Subtract 1 month -> Shahrivar (31 days) -> Day remains 30 -> 1403/06/30
    /// assert_eq!(date.sub_months(1), Ok(ParsiDate::new(1403, 6, 30).unwrap()));
    ///
    /// let date2 = ParsiDate::new(1404, 2, 29).unwrap(); // Ordibehesht (31 days) in common year
    /// // Subtract 2 months -> Esfand previous year (1403 is leap, 30 days) -> Day remains 29 -> 1403/12/29
    /// assert_eq!(date2.sub_months(2), Ok(ParsiDate::new(1403, 12, 29).unwrap()));
    /// ```
    pub fn sub_months(&self, months_to_sub: u32) -> Result<Self, DateError> {
        // Check for potential overflow before negation: u32 max > i32 max.
        if months_to_sub > i32::MAX as u32 {
            return Err(DateError::ArithmeticOverflow);
        }
        // Negate and call add_months.
        self.add_months(-(months_to_sub as i32))
    }

    /// Adds a specified number of years to the date. Handles positive and negative `years_to_add`.
    ///
    /// Special handling for leap day: If the original date is Esfand 30th (a leap day),
    /// and the target year is not a leap year, the resulting date will be clamped to Esfand 29th.
    ///
    /// # Arguments
    ///
    /// * `years_to_add`: The number of years to add. Can be positive or negative.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid.
    /// Returns `Err(DateError::ArithmeticOverflow)` if the calculation results in a year
    /// outside the supported range (1-9999) or causes internal integer overflow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1400, 5, 10).unwrap();
    /// assert_eq!(date.add_years(3), Ok(ParsiDate::new(1403, 5, 10).unwrap())); // 1400 common -> 1403 leap
    /// assert_eq!(date.add_years(-1), Ok(ParsiDate::new(1399, 5, 10).unwrap())); // 1400 common -> 1399 leap
    ///
    /// let leap_day = ParsiDate::new(1403, 12, 30).unwrap(); // Esfand 30 on leap year
    /// // Add 1 year -> 1404 (common year) -> Day clamped to 29 -> 1404/12/29
    /// assert_eq!(leap_day.add_years(1), Ok(ParsiDate::new(1404, 12, 29).unwrap()));
    /// // Add 5 years -> 1408 (leap year) -> Day remains 30 -> 1408/12/30
    /// assert_eq!(leap_day.add_years(5), Ok(ParsiDate::new(1408, 12, 30).unwrap()));
    ///
    /// // Subtract 1 year from leap day -> 1402 (common year) -> Day clamped to 29 -> 1402/12/29
    /// assert_eq!(leap_day.sub_years(1), Ok(ParsiDate::new(1402, 12, 29).unwrap()));
    /// ```
    pub fn add_years(&self, years_to_add: i32) -> Result<Self, DateError> {
        // Ensure the starting date is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // If adding zero years, return the original date.
        if years_to_add == 0 {
            return Ok(*self);
        }

        // Calculate the target year using checked addition.
        let target_year = self
            .year
            .checked_add(years_to_add)
            .ok_or(DateError::ArithmeticOverflow)?;

        // Validate the target year is within the supported range.
        if target_year < MIN_PARSI_DATE.year || target_year > MAX_PARSI_DATE.year {
            return Err(DateError::ArithmeticOverflow); // Year out of range [1, 9999]
        }

        // Handle the leap day adjustment:
        // If the original date is Esfand 30 (only possible in a leap year),
        // and the target year is *not* a leap year, clamp the day to 29.
        let mut target_day = self.day;
        if self.month == 12 && self.day == 30 && !Self::is_persian_leap_year(target_year) {
            target_day = 29;
        }

        // Use new() for final validation. Month remains the same. Day might be adjusted.
        // new() will ensure the adjusted day (29) is valid for Esfand in the target year.
        ParsiDate::new(target_year, self.month, target_day)
    }

    /// Subtracts a specified number of years from the date.
    ///
    /// Equivalent to `add_years(-years)`. `years_to_sub` must be non-negative.
    /// Adjusts day for leap day (Esfand 30th) if necessary.
    ///
    /// # Arguments
    ///
    /// * `years_to_sub`: The non-negative number of years to subtract.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid.
    /// Returns `Err(DateError::ArithmeticOverflow)` if `years_to_sub` is too large (exceeds `i32::MAX`)
    /// or if the calculation results in a year outside the supported range (1-9999).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 10).unwrap(); // Leap year
    /// assert_eq!(date.sub_years(1), Ok(ParsiDate::new(1402, 5, 10).unwrap())); // To common year
    /// assert_eq!(date.sub_years(4), Ok(ParsiDate::new(1399, 5, 10).unwrap())); // To leap year
    ///
    /// let leap_day = ParsiDate::new(1403, 12, 30).unwrap();
    /// // Subtract 1 year -> 1402 (common) -> Clamp day to 29 -> 1402/12/29
    /// assert_eq!(leap_day.sub_years(1), Ok(ParsiDate::new(1402, 12, 29).unwrap()));
    /// ```
    pub fn sub_years(&self, years_to_sub: u32) -> Result<Self, DateError> {
        // Check for potential overflow before negation.
        if years_to_sub > i32::MAX as u32 {
            return Err(DateError::ArithmeticOverflow);
        }
        // Negate and call add_years.
        self.add_years(-(years_to_sub as i32))
    }

    /// Calculates the absolute difference in days between this `ParsiDate` and another `ParsiDate`.
    ///
    /// This is done by converting both dates to their Gregorian equivalents and calculating
    /// the difference between them.
    ///
    /// # Arguments
    ///
    /// * `other`: The other `ParsiDate` to compare against.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if either `self` or `other` is invalid.
    /// Returns `Err(DateError::GregorianConversionError)` if the conversion of either date
    /// to Gregorian fails.
    ///
    /// # Returns
    ///
    /// The absolute number of days between the two dates as an `i64`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let d1 = ParsiDate::new(1403, 1, 1).unwrap();
    /// let d2 = ParsiDate::new(1403, 1, 11).unwrap();
    /// assert_eq!(d1.days_between(&d2), Ok(10));
    /// assert_eq!(d2.days_between(&d1), Ok(10)); // Absolute difference
    ///
    /// let d3 = ParsiDate::new(1404, 1, 1).unwrap(); // Next year (1403 is leap)
    /// assert_eq!(d1.days_between(&d3), Ok(366));
    /// ```
    pub fn days_between(&self, other: &ParsiDate) -> Result<i64, DateError> {
        // Ensure both dates are valid before proceeding.
        if !self.is_valid() || !other.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Convert both dates to Gregorian.
        let g1 = self.to_gregorian_internal()?; // Use internal conversion as validity is checked.
        let g2 = other.to_gregorian_internal()?;
        // Calculate the signed duration using chrono and return the absolute number of days.
        Ok(g1.signed_duration_since(g2).num_days().abs())
    }

    // --- Helper Methods ---

    /// Creates a new `ParsiDate` with the year component modified.
    ///
    /// Adjusts the day to 29 if the original date was Esfand 30th (leap day)
    /// and the target `year` is not a leap year.
    ///
    /// # Arguments
    ///
    /// * `year`: The desired year for the new date (1-9999).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid,
    /// or if the target `year` is outside the supported range (1-9999), or if the
    /// resulting date (after potential day clamping) is somehow invalid (should not happen
    /// if target year is valid).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// assert_eq!(date.with_year(1404), Ok(ParsiDate::new(1404, 5, 2).unwrap()));
    ///
    /// let leap_day = ParsiDate::new(1403, 12, 30).unwrap();
    /// assert_eq!(leap_day.with_year(1404), Ok(ParsiDate::new(1404, 12, 29).unwrap())); // Day clamped
    /// assert_eq!(leap_day.with_year(1408), Ok(ParsiDate::new(1408, 12, 30).unwrap())); // Leap to leap
    ///
    /// assert!(date.with_year(0).is_err()); // Invalid target year
    /// ```
    pub fn with_year(&self, year: i32) -> Result<Self, DateError> {
        // Ensure the starting date is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Validate the target year range immediately.
        if year < MIN_PARSI_DATE.year || year > MAX_PARSI_DATE.year {
            // Using InvalidDate error for out-of-range years for simplicity.
            return Err(DateError::InvalidDate);
        }

        // Check if leap day adjustment is needed.
        let mut day = self.day;
        if self.month == 12 && self.day == 30 && !Self::is_persian_leap_year(year) {
            // Original is Esfand 30 (must be leap), target year is not leap. Clamp day.
            day = 29;
        }

        // Use new() for final validation. It ensures the combination is valid.
        ParsiDate::new(year, self.month, day)
    }

    /// Creates a new `ParsiDate` with the month component modified.
    ///
    /// If the original day component is invalid for the target `month` in the same year
    /// (e.g., changing from Farvardin 31st to Mehr), the day is clamped to the
    /// last valid day of the target `month`.
    ///
    /// # Arguments
    ///
    /// * `month`: The desired month for the new date (1-12).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid,
    /// or if the target `month` is outside the range (1-12).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 1, 31).unwrap(); // Farvardin 31
    /// assert_eq!(date.with_month(2), Ok(ParsiDate::new(1403, 2, 31).unwrap())); // To Ordibehesht (31 days)
    /// assert_eq!(date.with_month(7), Ok(ParsiDate::new(1403, 7, 30).unwrap())); // To Mehr (30 days, clamped)
    ///
    /// let date2 = ParsiDate::new(1404, 7, 15).unwrap(); // Mehr 15 (common year)
    /// assert_eq!(date2.with_month(12), Ok(ParsiDate::new(1404, 12, 15).unwrap())); // To Esfand (29 days)
    ///
    /// assert!(date.with_month(0).is_err()); // Invalid target month
    /// assert!(date.with_month(13).is_err());
    /// ```
    pub fn with_month(&self, month: u32) -> Result<Self, DateError> {
        // Ensure the starting date is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Validate the target month range immediately.
        if month == 0 || month > 12 {
            return Err(DateError::InvalidDate); // Invalid target month number
        }

        // Determine the maximum valid day for the target month in the current year.
        let max_days = Self::days_in_month(self.year, month);
        // This check should be redundant if month is 1-12, as days_in_month returns > 0.
        if max_days == 0 {
            return Err(DateError::InvalidDate); // Should not happen
        }

        // Clamp the original day to the maximum allowed day of the target month.
        let day = self.day.min(max_days);

        // Use new() for final validation. Year remains the same.
        ParsiDate::new(self.year, month, day)
    }

    /// Creates a new `ParsiDate` with the day component modified.
    ///
    /// # Arguments
    ///
    /// * `day`: The desired day for the new date (1-31).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid,
    /// or if the target `day` is 0 or greater than the number of days in the
    /// current month and year.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// let date = ParsiDate::new(1403, 7, 1).unwrap(); // Mehr 1st (30 days)
    /// assert_eq!(date.with_day(15), Ok(ParsiDate::new(1403, 7, 15).unwrap()));
    /// assert_eq!(date.with_day(30), Ok(ParsiDate::new(1403, 7, 30).unwrap()));
    /// assert_eq!(date.with_day(31), Err(DateError::InvalidDate)); // Invalid day for Mehr
    /// assert_eq!(date.with_day(0), Err(DateError::InvalidDate)); // Day 0 is invalid
    /// ```
    pub fn with_day(&self, day: u32) -> Result<Self, DateError> {
        // Ensure the starting date is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Basic check for day > 0. The upper bound check is handled by ParsiDate::new.
        if day == 0 {
            return Err(DateError::InvalidDate);
        }

        // Let ParsiDate::new perform the full validation (checks day <= days_in_month).
        ParsiDate::new(self.year, self.month, day)
    }

    /// Returns the date of the first day of the month for the current date.
    ///
    /// Creates a new `ParsiDate` with the same year and month, but with the day set to 1.
    /// Assumes `self` is already a valid date.
    ///
    /// # Safety
    ///
    /// This method uses `unsafe { ParsiDate::new_unchecked }` internally for performance,
    /// relying on the assumption that `self` is valid. Day 1 is always valid for any valid
    /// month (1-12) and year (1-9999). A `debug_assert!` is included.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 15).unwrap();
    /// assert_eq!(date.first_day_of_month(), ParsiDate::new(1403, 5, 1).unwrap());
    /// ```
    #[inline]
    pub fn first_day_of_month(&self) -> Self {
        // Ensure self is valid in debug builds.
        debug_assert!(self.is_valid(), "first_day_of_month called on invalid date");
        // Safety: Day 1 is always valid for the (assumed valid) self.month and self.year.
        unsafe { ParsiDate::new_unchecked(self.year, self.month, 1) }
    }

    /// Returns the date of the last day of the month for the current date.
    ///
    /// Calculates the last day based on the month and whether the year is a leap year.
    /// Assumes `self` is already a valid date.
    ///
    /// # Safety
    ///
    /// This method uses `unsafe { ParsiDate::new_unchecked }` internally for performance.
    /// It relies on `days_in_month` returning the correct last day for the valid `self.year`
    /// and `self.month`. A `debug_assert!` is included.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 15).unwrap(); // Mordad (31 days)
    /// assert_eq!(date.last_day_of_month(), ParsiDate::new(1403, 5, 31).unwrap());
    ///
    /// let date_mehr = ParsiDate::new(1403, 7, 10).unwrap(); // Mehr (30 days)
    /// assert_eq!(date_mehr.last_day_of_month(), ParsiDate::new(1403, 7, 30).unwrap());
    ///
    /// let date_esfand_leap = ParsiDate::new(1403, 12, 5).unwrap(); // Esfand (leap year, 30 days)
    /// assert_eq!(date_esfand_leap.last_day_of_month(), ParsiDate::new(1403, 12, 30).unwrap());
    ///
    /// let date_esfand_common = ParsiDate::new(1404, 12, 5).unwrap(); // Esfand (common year, 29 days)
    /// assert_eq!(date_esfand_common.last_day_of_month(), ParsiDate::new(1404, 12, 29).unwrap());
    /// ```
    #[inline]
    pub fn last_day_of_month(&self) -> Self {
        // Ensure self is valid in debug builds.
        debug_assert!(self.is_valid(), "last_day_of_month called on invalid date");
        // Calculate the last day of the current month/year.
        let last_day = Self::days_in_month(self.year, self.month);
        // Safety: days_in_month returns a valid day number (29, 30, or 31) for the valid self.month/self.year.
        unsafe { ParsiDate::new_unchecked(self.year, self.month, last_day) }
    }

    /// Returns the date of the first day of the year (Farvardin 1st) for the current date's year.
    ///
    /// Creates a new `ParsiDate` with the same year, but month set to 1 and day set to 1.
    /// Assumes `self` is already a valid date.
    ///
    /// # Safety
    ///
    /// Uses `unsafe { ParsiDate::new_unchecked }`. Relies on the assumption that `self` is valid,
    /// meaning `self.year` is valid. Month 1, Day 1 is always valid for any valid year.
    /// A `debug_assert!` is included.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 15).unwrap();
    /// assert_eq!(date.first_day_of_year(), ParsiDate::new(1403, 1, 1).unwrap());
    /// ```
    #[inline]
    pub fn first_day_of_year(&self) -> Self {
        // Ensure self is valid in debug builds.
        debug_assert!(self.is_valid(), "first_day_of_year called on invalid date");
        // Safety: Month 1, Day 1 is always valid for the (assumed valid) self.year.
        unsafe { ParsiDate::new_unchecked(self.year, 1, 1) }
    }

    /// Returns the date of the last day of the year (Esfand 29th or 30th) for the current date's year.
    ///
    /// Calculates the last day (29 or 30) based on whether the year is a leap year.
    /// Assumes `self` is already a valid date.
    ///
    /// # Safety
    ///
    /// Uses `unsafe { ParsiDate::new_unchecked }`. Relies on `is_persian_leap_year` correctly
    /// determining the last day (29 or 30), which is always valid for month 12 (Esfand)
    /// in the valid `self.year`. A `debug_assert!` is included.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// // Leap year
    /// let date_leap = ParsiDate::new(1403, 5, 15).unwrap();
    /// assert_eq!(date_leap.last_day_of_year(), ParsiDate::new(1403, 12, 30).unwrap());
    ///
    /// // Common year
    /// let date_common = ParsiDate::new(1404, 7, 10).unwrap();
    /// assert_eq!(date_common.last_day_of_year(), ParsiDate::new(1404, 12, 29).unwrap());
    /// ```
    #[inline]
    pub fn last_day_of_year(&self) -> Self {
        // Ensure self is valid in debug builds.
        debug_assert!(self.is_valid(), "last_day_of_year called on invalid date");
        // Determine the last day based on leap year status.
        let last_day = if Self::is_persian_leap_year(self.year) {
            30
        } else {
            29
        };
        // Safety: Month 12 is valid, and last_day (29 or 30) is the valid last day for month 12
        // in the (assumed valid) self.year.
        unsafe { ParsiDate::new_unchecked(self.year, 12, last_day) }
    }
} // end impl ParsiDate

// --- Trait Implementations ---

/// Implements the `Display` trait for `ParsiDate`.
///
/// Formats the date using the default "short" style: "YYYY/MM/DD".
///
/// Note: This formatting assumes the `ParsiDate` instance is valid. If an invalid date
/// (e.g., created via `unsafe`) is displayed, the output might show the invalid components
/// directly (e.g., "1404/12/30").
impl fmt::Display for ParsiDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the "short" format: YYYY/MM/DD with zero-padding for month and day.
        write!(f, "{}/{:02}/{:02}", self.year, self.month, self.day)
    }
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    // Helper function to create a ParsiDate for tests, panicking on failure.
    fn pd(year: i32, month: u32, day: u32) -> ParsiDate {
        ParsiDate::new(year, month, day)
            .unwrap_or_else(|e| panic!("Invalid test date {}-{}-{}: {:?}", year, month, day, e))
    }

    // --- Constructor & Validation Tests ---
    #[test]
    fn test_new_constructor() {
        assert_eq!(ParsiDate::new(1403, 5, 2), Ok(pd(1403, 5, 2)));
        assert_eq!(ParsiDate::new(1403, 12, 30), Ok(pd(1403, 12, 30))); // Leap year valid end
        assert_eq!(ParsiDate::new(1404, 12, 29), Ok(pd(1404, 12, 29))); // Common year valid end
        assert_eq!(
            ParsiDate::new(1404, 12, 30),
            Err(DateError::InvalidDate),
            "Esfand 30 invalid in common year 1404"
        );
        assert_eq!(
            ParsiDate::new(1403, 13, 1),
            Err(DateError::InvalidDate),
            "Month 13 invalid"
        );
        assert_eq!(
            ParsiDate::new(1403, 0, 1),
            Err(DateError::InvalidDate),
            "Month 0 invalid"
        );
        assert_eq!(
            ParsiDate::new(1403, 1, 0),
            Err(DateError::InvalidDate),
            "Day 0 invalid"
        );
        assert_eq!(
            ParsiDate::new(1403, 7, 31),
            Err(DateError::InvalidDate),
            "Day 31 invalid for Mehr (Month 7)"
        );
        // Test year bounds defined by MIN/MAX constants
        assert_eq!(
            ParsiDate::new(MIN_PARSI_DATE.year - 1, 1, 1),
            Err(DateError::InvalidDate),
            "Year 0 invalid"
        );
        assert_eq!(
            ParsiDate::new(MAX_PARSI_DATE.year + 1, 1, 1),
            Err(DateError::InvalidDate),
            "Year 10000 invalid"
        );
        assert!(ParsiDate::new(MIN_PARSI_DATE.year, 1, 1).is_ok());
        assert!(ParsiDate::new(MAX_PARSI_DATE.year, 12, 29).is_ok());
    }

    #[test]
    fn test_new_unchecked() {
        // Create a valid date using unsafe constructor
        let d = unsafe { ParsiDate::new_unchecked(1403, 5, 2) };
        assert!(d.is_valid());
        assert_eq!(d.year(), 1403);

        // Create a logically invalid date using unsafe constructor
        let invalid = unsafe { ParsiDate::new_unchecked(1404, 12, 30) }; // Esfand 30 in common year
        assert!(
            !invalid.is_valid(),
            "is_valid correctly identifies invalid date created with new_unchecked"
        );
        // Accessing fields still works, but operations might fail or give wrong results
        assert_eq!(invalid.year(), 1404);
        assert_eq!(invalid.month(), 12);
        assert_eq!(invalid.day(), 30);
    }

    #[test]
    fn test_from_ordinal() {
        // --- Valid cases ---
        assert_eq!(
            ParsiDate::from_ordinal(1403, 1),
            Ok(pd(1403, 1, 1)),
            "Ordinal 1 -> Farvardin 1"
        );
        assert_eq!(
            ParsiDate::from_ordinal(1403, 31),
            Ok(pd(1403, 1, 31)),
            "Ordinal 31 -> Farvardin 31"
        );
        assert_eq!(
            ParsiDate::from_ordinal(1403, 32),
            Ok(pd(1403, 2, 1)),
            "Ordinal 32 -> Ordibehesht 1"
        );
        assert_eq!(
            ParsiDate::from_ordinal(1403, 186),
            Ok(pd(1403, 6, 31)),
            "Ordinal 186 -> Shahrivar 31 (end of first 6 months)"
        );
        assert_eq!(
            ParsiDate::from_ordinal(1403, 187),
            Ok(pd(1403, 7, 1)),
            "Ordinal 187 -> Mehr 1"
        );
        assert_eq!(
            ParsiDate::from_ordinal(1403, 366),
            Ok(pd(1403, 12, 30)),
            "Ordinal 366 -> Last day of leap year 1403"
        );
        assert_eq!(
            ParsiDate::from_ordinal(1404, 365),
            Ok(pd(1404, 12, 29)),
            "Ordinal 365 -> Last day of common year 1404"
        );

        // --- Invalid cases ---
        assert_eq!(
            ParsiDate::from_ordinal(1403, 0),
            Err(DateError::InvalidOrdinal),
            "Ordinal 0 is invalid"
        );
        assert_eq!(
            ParsiDate::from_ordinal(1403, 367),
            Err(DateError::InvalidOrdinal),
            "Ordinal 367 invalid for leap year 1403"
        );
        assert_eq!(
            ParsiDate::from_ordinal(1404, 366),
            Err(DateError::InvalidOrdinal),
            "Ordinal 366 invalid for common year 1404"
        );
        // Test with invalid year (should be caught by the final `new` call)
        // assert_eq!(ParsiDate::from_ordinal(0, 100), Err(DateError::InvalidDate)); // Example check
    }

    // --- Conversion Tests ---
    #[test]
    fn test_gregorian_to_persian() {
        // Standard conversion
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(2024, 7, 23).unwrap()),
            Ok(pd(1403, 5, 2))
        );
        // Nowruz (Persian New Year)
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(2024, 3, 20).unwrap()),
            Ok(pd(1403, 1, 1)),
            "Nowruz 1403"
        );
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(2025, 3, 21).unwrap()),
            Ok(pd(1404, 1, 1)),
            "Nowruz 1404"
        );
        // Day before Nowruz
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(2024, 3, 19).unwrap()),
            Ok(pd(1402, 12, 29)), // 1402 was common year
            "Day before Nowruz 1403"
        );
        // Specific historical date
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(1979, 2, 11).unwrap()),
            Ok(pd(1357, 11, 22))
        );
        // Epoch start
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(622, 3, 21).unwrap()),
            Ok(pd(1, 1, 1)),
            "Persian epoch start"
        );
        // Before epoch
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(622, 3, 20).unwrap()),
            Err(DateError::GregorianConversionError),
            "Date before Persian epoch"
        );
        // Test around year boundary (end of a leap year 1403)
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(2025, 3, 20).unwrap()),
            Ok(pd(1403, 12, 30)),
            "Last day of Persian leap year 1403"
        );
        // Test around year boundary (end of a common year 1404)
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(2026, 3, 20).unwrap()),
            Ok(pd(1404, 12, 29)),
            "Last day of Persian common year 1404"
        );
        // Test a date far in the future
        assert_eq!(
            ParsiDate::from_gregorian(NaiveDate::from_ymd_opt(2622, 3, 21).unwrap()),
            Ok(pd(2001, 1, 1)), // Example calculation, needs verification if precise relation is needed
            "Future date conversion"
        );
    }

    #[test]
    fn test_persian_to_gregorian() {
        // Standard conversion
        assert_eq!(
            pd(1403, 5, 2).to_gregorian(),
            Ok(NaiveDate::from_ymd_opt(2024, 7, 23).unwrap())
        );
        // Nowruz
        assert_eq!(
            pd(1403, 1, 1).to_gregorian(),
            Ok(NaiveDate::from_ymd_opt(2024, 3, 20).unwrap())
        );
        assert_eq!(
            pd(1404, 1, 1).to_gregorian(),
            Ok(NaiveDate::from_ymd_opt(2025, 3, 21).unwrap())
        );
        // Last day of leap year
        assert_eq!(
            pd(1403, 12, 30).to_gregorian(),
            Ok(NaiveDate::from_ymd_opt(2025, 3, 20).unwrap())
        );
        // Last day of common year
        assert_eq!(
            pd(1404, 12, 29).to_gregorian(),
            Ok(NaiveDate::from_ymd_opt(2026, 3, 20).unwrap())
        );
        // Specific historical date
        assert_eq!(
            pd(1357, 11, 22).to_gregorian(),
            Ok(NaiveDate::from_ymd_opt(1979, 2, 11).unwrap())
        );
        // Epoch start
        assert_eq!(
            pd(1, 1, 1).to_gregorian(),
            Ok(NaiveDate::from_ymd_opt(622, 3, 21).unwrap())
        );
        // Test invalid date conversion attempt (created via unsafe)
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert!(!invalid_date.is_valid());
        // `to_gregorian` performs validation first.
        assert_eq!(invalid_date.to_gregorian(), Err(DateError::InvalidDate));
        // Test internal conversion directly (bypasses initial validation check)
        // This might succeed or fail depending on internal logic robustness,
        // but its behavior on invalid input isn't guaranteed. For safety, don't rely on it.
        // let internal_result = invalid_date.to_gregorian_internal();
        // println!("Internal conversion result for invalid date: {:?}", internal_result);
    }

    #[test]
    fn test_today_function() {
        // This test checks if `today()` runs successfully and returns a logically valid date
        // within the expected Persian year range based on the system clock at runtime.
        match ParsiDate::today() {
            Ok(today) => {
                // Print for info during test runs.
                println!(
                    "Today's Persian date (captured by test): {}",
                    today.format("long")
                );
                // Check if the returned date is valid according to library rules.
                assert!(
                    today.is_valid(),
                    "ParsiDate::today() returned an invalid date object: y={}, m={}, d={}",
                    today.year(),
                    today.month(),
                    today.day()
                );
                // Check if the year falls within the globally supported range.
                assert!(
                    today.year() >= MIN_PARSI_DATE.year() && today.year() <= MAX_PARSI_DATE.year(),
                    "Today's Persian year {} is outside the supported range [{}, {}]",
                    today.year(),
                    MIN_PARSI_DATE.year(),
                    MAX_PARSI_DATE.year()
                );
                // We could also convert back to Gregorian and check if it's close to chrono::Local::now().date_naive()
                // let today_gregorian_check = chrono::Local::now().date_naive();
                // assert_eq!(today.to_gregorian().unwrap(), today_gregorian_check);
            }
            Err(e) => {
                // This should only fail if the system clock is drastically wrong, leading to
                // a Gregorian date outside chrono's or this library's conversion range.
                panic!("ParsiDate::today() failed unexpectedly: {}", e);
            }
        }
    }

    // --- Leap Year & DaysInMonth Tests ---
    #[test]
    fn test_leap_years() {
        // Test cases based on the 33-year cycle rule: year % 33 in {1, 5, 9, 13, 17, 22, 26, 30}
        assert!(
            ParsiDate::is_persian_leap_year(1399),
            "1399 % 33 = 30 -> leap"
        );
        assert!(
            ParsiDate::is_persian_leap_year(1403),
            "1403 % 33 = 5 -> leap"
        );
        assert!(
            !ParsiDate::is_persian_leap_year(1404),
            "1404 % 33 = 6 -> common"
        );
        assert!(
            !ParsiDate::is_persian_leap_year(1407),
            "1407 % 33 = 9 -> common"
        ); // Corrected based on rule
        assert!(
            ParsiDate::is_persian_leap_year(1408),
            "1408 % 33 = 10 -> leap"
        ); // Corrected based on rule
        assert!(
            ParsiDate::is_persian_leap_year(1420),
            "1420 % 33 = 22 -> leap"
        );
        assert!(
            ParsiDate::is_persian_leap_year(1424),
            "1424 % 33 = 26 -> leap"
        );
        assert!(
            ParsiDate::is_persian_leap_year(1428),
            "1428 % 33 = 30 -> leap"
        );
        assert!(
            ParsiDate::is_persian_leap_year(1432),
            "1432 % 33 = 1 -> leap"
        ); // Cycle restart
        assert!(
            !ParsiDate::is_persian_leap_year(1400),
            "1400 % 33 = 1 -> common"
        ); // Corrected assertion
        assert!(
            !ParsiDate::is_persian_leap_year(9999),
            "9999 % 33 = 3 -> common (MAX_PARSI_DATE year)"
        );
        // Invalid years should return false
        assert!(!ParsiDate::is_persian_leap_year(0), "Year 0 is not leap");
        assert!(
            !ParsiDate::is_persian_leap_year(-10),
            "Negative year is not leap"
        );
    }

    #[test]
    fn test_days_in_month() {
        // Months 1-6 always have 31 days
        assert_eq!(ParsiDate::days_in_month(1403, 1), 31, "Farvardin");
        assert_eq!(ParsiDate::days_in_month(1404, 6), 31, "Shahrivar");
        // Months 7-11 always have 30 days
        assert_eq!(ParsiDate::days_in_month(1403, 7), 30, "Mehr");
        assert_eq!(ParsiDate::days_in_month(1404, 11), 30, "Bahman");
        // Month 12 (Esfand) depends on leap year
        assert_eq!(
            ParsiDate::days_in_month(1403, 12),
            30,
            "Esfand in leap year 1403"
        );
        assert_eq!(
            ParsiDate::days_in_month(1404, 12),
            29,
            "Esfand in common year 1404"
        );
        assert_eq!(
            ParsiDate::days_in_month(1408, 12),
            30,
            "Esfand in Leap year 1408"
        ); // Corrected based on leap year test

        // Test invalid month numbers should return 0
        assert_eq!(ParsiDate::days_in_month(1403, 0), 0, "Invalid month 0");
        assert_eq!(ParsiDate::days_in_month(1403, 13), 0, "Invalid month 13");
    }

    // --- Formatting Tests ---
    #[test]
    fn test_format_predefined() {
        let date = pd(1403, 5, 2);
        assert_eq!(date.format("short"), "1403/05/02");
        assert_eq!(date.format("long"), "2 مرداد 1403"); // Day not padded in "long"
        assert_eq!(date.format("iso"), "1403-05-02");
        // Test Display trait (should default to "short")
        assert_eq!(date.to_string(), "1403/05/02");

        // Test with single digit month/day to ensure padding in short/iso
        let date_single_digit = pd(1400, 1, 9);
        assert_eq!(date_single_digit.format("short"), "1400/01/09");
        assert_eq!(date_single_digit.format("long"), "9 فروردین 1400");
        assert_eq!(date_single_digit.format("iso"), "1400-01-09");
        assert_eq!(date_single_digit.to_string(), "1400/01/09");
    }

    #[test]
    fn test_format_strftime() {
        let date = pd(1403, 1, 7); // 1403-01-07 is a Tue/سه‌شنبه (Gregorian: 2024-03-26)
        let date_common_end = pd(1404, 12, 29); // 1404-12-29 is a Fri/جمعه (Gregorian: 2026-03-20)
        let date_leap_end = pd(1403, 12, 30); // 1403-12-30 is a Thu/پنجشنبه (Gregorian: 2025-03-20)
        let date_sat = pd(1403, 1, 4); // 1403-01-04 is a Sat/شنبه (Gregorian: 2024-03-23)
        let date_sun = pd(1403, 1, 5); // 1403-01-05 is a Sun/یکشنبه (Gregorian: 2024-03-24)

        // Basic specifiers (%Y, %m, %d, %B)
        assert_eq!(date.format("%Y/%m/%d"), "1403/01/07");
        assert_eq!(date.format("%d %B %Y"), "07 فروردین 1403"); // %d pads day
        assert_eq!(date_common_end.format("%Y/%m/%d"), "1404/12/29");
        assert_eq!(date_common_end.format("%d %B %Y"), "29 اسفند 1404");

        // Ordinal day (%j) - 3 digits zero-padded
        assert_eq!(date.format("Day %j of %Y"), "Day 007 of 1403");
        assert_eq!(
            date_common_end.format("Day %j"),
            "Day 365",
            "Last day of common year"
        );
        assert_eq!(date_leap_end.format("%j"), "366", "Last day of leap year");

        // Weekday (%A name, %w number Sat=0)
        assert_eq!(
            date_common_end.format("Weekday %A (num %w)"),
            "Weekday جمعه (num 6)"
        ); // Friday
        assert_eq!(date.format("%A"), "سه‌شنبه"); // Tuesday
        assert_eq!(date_sat.format("%A (%w)"), "شنبه (0)"); // Saturday
        assert_eq!(date_sun.format("%A (%w)"), "یکشنبه (1)"); // Sunday

        // Literal percent sign (%%)
        assert_eq!(date.format("%% %Y %%"), "% 1403 %");

        // Combined and complex patterns
        assert_eq!(date.format("%d-%B-%Y (%A)"), "07-فروردین-1403 (سه‌شنبه)");

        // Unknown specifier (%x) should be output literally
        assert_eq!(date.format("%Y-%m-%d %x %!"), "1403-01-07 %x %!");

        // Test formatting of potentially invalid date (via unsafe)
        let invalid_date = unsafe { ParsiDate::new_unchecked(1400, 13, 1) }; // Invalid month 13
                                                                             // Behavior here depends on implementation; robust formatting handles invalid components gracefully.
        assert!(
            invalid_date.format("%Y/%m/%d").contains("1400/13/01"),
            "Display might show raw invalid data"
        );
        assert!(
            invalid_date.format("%B").contains("?InvalidMonth?"),
            "Formatting %B for invalid month should indicate error"
        );
        // Weekday/Ordinal calculation on invalid date should indicate error
        assert!(
            invalid_date.format("%A").contains("?WeekdayError?"),
            "Formatting %A for invalid date should indicate error"
        );
        assert!(
            invalid_date.format("%j").contains("???"),
            "Formatting %j for invalid date should indicate error"
        );
    }

    // --- Parsing Tests ---
    #[test]
    fn test_parse_simple() {
        // Basic YMD formats with different separators
        assert_eq!(
            ParsiDate::parse("1403/05/02", "%Y/%m/%d"),
            Ok(pd(1403, 5, 2))
        );
        assert_eq!(
            ParsiDate::parse("1403-01-31", "%Y-%m-%d"),
            Ok(pd(1403, 1, 31))
        );
        // Different order of components
        assert_eq!(
            ParsiDate::parse("07/04/1399", "%d/%m/%Y"),
            Ok(pd(1399, 4, 7))
        );
        // Test parsing epoch start and max supported date
        assert_eq!(ParsiDate::parse("0001/01/01", "%Y/%m/%d"), Ok(pd(1, 1, 1)));
        assert_eq!(
            ParsiDate::parse("9999/12/29", "%Y/%m/%d"),
            Ok(pd(9999, 12, 29)),
            "Max date (9999 is common)"
        );
    }

    #[test]
    fn test_parse_month_name() {
        // %d requires padded day (2 digits)
        assert_eq!(
            ParsiDate::parse("02 مرداد 1403", "%d %B %Y"),
            Ok(pd(1403, 5, 2))
        );
        // End of leap year with month name
        assert_eq!(
            ParsiDate::parse("30 اسفند 1399", "%d %B %Y"),
            Ok(pd(1399, 12, 30)), // 1399 is leap
            "End of leap year with %B"
        );
        // End of common year with month name
        assert_eq!(
            ParsiDate::parse("29 اسفند 1404", "%d %B %Y"),
            Ok(pd(1404, 12, 29)), // 1404 is common
            "End of common year with %B"
        );
        // Test with exact single spaces as required by the current parser implementation
        assert_eq!(
            ParsiDate::parse("10 دی 1400", "%d %B %Y"),
            Ok(pd(1400, 10, 10))
        );
        // Test month name at different positions in format string
        assert_eq!(
            ParsiDate::parse("1400-دی-10", "%Y-%B-%d"),
            Ok(pd(1400, 10, 10))
        );
        assert_eq!(
            ParsiDate::parse("فروردین-01-1390", "%B-%d-%Y"),
            Ok(pd(1390, 1, 1))
        );
        // Test month name followed immediately by year
        assert_eq!(
            ParsiDate::parse("01اردیبهشت1395", "%d%B%Y"),
            Ok(pd(1395, 2, 1))
        );
    }

    #[test]
    fn test_parse_errors() {
        // --- Invalid Number Errors ---
        // %m and %d require exactly two digits
        assert_eq!(
            ParsiDate::parse("1403/5/02", "%Y/%m/%d").unwrap_err(),
            DateError::ParseError(ParseErrorKind::InvalidNumber),
            "Single digit month for %m"
        );
        assert_eq!(
            ParsiDate::parse("1403/05/2", "%Y/%m/%d").unwrap_err(),
            DateError::ParseError(ParseErrorKind::InvalidNumber),
            "Single digit day for %d"
        );
        // %Y requires exactly four digits
        assert_eq!(
            ParsiDate::parse("403/01/01", "%Y/%m/%d").unwrap_err(),
            DateError::ParseError(ParseErrorKind::InvalidNumber),
            "Three digit year for %Y"
        );
        // Non-digit characters where digits are expected
        assert_eq!(
            ParsiDate::parse("1403/XX/01", "%Y/%m/%d").unwrap_err(),
            DateError::ParseError(ParseErrorKind::InvalidNumber),
            "Non-digit month"
        );
        assert_eq!(
            ParsiDate::parse("ABCD/01/01", "%Y/%m/%d").unwrap_err(),
            DateError::ParseError(ParseErrorKind::InvalidNumber),
            "Non-digit year"
        );

        // --- Format Mismatch Errors ---
        // Missing separators
        assert_eq!(
            ParsiDate::parse("14030502", "%Y/%m/%d").unwrap_err(),
            DateError::ParseError(ParseErrorKind::FormatMismatch), // Expected '/', got '0'
            "Missing separators"
        );
        // Wrong separator
        assert_eq!(
            ParsiDate::parse("1403 05 02", "%Y/%m/%d").unwrap_err(),
            DateError::ParseError(ParseErrorKind::FormatMismatch), // Expected '/', got ' '
            "Wrong separator (space vs /)"
        );
        // Trailing text not in format
        assert_eq!(
            ParsiDate::parse("1403/01/01extra", "%Y/%m/%d").unwrap_err(),
            DateError::ParseError(ParseErrorKind::FormatMismatch),
            "Trailing text"
        );
        // Incomplete input for format
        assert_eq!(
            ParsiDate::parse("1403/05", "%Y/%m/%d").unwrap_err(), // Input ends before matching %d
            // This error might depend on where the mismatch occurs. If '/' matches but digits fail, could be InvalidNumber.
            // If input ends where '/' is expected, it's FormatMismatch. Let's assume FormatMismatch.
            DateError::ParseError(ParseErrorKind::FormatMismatch),
            "Incomplete input"
        );
        // Mismatch with literal format chars
        assert_eq!(
            ParsiDate::parse("Year: 1403", "Date: %Y").unwrap_err(),
            DateError::ParseError(ParseErrorKind::FormatMismatch),
            "Literal prefix mismatch"
        );

        // --- Invalid Date Value Errors (parsed components are invalid logically) ---
        assert_eq!(
            ParsiDate::parse("1403/13/01", "%Y/%m/%d").unwrap_err(), // Month > 12
            DateError::ParseError(ParseErrorKind::InvalidDateValue),
            "Invalid month value > 12"
        );
        assert_eq!(
            ParsiDate::parse("1403/00/01", "%Y/%m/%d").unwrap_err(), // Month 0
            DateError::ParseError(ParseErrorKind::InvalidDateValue),
            "Invalid month value 0"
        );
        assert_eq!(
            ParsiDate::parse("1404/12/30", "%Y/%m/%d").unwrap_err(), // Day 30 invalid for Esfand in common year 1404
            DateError::ParseError(ParseErrorKind::InvalidDateValue),
            "Invalid day (Esfand 30 common year)"
        );
        assert_eq!(
            ParsiDate::parse("1403/07/31", "%Y/%m/%d").unwrap_err(), // Day 31 invalid for Mehr (Month 7)
            DateError::ParseError(ParseErrorKind::InvalidDateValue),
            "Invalid day (Mehr 31)"
        );
        assert_eq!(
            ParsiDate::parse("1403/01/00", "%Y/%m/%d").unwrap_err(), // Day 0
            DateError::ParseError(ParseErrorKind::InvalidDateValue),
            "Invalid day value 0"
        );
        // Invalid year value (although usually caught earlier by InvalidNumber if digits mismatch)
        // If format was just '%Y' and input was '0000', InvalidNumber happens first.
        // If ParsiDate::new rejects year 0, that leads to InvalidDateValue.
        assert_eq!(
            ParsiDate::parse("0000/01/01", "%Y/%m/%d").unwrap_err(), // Year 0
            DateError::ParseError(ParseErrorKind::InvalidDateValue), // Assuming ParsiDate::new(0, ..) fails
            "Invalid year value 0"
        );

        // --- Invalid Month Name Errors (%B) ---
        assert_eq!(
            ParsiDate::parse("02 Mordad 1403", "%d %B %Y").unwrap_err(), // Non-Persian name
            DateError::ParseError(ParseErrorKind::InvalidMonthName),
            "Non-Persian month name"
        );
        assert_eq!(
            ParsiDate::parse("01 XXX 1400", "%d %B %Y").unwrap_err(), // Completely wrong name
            DateError::ParseError(ParseErrorKind::InvalidMonthName),
            "Unrecognized month name"
        );
        // Check separator matching *after* month name
        assert_eq!(
            ParsiDate::parse("01 فروردین-1400", "%d %B %Y").unwrap_err(), // Expected space after name, got '-'
            DateError::ParseError(ParseErrorKind::FormatMismatch), // Fails matching the literal space in format
            "Wrong separator after month name"
        );

        // --- Unsupported Specifier Error ---
        assert_eq!(
            ParsiDate::parse("Some text", "%j").unwrap_err(), // %j not supported for parsing
            DateError::ParseError(ParseErrorKind::UnsupportedSpecifier),
            "Unsupported specifier %j for parse"
        );
        assert_eq!(
            ParsiDate::parse("Some text", "%A").unwrap_err(), // %A not supported for parsing
            DateError::ParseError(ParseErrorKind::UnsupportedSpecifier),
            "Unsupported specifier %A for parse"
        );
    }

    // --- Date Info Tests ---
    #[test]
    fn test_weekday() {
        // Use known Gregorian dates and verify Persian weekday mapping (Sat=0..Fri=6)
        // Gregorian: Wed 2024-03-20 -> Persian: Chaharshanbe 1403-01-01 (Day 3)
        assert_eq!(
            pd(1403, 1, 1).weekday(),
            Ok("چهارشنبه".to_string()),
            "1403-01-01 -> Wed"
        );
        // Gregorian: Tue 2024-07-23 -> Persian: Seshanbe 1403-05-02 (Day 3)
        assert_eq!(
            pd(1403, 5, 2).weekday(),
            Ok("سه‌شنبه".to_string()),
            "1403-05-02 -> Tue"
        );
        // Gregorian: Fri 2025-03-21 -> Persian: Jomeh 1404-01-01 (Day 6)
        assert_eq!(
            pd(1404, 1, 1).weekday(),
            Ok("جمعه".to_string()),
            "1404-01-01 -> Fri"
        );
        // Gregorian: Sun 1979-02-11 -> Persian: Yekshanbe 1357-11-22 (Day 1)
        assert_eq!(
            pd(1357, 11, 22).weekday(),
            Ok("یکشنبه".to_string()),
            "1357-11-22 -> Sun"
        );
        // Gregorian: Fri 2026-03-20 -> Persian: Jomeh 1404-12-29 (Day 6)
        assert_eq!(
            pd(1404, 12, 29).weekday(),
            Ok("جمعه".to_string()),
            "1404-12-29 -> Fri"
        );
        // Gregorian: Sat 2024-03-23 -> Persian: Shanbe 1403-01-04 (Day 0)
        assert_eq!(
            pd(1403, 1, 4).weekday(),
            Ok("شنبه".to_string()),
            "1403-01-04 -> Sat"
        );
        // Test on invalid date (created via unsafe)
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(invalid_date.weekday(), Err(DateError::InvalidDate)); // Should fail validation first
    }

    #[test]
    fn test_ordinal() {
        assert_eq!(pd(1403, 1, 1).ordinal(), Ok(1));
        assert_eq!(pd(1403, 1, 31).ordinal(), Ok(31));
        assert_eq!(pd(1403, 2, 1).ordinal(), Ok(32)); // 31 (Far) + 1
        assert_eq!(pd(1403, 5, 2).ordinal(), Ok(126)); // 4*31 (Far-Tir) + 2 = 124 + 2 = 126
        assert_eq!(pd(1403, 7, 1).ordinal(), Ok(187)); // 6*31 (Far-Sha) + 1 = 186 + 1 = 187
        assert_eq!(pd(1403, 12, 30).ordinal(), Ok(366)); // Last day of leap year 1403
        assert_eq!(pd(1404, 1, 1).ordinal(), Ok(1));
        assert_eq!(pd(1404, 12, 29).ordinal(), Ok(365)); // Last day of common year 1404
                                                         // Test on invalid date
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(invalid_date.ordinal(), Err(DateError::InvalidDate)); // Fails validation
    }

    // --- Arithmetic Tests ---
    #[test]
    fn test_add_sub_days() {
        let d_mid_month = pd(1403, 6, 30); // End of 31-day month
        assert_eq!(d_mid_month.add_days(1), Ok(pd(1403, 6, 31)));
        assert_eq!(d_mid_month.add_days(2), Ok(pd(1403, 7, 1))); // Cross to 30-day month
        assert_eq!(d_mid_month.add_days(32), Ok(pd(1403, 8, 1))); // Cross Shahrivar (1d) + Mehr (30d) = 31d -> Aban 1st

        let d_leap_end = pd(1403, 12, 29); // Near end of leap year
        assert_eq!(d_leap_end.add_days(1), Ok(pd(1403, 12, 30))); // To last day
        assert_eq!(d_leap_end.add_days(2), Ok(pd(1404, 1, 1))); // Cross to common year

        let d_common_end = pd(1404, 12, 29); // Last day of common year
        assert_eq!(d_common_end.add_days(1), Ok(pd(1405, 1, 1))); // Cross to common year (1405 is common)
        assert!(!ParsiDate::is_persian_leap_year(1405)); // Verify 1405 is common (1405 % 33 = 7)

        // Subtraction
        let d_start_common = pd(1404, 1, 1); // Start of common year
        assert_eq!(d_start_common.add_days(-1), Ok(pd(1403, 12, 30))); // Subtract 1 day -> end of leap year
        assert_eq!(d_start_common.sub_days(1), Ok(pd(1403, 12, 30))); // Using sub_days

        let d_start_common2 = pd(1405, 1, 1); // Start of common year
        assert_eq!(d_start_common2.sub_days(1), Ok(pd(1404, 12, 29))); // Subtract 1 day -> end of common year

        // Add/subtract zero
        assert_eq!(d_mid_month.add_days(0), Ok(d_mid_month));
        assert_eq!(d_mid_month.sub_days(0), Ok(d_mid_month));

        // Add/subtract large number of days
        let base = pd(1400, 1, 1); // Gregorian: 2021-03-21 (assuming this was Nowruz 1400)
        let expected_greg_plus_1000 = NaiveDate::from_ymd_opt(2021, 3, 21)
            .unwrap()
            .checked_add_days(chrono::Days::new(1000))
            .unwrap(); // Approx 2023-12-16
        let expected_parsi_plus_1000 = ParsiDate::from_gregorian(expected_greg_plus_1000).unwrap();
        assert_eq!(base.add_days(1000), Ok(expected_parsi_plus_1000));
        assert_eq!(expected_parsi_plus_1000.sub_days(1000), Ok(base));
        assert_eq!(expected_parsi_plus_1000.add_days(-1000), Ok(base));

        // Test potential overflow (depends on chrono's limits, likely results in error)
        // Add extremely large number of days - expect ArithmeticOverflow or GregorianConversionError
        let large_days = i64::MAX / 10; // Still huge, but less likely to hit chrono internal limits immediately
        let far_future_result = base.add_days(large_days);
        assert!(far_future_result.is_err()); // Expecting some error
                                             // println!("Adding large days result: {:?}", far_future_result.unwrap_err()); // Check specific error if needed

        let far_past_result = base.add_days(-large_days);
        assert!(far_past_result.is_err());
        // println!("Subtracting large days result: {:?}", far_past_result.unwrap_err());

        // Test arithmetic on invalid date
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(invalid_date.add_days(1), Err(DateError::InvalidDate)); // Fails validation first
        assert_eq!(invalid_date.sub_days(1), Err(DateError::InvalidDate));
    }

    #[test]
    fn test_add_sub_months() {
        let d_31 = pd(1403, 1, 31); // End of 31-day month (Farvardin, leap year)
        assert_eq!(
            d_31.add_months(1),
            Ok(pd(1403, 2, 31)),
            "To Ordibehesht (31d)"
        );
        assert_eq!(
            d_31.add_months(5),
            Ok(pd(1403, 6, 31)),
            "To Shahrivar (31d)"
        );
        assert_eq!(
            d_31.add_months(6),
            Ok(pd(1403, 7, 30)),
            "To Mehr (30d), clamped"
        );
        assert_eq!(
            d_31.add_months(11),
            Ok(pd(1403, 12, 30)),
            "To Esfand (30d, leap), clamped"
        );

        let d_31_common = pd(1404, 1, 31); // End of 31-day month (Farvardin, common year)
        assert_eq!(
            d_31_common.add_months(11),
            Ok(pd(1404, 12, 29)),
            "To Esfand (29d, common), clamped"
        );

        let d_mid = pd(1403, 5, 15); // Middle of 31-day month
        assert_eq!(d_mid.add_months(1), Ok(pd(1403, 6, 15)));
        assert_eq!(
            d_mid.add_months(7),
            Ok(pd(1403, 12, 15)),
            "To Esfand (leap)"
        );
        assert_eq!(d_mid.add_months(12), Ok(pd(1404, 5, 15)), "Add full year");
        assert_eq!(
            d_mid.add_months(19),
            Ok(pd(1404, 12, 15)),
            "To Esfand (common)"
        );

        // Subtraction
        assert_eq!(
            d_mid.add_months(-5),
            Ok(pd(1402, 12, 15)),
            "Subtract 5 months -> Esfand 1402 (common)"
        );
        assert_eq!(d_mid.sub_months(5), Ok(pd(1402, 12, 15)));
        assert_eq!(
            d_mid.sub_months(17),
            Ok(pd(1401, 12, 15)),
            "Subtract 17 months -> Esfand 1401 (common)"
        );
        assert_eq!(
            d_31.sub_months(1),
            Ok(pd(1402, 12, 29)),
            "1403-01-31 minus 1m -> Esfand 1402 (common), clamped"
        );

        // Test clamping when subtracting into longer months (day should not change)
        let d_30 = pd(1403, 8, 30); // End of Aban (30 days)
        assert_eq!(d_30.sub_months(1), Ok(pd(1403, 7, 30)), "To Mehr (30d)");
        assert_eq!(
            d_30.sub_months(2),
            Ok(pd(1403, 6, 30)),
            "To Shahrivar (31d), day stays 30"
        );

        // Add zero
        assert_eq!(d_mid.add_months(0), Ok(d_mid));

        // Test large values crossing multiple years
        assert_eq!(d_mid.add_months(24), Ok(pd(1405, 5, 15)));
        assert_eq!(d_mid.sub_months(24), Ok(pd(1401, 5, 15)));

        // Test arithmetic on invalid date
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(invalid_date.add_months(1), Err(DateError::InvalidDate));
        assert_eq!(invalid_date.sub_months(1), Err(DateError::InvalidDate));

        // Test potential overflow
        let max_date = pd(9999, 1, 1);
        assert!(
            max_date.add_months(12).is_err(),
            "Adding year to max year should fail"
        );
        let min_date = pd(1, 1, 1);
        assert!(
            min_date.sub_months(1).is_err(),
            "Subtracting month from min date should fail"
        );
    }

    #[test]
    fn test_add_sub_years() {
        let d1 = pd(1403, 5, 2); // Leap year
        assert_eq!(d1.add_years(1), Ok(pd(1404, 5, 2)), "Leap -> Common");
        assert_eq!(d1.add_years(-1), Ok(pd(1402, 5, 2)), "Leap -> Common");
        assert_eq!(d1.sub_years(1), Ok(pd(1402, 5, 2)));

        // Test leap day adjustment
        let d_leap_end = pd(1403, 12, 30); // Last day of leap year
        assert_eq!(
            d_leap_end.add_years(1),
            Ok(pd(1404, 12, 29)),
            "Leap day + 1y -> Common year, clamped"
        );
        assert_eq!(
            d_leap_end.sub_years(4),
            Ok(pd(1399, 12, 30)),
            "Leap day - 4y -> Leap year 1399"
        ); // 1399 is leap
        assert_eq!(
            d_leap_end.sub_years(1),
            Ok(pd(1402, 12, 29)),
            "Leap day - 1y -> Common year 1402, clamped"
        );

        let d_common_esfand = pd(1404, 12, 29); // Last day of common year
        assert_eq!(
            d_common_esfand.add_years(1),
            Ok(pd(1405, 12, 29)),
            "Common Esfand -> Common Esfand"
        ); // 1405 common
        assert_eq!(
            d_common_esfand.add_years(3),
            Ok(pd(1407, 12, 29)),
            "Common Esfand -> Leap Esfand"
        ); // 1407 leap, day 29 is fine
        assert_eq!(
            d_common_esfand.sub_years(1),
            Ok(pd(1403, 12, 29)),
            "Common Esfand -> Leap Esfand"
        ); // 1403 leap, day 29 is fine

        // Add zero
        assert_eq!(d1.add_years(0), Ok(d1));

        // Test arithmetic on invalid date
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(invalid_date.add_years(1), Err(DateError::InvalidDate));
        assert_eq!(invalid_date.sub_years(1), Err(DateError::InvalidDate));

        // Test year range limits
        assert_eq!(
            pd(MAX_PARSI_DATE.year, 1, 1).add_years(1),
            Err(DateError::ArithmeticOverflow)
        );
        assert_eq!(
            pd(MIN_PARSI_DATE.year, 1, 1).sub_years(1),
            Err(DateError::ArithmeticOverflow)
        );
        assert_eq!(
            pd(MIN_PARSI_DATE.year, 1, 1).add_years(-1),
            Err(DateError::ArithmeticOverflow)
        );
    }

    #[test]
    fn test_days_between() {
        let d1 = pd(1403, 1, 1);
        let d2 = pd(1403, 1, 11);
        let d3 = pd(1404, 1, 1); // Start of next year (1403 is leap, so 366 days)
        let d4 = pd(1402, 12, 29); // Day before d1 (1402 common year end)
        let d5 = pd(1405, 1, 1); // Start of year after d3 (1404 common, so 365 days)

        assert_eq!(d1.days_between(&d1), Ok(0));
        assert_eq!(d1.days_between(&d2), Ok(10), "Within same month");
        assert_eq!(
            d2.days_between(&d1),
            Ok(10),
            "Order doesn't matter for abs value"
        );

        assert_eq!(d1.days_between(&d3), Ok(366), "Across leap year boundary");
        assert_eq!(d3.days_between(&d1), Ok(366));

        assert_eq!(d3.days_between(&d5), Ok(365), "Across common year boundary");
        assert_eq!(d5.days_between(&d3), Ok(365));

        assert_eq!(
            d1.days_between(&d4),
            Ok(1),
            "Adjacent days across year boundary"
        );
        assert_eq!(d4.days_between(&d1), Ok(1));

        // Longer duration test
        let d_start = pd(1357, 11, 22); // Gregorian: 1979-02-11
        let d_end = pd(1403, 5, 2); // Gregorian: 2024-07-23
                                    // Verify using chrono
        let g_start = NaiveDate::from_ymd_opt(1979, 2, 11).unwrap();
        let g_end = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
        let expected_diff = g_end.signed_duration_since(g_start).num_days(); // Should be positive
        assert!(expected_diff > 0);
        assert_eq!(d_start.days_between(&d_end), Ok(expected_diff.abs()));
        assert_eq!(d_end.days_between(&d_start), Ok(expected_diff.abs()));

        // Test with invalid dates
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(d1.days_between(&invalid_date), Err(DateError::InvalidDate)); // `other` is invalid
        assert_eq!(invalid_date.days_between(&d1), Err(DateError::InvalidDate));
        // `self` is invalid
    }

    // --- Helper Method Tests ---
    #[test]
    fn test_with_year() {
        let d_mid_leap = pd(1403, 5, 2); // Mid-month in leap year
        let d_leap_end = pd(1403, 12, 30); // End of leap year
        let d_common_mid = pd(1404, 7, 15); // Mid-month in common year
        let d_common_end = pd(1404, 12, 29); // End of common year

        // Leap -> Common (mid-month, no change needed)
        assert_eq!(d_mid_leap.with_year(1404), Ok(pd(1404, 5, 2)));
        // Leap End -> Common (day clamped)
        assert_eq!(d_leap_end.with_year(1404), Ok(pd(1404, 12, 29)));
        // Common -> Leap (mid-month, no change needed)
        assert_eq!(d_common_mid.with_year(1403), Ok(pd(1403, 7, 15)));
        // Common End -> Leap (day 29 exists, no change needed)
        assert_eq!(d_common_end.with_year(1403), Ok(pd(1403, 12, 29)));
        // Leap End -> Leap

        // Test invalid target year
        assert_eq!(
            d_mid_leap.with_year(0),
            Err(DateError::InvalidDate),
            "Target year 0"
        );
        assert_eq!(
            d_mid_leap.with_year(10000),
            Err(DateError::InvalidDate),
            "Target year 10000"
        );

        // Test with invalid self
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(invalid_date.with_year(1405), Err(DateError::InvalidDate)); // Fails self validation
    }

    #[test]
    fn test_with_month() {
        let d_31 = pd(1403, 1, 31); // End of 31-day month (Farvardin)
        let d_mid_30 = pd(1403, 7, 10); // Mid 30-day month (Mehr)
        let d_start_29_common = pd(1404, 12, 5); // Start of 29-day Esfand (common)
        let d_end_30_leap = pd(1403, 12, 30); // End of 30-day Esfand (leap)

        // From 31-day month
        assert_eq!(
            d_31.with_month(2),
            Ok(pd(1403, 2, 31)),
            "To Ordibehesht (31d)"
        );
        assert_eq!(
            d_31.with_month(7),
            Ok(pd(1403, 7, 30)),
            "To Mehr (30d), clamped"
        );
        assert_eq!(
            d_31.with_month(12),
            Ok(pd(1403, 12, 30)),
            "To Esfand (30d, leap), clamped"
        );
        assert_eq!(
            pd(1404, 1, 31).with_month(12),
            Ok(pd(1404, 12, 29)),
            "To Esfand (29d, common), clamped"
        );

        // From 30-day month
        assert_eq!(
            d_mid_30.with_month(6),
            Ok(pd(1403, 6, 10)),
            "To Shahrivar (31d)"
        );
        assert_eq!(
            d_mid_30.with_month(11),
            Ok(pd(1403, 11, 10)),
            "To Bahman (30d)"
        );

        // From 29-day month
        assert_eq!(
            d_start_29_common.with_month(1),
            Ok(pd(1404, 1, 5)),
            "To Farvardin (31d)"
        );

        // From end of leap Esfand
        assert_eq!(
            d_end_30_leap.with_month(1),
            Ok(pd(1403, 1, 30)),
            "To Farvardin (31d), day stays 30"
        );
        assert_eq!(
            d_end_30_leap.with_month(7),
            Ok(pd(1403, 7, 30)),
            "To Mehr (30d), day stays 30"
        );

        // Test invalid target month
        assert_eq!(
            d_31.with_month(0),
            Err(DateError::InvalidDate),
            "Target month 0"
        );
        assert_eq!(
            d_31.with_month(13),
            Err(DateError::InvalidDate),
            "Target month 13"
        );

        // Test with invalid self
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(invalid_date.with_month(1), Err(DateError::InvalidDate)); // Fails self validation
    }

    #[test]
    fn test_with_day() {
        let d_mehr = pd(1403, 7, 1); // Start of Mehr (30 days)
        let d_esfand_common = pd(1404, 12, 1); // Start of Esfand (29 days, common)
        let d_esfand_leap = pd(1403, 12, 1); // Start of Esfand (30 days, leap)

        // Valid day changes
        assert_eq!(d_mehr.with_day(15), Ok(pd(1403, 7, 15)));
        assert_eq!(
            d_mehr.with_day(30),
            Ok(pd(1403, 7, 30)),
            "To valid last day"
        );

        // Invalid day changes (exceeds month length)
        assert_eq!(
            d_mehr.with_day(31),
            Err(DateError::InvalidDate),
            "Invalid day 31 for Mehr"
        );
        assert_eq!(
            d_esfand_common.with_day(29),
            Ok(pd(1404, 12, 29)),
            "To valid last day (common)"
        );
        assert_eq!(
            d_esfand_common.with_day(30),
            Err(DateError::InvalidDate),
            "Invalid day 30 for Esfand common"
        );
        assert_eq!(
            d_esfand_leap.with_day(30),
            Ok(pd(1403, 12, 30)),
            "To valid last day (leap)"
        );
        assert_eq!(
            d_esfand_leap.with_day(31),
            Err(DateError::InvalidDate),
            "Invalid day 31 for Esfand leap"
        );

        // Invalid target day 0
        assert_eq!(
            d_mehr.with_day(0),
            Err(DateError::InvalidDate),
            "Target day 0"
        );

        // Test with invalid self
        let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
        assert_eq!(invalid_date.with_day(1), Err(DateError::InvalidDate)); // Fails self validation
    }

    #[test]
    fn test_day_of_boundaries() {
        let d_mid_leap = pd(1403, 5, 15); // Leap year, 31-day month (Mordad)
        assert_eq!(d_mid_leap.first_day_of_month(), pd(1403, 5, 1));
        assert_eq!(d_mid_leap.last_day_of_month(), pd(1403, 5, 31));
        assert_eq!(d_mid_leap.first_day_of_year(), pd(1403, 1, 1));
        assert_eq!(
            d_mid_leap.last_day_of_year(),
            pd(1403, 12, 30),
            "Last day of leap year 1403"
        );

        let d_mid_common = pd(1404, 7, 10); // Common year, 30-day month (Mehr)
        assert_eq!(d_mid_common.first_day_of_month(), pd(1404, 7, 1));
        assert_eq!(d_mid_common.last_day_of_month(), pd(1404, 7, 30));
        assert_eq!(d_mid_common.first_day_of_year(), pd(1404, 1, 1));
        assert_eq!(
            d_mid_common.last_day_of_year(),
            pd(1404, 12, 29),
            "Last day of common year 1404"
        );

        let d_esfand_leap = pd(1403, 12, 10); // Leap year, Esfand
        assert_eq!(d_esfand_leap.first_day_of_month(), pd(1403, 12, 1));
        assert_eq!(d_esfand_leap.last_day_of_month(), pd(1403, 12, 30));

        let d_esfand_common = pd(1404, 12, 10); // Common year, Esfand
        assert_eq!(d_esfand_common.first_day_of_month(), pd(1404, 12, 1));
        assert_eq!(d_esfand_common.last_day_of_month(), pd(1404, 12, 29));

        // Check idempotency (calling again should yield same result)
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
    fn test_constants_validity_and_values() {
        // Check MIN_PARSI_DATE
        assert!(MIN_PARSI_DATE.is_valid(), "MIN_PARSI_DATE should be valid");
        assert_eq!(MIN_PARSI_DATE.year(), 1);
        assert_eq!(MIN_PARSI_DATE.month(), 1);
        assert_eq!(MIN_PARSI_DATE.day(), 1);
        // Check Gregorian equivalent (approximate check)
        assert_eq!(MIN_PARSI_DATE.to_gregorian().unwrap().year(), 622);

        // Check MAX_PARSI_DATE
        assert!(MAX_PARSI_DATE.is_valid(), "MAX_PARSI_DATE should be valid");
        assert_eq!(MAX_PARSI_DATE.year(), 9999);
        assert_eq!(MAX_PARSI_DATE.month(), 12);
        assert_eq!(
            MAX_PARSI_DATE.day(),
            29,
            "Year 9999 is not leap, should end on 29th"
        );
        // Check that 9999 is indeed not leap according to the function
        assert!(!ParsiDate::is_persian_leap_year(9999));
    }

    // --- Serde Tests (conditional on 'serde' feature) ---
    #[cfg(feature = "serde")]
    mod serde_tests {
        use super::*; // Import items from outer scope
        use serde_json; // Assuming serde_json is a dev-dependency

        #[test]
        fn test_serialization_deserialization_valid() {
            let date = pd(1403, 5, 2);
            // Expected JSON based on field names
            let expected_json = r#"{"year":1403,"month":5,"day":2}"#;

            // Serialize the ParsiDate object
            let json = serde_json::to_string(&date).expect("Serialization failed");
            assert_eq!(json, expected_json, "Serialized JSON mismatch");

            // Deserialize the JSON string back into a ParsiDate object
            let deserialized: ParsiDate =
                serde_json::from_str(&json).expect("Deserialization failed");
            assert_eq!(deserialized, date, "Deserialized object mismatch");
            // Verify the deserialized object is valid (as the original was valid)
            assert!(
                deserialized.is_valid(),
                "Deserialized valid date should be valid"
            );
        }

        #[test]
        fn test_deserialize_structurally_valid_but_logically_invalid() {
            // This JSON is structurally valid (correct field names and types) for ParsiDate,
            // but represents a logically invalid date (Esfand 30 in common year 1404).
            let json_invalid_day = r#"{"year":1404,"month":12,"day":30}"#;

            // Default serde derive will successfully deserialize this into the struct fields.
            // It does *not* automatically call `ParsiDate::new` or `is_valid`.
            let deserialized_invalid: ParsiDate = serde_json::from_str(json_invalid_day)
                .expect("Default derive should deserialize structurally valid JSON");

            // Check that the fields were populated directly from the JSON.
            assert_eq!(deserialized_invalid.year(), 1404);
            assert_eq!(deserialized_invalid.month(), 12);
            assert_eq!(deserialized_invalid.day(), 30);

            // Crucially, the resulting ParsiDate object should report itself as *invalid*
            // when `is_valid()` is called, because the combination is logically incorrect.
            assert!(
                !deserialized_invalid.is_valid(),
                "Deserialized date (1404-12-30) should be identified as invalid by is_valid()"
            );

            // Example with invalid month
            let json_invalid_month = r#"{"year":1403,"month":13,"day":1}"#;
            let deserialized_invalid_month: ParsiDate = serde_json::from_str(json_invalid_month)
                .expect("Deserialization of month 13 should succeed structurally");
            assert!(
                !deserialized_invalid_month.is_valid(),
                "Month 13 should be invalid"
            );
        }

        #[test]
        fn test_deserialize_structurally_invalid() {
            // Field type mismatch (month as string instead of number)
            let json_invalid_month_type = r#"{"year":1403,"month":"May","day":2}"#;
            assert!(
                serde_json::from_str::<ParsiDate>(json_invalid_month_type).is_err(),
                "Should fail deserialization due to wrong type for 'month'"
            );

            // Field type mismatch (year as bool)
            let json_invalid_year_type = r#"{"year":true,"month":5,"day":2}"#;
            assert!(
                serde_json::from_str::<ParsiDate>(json_invalid_year_type).is_err(),
                "Should fail deserialization due to wrong type for 'year'"
            );

            // Missing field ('day' is absent)
            let json_missing_field = r#"{"year":1403,"month":5}"#;
            assert!(
                serde_json::from_str::<ParsiDate>(json_missing_field).is_err(),
                "Should fail deserialization due to missing 'day' field"
            );

            // Extra field ('extra' field is present)
            let json_extra_field = r#"{"year":1403,"month":5,"day":2,"extra":"data"}"#;
            // Default serde behavior is often to ignore unknown fields.
            // Use `#[serde(deny_unknown_fields)]` on the struct if this should be an error.
            match serde_json::from_str::<ParsiDate>(json_extra_field) {
                Ok(pd) => {
                    // If this succeeds, it means unknown fields are ignored.
                    assert_eq!(
                        pd,
                        ParsiDate::new(1403, 5, 2).unwrap(),
                        "Data mismatch despite extra field"
                    );
                    println!("Note: Deserialization succeeded despite extra field (default serde behavior).");
                }
                Err(_) => {
                    // This path would be taken if #[serde(deny_unknown_fields)] was active.
                    // For this test assuming default behavior, success is expected.
                    panic!("Deserialization failed unexpectedly on extra field. Is deny_unknown_fields active?");
                }
            }

            // Completely wrong JSON structure (array instead of object)
            let json_wrong_structure = r#"[1403, 5, 2]"#;
            assert!(
                serde_json::from_str::<ParsiDate>(json_wrong_structure).is_err(),
                "Should fail deserialization due to wrong JSON structure (array vs object)"
            );

            // Empty JSON object
            let json_empty = r#"{}"#;
            assert!(
                serde_json::from_str::<ParsiDate>(json_empty).is_err(),
                "Should fail deserialization due to missing all fields"
            );
        }
    } // end serde_tests module
} // end tests module
