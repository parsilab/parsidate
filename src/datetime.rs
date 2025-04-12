//! ~/src/datetime.rs
//
//  * Copyright (C) Mohammad (Sina) Jalalvandi 2024-2025 <jalalvandi.sina@gmail.com>
//  * Package : parsidate
//  * License : Apache-2.0
//  * Version : 1.4.1
//  * URL     : https://github.com/jalalvandi/parsidate
//  * Sign: parsidate-20250410-f747d2246203-e40c0c12e3ffd6632e4a2ccd1b2b7e7d
//
//! Contains the `ParsiDateTime` struct definition and its implementation for handling
//! date and time within the Persian (Jalali or Shamsi) calendar system.

use crate::constants::MONTH_NAMES_PERSIAN; // Reuse constants
use crate::date::ParsiDate;
use crate::error::{DateError, ParseErrorKind};
use crate::season::Season; // Import the Season enum
use chrono::{Duration, Local, NaiveDateTime, Timelike};
use std::fmt;
use std::ops::{Add, Sub};

// --- Data Structures ---

/// Represents a specific date and time in the Persian (Jalali or Shamsi) calendar system.
///
/// This struct combines a [`ParsiDate`] (representing the year, month, and day in the Persian calendar)
/// with time components (hour, minute, second). It facilitates operations involving both date and time,
/// such as creation, validation, conversion to/from Gregorian [`NaiveDateTime`], formatting, parsing,
/// and date/time arithmetic.
///
/// **Nanosecond Precision:** Note that while conversions from `chrono::NaiveDateTime` might involve
/// nanoseconds, this struct currently only stores precision up to the second. Nanoseconds are effectively
/// truncated during conversion to `ParsiDateTime` but are preserved during calculations involving `Duration`
/// by converting to Gregorian, performing the operation, and converting back.
///
/// **Supported Range:** The valid range for the date component is the same as [`ParsiDate`], typically
/// Persian years 1 through 9999. Time components must be valid according to a standard 24-hour clock
/// (Hour: 0-23, Minute: 0-59, Second: 0-59).
///
/// **Serialization:** If the `serde` feature is enabled, this struct derives `Serialize` and `Deserialize`.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParsiDateTime {
    /// The Persian date component ([`ParsiDate`]).
    date: ParsiDate,
    /// The hour component, based on a 24-hour clock (0-23).
    hour: u32,
    /// The minute component (0-59).
    minute: u32,
    /// The second component (0-59).
    second: u32,
}

// --- Core Implementation ---

impl ParsiDateTime {
    // --- Constructors and Converters ---

    /// Creates a new `ParsiDateTime` instance from individual Persian date and time components.
    ///
    /// This function performs validation on both the date and time parts.
    /// 1. The `year`, `month`, and `day` must form a valid date in the Persian calendar
    ///    (e.g., 1403/12/30 is valid in a leap year, but 1404/12/30 is not).
    /// 2. The `hour` must be between 0 and 23 (inclusive).
    /// 3. The `minute` must be between 0 and 59 (inclusive).
    /// 4. The `second` must be between 0 and 59 (inclusive).
    ///
    /// # Arguments
    ///
    /// * `year`: The Persian year (e.g., 1403).
    /// * `month`: The Persian month (1 for Farvardin, 12 for Esfand).
    /// * `day`: The day of the month (1-29, 30, or 31 depending on month and leap year).
    /// * `hour`: The hour of the day (0-23).
    /// * `minute`: The minute of the hour (0-59).
    /// * `second`: The second of the minute (0-59).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the combination of `year`, `month`, and `day`
    /// does not form a valid Persian date according to [`ParsiDate::new`].
    /// Returns `Err(DateError::InvalidTime)` if `hour`, `minute`, or `second` are outside
    /// their valid ranges.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, DateError, ParsiDate};
    ///
    /// // Create a valid ParsiDateTime
    /// let dt_result = ParsiDateTime::new(1403, 5, 2, 15, 30, 45);
    /// assert!(dt_result.is_ok());
    /// let dt = dt_result.unwrap();
    /// assert_eq!(dt.date(), ParsiDate::new(1403, 5, 2).unwrap());
    /// assert_eq!(dt.hour(), 15);
    ///
    /// // Example of an invalid date
    /// assert_eq!(
    ///     ParsiDateTime::new(1404, 12, 30, 10, 0, 0),
    ///     Err(DateError::InvalidDate) // 1404 is not a leap year
    /// );
    ///
    /// // Example of an invalid time
    /// assert_eq!(
    ///     ParsiDateTime::new(1403, 5, 2, 24, 0, 0),
    ///     Err(DateError::InvalidTime) // Hour 24
    /// );
    /// ```
    pub fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Result<Self, DateError> {
        let date = ParsiDate::new(year, month, day)?;
        if hour > 23 || minute > 59 || second > 59 {
            return Err(DateError::InvalidTime);
        }
        Ok(ParsiDateTime {
            date,
            hour,
            minute,
            second,
        })
    }

