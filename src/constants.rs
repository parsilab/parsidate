// ~/src/constants.rs
//
//  * Copyright (C) ParsiCore (parsidate) 2024-2025 <parsicore.dev@gmail.com>
//  * Package : parsidate
//  * License : Apache-2.0
//  * Version : 1.7.1
//  * URL     : https://github.com/parsicore/parsidate
//  * Sign: parsidate-20250607-fea13e856dcd-459c6e73c83e49e10162ee28b26ac7cd
//
//! # Library Constants
//!
//! This module centralizes constant definitions used throughout the `parsidate` library.
//! It includes fundamental values such as the supported date range, as well as internal
//! helper constants for names of months, weekdays, and seasons.
//!
//! Centralizing these values ensures consistency, simplifies maintenance, and clarifies
//! the library's operational boundaries and conventions.

use crate::date::ParsiDate;

// --- Public Constants ---

/// The minimum supported `ParsiDate`: Year 1, Month 1 (Farvardin), Day 1.
///
/// This constant defines the lower boundary for all date operations within the library.
/// It represents the epoch start of the Persian (Jalali) calendar, which corresponds
/// to the Gregorian date March 21, 622 CE in the proleptic Gregorian calendar.
///
/// # Examples
///
/// ```rust
/// use parsidate::{ParsiDate, MIN_PARSI_DATE};
///
/// let first_day_ever = ParsiDate::new(1, 1, 1).unwrap();
/// assert_eq!(first_day_ever, MIN_PARSI_DATE);
///
/// // Any attempt to create a date before this will fail.
/// assert!(ParsiDate::new(0, 12, 29).is_err());
/// ```
pub const MIN_PARSI_DATE: ParsiDate = ParsiDate {
    year: 1,
    month: 1,
    day: 1,
};

/// The maximum supported `ParsiDate`: Year 9999, Month 12 (Esfand), Day 29.
///
/// This constant defines the upper boundary for all date operations. The year 9999
/// is chosen as a practical upper limit for the library.
///
/// According to the 33-year cycle algorithm used for leap year calculations in this library,
/// the year 9999 is a common (non-leap) year (`9999 % 33 == 3`), and therefore its final
/// month, Esfand, contains 29 days.
///
/// # Examples
///
/// ```rust
/// use parsidate::{ParsiDate, MAX_PARSI_DATE};
///
/// let last_supported_day = ParsiDate::new(9999, 12, 29).unwrap();
/// assert_eq!(last_supported_day, MAX_PARSI_DATE);
///
/// // Any attempt to create a date after this will fail.
/// assert!(ParsiDate::new(10000, 1, 1).is_err());
///
/// // The 30th of Esfand in year 9999 is invalid.
/// assert!(ParsiDate::new(9999, 12, 30).is_err());
/// ```
pub const MAX_PARSI_DATE: ParsiDate = ParsiDate {
    year: 9999,
    month: 12,
    day: 29,
};

// --- Internal Helper Constants ---

/// An array of Persian month names, indexed from 0.
///
/// This is used internally for formatting dates, specifically for the `%B` format specifier
/// in methods like [`ParsiDate::format_strftime`] and [`ParsiDateTime::format`].
///
/// - `index 0`: "فروردین" (Farvardin)
/// - `index 1`: "اردیبهشت" (Ordibehesht)
/// - ...
/// - `index 11`: "اسفند" (Esfand)
pub(crate) const MONTH_NAMES_PERSIAN: [&str; 12] = [
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

/// An array of Persian weekday names, indexed from 0, starting with Saturday.
///
/// This is used internally for formatting dates, specifically for the `%A` format specifier.
/// The indexing follows the convention where Saturday is the beginning of the week.
///
/// - `index 0`: "شنبه" (Shanbeh / Saturday)
/// - `index 1`: "یکشنبه" (Yekshanbeh / Sunday)
/// - ...
/// - `index 6`: "جمعه" (Jomeh / Friday)
pub(crate) const WEEKDAY_NAMES_PERSIAN: [&str; 7] = [
    "شنبه",
    "یکشنبه",
    "دوشنبه",
    "سه‌شنبه",
    "چهارشنبه",
    "پنجشنبه",
    "جمعه",
];

/// An array of Persian season names, indexed from 0.
///
/// This is used internally by the [`Season`](crate::season::Season) enum to provide string representations,
/// for example, via the `%K` format specifier.
///
/// - `index 0`: "بهار" (Bahar / Spring)
/// - `index 1`: "تابستان" (Tabestan / Summer)
/// - `index 2`: "پاییز" (Paeez / Autumn)
/// - `index 3`: "زمستان" (Zemestan / Winter)
pub(crate) const SEASON_NAMES_PERSIAN: [&str; 4] = ["بهار", "تابستان", "پاییز", "زمستان"];

/// An array of English season names, corresponding to the Persian seasons.
///
/// This is used internally by the [`Season`](crate::season::Season) enum to provide English
/// string representations. The order matches `SEASON_NAMES_PERSIAN`.
///
/// - `index 0`: "Spring"
/// - `index 1`: "Summer"
/// - `index 2`: "Autumn"
/// - `index 3`: "Winter"
pub(crate) const SEASON_NAMES_ENGLISH: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"];
