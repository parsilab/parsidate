//  * Copyright (C) Mohammad (Sina) Jalalvandi (parsidate) 2024-2025 <jalalvandi.sina@gmail.com>
//  * Version : 1.3.3
//  * 128558ad-c066-4c4a-9b93-bca896bf4465 - Change Project Structure
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
//! use parsidate::{ParsiDate, DateError}; // Make sure to import from the crate root
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
//! use parsidate::ParseErrorKind; // Import ParseErrorKind if needed for error matching
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
//! let today = ParsiDate::today().unwrap();
//! println!("Today's Persian date: {}", today);
//!
//! // --- Serde (requires 'serde' feature) ---
//! 
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
//!
//! ```
//!
//! ## Features
//!
//! *   `serde`: Enables serialization and deserialization support via the `serde` crate.

// Declare the modules within the src directory
mod constants;
mod date;
mod error;

// Conditionally declare the tests module
#[cfg(test)]
mod tests;

// Re-export the public API elements for easy access from the crate root
pub use constants::{MAX_PARSI_DATE, MIN_PARSI_DATE};
pub use date::ParsiDate;
pub use error::{DateError, ParseErrorKind};