    /// Creates a `ParsiDateTime` from components without performing any validation checks.
    ///
    /// **Warning:** This function is marked `unsafe` because it bypasses all validity checks
    /// for both the date (`year`, `month`, `day`) and time (`hour`, `minute`, `second`) components.
    /// Using invalid components (e.g., month 13, day 32, hour 25) will result in a `ParsiDateTime`
    /// instance containing invalid data. This can lead to unexpected behavior, panics, or incorrect
    /// results in subsequent operations.
    ///
    /// Only use this function if you can absolutely guarantee that all input components represent
    /// a valid Persian date and time. Prefer using the safe [`ParsiDateTime::new`] constructor.
    ///
    /// # Safety
    ///
    /// The caller *must* ensure that:
    /// 1. `year`, `month`, and `day` form a valid date in the Persian calendar.
    /// 2. `hour` is in the range 0-23.
    /// 3. `minute` is in the range 0-59.
    /// 4. `second` is in the range 0-59.
    ///
    /// Failure to meet these requirements constitutes undefined behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use parsidate::{ParsiDate, ParsiDateTime};
    /// // Assume components have been rigorously validated elsewhere
    /// let p_year = 1403; let p_month = 10; let p_day = 11;
    /// let hour = 9; let minute = 0; let second = 0;
    /// // Assume is_valid_date and is_valid_time checks passed
    /// // Safe to use new_unchecked because we *know* the inputs are valid
    /// let dt = unsafe {
    ///     ParsiDateTime::new_unchecked(p_year, p_month, p_day, hour, minute, second)
    /// };
    /// assert_eq!(dt.year(), 1403);
    /// assert_eq!(dt.hour(), 9);
    /// assert!(dt.is_valid());
    ///
    /// // --- Incorrect Usage (creating an invalid time) ---
    /// let invalid_dt = unsafe { ParsiDateTime::new_unchecked(1403, 1, 1, 25, 0, 0) };
    /// assert!(!invalid_dt.is_valid()); // Fails validation
    /// ```
    pub const unsafe fn new_unchecked(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Self {
        ParsiDateTime {
            date: unsafe { ParsiDate::new_unchecked(year, month, day) },
            hour,
            minute,
            second,
        }
    }

    /// Creates a `ParsiDateTime` from a pre-validated `ParsiDate` object and time components.
    ///
    /// This function assumes the provided `date` argument is already a valid `ParsiDate`.
    /// It only performs validation on the `hour`, `minute`, and `second` components.
    ///
    /// This can be slightly more efficient than `ParsiDateTime::new` if you already have a valid
    /// `ParsiDate` instance.
    ///
    /// # Arguments
    ///
    /// * `date`: A valid `ParsiDate` object.
    /// * `hour`: The hour of the day (0-23).
    /// * `minute`: The minute of the hour (0-59).
    /// * `second`: The second of the minute (0-59).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidTime)` if `hour`, `minute`, or `second` are outside
    /// their valid ranges.
    /// Note: If an invalid `ParsiDate` is passed, the function might return `Ok`, but the resulting `ParsiDateTime`
    /// will contain an invalid date part. Use [`ParsiDateTime::is_valid`] afterwards if unsure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, ParsiDateTime, DateError};
    ///
    /// let my_date = ParsiDate::new(1399, 11, 22).unwrap();
    /// let dt_result = ParsiDateTime::from_date_and_time(my_date, 20, 15, 0);
    /// assert!(dt_result.is_ok());
    /// assert_eq!(dt_result.unwrap().date(), my_date);
    ///
    /// assert_eq!(
    ///     ParsiDateTime::from_date_and_time(my_date, 10, 60, 0), // Invalid minute
    ///     Err(DateError::InvalidTime)
    /// );
    /// ```
    pub fn from_date_and_time(
        date: ParsiDate,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Result<Self, DateError> {
        if hour > 23 || minute > 59 || second > 59 {
            return Err(DateError::InvalidTime);
        }
        Ok(ParsiDateTime {
            date,
            hour,
            minute,
            second,
        })
    }

    /// Converts a Gregorian `chrono::NaiveDateTime` to its equivalent `ParsiDateTime`.
    ///
    /// This function first converts the date part (`NaiveDate`) to `ParsiDate` using
    /// [`ParsiDate::from_gregorian`] and then combines it with the time components
    /// (hour, minute, second) from the `NaiveDateTime`. Nanoseconds from the input
    /// `NaiveDateTime` are ignored.
    ///
    /// # Arguments
    ///
    /// * `gregorian_dt`: The `chrono::NaiveDateTime` instance to convert.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::GregorianConversionError)` if the date part conversion fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    ///
    /// let g_dt = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap()
    ///                 .and_hms_opt(15, 30, 45).unwrap();
    /// let pd_dt_result = ParsiDateTime::from_gregorian(g_dt);
    /// assert!(pd_dt_result.is_ok());
    /// let pd_dt = pd_dt_result.unwrap();
    /// assert_eq!(pd_dt.date(), ParsiDate::new(1403, 5, 2).unwrap());
    /// assert_eq!(pd_dt.time(), (15, 30, 45));
    ///
    /// // Example of a date before the Persian epoch
    /// let g_dt_early = NaiveDate::from_ymd_opt(600, 1, 1).unwrap().and_hms_opt(0,0,0).unwrap();
    /// assert_eq!(ParsiDateTime::from_gregorian(g_dt_early), Err(DateError::GregorianConversionError));
    /// ```
    pub fn from_gregorian(gregorian_dt: NaiveDateTime) -> Result<Self, DateError> {
        let parsi_date = ParsiDate::from_gregorian(gregorian_dt.date())?;
        let hour = gregorian_dt.hour();
        let minute = gregorian_dt.minute();
        let second = gregorian_dt.second();
        // Construct directly as time components from NaiveDateTime are guaranteed valid
        Ok(ParsiDateTime {
            date: parsi_date,
            hour,
            minute,
            second,
        })
    }

    /// Converts this `ParsiDateTime` instance to its equivalent Gregorian `chrono::NaiveDateTime`.
    ///
    /// This function first checks if the `ParsiDateTime` itself is valid. If it is, it converts
    /// the `ParsiDate` component to a `chrono::NaiveDate` and then combines it with the
    /// stored `hour`, `minute`, and `second` to create the `NaiveDateTime`.
    ///
    /// # Errors
    ///
    /// Returns `Err` in the following cases:
    /// *   `DateError::InvalidDate`: If the date part (`self.date`) is invalid.
    /// *   `DateError::InvalidTime`: If the time part (`self.hour`, `self.minute`, `self.second`) is invalid.
    /// *   `DateError::GregorianConversionError`: If the conversion of the valid `ParsiDate` component
    ///     to `NaiveDate` fails internally or if combining with valid time fails unexpectedly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    ///
    /// let pd_dt = ParsiDateTime::new(1403, 5, 2, 15, 30, 45).unwrap();
    /// let g_dt_result = pd_dt.to_gregorian();
    /// assert!(g_dt_result.is_ok());
    /// let g_dt = g_dt_result.unwrap();
    /// let expected_g_dt = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap()
    ///                         .and_hms_opt(15, 30, 45).unwrap();
    /// assert_eq!(g_dt, expected_g_dt);
    ///
    /// // Example with an invalid ParsiDateTime created via unsafe
    /// let invalid_pd_dt = unsafe { ParsiDateTime::new_unchecked(1404, 12, 30, 10, 0, 0) }; // Invalid date
    /// assert!(!invalid_pd_dt.is_valid());
    /// assert_eq!(invalid_pd_dt.to_gregorian(), Err(DateError::InvalidDate));
    /// ```
    pub fn to_gregorian(&self) -> Result<NaiveDateTime, DateError> {
        if !self.is_valid() {
            if !self.date.is_valid() {
                return Err(DateError::InvalidDate);
            } else {
                return Err(DateError::InvalidTime);
            }
        }
        let gregorian_date = self.date.to_gregorian_internal()?;
        gregorian_date
            .and_hms_opt(self.hour, self.minute, self.second)
            .ok_or(DateError::GregorianConversionError)
    }

    /// Returns the current system date and time, converted to `ParsiDateTime`.
    ///
    /// This function obtains the current local date and time from the operating system
    /// using `chrono::Local::now()`, gets the naive representation (without timezone),
    /// and then converts this `NaiveDateTime` to `ParsiDateTime` using [`from_gregorian`].
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::GregorianConversionError)` if the conversion from the current
    /// Gregorian date/time provided by the system fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// match ParsiDateTime::now() {
    ///     Ok(now) => {
    ///         println!("Current Persian date and time: {}", now);
    ///     }
    ///     Err(e) => {
    ///         eprintln!("Failed to get current Persian date and time: {}", e);
    ///     }
    /// }
    /// ```
    pub fn now() -> Result<Self, DateError> {
        let now_local: chrono::DateTime<Local> = Local::now();
        let naive_local: NaiveDateTime = now_local.naive_local();
        Self::from_gregorian(naive_local)
    }

    // --- Accessors ---

    /// Returns the [`ParsiDate`] component of this `ParsiDateTime`.
    #[inline]
    pub const fn date(&self) -> ParsiDate {
        self.date
    }

    /// Returns the year component of the Persian date.
    #[inline]
    pub const fn year(&self) -> i32 {
        self.date.year()
    }

    /// Returns the month component of the Persian date (1-12).
    #[inline]
    pub const fn month(&self) -> u32 {
        self.date.month()
    }

    /// Returns the day component of the Persian date (1-31).
    #[inline]
    pub const fn day(&self) -> u32 {
        self.date.day()
    }

    /// Returns the hour component of the time (0-23).
    #[inline]
    pub const fn hour(&self) -> u32 {
        self.hour
    }

    /// Returns the minute component of the time (0-59).
    #[inline]
    pub const fn minute(&self) -> u32 {
        self.minute
    }

    /// Returns the second component of the time (0-59).
    #[inline]
    pub const fn second(&self) -> u32 {
        self.second
    }

    /// Returns the time components as a tuple `(hour, minute, second)`.
    #[inline]
    pub const fn time(&self) -> (u32, u32, u32) {
        (self.hour, self.minute, self.second)
    }

    /// Returns the Persian season this `ParsiDateTime`'s date falls into.
    ///
    /// Delegates to [`ParsiDate::season`]. See its documentation for details.
    ///
    /// # Errors
    /// Returns `Err(DateError::InvalidDate)` if the date part is invalid.
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, Season};
    ///
    /// // Summer date
    /// let dt = ParsiDateTime::new(1403, 5, 10, 12, 0, 0).unwrap();
    /// assert_eq!(dt.season(), Ok(Season::Tabestan));
    /// ```
    #[inline]
    pub fn season(&self) -> Result<Season, DateError> {
        self.date.season() // Delegate to ParsiDate's season method
    }

