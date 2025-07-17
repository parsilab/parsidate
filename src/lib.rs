// ~/src/lib.rs
//
//  * Copyright (C) ParsiCore (parsidate) 2024-2025 <parsicore.dev@gmail.com>
//  * Package : parsidate
//  * License : Apache-2.0
//  * Version : 1.7.1
//  * URL     : https://github.com/parsicore/parsidate
//  * Sign: parsidate-20250607-fea13e856dcd-459c6e73c83e49e10162ee28b26ac7cd
//
//! # ParsiDate: A Comprehensive Persian (Jalali) Calendar Library for Rust
//!
//! `parsidate` offers a powerful, ergonomic, and comprehensive suite of tools for working with the
//! Persian (also known as Jalali or Shamsi) calendar in Rust. Built on top of `chrono`, it provides
//! a familiar and robust API for date and time manipulations, conversions, and formatting.
//!
//! The library's core types are:
//! - [`ParsiDate`]: Represents a date (year, month, day) in the Persian calendar.
//! - [`ParsiDateTime`]: Represents a specific date and time, without timezone information.
//! - [`ZonedParsiDateTime`]: A timezone-aware date and time (requires the `timezone` feature).
//!
//! ## Key Features
//!
//! *   **Seamless Gregorian Conversion**: Effortlessly convert between Persian and Gregorian (`chrono`) types.
//! *   **Robust Validation**: Ensure date and time integrity with strict validation rules.
//! *   **Flexible Formatting & Parsing**: Use `strftime`-like patterns for custom string representations.
//! *   **Full-Featured Arithmetic**: Reliably perform calculations with days, months, years, and `chrono::Duration`.
//! *   **Rich Date Information**: Access properties like weekday, ordinal day, and leap year status.
//! *   **Season Calculation**: Determine the Persian season (`Bahar`, `Tabestan`, etc.) for any date.
//! *   **Timezone Aware (Optional)**: Full support for timezones via the `timezone` feature.
//! *   **Serialization (Optional)**: `serde` support for easy integration with data formats like JSON.
//!
//! ## Getting Started
//!
//! Add `parsidate` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! parsidate = "1.7.1"
//! # For timezone and/or serde support, enable features:
//! # parsidate = { version = "1.7.1", features = ["timezone", "serde"] }
//! ```
//!
//! A quick example of creating a date, formatting it, and getting its season:
//!
//! ```rust
//! use parsidate::{ParsiDate, ParsiDateTime, Season};
//! use chrono::NaiveDate;
//!
//! fn main() -> Result<(), parsidate::DateError> {
//!     // Create a ParsiDate for a summer day
//!     let pd = ParsiDate::new(1403, 5, 2)?;
//!
//!     // Format it
//!     println!("Formatted date: {}", pd.format("%A، %d %B %Y"));
//!
//!     // Get its season
//!     assert_eq!(pd.season()?, Season::Tabestan);
//!     println!("The season is: {}", pd.season()?.name_persian()); // "تابستان"
//!
//!     // Convert it to Gregorian
//!     let gregorian_date = pd.to_gregorian()?;
//!     assert_eq!(gregorian_date, NaiveDate::from_ymd_opt(2024, 7, 23).unwrap());
//!     println!("Equivalent Gregorian date: {}", gregorian_date);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Examples
//!
//! A more detailed look at the library's capabilities.
//!
//! ```rust
//! use chrono::{NaiveDate, NaiveDateTime, Duration};
//! use parsidate::{ParsiDate, ParsiDateTime, DateError, Season};
//!
//! // --- ParsiDate (Date only) ---
//! let pd = ParsiDate::new(1403, 5, 2).unwrap();
//! assert_eq!(pd.format("%Y-%m-%d"), "1403-05-02");
//!
//! // Get today's date
//! let today_date = ParsiDate::today().unwrap();
//! println!("Today's Persian date: {}", today_date);
//!
//! // Get date properties
//! assert_eq!(pd.weekday(), Ok("سه‌شنبه".to_string()));
//! assert_eq!(pd.ordinal(), Ok(126));
//! assert_eq!(pd.week_of_year(), Ok(19));
//! assert!(!ParsiDate::is_persian_leap_year(1404));
//!
//! // --- Season Usage ---
//! let autumn_date = ParsiDate::new(1403, 8, 15).unwrap();
//! let season = autumn_date.season().unwrap();
//! assert_eq!(season, Season::Paeez);
//! assert_eq!(season.name_persian(), "پاییز");
//! assert_eq!(season.name_english(), "Autumn");
//!
//! // --- ParsiDateTime (Date and Time) ---
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
//! // --- Serde Integration (requires `serde` feature) ---
//! #[cfg(feature = "serde")]
//! {
//!     use serde_json;
//!     // ParsiDate example
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
//! This crate has two optional features:
//!
//! -   `serde`: Enables serialization and deserialization for `ParsiDate`, `ParsiDateTime`, and `Season`
//!     via the `serde` crate. Add to `Cargo.toml` with `features = ["serde"]`.
//! -   `timezone`: Enables the [`ZonedParsiDateTime`] struct for timezone-aware operations,
//!     powered by the `chrono-tz` crate. Add to `Cargo.toml` with `features = ["timezone"]`.
//!
//! You can enable both with `features = ["serde", "timezone"]`.

// --- Library Module Declarations ---
// These `mod` statements declare the modules that form the library's internal structure.

mod constants;
mod date;
mod datetime;
mod error;
mod season;

// Conditionally compile and declare the `zoned` module only when the `timezone` feature is enabled.
#[cfg(feature = "timezone")]
mod zoned;

// Conditionally compile the tests module, ensuring it's only included during `cargo test`.
#[cfg(test)]
mod tests;

// --- Public API Re-exports ---
// Re-export the core public types to make them accessible directly from the crate root
// (e.g., `use parsidate::ParsiDate;` instead of `use parsidate::date::ParsiDate;`).

pub use constants::{MAX_PARSI_DATE, MIN_PARSI_DATE};
pub use date::ParsiDate;
pub use datetime::ParsiDateTime;
pub use error::{DateError, ParseErrorKind};
pub use season::Season;

// Conditionally re-export the `ZonedParsiDateTime` struct if the `timezone` feature is active.
#[cfg(feature = "timezone")]
pub use zoned::ZonedParsiDateTime;
