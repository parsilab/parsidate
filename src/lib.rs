// ~/src/lib.rs
//
//  * Copyright (C) Mohammad (Sina) Jalalvandi 2024-2025 <jalalvandi.sina@gmail.com>
//  * Package : parsidate
//  * License : Apache-2.0
//  * Version : 1.7.0
//  * URL     : https://github.com/jalalvandi/parsidate
//  * Sign: parsidate-20250607-fea13e856dcd-459c6e73c83e49e10162ee28b26ac7cd
//
//! # ParsiDate: Comprehensive Persian (Jalali) Calendar Implementation in Rust

//! This crate provides comprehensive functionality for working with the Persian (Jalali or Shamsi) calendar system.
//! It allows for:
//!
//! *   **Conversion:** Seamlessly convert dates and date-times between the Gregorian and Persian calendars.
//! *   **Validation:** Check if a given year, month, day and date-times combination forms a valid Persian date.
//! *   **Formatting:** Display Persian dates and times in various predefined formats ("short", "long", "iso") and using custom `strftime`-like patterns.
//! *   **Parsing:** Parse strings into `ParsiDate` or `ParsiDateTime` objects based on specified formats.
//! *   **Date Arithmetic:** Add or subtract days, months, and years from a date.
//! *   **Leap Year Calculation:** Determine if a Persian or Gregorian year is a leap year (using a common 33-year cycle approximation for Persian).
//! *   **Weekday Calculation:** Find the Persian name for the day of the week.
//! *   **Ordinal Day Calculation:** Get the day number within the year (1-366).
//! *   **Helper Functions:** Easily get the first/last day of the month/year or manipulate date/time components.
//! *   **Current Date:** Get the current system date as a `ParsiDate`.
//! *   **Week of Year:** Calculate the week number within the Persian year (Saturday start).
//! *   **Seasons:** Get the current season based on the Persian date.
//! *   **Timezone Support:** Create timezone-aware datetimes and perform conversions between timezones (requires the `timezone` feature).
//! *   **Serde Support:** Optional serialization/deserialization using the `serde` feature.
//!
//! It relies on the `chrono` crate for underlying Gregorian date representations, current time, and some calculations.
//!
//! ## Examples
//!
//! ```rust
//! use chrono::{NaiveDate, NaiveDateTime, Duration};
//! use parsidate::{ParsiDate, ParsiDateTime, DateError,Season}; // Import both
//!
//! // --- ParsiDate Usage (Date only) ---
//! let pd = ParsiDate::new(1403, 5, 2).unwrap();
//! assert_eq!(pd.format("%Y-%m-%d"), "1403-05-02");
//! let today_date = ParsiDate::today().unwrap();
//! println!("Today's Persian date: {}", today_date);
//!
//! // --- ParsiDateTime Usage (Date and Time) ---
//! let pdt = ParsiDateTime::new(1403, 5, 2, 15, 30, 45).unwrap();
//! assert_eq!(pdt.year(), 1403);
//! assert_eq!(pdt.hour(), 15);
//!
//! // Convert Gregorian DateTime to Persian DateTime
//! let g_dt = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap().and_hms_opt(15, 30, 45).unwrap();
//! let pdt_from_g = ParsiDateTime::from_gregorian(g_dt).unwrap();
//! assert_eq!(pdt_from_g, pdt);
//!
//! // Convert Persian DateTime to Gregorian DateTime
//! let g_dt_conv = pdt.to_gregorian().unwrap();
//! assert_eq!(g_dt_conv, g_dt);
//!
//! // Formatting DateTime
//! assert_eq!(pdt.format("%Y/%m/%d %H:%M:%S"), "1403/05/02 15:30:45");
//! assert_eq!(pdt.format("%A %d %B ساعت %T"), "سه‌شنبه 02 مرداد ساعت 15:30:45");
//! assert_eq!(pdt.to_string(), "1403/05/02 15:30:45"); // Default Display format
//!
//! // Parsing DateTime
//! let parsed_dt = ParsiDateTime::parse("1403/05/02 15:30:45", "%Y/%m/%d %H:%M:%S").unwrap();
//! assert_eq!(parsed_dt, pdt);
//!
//! // DateTime Arithmetic
//! let next_hour = pdt.add_duration(Duration::hours(1)).unwrap();
//! assert_eq!(next_hour, ParsiDateTime::new(1403, 5, 2, 16, 30, 45).unwrap());
//!
//! let next_day_dt = pdt.add_days(1).unwrap(); // Preserves time
//! assert_eq!(next_day_dt, ParsiDateTime::new(1403, 5, 3, 15, 30, 45).unwrap());
//!
//! // Current DateTime
//! let now_dt = ParsiDateTime::now().unwrap();
//! println!("Current Persian DateTime: {}", now_dt);
//!
//! // Week of Year
//! let week_of_year = pdt.week_of_year();
//! assert_eq!(week_of_year, Ok(19));
//!
//! // Seasons
//! let season = pdt.season();
//! assert_eq!(season, Ok(Season::Tabestan)); // Assuming 1403/05/02 is in summer
//!
//! // Weekday Calculation
//! let weekday = pd.weekday();
//! assert_eq!(weekday, Ok("سه‌شنبه".to_string())); // Assuming 1403/05/02 is a Tuesday
//!
//! // --- ZonedParsiDateTime (requires 'timezone' feature) ---
//! #[cfg(feature = "timezone")]
//! {
//!     use parsidate::ZonedParsiDateTime;
//!     use chrono_tz::Asia::Tehran;
//!     use chrono_tz::Europe::London;
//!
//!     // Get the current time in a specific timezone
//!     let tehran_now = ZonedParsiDateTime::now(Tehran);
//!     println!("The current time in Tehran is: {}", tehran_now);
//!
//!     // Create a specific zoned time
//!     let dt = ZonedParsiDateTime::new(1403, 8, 15, 12, 0, 0, Tehran).unwrap();
//!     assert_eq!(dt.hour(), 12);
//!
//!     // Convert to another timezone
//!     let london_dt = dt.with_timezone(&London);
//!     println!("{} in Tehran is {} in London.", dt, london_dt);
//!     // In winter, Tehran is UTC+3:30, London is UTC+0.
//!     assert_eq!(london_dt.hour(), 8);
//!     assert_eq!(london_dt.minute(), 30);
//! }
//!
//! // --- Serde (requires 'serde' feature) ---
//! #[cfg(feature = "serde")]
//! {
//!     use serde_json;
//!     // ParsiDate example (unchanged)
//!     let pd_serde = ParsiDate::new(1403, 1, 1).unwrap();
//!     let json_pd = serde_json::to_string(&pd_serde).unwrap();
//!     println!("Serialized ParsiDate: {}", json_pd); // {"year":1403,"month":1,"day":1}
//!     let deser_pd: ParsiDate = serde_json::from_str(&json_pd).unwrap();
//!     assert_eq!(deser_pd, pd_serde);
//!
//!     // ParsiDateTime example
//!     let pdt_serde = ParsiDateTime::new(1403, 5, 2, 10, 20, 30).unwrap();
//!     // Expected structure includes the nested ParsiDate
//!     let json_pdt = serde_json::to_string(&pdt_serde).unwrap();
//!     println!("Serialized ParsiDateTime: {}", json_pdt); // {"date":{"year":1403,"month":5,"day":2},"hour":10,"minute":20,"second":30}
//!     let deser_pdt: ParsiDateTime = serde_json::from_str(&json_pdt).unwrap();
//!     assert_eq!(deser_pdt, pdt_serde);
//!     assert!(deser_pdt.is_valid());
//!
//!     // Deserializing potentially invalid ParsiDateTime
//!     let json_invalid_dt = r#"{"date":{"year":1404,"month":12,"day":30},"hour":25,"minute":0,"second":0}"#; // Invalid day AND hour
//!     let deser_invalid_dt: ParsiDateTime = serde_json::from_str(json_invalid_dt).unwrap();
//!     // Default serde derive populates fields, but is_valid() should fail
//!     assert!(!deser_invalid_dt.is_valid());
//! }
//! ```
//!
//! ## Features
//!
//! *   `serde`: Enables serialization and deserialization support via the `serde` crate.
//! *   `timezone`: Enables timezone-aware functionality via `ZonedParsiDateTime` and the `chrono-tz` crate.

// Declare the modules within the src directory
mod constants;
mod date;
mod datetime;
mod error;
mod season;

// Conditionally declare the new 'zoned' module.
#[cfg(feature = "timezone")]
mod zoned;

// Conditionally declare the tests module
#[cfg(test)]
mod tests;

// Re-export the public API elements for easy access from the crate root
pub use constants::{MAX_PARSI_DATE, MIN_PARSI_DATE};
pub use date::ParsiDate;
pub use datetime::ParsiDateTime;
pub use error::{DateError, ParseErrorKind};
pub use season::Season;

// Conditionally re-export the new 'ZonedParsiDateTime' struct.
#[cfg(feature = "timezone")]
pub use zoned::ZonedParsiDateTime;