    // --- Validation ---

    /// Checks if the current `ParsiDateTime` instance represents a valid Persian date and time.
    ///
    /// Checks if the date part is valid via [`ParsiDate::is_valid`] and if the time components
    /// (hour, minute, second) are within their valid ranges (0-23, 0-59, 0-59).
    ///
    /// # Returns
    ///
    /// *   `true` if both the date part and the time part are valid.
    /// *   `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// // Valid instance
    /// let valid_dt = ParsiDateTime::new(1403, 12, 30, 23, 59, 59).unwrap(); // Leap year end
    /// assert!(valid_dt.is_valid());
    ///
    /// // Instance with invalid time created via `unsafe new_unchecked`
    /// let invalid_time_dt = unsafe { ParsiDateTime::new_unchecked(1403, 1, 1, 24, 0, 0) };
    /// assert!(!invalid_time_dt.is_valid());
    ///
    /// // Instance with invalid date created via `unsafe new_unchecked`
    /// let invalid_date_dt = unsafe { ParsiDateTime::new_unchecked(1404, 12, 30, 10, 0, 0) };
    /// assert!(!invalid_date_dt.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.date.is_valid() && self.hour <= 23 && self.minute <= 59 && self.second <= 59
    }

    // --- Formatting ---

    /// Formats the `ParsiDateTime` into a string according to a given format pattern.
    ///
    /// Works like `strftime`, interpreting percent-prefixed specifiers for date and time components.
    ///
    /// # Supported Format Specifiers
    ///
    /// **Date Specifiers (from `ParsiDate`):**
    /// *   `%Y`: Year (e.g., `1403`).
    /// *   `%m`: Month (01-12).
    /// *   `%d`: Day (01-31).
    /// *   `%B`: Full Persian month name (e.g., "مرداد").
    /// *   `%A`: Full Persian weekday name (e.g., "سه‌شنبه").
    /// *   `%w`: Weekday number (Saturday=0, ..., Friday=6).
    /// *   `%j`: Day of the year (001-366).
    /// *   `%K`: Full Persian season name (e.g., "تابستان").
    /// *   `%%`: Literal `%`.
    ///
    /// **Time Specifiers:**
    /// *   `%H`: Hour (00-23).
    /// *   `%M`: Minute (00-59).
    /// *   `%S`: Second (00-59).
    /// *   `%T`: Equivalent to `%H:%M:%S`.
    ///
    /// # Arguments
    /// * `pattern`: The format string with specifiers.
    ///
    /// # Returns
    /// A `String` containing the formatted date and time. Error indicators might appear for invalid underlying data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// let dt = ParsiDateTime::new(1403, 5, 2, 8, 5, 30).unwrap(); // 1403/Mordad/02 08:05:30 (Tuesday, Tabestan)
    ///
    /// assert_eq!(dt.format("%Y-%m-%d %H:%M:%S"), "1403-05-02 08:05:30");
    /// assert_eq!(dt.format("%d %B (%K) %Y ساعت %T"), "02 مرداد (تابستان) 1403 ساعت 08:05:30");
    /// assert_eq!(dt.format("%A - %Y/%m/%d"), "سه‌شنبه - 1403/05/02");
    /// ```
    pub fn format(&self, pattern: &str) -> String {
        let mut result = String::with_capacity(pattern.len() + 20);
        let mut chars = pattern.chars().peekable();

        // Caching for date-related calculated values
        let mut weekday_name_cache: Option<Result<String, DateError>> = None;
        let mut ordinal_day_cache: Option<Result<u32, DateError>> = None;
        let mut weekday_num_cache: Option<Result<u32, DateError>> = None;
        let mut season_cache: Option<Result<Season, DateError>> = None;

        while let Some(c) = chars.next() {
            if c == '%' {
                match chars.next() {
                    // Time
                    Some('H') => result.push_str(&format!("{:02}", self.hour)),
                    Some('M') => result.push_str(&format!("{:02}", self.minute)),
                    Some('S') => result.push_str(&format!("{:02}", self.second)),
                    Some('T') => result.push_str(&format!(
                        "{:02}:{:02}:{:02}",
                        self.hour, self.minute, self.second
                    )),
                    // Date (delegate or access directly)
                    Some('%') => result.push('%'),
                    Some('Y') => result.push_str(&self.year().to_string()),
                    Some('m') => result.push_str(&format!("{:02}", self.month())),
                    Some('d') => result.push_str(&format!("{:02}", self.day())),
                    Some('B') => {
                        let month_index = self.month().saturating_sub(1) as usize;
                        if let Some(name) = MONTH_NAMES_PERSIAN.get(month_index) {
                            result.push_str(name);
                        } else {
                            result.push_str("?InvalidMonth?");
                        }
                    }
                    Some('A') => {
                        if weekday_name_cache.is_none() {
                            weekday_name_cache = Some(self.date.weekday_internal());
                        }
                        match weekday_name_cache.as_ref().unwrap() {
                            Ok(name) => result.push_str(name),
                            Err(_) => result.push_str("?WeekdayError?"),
                        }
                    }
                    Some('w') => {
                        if weekday_num_cache.is_none() {
                            weekday_num_cache = Some(self.date.weekday_num_sat_0());
                        }
                        match weekday_num_cache.as_ref().unwrap() {
                            Ok(num) => result.push_str(&num.to_string()),
                            Err(_) => result.push('?'),
                        }
                    }
                    Some('j') => {
                        if ordinal_day_cache.is_none() {
                            ordinal_day_cache = Some(self.date.ordinal_internal());
                        }
                        match ordinal_day_cache.as_ref().unwrap() {
                            Ok(ord) => result.push_str(&format!("{:03}", ord)),
                            Err(_) => result.push_str("???"),
                        }
                    }
                    Some('K') => {
                        if season_cache.is_none() {
                            season_cache = Some(self.date.season());
                        }
                        match season_cache.as_ref().unwrap() {
                            Ok(season) => result.push_str(season.name_persian()),
                            Err(_) => result.push_str("?SeasonError?"),
                        }
                    }
                    // Unknown
                    Some(other) => {
                        result.push('%');
                        result.push(other);
                    }
                    None => {
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

    /// Parses a string containing a Persian date and time into a `ParsiDateTime` instance,
    /// based on a specified format pattern.
    ///
    /// Requires an exact match between the input string and the format pattern, including literals.
    /// Validates the extracted components to ensure they form a valid date and time.
    ///
    /// # Supported Format Specifiers for Parsing
    ///
    /// *   `%Y`: 4-digit year.
    /// *   `%m`: 2-digit month (01-12).
    /// *   `%d`: 2-digit day (01-31).
    /// *   `%B`: Full Persian month name (e.g., "فروردین").
    /// *   `%H`: 2-digit hour (00-23).
    /// *   `%M`: 2-digit minute (00-59).
    /// *   `%S`: 2-digit second (00-59).
    /// *   `%T`: Time in "HH:MM:SS" format.
    /// *   `%%`: Literal `%`.
    ///
    /// **Unsupported:** `%A`, `%w`, `%j`, `%K` are not supported for parsing.
    ///
    /// # Arguments
    /// * `s`: The input string slice.
    /// * `format`: The format string pattern.
    ///
    /// # Errors
    /// Returns `Err(DateError::ParseError(kind))` on failure, with `kind` indicating the reason
    /// (e.g., `FormatMismatch`, `InvalidNumber`, `InvalidMonthName`, `UnsupportedSpecifier`, `InvalidDateValue`, `InvalidTimeValue`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError, ParseErrorKind};
    ///
    /// let s1 = "1403/05/02 15:30:45";
    /// let fmt1 = "%Y/%m/%d %H:%M:%S";
    /// let expected1 = ParsiDateTime::new(1403, 5, 2, 15, 30, 45).unwrap();
    /// assert_eq!(ParsiDateTime::parse(s1, fmt1), Ok(expected1));
    ///
    /// // Using %T
    /// let s2 = "1403-05-02T09:05:00";
    /// let fmt2 = "%Y-%m-%dT%T";
    /// assert!(ParsiDateTime::parse(s2, fmt2).is_ok());
    ///
    /// // Using %B
    /// let s3 = "22 بهمن 1399 - 23:59:59";
    /// let fmt3 = "%d %B %Y - %T";
    /// assert!(ParsiDateTime::parse(s3, fmt3).is_ok());
    ///
    /// // Error: Invalid time value (hour 24)
    /// assert_eq!(ParsiDateTime::parse("1403/05/02 24:00:00", fmt1),
    ///            Err(DateError::ParseError(ParseErrorKind::InvalidTimeValue)));
    ///
    /// // Error: Invalid date value (Esfand 30 in non-leap year 1404)
    /// assert_eq!(ParsiDateTime::parse("1404/12/30 10:00:00", fmt1),
    ///            Err(DateError::ParseError(ParseErrorKind::InvalidDateValue)));
    ///
    /// // Error: Format mismatch
    /// assert_eq!(ParsiDateTime::parse("1403/05/02 15-30-45", fmt1),
    ///            Err(DateError::ParseError(ParseErrorKind::FormatMismatch)));
    /// ```
    pub fn parse(s: &str, format: &str) -> Result<Self, DateError> {
        let mut parsed_year: Option<i32> = None;
        let mut parsed_month: Option<u32> = None;
        let mut parsed_day: Option<u32> = None;
        let mut parsed_hour: Option<u32> = None;
        let mut parsed_minute: Option<u32> = None;
        let mut parsed_second: Option<u32> = None;

        let mut s_bytes = s.as_bytes();
        let mut fmt_bytes = format.as_bytes();

        while !fmt_bytes.is_empty() {
            if fmt_bytes[0] == b'%' {
                if fmt_bytes.len() < 2 {
                    return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                }
                match fmt_bytes[1] {
                    // Time
                    b'H' | b'M' | b'S' => {
                        if s_bytes.len() < 2 || !s_bytes[0..2].iter().all(|b| b.is_ascii_digit()) {
                            return Err(DateError::ParseError(ParseErrorKind::InvalidNumber));
                        }
                        let num_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..2]) };
                        let val: u32 = num_str
                            .parse()
                            .map_err(|_| DateError::ParseError(ParseErrorKind::InvalidNumber))?;
                        match fmt_bytes[1] {
                            b'H' => parsed_hour = Some(val),
                            b'M' => parsed_minute = Some(val),
                            b'S' => parsed_second = Some(val),
                            _ => unreachable!(),
                        }
                        s_bytes = &s_bytes[2..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    b'T' => {
                        if s_bytes.len() < 8
                            || !s_bytes[0..2].iter().all(|b| b.is_ascii_digit())
                            || s_bytes[2] != b':'
                            || !s_bytes[3..5].iter().all(|b| b.is_ascii_digit())
                            || s_bytes[5] != b':'
                            || !s_bytes[6..8].iter().all(|b| b.is_ascii_digit())
                        {
                            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                        }
                        let h_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..2]) };
                        let m_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[3..5]) };
                        let s_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[6..8]) };
                        parsed_hour =
                            Some(h_str.parse().map_err(|_| {
                                DateError::ParseError(ParseErrorKind::InvalidNumber)
                            })?);
                        parsed_minute =
                            Some(m_str.parse().map_err(|_| {
                                DateError::ParseError(ParseErrorKind::InvalidNumber)
                            })?);
                        parsed_second =
                            Some(s_str.parse().map_err(|_| {
                                DateError::ParseError(ParseErrorKind::InvalidNumber)
                            })?);
                        s_bytes = &s_bytes[8..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    // Date
                    b'%' => {
                        if s_bytes.is_empty() || s_bytes[0] != b'%' {
                            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                        }
                        s_bytes = &s_bytes[1..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    b'Y' => {
                        if s_bytes.len() < 4 || !s_bytes[0..4].iter().all(|b| b.is_ascii_digit()) {
                            return Err(DateError::ParseError(ParseErrorKind::InvalidNumber));
                        }
                        let year_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..4]) };
                        parsed_year =
                            Some(year_str.parse().map_err(|_| {
                                DateError::ParseError(ParseErrorKind::InvalidNumber)
                            })?);
                        s_bytes = &s_bytes[4..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    b'm' | b'd' => {
                        if s_bytes.len() < 2 || !s_bytes[0..2].iter().all(|b| b.is_ascii_digit()) {
                            return Err(DateError::ParseError(ParseErrorKind::InvalidNumber));
                        }
                        let num_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..2]) };
                        let val: u32 = num_str
                            .parse()
                            .map_err(|_| DateError::ParseError(ParseErrorKind::InvalidNumber))?;
                        if fmt_bytes[1] == b'm' {
                            parsed_month = Some(val);
                        } else {
                            parsed_day = Some(val);
                        }
                        s_bytes = &s_bytes[2..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    b'B' => {
                        fmt_bytes = &fmt_bytes[2..];
                        let mut found_month = false;
                        let mut best_match_len = 0;
                        let mut matched_month_idx = 0;
                        let current_s_str = match std::str::from_utf8(s_bytes) {
                            Ok(s_str) => s_str,
                            Err(_) => {
                                return Err(DateError::ParseError(
                                    ParseErrorKind::InvalidMonthName,
                                ));
                            }
                        };
                        for (idx, month_name) in MONTH_NAMES_PERSIAN.iter().enumerate() {
                            if current_s_str.starts_with(month_name) {
                                best_match_len = month_name.len();
                                matched_month_idx = idx;
                                found_month = true;
                                break;
                            }
                        }
                        if !found_month {
                            return Err(DateError::ParseError(ParseErrorKind::InvalidMonthName));
                        }
                        parsed_month = Some((matched_month_idx + 1) as u32);
                        s_bytes = &s_bytes[best_match_len..];
                    }
                    // Unsupported
                    b'A' | b'w' | b'j' | b'K' => {
                        return Err(DateError::ParseError(ParseErrorKind::UnsupportedSpecifier));
                    }
                    _ => return Err(DateError::ParseError(ParseErrorKind::UnsupportedSpecifier)),
                }
            } else {
                // Literal
                if s_bytes.is_empty() || s_bytes[0] != fmt_bytes[0] {
                    return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                }
                s_bytes = &s_bytes[1..];
                fmt_bytes = &fmt_bytes[1..];
            }
        }

        if !s_bytes.is_empty() {
            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
        }

        match (
            parsed_year,
            parsed_month,
            parsed_day,
            parsed_hour,
            parsed_minute,
            parsed_second,
        ) {
            (Some(y), Some(m), Some(d), Some(h), Some(min), Some(s)) => {
                ParsiDateTime::new(y, m, d, h, min, s).map_err(|e| match e {
                    DateError::InvalidDate => {
                        DateError::ParseError(ParseErrorKind::InvalidDateValue)
                    }
                    DateError::InvalidTime => {
                        DateError::ParseError(ParseErrorKind::InvalidTimeValue)
                    }
                    other_error => other_error,
                })
            }
            _ => Err(DateError::ParseError(ParseErrorKind::FormatMismatch)),
        }
    }

    // --- Arithmetic ---

    /// Adds a `chrono::Duration` to this `ParsiDateTime`.
    ///
    /// Converts to Gregorian `NaiveDateTime`, adds the duration, and converts back.
    /// Handles date and time rollovers correctly.
    ///
    /// # Arguments
    /// * `duration`: The `chrono::Duration` to add (can be positive or negative).
    ///
    /// # Errors
    /// Returns `Err` if the initial `ParsiDateTime` is invalid, conversion fails, or arithmetic overflows.
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    /// use chrono::Duration;
    ///
    /// let dt = ParsiDateTime::new(1403, 1, 1, 23, 59, 58).unwrap();
    /// // Add 3 seconds, crossing midnight
    /// let dt_next_day = dt.add_duration(Duration::seconds(3)).unwrap();
    /// assert_eq!(dt_next_day.date(), ParsiDate::new(1403, 1, 2).unwrap());
    /// assert_eq!(dt_next_day.time(), (0, 0, 1));
    ///
    /// // Add 25 hours
    /// let dt_plus_25h = dt.add_duration(Duration::hours(25)).unwrap();
    /// assert_eq!(dt_plus_25h.date(), ParsiDate::new(1403, 1, 3).unwrap()); // Day advances by 1 (+1 hr remains)
    /// assert_eq!(dt_plus_25h.time(), (0, 59, 58)); // 23:59:58 + 1hr
    /// ```
    pub fn add_duration(&self, duration: Duration) -> Result<Self, DateError> {
        if !self.is_valid() {
            return Err(if !self.date.is_valid() {
                DateError::InvalidDate
            } else {
                DateError::InvalidTime
            });
        }
        let gregorian_dt = self.to_gregorian()?;
        let new_gregorian_dt = gregorian_dt
            .checked_add_signed(duration)
            .ok_or(DateError::ArithmeticOverflow)?;
        Self::from_gregorian(new_gregorian_dt)
    }

    /// Subtracts a `chrono::Duration` from this `ParsiDateTime`.
    ///
    /// Equivalent to `add_duration` with a negated duration.
    ///
    /// # Arguments
    /// * `duration`: The `chrono::Duration` to subtract.
    ///
    /// # Errors
    /// Returns `Err` under the same conditions as [`add_duration`].
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    /// use chrono::Duration;
    ///
    /// let dt = ParsiDateTime::new(1403, 1, 1, 0, 0, 5).unwrap(); // 5 seconds past midnight
    /// // Subtract 10 seconds, crossing midnight backwards
    /// let dt_prev_day = dt.sub_duration(Duration::seconds(10)).unwrap();
    /// assert_eq!(dt_prev_day.date(), ParsiDate::new(1402, 12, 29).unwrap()); // Esfand 29 (1402 common)
    /// assert_eq!(dt_prev_day.time(), (23, 59, 55));
    /// ```
    pub fn sub_duration(&self, duration: Duration) -> Result<Self, DateError> {
        self.add_duration(-duration)
    }

    /// Adds a specified number of days to the date part, preserving the time component.
    ///
    /// Delegates date calculation to [`ParsiDate::add_days`]. Time remains unchanged.
    ///
    /// # Arguments
    /// * `days`: Number of days to add (can be negative).
    ///
    /// # Errors
    /// Returns `Err` if the initial `ParsiDateTime` is invalid or if `ParsiDate::add_days` fails.
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate};
    ///
    /// let dt = ParsiDateTime::new(1403, 1, 15, 10, 30, 0).unwrap();
    /// // Add 20 days
    /// let dt_plus_20d = dt.add_days(20).unwrap();
    /// assert_eq!(dt_plus_20d.date(), ParsiDate::new(1403, 2, 4).unwrap()); // 1403/02/04
    /// assert_eq!(dt_plus_20d.time(), (10, 30, 0)); // Time unchanged
    /// ```
    pub fn add_days(&self, days: i64) -> Result<Self, DateError> {
        if !self.is_valid() {
            return Err(if !self.date.is_valid() {
                DateError::InvalidDate
            } else {
                DateError::InvalidTime
            });
        }
        let new_date = self.date.add_days(days)?;
        Ok(ParsiDateTime {
            date: new_date,
            ..*self
        }) // Reuse other fields
    }

    /// Subtracts a specified number of days from the date part, preserving the time component.
    ///
    /// Delegates date calculation to [`ParsiDate::sub_days`]. Equivalent to `add_days(-days)`.
    ///
    /// # Arguments
    /// * `days`: Non-negative number of days to subtract.
    ///
    /// # Errors
    /// Returns `Err` under the same conditions as [`add_days`].
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate};
    ///
    /// let dt = ParsiDateTime::new(1403, 1, 15, 10, 30, 0).unwrap();
    /// // Subtract 20 days
    /// let dt_minus_20d = dt.sub_days(20).unwrap();
    /// assert_eq!(dt_minus_20d.date(), ParsiDate::new(1402, 12, 24).unwrap()); // 1402/12/24
    /// assert_eq!(dt_minus_20d.time(), (10, 30, 0)); // Time unchanged
    /// ```
    pub fn sub_days(&self, days: u64) -> Result<Self, DateError> {
        if !self.is_valid() {
            return Err(if !self.date.is_valid() {
                DateError::InvalidDate
            } else {
                DateError::InvalidTime
            });
        }
        let new_date = self.date.sub_days(days)?;
        Ok(ParsiDateTime {
            date: new_date,
            ..*self
        })
    }

    /// Adds months to the date part, preserving time and clamping day if necessary.
    ///
    /// Delegates date calculation to [`ParsiDate::add_months`].
    ///
    /// # Arguments
    /// * `months`: Number of months to add (can be negative).
    ///
    /// # Errors
    /// Returns `Err` if the initial `ParsiDateTime` is invalid or if `ParsiDate::add_months` fails.
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate};
    ///
    /// let dt = ParsiDateTime::new(1403, 6, 31, 12, 0, 0).unwrap(); // Shahrivar 31st
    /// // Add 1 month -> Mehr 30th (day clamped)
    /// let dt_plus_1m = dt.add_months(1).unwrap();
    /// assert_eq!(dt_plus_1m.date(), ParsiDate::new(1403, 7, 30).unwrap());
    /// assert_eq!(dt_plus_1m.time(), (12, 0, 0)); // Time unchanged
    /// ```
    pub fn add_months(&self, months: i32) -> Result<Self, DateError> {
        if !self.is_valid() {
            return Err(if !self.date.is_valid() {
                DateError::InvalidDate
            } else {
                DateError::InvalidTime
            });
        }
        let new_date = self.date.add_months(months)?;
        Ok(ParsiDateTime {
            date: new_date,
            ..*self
        })
    }

    /// Subtracts months from the date part, preserving time and clamping day.
    ///
    /// Delegates date calculation to [`ParsiDate::sub_months`]. Equivalent to `add_months(-months)`.
    ///
    /// # Arguments
    /// * `months`: Non-negative number of months to subtract.
    ///
    /// # Errors
    /// Returns `Err` under the same conditions as [`add_months`].
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate};
    ///
    /// let dt = ParsiDateTime::new(1403, 1, 31, 9, 0, 0).unwrap(); // Farvardin 31st
    /// // Subtract 7 months -> Shahrivar 31st prev year (1402)
    /// let dt_minus_7m = dt.sub_months(7).unwrap();
    /// assert_eq!(dt_minus_7m.date(), ParsiDate::new(1402, 6, 31).unwrap()); // Shahrivar has 31 days
    /// assert_eq!(dt_minus_7m.time(), (9, 0, 0));
    /// ```
    pub fn sub_months(&self, months: u32) -> Result<Self, DateError> {
        if !self.is_valid() {
            return Err(if !self.date.is_valid() {
                DateError::InvalidDate
            } else {
                DateError::InvalidTime
            });
        }
        let new_date = self.date.sub_months(months)?;
        Ok(ParsiDateTime {
            date: new_date,
            ..*self
        })
    }

    /// Adds years to the date part, preserving time and adjusting leap day if necessary.
    ///
    /// Delegates date calculation to [`ParsiDate::add_years`].
    ///
    /// # Arguments
    /// * `years`: Number of years to add (can be negative).
    ///
    /// # Errors
    /// Returns `Err` if the initial `ParsiDateTime` is invalid or if `ParsiDate::add_years` fails.
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate};
    ///
    /// let dt_leap_day = ParsiDateTime::new(1403, 12, 30, 10, 0, 0).unwrap(); // Esfand 30th, 1403 (leap)
    /// // Add 1 year -> 1404 (non-leap), day becomes 29
    /// let dt_next_year = dt_leap_day.add_years(1).unwrap();
    /// assert_eq!(dt_next_year.date(), ParsiDate::new(1404, 12, 29).unwrap());
    /// assert_eq!(dt_next_year.time(), (10, 0, 0)); // Time unchanged
    /// ```
    pub fn add_years(&self, years: i32) -> Result<Self, DateError> {
        if !self.is_valid() {
            return Err(if !self.date.is_valid() {
                DateError::InvalidDate
            } else {
                DateError::InvalidTime
            });
        }
        let new_date = self.date.add_years(years)?;
        Ok(ParsiDateTime {
            date: new_date,
            ..*self
        })
    }

    /// Subtracts years from the date part, preserving time and adjusting leap day.
    ///
    /// Delegates date calculation to [`ParsiDate::sub_years`]. Equivalent to `add_years(-years)`.
    ///
    /// # Arguments
    /// * `years`: Non-negative number of years to subtract.
    ///
    /// # Errors
    /// Returns `Err` under the same conditions as [`add_years`].
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate};
    ///
    /// let dt_leap_day = ParsiDateTime::new(1403, 12, 30, 10, 0, 0).unwrap(); // Esfand 30th, 1403 (leap)
    /// // Subtract 1 year -> 1402 (non-leap), day becomes 29
    /// let dt_prev_year = dt_leap_day.sub_years(1).unwrap();
    /// assert_eq!(dt_prev_year.date(), ParsiDate::new(1402, 12, 29).unwrap());
    /// assert_eq!(dt_prev_year.time(), (10, 0, 0));
    /// ```
    pub fn sub_years(&self, years: u32) -> Result<Self, DateError> {
        if !self.is_valid() {
            return Err(if !self.date.is_valid() {
                DateError::InvalidDate
            } else {
                DateError::InvalidTime
            });
        }
        let new_date = self.date.sub_years(years)?;
        Ok(ParsiDateTime {
            date: new_date,
            ..*self
        })
    }

    // --- Helper Methods ---

    /// Creates a new `ParsiDateTime` with only the hour component changed.
    ///
    /// # Arguments
    /// * `hour`: New hour (0-23).
    ///
    /// # Errors
    /// Returns `Err(DateError::InvalidTime)` if hour is invalid.
    /// Returns `Err(DateError::InvalidDate)` if the original date part was invalid.
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, DateError};
    /// let dt = ParsiDateTime::new(1403, 5, 2, 10, 30, 45).unwrap();
    /// assert_eq!(dt.with_hour(18).unwrap().hour(), 18);
    /// assert_eq!(dt.with_hour(24), Err(DateError::InvalidTime));
    /// ```
    pub fn with_hour(&self, hour: u32) -> Result<Self, DateError> {
        if !self.date.is_valid() {
            return Err(DateError::InvalidDate);
        }
        if hour > 23 {
            return Err(DateError::InvalidTime);
        }
        Ok(ParsiDateTime { hour, ..*self })
    }

    /// Creates a new `ParsiDateTime` with only the minute component changed.
    ///
    /// # Arguments
    /// * `minute`: New minute (0-59).
    ///
    /// # Errors
    /// Returns `Err(DateError::InvalidTime)` if minute is invalid.
    /// Returns `Err(DateError::InvalidDate)` if the original date part was invalid.
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, DateError};
    /// let dt = ParsiDateTime::new(1403, 5, 2, 10, 30, 45).unwrap();
    /// assert_eq!(dt.with_minute(55).unwrap().minute(), 55);
    /// assert_eq!(dt.with_minute(60), Err(DateError::InvalidTime));
    /// ```
    pub fn with_minute(&self, minute: u32) -> Result<Self, DateError> {
        if !self.date.is_valid() {
            return Err(DateError::InvalidDate);
        }
        if minute > 59 {
            return Err(DateError::InvalidTime);
        }
        Ok(ParsiDateTime { minute, ..*self })
    }

    /// Creates a new `ParsiDateTime` with only the second component changed.
    ///
    /// # Arguments
    /// * `second`: New second (0-59).
    ///
    /// # Errors
    /// Returns `Err(DateError::InvalidTime)` if second is invalid.
    /// Returns `Err(DateError::InvalidDate)` if the original date part was invalid.
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, DateError};
    /// let dt = ParsiDateTime::new(1403, 5, 2, 10, 30, 45).unwrap();
    /// assert_eq!(dt.with_second(0).unwrap().second(), 0);
    /// assert_eq!(dt.with_second(60), Err(DateError::InvalidTime));
    /// ```
    pub fn with_second(&self, second: u32) -> Result<Self, DateError> {
        if !self.date.is_valid() {
            return Err(DateError::InvalidDate);
        }
        if second > 59 {
            return Err(DateError::InvalidTime);
        }
        Ok(ParsiDateTime { second, ..*self })
    }

    /// Creates a new `ParsiDateTime` with new time components (hour, minute, second).
    ///
    /// The date component remains the same.
    ///
    /// # Arguments
    /// * `hour`: New hour (0-23).
    /// * `minute`: New minute (0-59).
    /// * `second`: New second (0-59).
    ///
    /// # Errors
    /// Returns `Err(DateError::InvalidTime)` if any time component is invalid.
    /// Returns `Err(DateError::InvalidDate)` if the original date part was invalid.
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, DateError};
    /// let dt = ParsiDateTime::new(1403, 5, 2, 10, 30, 45).unwrap();
    /// let new_dt = dt.with_time(23, 59, 59).unwrap();
    /// assert_eq!(new_dt.time(), (23, 59, 59));
    /// assert_eq!(dt.with_time(11, 60, 0), Err(DateError::InvalidTime));
    /// ```
    pub fn with_time(&self, hour: u32, minute: u32, second: u32) -> Result<Self, DateError> {
        if !self.date.is_valid() {
            return Err(DateError::InvalidDate);
        }
        if hour > 23 || minute > 59 || second > 59 {
            return Err(DateError::InvalidTime);
        }
        Ok(ParsiDateTime {
            date: self.date,
            hour,
            minute,
            second,
        })
    }

    /// Creates a new `ParsiDateTime` with only the year component of the date changed.
    ///
    /// Delegates to [`ParsiDate::with_year`], preserving time. Adjusts day for leap years if needed.
    ///
    /// # Arguments
    /// * `year`: New Persian year.
    ///
    /// # Errors
    /// Returns `Err` if `ParsiDate::with_year` fails.
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate};
    /// let dt_leap = ParsiDateTime::new(1403, 12, 30, 11, 0, 0).unwrap(); // Leap year
    /// // Change to non-leap year 1404 -> day becomes 29
    /// let dt_non_leap = dt_leap.with_year(1404).unwrap();
    /// assert_eq!(dt_non_leap.date(), ParsiDate::new(1404, 12, 29).unwrap());
    /// assert_eq!(dt_non_leap.time(), (11, 0, 0));
    /// ```
    pub fn with_year(&self, year: i32) -> Result<Self, DateError> {
        let new_date = self.date.with_year(year)?;
        Ok(ParsiDateTime {
            date: new_date,
            ..*self
        })
    }

    /// Creates a new `ParsiDateTime` with only the month component of the date changed.
    ///
    /// Delegates to [`ParsiDate::with_month`], preserving time. Clamps day if needed.
    ///
    /// # Arguments
    /// * `month`: New Persian month (1-12).
    ///
    /// # Errors
    /// Returns `Err` if `ParsiDate::with_month` fails.
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate};
    /// let dt = ParsiDateTime::new(1403, 1, 31, 10, 0, 0).unwrap(); // Farvardin 31st
    /// // Change to Mehr (month 7, 30 days) -> day becomes 30
    /// let dt_new_month = dt.with_month(7).unwrap();
    /// assert_eq!(dt_new_month.date(), ParsiDate::new(1403, 7, 30).unwrap());
    /// assert_eq!(dt_new_month.time(), (10, 0, 0));
    /// ```
    pub fn with_month(&self, month: u32) -> Result<Self, DateError> {
        let new_date = self.date.with_month(month)?;
        Ok(ParsiDateTime {
            date: new_date,
            ..*self
        })
    }

    /// Creates a new `ParsiDateTime` with only the day component of the date changed.
    ///
    /// Delegates to [`ParsiDate::with_day`], preserving time.
    ///
    /// # Arguments
    /// * `day`: New day of the month (1-31).
    ///
    /// # Errors
    /// Returns `Err` if `ParsiDate::with_day` fails (e.g., day invalid for month/year).
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, DateError, ParsiDate};
    /// let dt = ParsiDateTime::new(1403, 7, 15, 12, 0, 0).unwrap(); // Mehr 15th (30 days)
    /// assert_eq!(dt.with_day(30).unwrap().day(), 30);
    /// assert_eq!(dt.with_day(31), Err(DateError::InvalidDate)); // Mehr only has 30 days
    /// ```
    pub fn with_day(&self, day: u32) -> Result<Self, DateError> {
        let new_date = self.date.with_day(day)?;
        Ok(ParsiDateTime {
            date: new_date,
            ..*self
        })
    }

    // --- Season Boundaries ---

    /// Returns the `ParsiDateTime` corresponding to the first day of the season this date falls into,
    /// preserving the original time component.
    ///
    /// Delegates the date calculation to [`ParsiDate::start_of_season`].
    ///
    /// # Errors
    /// Returns `Err(DateError::InvalidDate)` if the original date part is invalid.
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate};
    ///
    /// let dt = ParsiDateTime::new(1403, 8, 20, 15, 30, 0).unwrap(); // Aban 20th (Paeez)
    /// let start_dt = dt.start_of_season().unwrap();
    /// // Paeez starts Mehr 1st
    /// assert_eq!(start_dt.date(), ParsiDate::new(1403, 7, 1).unwrap());
    /// assert_eq!(start_dt.time(), (15, 30, 0)); // Time preserved
    /// ```
    pub fn start_of_season(&self) -> Result<Self, DateError> {
        let new_date = self.date.start_of_season()?;
        Ok(ParsiDateTime {
            date: new_date,
            ..*self
        })
    }

    /// Returns the `ParsiDateTime` corresponding to the last day of the season this date falls into,
    /// preserving the original time component.
    ///
    /// Delegates the date calculation to [`ParsiDate::end_of_season`].
    ///
    /// # Errors
    /// Returns `Err(DateError::InvalidDate)` if the original date part is invalid.
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate};
    ///
    /// // Winter of a leap year
    /// let dt = ParsiDateTime::new(1403, 11, 10, 10, 0, 0).unwrap(); // Bahman 10th, 1403 (leap)
    /// let end_dt = dt.end_of_season().unwrap();
    /// // Winter 1403 ends Esfand 30th
    /// assert_eq!(end_dt.date(), ParsiDate::new(1403, 12, 30).unwrap());
    /// assert_eq!(end_dt.time(), (10, 0, 0)); // Time preserved
    /// ```
    pub fn end_of_season(&self) -> Result<Self, DateError> {
        let new_date = self.date.end_of_season()?;
        Ok(ParsiDateTime {
            date: new_date,
            ..*self
        })
    }
} // End impl ParsiDateTime

// --- Trait Implementations ---

/// Implements the `Display` trait for `ParsiDateTime`.
///
/// The default format is `"YYYY/MM/DD HH:MM:SS"`.
///
/// # Examples
///
/// ```rust
/// use parsidate::ParsiDateTime;
///
/// let dt = ParsiDateTime::new(1403, 5, 2, 8, 5, 30).unwrap();
/// assert_eq!(dt.to_string(), "1403/05/02 08:05:30");
///
/// let dt_single_digit = ParsiDateTime::new(1399, 12, 9, 23, 59, 9).unwrap();
/// assert_eq!(format!("{}", dt_single_digit), "1399/12/09 23:59:09"); // Zero-padded
/// ```
impl fmt::Display for ParsiDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:02}:{:02}:{:02}", // Uses ParsiDate Display for date part
            self.date, self.hour, self.minute, self.second
        )
    }
}

// --- Operator Overloads for Duration ---

/// Implements `Add<Duration>` for `ParsiDateTime`. Allows `dt + duration`.
/// Returns `Result` to handle potential errors.
impl Add<Duration> for ParsiDateTime {
    type Output = Result<ParsiDateTime, DateError>;

    #[inline]
    fn add(self, duration: Duration) -> Self::Output {
        self.add_duration(duration)
    }
}

/// Implements `Sub<Duration>` for `ParsiDateTime`. Allows `dt - duration`.
/// Returns `Result` to handle potential errors.
impl Sub<Duration> for ParsiDateTime {
    type Output = Result<ParsiDateTime, DateError>;

    #[inline]
    fn sub(self, duration: Duration) -> Self::Output {
        self.sub_duration(duration)
    }
}

/// Implements `Sub<ParsiDateTime>` for `ParsiDateTime`. Allows `dt1 - dt2`.
/// Returns `Result<Duration, DateError>` to handle potential errors during conversion.
impl Sub<ParsiDateTime> for ParsiDateTime {
    /// The result type of the subtraction: a `chrono::Duration` or a `DateError`.
    type Output = Result<Duration, DateError>;

    /// Calculates the `chrono::Duration` between two `ParsiDateTime` instances (`self` - `other`).
    ///
    /// Converts both instances to `NaiveDateTime` before calculating the difference.
    ///
    /// # Errors
    ///
    /// Returns `Err` if either `self` or `other` cannot be successfully converted to `NaiveDateTime`
    /// via [`ParsiDateTime::to_gregorian`] (e.g., due to invalid date/time components).
    fn sub(self, other: ParsiDateTime) -> Self::Output {
        // Convert self to Gregorian NaiveDateTime. Propagates errors.
        let self_gregorian = self.to_gregorian()?;
        // Convert other to Gregorian NaiveDateTime. Propagates errors.
        let other_gregorian = other.to_gregorian()?;

        // Calculate the signed duration between the two NaiveDateTime instances.
        // `signed_duration_since` calculates self - other. This chrono method does not typically fail.
        Ok(self_gregorian.signed_duration_since(other_gregorian))
    }
}
