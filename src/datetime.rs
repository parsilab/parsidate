// ~/src/datetime.rs
//
//  * Copyright (C) ParsiCore (parsidate) 2024-2025 <parsicore.dev@gmail.com>
//  * Package : parsidate
//  * License : Apache-2.0
//  * Version : 1.7.0
//  * URL     : https://github.com/parsicore/parsidate
//  * Sign: parsidate-20250607-fea13e856dcd-459c6e73c83e49e10162ee28b26ac7cd
//
//! Contains the `ParsiDateTime` struct definition and its implementation for handling
//! date and time within the Persian (Jalali or Shamsi) calendar system.

use crate::constants::MONTH_NAMES_PERSIAN;
use crate::date::ParsiDate;
use crate::error::{DateError, ParseErrorKind};
use crate::season::Season;
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
    // Nanoseconds are not stored directly to keep the struct simple and focused
    // on common use cases. Conversions involving chrono::Duration handle them
    // implicitly during the Gregorian conversion round-trip.
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
    /// assert_eq!(dt.minute(), 30);
    /// assert_eq!(dt.second(), 45);
    ///
    /// // Example of an invalid date (Esfand 30th in a non-leap year)
    /// assert_eq!(
    ///     ParsiDateTime::new(1404, 12, 30, 10, 0, 0),
    ///     Err(DateError::InvalidDate) // 1404 is not a leap year
    /// );
    ///
    /// // Example of an invalid time (hour 24)
    /// assert_eq!(
    ///     ParsiDateTime::new(1403, 5, 2, 24, 0, 0),
    ///     Err(DateError::InvalidTime)
    /// );
    ///
    /// // Example of an invalid time (minute 60)
    /// assert_eq!(
    ///     ParsiDateTime::new(1403, 5, 2, 10, 60, 0),
    ///     Err(DateError::InvalidTime)
    /// );
    ///
    /// // Example of an invalid time (second 60)
    /// assert_eq!(
    ///     ParsiDateTime::new(1403, 5, 2, 10, 0, 60),
    ///     Err(DateError::InvalidTime)
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
        // Validate date first using ParsiDate::new
        let date = ParsiDate::new(year, month, day)?;
        // Validate time components
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
    /// results in subsequent operations (like formatting, arithmetic, or conversion).
    ///
    /// Only use this function if you can absolutely guarantee that all input components represent
    /// a valid Persian date and time. In most cases, prefer using the safe [`ParsiDateTime::new`] constructor.
    ///
    /// # Safety
    ///
    /// The caller *must* ensure that:
    /// 1. `year`, `month`, and `day` form a valid date in the Persian calendar (considering month lengths and leap years).
    /// 2. `hour` is in the range 0-23.
    /// 3. `minute` is in the range 0-59.
    /// 4. `second` is in the range 0-59.
    ///
    /// Failure to meet these requirements constitutes undefined behavior from the perspective of this library's guarantees.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, ParsiDateTime};
    ///
    /// // Assume components have been rigorously validated elsewhere
    /// let p_year = 1403;
    /// let p_month = 10; // Dey
    /// let p_day = 11;
    /// let hour = 9;
    /// let minute = 0;
    /// let second = 0;
    ///
    /// // Pre-validation (example - real validation might be more complex)
    /// let is_valid_date = ParsiDate::new(p_year, p_month, p_day).is_ok();
    /// let is_valid_time = hour <= 23 && minute <= 59 && second <= 59;
    ///
    /// if is_valid_date && is_valid_time {
    ///     // Safe to use new_unchecked because we *know* the inputs are valid
    ///     let dt = unsafe {
    ///         ParsiDateTime::new_unchecked(p_year, p_month, p_day, hour, minute, second)
    ///     };
    ///     assert_eq!(dt.year(), 1403);
    ///     assert_eq!(dt.hour(), 9);
    /// } else {
    ///     // Handle the case where inputs were not valid
    ///     eprintln!("Cannot use new_unchecked with invalid inputs!");
    /// }
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
            // Creates the inner ParsiDate unsafely as well, assuming its components are valid
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
    /// Note: If an invalid `ParsiDate` (e.g., created via `ParsiDate::new_unchecked` with invalid data)
    /// is passed to this function, the function might return `Ok`, but the resulting `ParsiDateTime`
    /// will contain an invalid date part. Use [`ParsiDateTime::is_valid`] afterwards if the validity
    /// of the input `date` is uncertain.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, ParsiDateTime, DateError};
    ///
    /// // Assume `my_date` is known to be valid
    /// let my_date = ParsiDate::new(1399, 11, 22).expect("Date should be valid"); // Bahman 22nd, 1399
    ///
    /// // Create a ParsiDateTime using the valid date
    /// let dt_result = ParsiDateTime::from_date_and_time(my_date, 20, 15, 0);
    /// assert!(dt_result.is_ok());
    /// let dt = dt_result.unwrap();
    /// assert_eq!(dt.date(), my_date);
    /// assert_eq!(dt.hour(), 20);
    ///
    /// // Example of invalid time components
    /// assert_eq!(
    ///     ParsiDateTime::from_date_and_time(my_date, 10, 60, 0), // Invalid minute
    ///     Err(DateError::InvalidTime)
    /// );
    /// assert_eq!(
    ///     ParsiDateTime::from_date_and_time(my_date, 24, 0, 0), // Invalid hour
    ///     Err(DateError::InvalidTime)
    /// );
    /// ```
    pub fn from_date_and_time(
        date: ParsiDate,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Result<Self, DateError> {
        // Validate only the time components
        if hour > 23 || minute > 59 || second > 59 {
            return Err(DateError::InvalidTime);
        }
        // Assume the input `date` is valid as per the function's contract
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
    /// * `gregorian_dt`: The `chrono::NaiveDateTime` instance to convert. This represents a
    ///   Gregorian date and time without any timezone information.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::GregorianConversionError)` if the date part conversion fails.
    /// This typically happens if the Gregorian date is before the Persian epoch (March 21, 622 CE)
    /// or falls outside the range supported by `ParsiDate`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    ///
    /// // Gregorian date: July 23, 2024, 15:30:45
    /// let g_date = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
    /// let g_time = NaiveTime::from_hms_opt(15, 30, 45).unwrap();
    /// let g_dt = NaiveDateTime::new(g_date, g_time);
    ///
    /// // Convert to ParsiDateTime
    /// let pd_dt_result = ParsiDateTime::from_gregorian(g_dt);
    /// assert!(pd_dt_result.is_ok());
    /// let pd_dt = pd_dt_result.unwrap();
    ///
    /// // Expected Persian date: Mordad 2, 1403
    /// assert_eq!(pd_dt.date(), ParsiDate::new(1403, 5, 2).unwrap());
    /// assert_eq!(pd_dt.hour(), 15);
    /// assert_eq!(pd_dt.minute(), 30);
    /// assert_eq!(pd_dt.second(), 45);
    ///
    /// // Example with nanoseconds (they get truncated)
    /// let g_dt_nano = NaiveDate::from_ymd_opt(2023, 3, 21).unwrap() // Start of 1402
    ///                  .and_hms_nano_opt(0, 0, 1, 123456789).unwrap();
    /// let pd_dt_nano = ParsiDateTime::from_gregorian(g_dt_nano).unwrap();
    /// assert_eq!(pd_dt_nano.date(), ParsiDate::new(1402, 1, 1).unwrap());
    /// assert_eq!(pd_dt_nano.second(), 1); // Nanoseconds are lost
    ///
    /// // Example of a date before the Persian epoch (likely to fail)
    /// let g_dt_early = NaiveDate::from_ymd_opt(600, 1, 1).unwrap().and_hms_opt(0,0,0).unwrap();
    /// assert_eq!(ParsiDateTime::from_gregorian(g_dt_early), Err(DateError::GregorianConversionError));
    /// ```
    pub fn from_gregorian(gregorian_dt: NaiveDateTime) -> Result<Self, DateError> {
        // Convert the date part using ParsiDate's conversion logic
        let parsi_date = ParsiDate::from_gregorian(gregorian_dt.date())?;

        // Extract time components directly from the NaiveDateTime
        let hour = gregorian_dt.hour();
        let minute = gregorian_dt.minute();
        let second = gregorian_dt.second();
        // Nanoseconds present in gregorian_dt.nanosecond() are ignored

        // Since the time components from chrono::NaiveDateTime are guaranteed to be
        // valid (0-23, 0-59, 0-59), we can construct directly without re-validating time.
        // Using `from_date_and_time` would also work but adds a redundant time check.
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
    /// *   `DateError::InvalidDate`: If the date part (`self.date`) of this `ParsiDateTime` is invalid
    ///     (e.g., if created using `new_unchecked` with bad data).
    /// *   `DateError::InvalidTime`: If the time part (`self.hour`, `self.minute`, `self.second`)
    ///     of this `ParsiDateTime` is invalid (should ideally be caught earlier, but checked here for safety).
    /// *   `DateError::GregorianConversionError`: If the conversion of the valid `ParsiDate` component
    ///     to `NaiveDate` fails internally (this is unexpected for valid Persian dates within the supported range).
    ///     It might also theoretically occur if `NaiveDate::and_hms_opt` fails, but this shouldn't happen
    ///     if the time components (0-23, 0-59, 0-59) are valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    ///
    /// // Persian date: Mordad 2, 1403, 15:30:45
    /// let pd_dt = ParsiDateTime::new(1403, 5, 2, 15, 30, 45).unwrap();
    ///
    /// // Convert to Gregorian NaiveDateTime
    /// let g_dt_result = pd_dt.to_gregorian();
    /// assert!(g_dt_result.is_ok());
    /// let g_dt = g_dt_result.unwrap();
    ///
    /// // Expected Gregorian date: July 23, 2024, 15:30:45
    /// let expected_g_date = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
    /// let expected_g_time = NaiveTime::from_hms_opt(15, 30, 45).unwrap();
    /// let expected_g_dt = NaiveDateTime::new(expected_g_date, expected_g_time);
    /// assert_eq!(g_dt, expected_g_dt);
    ///
    /// // Example with an invalid ParsiDateTime created via unsafe
    /// let invalid_pd_dt = unsafe { ParsiDateTime::new_unchecked(1404, 12, 30, 10, 0, 0) }; // Invalid date
    /// assert!(!invalid_pd_dt.is_valid());
    /// assert_eq!(invalid_pd_dt.to_gregorian(), Err(DateError::InvalidDate));
    ///
    /// let invalid_time_pd_dt = unsafe { ParsiDateTime::new_unchecked(1403, 1, 1, 25, 0, 0) }; // Invalid time
    /// assert!(!invalid_time_pd_dt.is_valid());
    /// assert_eq!(invalid_time_pd_dt.to_gregorian(), Err(DateError::InvalidTime));
    /// ```
    pub fn to_gregorian(&self) -> Result<NaiveDateTime, DateError> {
        // First, perform a comprehensive validation of the ParsiDateTime instance.
        if !self.is_valid() {
            // Determine whether the date or time part caused the invalidity.
            if !self.date.is_valid() {
                return Err(DateError::InvalidDate);
            } else {
                return Err(DateError::InvalidTime);
            }
        }

        // If self is valid, convert the ParsiDate part to NaiveDate.
        let gregorian_date = self.date.to_gregorian_internal()?;

        // Combine the resulting NaiveDate with the (now known to be valid) time components.
        gregorian_date
            .and_hms_opt(self.hour, self.minute, self.second)
            .ok_or(DateError::GregorianConversionError)
    }

    /// Returns the current system date and time, converted to `ParsiDateTime`.
    ///
    /// This function obtains the current local date and time from the operating system
    /// using `chrono::Local::now()`, gets the naive representation (without timezone),
    /// and then converts this `NaiveDateTime` to `ParsiDateTime` using `\[`from_gregorian`\]`.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::GregorianConversionError)` if the conversion from the current
    /// Gregorian date/time provided by the system fails. This could potentially happen if the
    /// system clock is set to a date before the Persian epoch or encounters other issues during
    /// the conversion process handled by `\[`from_gregorian`\]`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// match ParsiDateTime::now() {
    ///     Ok(now) => {
    ///         println!("Current Persian date and time: {}", now);
    ///         // Example: Check if it's past noon
    ///         if now.hour() >= 12 {
    ///             println!("It's afternoon or evening in Persian time.");
    ///         }
    ///         // Example: Format the current date/time
    ///         println!("Formatted: {}", now.format("%Y-%m-%d %H:%M"));
    ///     }
    ///     Err(e) => {
    ///         eprintln!("Failed to get current Persian date and time: {}", e);
    ///     }
    /// }
    /// ```
    pub fn now() -> Result<Self, DateError> {
        // Get the current date and time in the system's local timezone.
        let now_local: chrono::DateTime<Local> = Local::now();
        // Convert it to a NaiveDateTime (ignoring timezone information).
        let naive_local: NaiveDateTime = now_local.naive_local();
        // Convert the NaiveDateTime (Gregorian) to ParsiDateTime.
        Self::from_gregorian(naive_local)
    }

    // --- Accessors ---

    /// Returns the [`ParsiDate`] component of this `ParsiDateTime`.
    ///
    /// This gives access to the year, month, and day part of the timestamp.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate};
    ///
    /// let dt = ParsiDateTime::new(1403, 8, 15, 10, 30, 0).unwrap(); // Aban 15, 1403
    /// let date_part: ParsiDate = dt.date();
    /// assert_eq!(date_part, ParsiDate::new(1403, 8, 15).unwrap());
    /// ```
    #[inline]
    pub const fn date(&self) -> ParsiDate {
        self.date
    }

    /// Returns the year component of the Persian date.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// let dt = ParsiDateTime::new(1403, 5, 2, 15, 30, 45).unwrap();
    /// assert_eq!(dt.year(), 1403);
    /// ```
    #[inline]
    pub const fn year(&self) -> i32 {
        self.date.year()
    }

    /// Returns the month component of the Persian date (1-12).
    ///
    /// 1 corresponds to Farvardin, 12 to Esfand.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// let dt = ParsiDateTime::new(1403, 5, 2, 15, 30, 45).unwrap(); // 5 = Mordad
    /// assert_eq!(dt.month(), 5);
    /// ```
    #[inline]
    pub const fn month(&self) -> u32 {
        self.date.month()
    }

    /// Returns the day component of the Persian date (1-31).
    ///
    /// The valid range depends on the month and whether the year is a leap year.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// let dt = ParsiDateTime::new(1403, 5, 2, 15, 30, 45).unwrap();
    /// assert_eq!(dt.day(), 2);
    ///
    /// let dt_leap_end = ParsiDateTime::new(1403, 12, 30, 23, 59, 59).unwrap(); // 1403 is a leap year
    /// assert_eq!(dt_leap_end.day(), 30);
    /// ```
    #[inline]
    pub const fn day(&self) -> u32 {
        self.date.day()
    }

    /// Returns the hour component of the time (0-23).
    ///
    /// Based on a 24-hour clock.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// let dt_morning = ParsiDateTime::new(1403, 1, 1, 9, 0, 0).unwrap();
    /// assert_eq!(dt_morning.hour(), 9);
    ///
    /// let dt_evening = ParsiDateTime::new(1403, 1, 1, 21, 0, 0).unwrap();
    /// assert_eq!(dt_evening.hour(), 21);
    ///
    /// let dt_midnight = ParsiDateTime::new(1403, 1, 1, 0, 0, 0).unwrap();
    /// assert_eq!(dt_midnight.hour(), 0);
    /// ```
    #[inline]
    pub const fn hour(&self) -> u32 {
        self.hour
    }

    /// Returns the minute component of the time (0-59).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// let dt = ParsiDateTime::new(1403, 5, 2, 15, 30, 45).unwrap();
    /// assert_eq!(dt.minute(), 30);
    ///
    /// let dt_on_the_hour = ParsiDateTime::new(1403, 5, 2, 16, 0, 0).unwrap();
    /// assert_eq!(dt_on_the_hour.minute(), 0);
    /// ```
    #[inline]
    pub const fn minute(&self) -> u32 {
        self.minute
    }

    /// Returns the second component of the time (0-59).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// let dt = ParsiDateTime::new(1403, 5, 2, 15, 30, 45).unwrap();
    /// assert_eq!(dt.second(), 45);
    ///
    /// let dt_on_the_minute = ParsiDateTime::new(1403, 5, 2, 15, 31, 0).unwrap();
    /// assert_eq!(dt_on_the_minute.second(), 0);
    /// ```
    #[inline]
    pub const fn second(&self) -> u32 {
        self.second
    }

    /// Returns the time components as a tuple `(hour, minute, second)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// let dt = ParsiDateTime::new(1403, 5, 2, 15, 30, 45).unwrap();
    /// let (h, m, s) = dt.time();
    /// assert_eq!(h, 15);
    /// assert_eq!(m, 30);
    /// assert_eq!(s, 45);
    /// assert_eq!(dt.time(), (15, 30, 45));
    /// ```
    #[inline]
    pub const fn time(&self) -> (u32, u32, u32) {
        (self.hour, self.minute, self.second)
    }

    // --- Season Accessor --- //

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
    /// This method checks two conditions:
    /// 1.  Whether the internal `ParsiDate` component is valid (using [`ParsiDate::is_valid`]).
    ///     This verifies the year, month, and day combination (e.g., day is within month bounds, considers leap years).
    /// 2.  Whether the time components (`hour`, `minute`, `second`) are within their standard ranges
    ///     (H: 0-23, M: 0-59, S: 0-59).
    ///
    /// This is particularly useful after creating an instance using `unsafe fn new_unchecked` or
    /// if the validity is otherwise uncertain. Instances created with `new`, `from_date_and_time`,
    /// `from_gregorian`, or `now` are generally expected to be valid unless an error occurred during creation.
    ///
    /// # Returns
    ///
    /// *   `true` if both the date part and the time part are valid.
    /// *   `false` if either the date part is invalid or any time component is out of range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// // Valid instance created via `new`
    /// let valid_dt = ParsiDateTime::new(1403, 12, 30, 23, 59, 59).unwrap(); // Leap year end
    /// assert!(valid_dt.is_valid());
    ///
    /// // Instance with invalid time created via `unsafe new_unchecked`
    /// let invalid_time_dt = unsafe { ParsiDateTime::new_unchecked(1403, 1, 1, 24, 0, 0) }; // Hour 24 is invalid
    /// assert!(!invalid_time_dt.is_valid());
    ///
    /// // Instance with invalid date created via `unsafe new_unchecked`
    /// let invalid_date_dt = unsafe { ParsiDateTime::new_unchecked(1404, 12, 30, 10, 0, 0) }; // Esfand 30 in non-leap year
    /// assert!(!invalid_date_dt.is_valid());
    ///
    /// // Instance with both invalid date and time
    /// let invalid_both_dt = unsafe { ParsiDateTime::new_unchecked(1404, 13, 32, 25, 60, 60) };
    /// assert!(!invalid_both_dt.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        // Check if the date part is valid AND all time components are in range.
        self.date.is_valid() // Checks year/month/day validity including leap years
            && self.hour <= 23
            && self.minute <= 59
            && self.second <= 59
    }

    /// Calculates the week number of the year for this date-time's date component.
    ///
    /// This method delegates the calculation to [`ParsiDate::week_of_year`] using the
    /// date part of this `ParsiDateTime`. See the documentation of that method for
    /// the definition of week numbering and potential errors. The time component is ignored.
    ///
    /// # Errors
    /// Returns `Err(DateError::InvalidDate)` or `Err(DateError::GregorianConversionError)`
    /// or `Err(DateError::ArithmeticOverflow)` if the underlying date calculation fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// // Farvardin 4th, 1403, 10:00 AM - Should be week 2
    /// let dt = ParsiDateTime::new(1403, 1, 4, 10, 0, 0).unwrap();
    /// assert_eq!(dt.week_of_year(), Ok(2));
    /// ```
    #[inline]
    pub fn week_of_year(&self) -> Result<u32, DateError> {
        self.date.week_of_year() // Delegate to the ParsiDate method
    }

    // --- Formatting ---

    /// Formats the `ParsiDateTime` into a string according to a given format pattern.
    ///
    /// This method works similarly to the `strftime` function found in C libraries or
    /// other date/time libraries. It interprets a format string containing special
    /// percent-prefixed specifiers (`%Y`, `%m`, `%d`, `%H`, etc.) and replaces them
    /// with the corresponding date or time components of the `ParsiDateTime` instance.
    /// Any characters in the format string that are not part of a specifier are included
    /// literally in the output string.
    ///
    /// This extends the formatting capabilities of [`ParsiDate::format_strftime`] by adding
    /// specifiers for time components.
    ///
    /// # Supported Format Specifiers
    ///
    /// **Date Specifiers (inherited from `ParsiDate`):**
    ///
    /// *   `%Y`: Year with century (e.g., `1403`).
    /// *   `%m`: Month as a zero-padded number (01-12).
    /// *   `%d`: Day of the month as a zero-padded number (01-31).
    /// *   `%B`: Full Persian month name (e.g., "فروردین", "مرداد"). Requires month to be valid.
    /// *   `%A`: Full Persian weekday name (e.g., "شنبه", "سه‌شنبه"). Requires date to be valid.
    /// *   `%w`: Weekday as a number (Saturday=0, Sunday=1, ..., Friday=6). Requires date to be valid.
    /// *   `%j`: Day of the year as a zero-padded number (001-365 or 366). Requires date to be valid.
    /// *   `%K`: Full Persian season name (e.g., "تابستان"). Requires date to be valid.
    /// *   `%W`: Week number of the year (Saturday start, 01-53). Requires date to be valid.
    /// *   `%%`: A literal percent sign (`%`).
    ///
    /// **Time Specifiers:**
    ///
    /// *   `%H`: Hour (24-hour clock) as a zero-padded number (00-23).
    /// *   `%M`: Minute as a zero-padded number (00-59).
    /// *   `%S`: Second as a zero-padded number (00-59).
    /// *   `%T`: Equivalent to `%H:%M:%S`.
    ///
    /// **Note:** If the `ParsiDateTime` instance contains invalid date or time components
    /// (e.g., created via `new_unchecked`), the output for the corresponding specifiers
    /// might be incorrect, nonsensical, or display error markers like `?InvalidMonth?` or `???`.
    /// Specifiers requiring calculation (like `%A`, `%w`, `%j`, `%K`) might return error indicators
    /// if the date part is invalid.
    ///
    /// # Arguments
    ///
    /// * `pattern`: A string slice (`&str`) containing the desired format. It can include
    ///   literal characters and the supported format specifiers listed above.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted date and time according to the pattern.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// let dt = ParsiDateTime::new(1403, 5, 2, 8, 5, 30).unwrap(); // 1403/Mordad/02 08:05:30 (Tuesday, Tabestan)
    ///
    /// // Common ISO-like format (Persian date)
    /// assert_eq!(dt.format("%Y-%m-%d %H:%M:%S"), "1403-05-02 08:05:30");
    ///
    /// // Format with Persian names and season
    /// assert_eq!(dt.format("%d %B (%K) %Y ساعت %H:%M"), "02 مرداد (تابستان) 1403 ساعت 08:05");
    ///
    /// // Format with week number
    /// assert_eq!(dt.format("%Y/Week %W %H:%M"), "1403/Week 19 08:05"); // 1403/05/02 is week 19
    ///
    /// // Using %T for time
    /// assert_eq!(dt.format("%Y-%m-%dT%T"), "1403-05-02T08:05:30");
    ///
    /// // Including weekday and day of year
    /// assert_eq!(dt.format("%A، %d %B %Y - %T (روز %j سال، روز هفته %w)"),
    ///              "سه‌شنبه، 02 مرداد 1403 - 08:05:30 (روز 126 سال، روز هفته 3)");
    ///
    /// // Literal percent sign
    /// assert_eq!(dt.format("Time is %H:%M %% %S seconds"), "Time is 08:05 % 30 seconds");
    ///
    /// // Formatting an invalid time (created unsafely)
    /// let invalid_dt = unsafe { ParsiDateTime::new_unchecked(1403, 1, 1, 25, 61, 99) };
    /// assert_eq!(invalid_dt.format("%H:%M:%S"), "25:61:99"); // Prints the invalid numbers
    /// ```
    pub fn format(&self, pattern: &str) -> String {
        // Preallocate string with a reasonable estimate capacity to reduce reallocations.
        let mut result = String::with_capacity(pattern.len() + 20); // Estimate extra space needed
                                                                    // Use a character iterator for correct handling of multi-byte UTF-8 characters in the pattern.
        let mut chars = pattern.chars().peekable();

        // Caching results for potentially expensive date calculations if used multiple times
        let mut weekday_name_cache: Option<Result<String, DateError>> = None;
        let mut ordinal_day_cache: Option<Result<u32, DateError>> = None;
        let mut weekday_num_cache: Option<Result<u32, DateError>> = None;
        let mut season_cache: Option<Result<Season, DateError>> = None;
        let mut week_of_year_cache: Option<Result<u32, DateError>> = None;

        while let Some(c) = chars.next() {
            if c == '%' {
                // Check the character immediately following the '%'
                match chars.next() {
                    // --- Time Specifiers ---
                    Some('H') => result.push_str(&format!("{:02}", self.hour)),
                    Some('M') => result.push_str(&format!("{:02}", self.minute)),
                    Some('S') => result.push_str(&format!("{:02}", self.second)),
                    Some('T') => result.push_str(&format!(
                        "{:02}:{:02}:{:02}",
                        self.hour, self.minute, self.second
                    )),

                    // --- Date Specifiers (using self.date() or direct access) ---
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
                    // --- Season Specifier --- //
                    Some('K') => {
                        if season_cache.is_none() {
                            season_cache = Some(self.date.season()); // Calculate using date part
                        }
                        match season_cache.as_ref().unwrap() {
                            Ok(season) => result.push_str(season.name_persian()),
                            Err(_) => result.push_str("?SeasonError?"),
                        }
                    }
                    // --- Week of Year '%W' --- //
                    Some('W') => {
                        if week_of_year_cache.is_none() {
                            // Use self.date for calculation
                            week_of_year_cache = Some(self.date.week_of_year());
                        }
                        match week_of_year_cache.as_ref().unwrap() {
                            Ok(week_num) => result.push_str(&format!("{:02}", week_num)), // Zero-padded
                            Err(_) => result.push_str("?WeekError?"),
                        }
                    }

                    // --- Unrecognized or Unsupported Specifier ---
                    Some(other) => {
                        result.push('%');
                        result.push(other);
                    }
                    // --- Dangling '%' ---
                    None => {
                        result.push('%');
                        break;
                    }
                }
            } else {
                // It's a literal character
                result.push(c);
            }
        }
        result // Return the final formatted string
    }

    // --- Parsing ---

    /// Parses a string containing a Persian date and time into a `ParsiDateTime` instance,
    /// based on a specified format pattern.
    ///
    /// This function attempts to match the input string `s` against the provided `format` string.
    /// It requires an *exact* match between the literal characters in the format string and the
    /// input string, and it expects the date/time components in the input string to correspond
    /// precisely to the format specifiers (`%Y`, `%m`, `%d`, `%H`, `%M`, `%S`, `%T`, `%B`, `%%`).
    ///
    /// Parsing uses the specifiers to extract the year, month, day, hour, minute, and second values.
    /// After successful extraction, it validates these values using [`ParsiDateTime::new`] to ensure
    /// they form a logically valid date and time.
    ///
    /// # Supported Format Specifiers for Parsing
    ///
    /// *   `%Y`: Parses a 4-digit Persian year.
    /// *   `%m`: Parses a 2-digit month (01-12).
    /// *   `%d`: Parses a 2-digit day (01-31).
    /// *   `%B`: Parses a full Persian month name (case-sensitive, must match names in `MONTH_NAMES_PERSIAN`, e.g., "فروردین").
    /// *   `%H`: Parses a 2-digit hour (00-23).
    /// *   `%M`: Parses a 2-digit minute (00-59).
    /// *   `%S`: Parses a 2-digit second (00-59).
    /// *   `%T`: Parses time in the exact format "HH:MM:SS" (e.g., "15:30:05").
    /// *   `%%`: Matches a literal percent sign (`%`) in the input string.
    ///
    /// **Unsupported Specifiers:** Specifiers like `%A`, `%w`, `%j`, `%K`, `%W` are *not* supported for parsing
    /// as they represent calculated values rather than primary inputs. Using them in the format string
    /// will result in a `ParseErrorKind::UnsupportedSpecifier` error.
    ///
    /// # Arguments
    ///
    /// * `s`: The input string slice (`&str`) to be parsed.
    /// * `format`: The format string slice (`&str`) describing the expected structure of the input `s`.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::ParseError(kind))` if parsing fails. The `kind` ([`ParseErrorKind`]) indicates the reason:
    /// *   `ParseErrorKind::FormatMismatch`: The input string `s` does not match the literal characters or overall structure defined by the `format` string, or expected components are missing, or there are trailing characters in `s`.
    /// *   `ParseErrorKind::InvalidNumber`: A numeric component (Year, Month, Day, Hour, Minute, Second) could not be parsed as a number, or it did not have the expected number of digits (e.g., `%m` expects exactly two digits).
    /// *   `ParseErrorKind::InvalidMonthName`: The input string did not contain a valid, recognized Persian month name where `%B` was expected.
    /// *   `ParseErrorKind::UnsupportedSpecifier`: The `format` string contained a specifier not supported for parsing (e.g., `%A`, `%j`, `%K`). // <-- Added %K here
    /// *   `ParseErrorKind::InvalidDateValue`: The extracted year, month, and day values were syntactically valid but do not form a logically valid Persian date (e.g., "1404/12/30" - day 30 in Esfand of a non-leap year). This is checked by the final call to `ParsiDateTime::new`.
    /// *   `ParseErrorKind::InvalidTimeValue`: The extracted hour, minute, or second values were syntactically valid but outside their allowed ranges (e.g., Hour 24, Minute 60). This is checked by the final call to `ParsiDateTime::new`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError, ParseErrorKind};
    ///
    /// // --- Success Cases ---
    /// let s1 = "1403/05/02 15:30:45";
    /// let fmt1 = "%Y/%m/%d %H:%M:%S";
    /// let expected1 = ParsiDateTime::new(1403, 5, 2, 15, 30, 45).unwrap();
    /// assert_eq!(ParsiDateTime::parse(s1, fmt1), Ok(expected1));
    ///
    /// // Using %T
    /// let s2 = "1403-05-02T09:05:00";
    /// let fmt2 = "%Y-%m-%dT%T";
    /// let expected2 = ParsiDateTime::new(1403, 5, 2, 9, 5, 0).unwrap();
    /// assert_eq!(ParsiDateTime::parse(s2, fmt2), Ok(expected2));
    ///
    /// // Using Persian month name %B
    /// let s3 = "22 بهمن 1399 - 23:59:59";
    /// let fmt3 = "%d %B %Y - %T";
    /// let expected3 = ParsiDateTime::new(1399, 11, 22, 23, 59, 59).unwrap();
    /// assert_eq!(ParsiDateTime::parse(s3, fmt3), Ok(expected3));
    ///
    /// // --- Error Cases ---
    /// // Invalid time value (hour 24)
    /// assert_eq!(ParsiDateTime::parse("1403/05/02 24:00:00", fmt1),
    ///            Err(DateError::ParseError(ParseErrorKind::InvalidTimeValue)));
    ///
    /// // Invalid date value (Esfand 30 in non-leap year 1404)
    /// assert_eq!(ParsiDateTime::parse("1404/12/30 10:00:00", fmt1),
    ///            Err(DateError::ParseError(ParseErrorKind::InvalidDateValue)));
    ///
    /// // Invalid number format (single digit minute where two expected)
    /// assert_eq!(ParsiDateTime::parse("1403/05/02 15:3:45", fmt1),
    ///            Err(DateError::ParseError(ParseErrorKind::InvalidNumber)));
    ///
    /// // Format mismatch (wrong separator)
    /// assert_eq!(ParsiDateTime::parse("1403/05/02 15-30-45", fmt1),
    ///            Err(DateError::ParseError(ParseErrorKind::FormatMismatch)));
    ///
    /// // Format mismatch (missing time part)
    /// assert_eq!(ParsiDateTime::parse("1403/05/02", fmt1),
    ///            Err(DateError::ParseError(ParseErrorKind::FormatMismatch)));
    ///
    /// // Format mismatch (trailing characters)
    /// assert_eq!(ParsiDateTime::parse("1403/05/02 15:30:45 extra", fmt1),
    ///            Err(DateError::ParseError(ParseErrorKind::FormatMismatch)));
    ///
    /// // Invalid month name
    /// assert_eq!(ParsiDateTime::parse("22 Бахман 1399 - 23:59:59", fmt3), // Using Cyrillic 'B'
    ///            Err(DateError::ParseError(ParseErrorKind::InvalidMonthName)));
    ///
    /// // Unsupported specifier in format string
    /// assert_eq!(ParsiDateTime::parse("Tuesday 1403", "%A %Y"),
    ///            Err(DateError::ParseError(ParseErrorKind::UnsupportedSpecifier)));
    /// ```
    pub fn parse(s: &str, format: &str) -> Result<Self, DateError> {
        // Options to store the parsed components. They start as None.
        let mut parsed_year: Option<i32> = None;
        let mut parsed_month: Option<u32> = None;
        let mut parsed_day: Option<u32> = None;
        let mut parsed_hour: Option<u32> = None;
        let mut parsed_minute: Option<u32> = None;
        let mut parsed_second: Option<u32> = None;

        // Use byte slices for efficient processing of ASCII parts of the format and input.
        let mut s_bytes = s.as_bytes();
        let mut fmt_bytes = format.as_bytes();

        // Iterate through the format string bytes
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
                    // Unsupported for parsing
                    b'A' | b'w' | b'j' | b'K' | b'W' => {
                        return Err(DateError::ParseError(ParseErrorKind::UnsupportedSpecifier));
                    }
                    _ => return Err(DateError::ParseError(ParseErrorKind::UnsupportedSpecifier)),
                }
            } else {
                // Literal character
                if s_bytes.is_empty() || s_bytes[0] != fmt_bytes[0] {
                    return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                }
                s_bytes = &s_bytes[1..];
                fmt_bytes = &fmt_bytes[1..];
            }
        } // End while loop

        // Check for remaining input characters
        if !s_bytes.is_empty() {
            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
        }

        // Final validation and construction
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
        // 1. Validate the starting ParsiDateTime.
        if !self.is_valid() {
            return Err(if !self.date.is_valid() {
                DateError::InvalidDate
            } else {
                DateError::InvalidTime
            });
        }
        // 2. Convert self to Gregorian NaiveDateTime.
        let gregorian_dt = self.to_gregorian()?;
        // 3. Add the duration using chrono's checked addition.
        let new_gregorian_dt = gregorian_dt
            .checked_add_signed(duration)
            .ok_or(DateError::ArithmeticOverflow)?;
        // 4. Convert the resulting Gregorian NaiveDateTime back to ParsiDateTime.
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
    /// Returns `Err` under the same conditions as `\[`add_duration`\]`.
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
        // Validate self first
        if !self.is_valid() {
            return Err(if !self.date.is_valid() {
                DateError::InvalidDate
            } else {
                DateError::InvalidTime
            });
        }
        // Delegate date addition to ParsiDate::add_days
        let new_date = self.date.add_days(days)?;
        // Recombine the new date with the original time components
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
    /// Returns `Err` under the same conditions as `\[`add_days`\]`.
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
        // Validate self first
        if !self.is_valid() {
            return Err(if !self.date.is_valid() {
                DateError::InvalidDate
            } else {
                DateError::InvalidTime
            });
        }
        // Delegate date subtraction to ParsiDate::sub_days
        let new_date = self.date.sub_days(days)?;
        // Recombine with original time
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
        // Delegate date addition to ParsiDate::add_months
        let new_date = self.date.add_months(months)?;
        // Recombine with original time
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
    /// Returns `Err` under the same conditions as `\[`add_months`\]`.
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
        // Delegate date subtraction to ParsiDate::sub_months
        let new_date = self.date.sub_months(months)?;
        // Recombine with original time
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
    ///
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
        // Delegate date addition to ParsiDate::add_years
        let new_date = self.date.add_years(years)?;
        // Recombine with original time
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
    /// Returns `Err` under the same conditions as `\[`add_years`\]`.
    ///
    /// # Examples
    ///
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
        // Delegate date subtraction to ParsiDate::sub_years
        let new_date = self.date.sub_years(years)?;
        // Recombine with original time
        Ok(ParsiDateTime {
            date: new_date,
            ..*self
        })
    }

    // --- Helper Methods ---

    /// Creates a new `ParsiDateTime` instance with only the hour component changed.
    ///
    /// The date (year, month, day) and the minute and second components remain the same.
    ///
    /// # Arguments
    ///
    /// * `hour`: The desired new hour (must be between 0 and 23).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidTime)` if the provided `hour` is outside the valid range (0-23).
    /// Returns `Err(DateError::InvalidDate)` if the date part of the original `ParsiDateTime` (`self`) was invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, DateError};
    ///
    /// let dt = ParsiDateTime::new(1403, 5, 2, 10, 30, 45).unwrap();
    ///
    /// // Set hour to 18 (6 PM)
    /// let dt_evening = dt.with_hour(18);
    /// assert!(dt_evening.is_ok());
    /// assert_eq!(dt_evening.unwrap().hour(), 18);
    ///
    /// // Try to set an invalid hour
    /// assert_eq!(dt.with_hour(24), Err(DateError::InvalidTime));
    /// ```
    pub fn with_hour(&self, hour: u32) -> Result<Self, DateError> {
        // Check if the original date part is valid first
        if !self.date.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Validate the new hour
        if hour > 23 {
            return Err(DateError::InvalidTime);
        }
        // Create new instance with updated hour, reusing other fields
        Ok(ParsiDateTime { hour, ..*self })
    }

    /// Creates a new `ParsiDateTime` instance with only the minute component changed.
    ///
    /// The date (year, month, day) and the hour and second components remain the same.
    ///
    /// # Arguments
    ///
    /// * `minute`: The desired new minute (must be between 0 and 59).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidTime)` if the provided `minute` is outside the valid range (0-59).
    /// Returns `Err(DateError::InvalidDate)` if the date part of the original `ParsiDateTime` (`self`) was invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, DateError};
    ///
    /// let dt = ParsiDateTime::new(1403, 5, 2, 10, 30, 45).unwrap();
    ///
    /// // Set minute to 55
    /// let dt_new_min = dt.with_minute(55);
    /// assert!(dt_new_min.is_ok());
    /// assert_eq!(dt_new_min.unwrap().minute(), 55);
    ///
    /// // Try to set an invalid minute
    /// assert_eq!(dt.with_minute(60), Err(DateError::InvalidTime));
    /// ```
    pub fn with_minute(&self, minute: u32) -> Result<Self, DateError> {
        // Check original date validity
        if !self.date.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Validate the new minute
        if minute > 59 {
            return Err(DateError::InvalidTime);
        }
        // Create new instance
        Ok(ParsiDateTime { minute, ..*self })
    }

    /// Creates a new `ParsiDateTime` instance with only the second component changed.
    ///
    /// The date (year, month, day) and the hour and minute components remain the same.
    ///
    /// # Arguments
    ///
    /// * `second`: The desired new second (must be between 0 and 59).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidTime)` if the provided `second` is outside the valid range (0-59).
    /// Returns `Err(DateError::InvalidDate)` if the date part of the original `ParsiDateTime` (`self`) was invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, DateError};
    ///
    /// let dt = ParsiDateTime::new(1403, 5, 2, 10, 30, 45).unwrap();
    ///
    /// // Set second to 0
    /// let dt_new_sec = dt.with_second(0);
    /// assert!(dt_new_sec.is_ok());
    /// assert_eq!(dt_new_sec.unwrap().second(), 0);
    ///
    /// // Try to set an invalid second
    /// assert_eq!(dt.with_second(60), Err(DateError::InvalidTime));
    /// ```
    pub fn with_second(&self, second: u32) -> Result<Self, DateError> {
        // Check original date validity
        if !self.date.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Validate the new second
        if second > 59 {
            return Err(DateError::InvalidTime);
        }
        // Create new instance
        Ok(ParsiDateTime { second, ..*self })
    }

    /// Creates a new `ParsiDateTime` instance with new time components (hour, minute, second).
    ///
    /// The date (year, month, day) component remains the same.
    ///
    /// # Arguments
    ///
    /// * `hour`: The desired new hour (0-23).
    /// * `minute`: The desired new minute (0-59).
    /// * `second`: The desired new second (0-59).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidTime)` if any of the provided `hour`, `minute`, or `second`
    /// are outside their valid ranges.
    /// Returns `Err(DateError::InvalidDate)` if the date part of the original `ParsiDateTime` (`self`) was invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, DateError};
    ///
    /// let dt = ParsiDateTime::new(1403, 5, 2, 10, 30, 45).unwrap();
    ///
    /// // Set time to 23:59:59
    /// let dt_new_time = dt.with_time(23, 59, 59);
    /// assert!(dt_new_time.is_ok());
    /// assert_eq!(dt_new_time.unwrap().time(), (23, 59, 59));
    /// assert_eq!(dt_new_time.unwrap().date(), dt.date()); // Date unchanged
    ///
    /// // Try to set an invalid time (minute 60)
    /// assert_eq!(dt.with_time(11, 60, 0), Err(DateError::InvalidTime));
    /// ```
    pub fn with_time(&self, hour: u32, minute: u32, second: u32) -> Result<Self, DateError> {
        // Check original date validity
        if !self.date.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Validate all new time components
        if hour > 23 || minute > 59 || second > 59 {
            return Err(DateError::InvalidTime);
        }
        // Create new instance with updated time
        Ok(ParsiDateTime {
            date: self.date,
            hour,
            minute,
            second,
        })
    }

    /// Creates a new `ParsiDateTime` instance with only the year component of the date changed.
    ///
    /// The month, day, and all time components remain the same. This method delegates the
    /// year change and associated validation (including leap day adjustments for Esfand 30th)
    /// to [`ParsiDate::with_year`].
    ///
    /// # Arguments
    ///
    /// * `year`: The desired new Persian year.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the underlying call to `ParsiDate::with_year` fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    ///
    /// let dt = ParsiDateTime::new(1403, 5, 2, 10, 30, 0).unwrap(); // 1403/05/02
    ///
    /// // Change year to 1399
    /// let dt_new_year = dt.with_year(1399).unwrap();
    /// assert_eq!(dt_new_year.year(), 1399);
    /// assert_eq!(dt_new_year.time(), (10, 30, 0)); // Time unchanged
    ///
    /// // Example with leap day adjustment
    /// let dt_leap = ParsiDateTime::new(1403, 12, 30, 11, 0, 0).unwrap(); // Leap year
    /// let dt_non_leap_year = dt_leap.with_year(1404).unwrap(); // 1404 is not leap
    /// assert_eq!(dt_non_leap_year.date(), ParsiDate::new(1404, 12, 29).unwrap());
    /// ```
    pub fn with_year(&self, year: i32) -> Result<Self, DateError> {
        // Delegate the date modification and validation to ParsiDate
        let new_date = self.date.with_year(year)?;
        // Recombine with original time
        Ok(ParsiDateTime {
            date: new_date,
            ..*self
        })
    }

    /// Creates a new `ParsiDateTime` instance with only the month component of the date changed.
    ///
    /// The year, day, and all time components remain the same. This method delegates the
    /// month change and associated validation (e.g., clamping the day) to [`ParsiDate::with_month`].
    ///
    /// # Arguments
    ///
    /// * `month`: The desired new Persian month (1-12).
    ///
    /// # Errors
    ///
    /// Returns `Err` if the underlying call to `ParsiDate::with_month` fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    ///
    /// let dt = ParsiDateTime::new(1403, 1, 31, 10, 0, 0).unwrap(); // Farvardin 31st
    ///
    /// // Change month to Mehr (month 7, 30 days) -> day becomes 30
    /// let dt_clamped_month = dt.with_month(7).unwrap();
    /// assert_eq!(dt_clamped_month.date(), ParsiDate::new(1403, 7, 30).unwrap());
    /// assert_eq!(dt_clamped_month.time(), (10, 0, 0));
    /// ```
    pub fn with_month(&self, month: u32) -> Result<Self, DateError> {
        // Delegate the date modification and validation to ParsiDate
        let new_date = self.date.with_month(month)?;
        // Recombine with original time
        Ok(ParsiDateTime {
            date: new_date,
            ..*self
        })
    }

    /// Creates a new `ParsiDateTime` instance with only the day component of the date changed.
    ///
    /// The year, month, and all time components remain the same. This method delegates the
    /// day change and associated validation to [`ParsiDate::with_day`].
    ///
    /// # Arguments
    ///
    /// * `day`: The desired new day of the month (1-31).
    ///
    /// # Errors
    ///
    /// Returns `Err` if the underlying call to `ParsiDate::with_day` fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, DateError, ParsiDate};
    /// let dt = ParsiDateTime::new(1403, 7, 15, 12, 0, 0).unwrap(); // Mehr 15th (30 days)
    ///
    /// // Change day to 30
    /// assert_eq!(dt.with_day(30).unwrap().day(), 30);
    ///
    /// // Try setting an invalid day (31 in Mehr)
    /// assert!(matches!(dt.with_day(31), Err(DateError::InvalidDate)));
    /// ```
    pub fn with_day(&self, day: u32) -> Result<Self, DateError> {
        // Delegate the date modification and validation to ParsiDate
        let new_date = self.date.with_day(day)?;
        // Recombine with original time
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
        let new_date = self.date.start_of_season()?; // Handles validation of self.date
        Ok(ParsiDateTime {
            date: new_date,
            ..*self
        }) // Reuse time components
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
        let new_date = self.date.end_of_season()?; // Handles validation of self.date
        Ok(ParsiDateTime {
            date: new_date,
            ..*self
        }) // Reuse time components
    }
} // <<<=== End impl ParsiDateTime ===>>>

// --- Trait Implementations ---

/// Implements the `Display` trait for `ParsiDateTime`.
///
/// This provides a default string representation for `ParsiDateTime` instances when used
/// with formatting macros like `println!`, `format!`, etc.
///
/// The default format is `"YYYY/MM/DD HH:MM:SS"`, using zero-padding for month, day,
/// hour, minute, and second. It utilizes the `Display` implementation of [`ParsiDate`]
/// for the date part.
///
/// # Examples
///
/// ```rust
/// use parsidate::ParsiDateTime;
///
/// let dt = ParsiDateTime::new(1403, 5, 2, 8, 5, 30).unwrap();
/// assert_eq!(dt.to_string(), "1403/05/02 08:05:30");
///
/// let dt_end_of_year = ParsiDateTime::new(1399, 12, 30, 23, 59, 9).unwrap();
/// // Note the zero-padding for second < 10
/// assert_eq!(format!("{}", dt_end_of_year), "1399/12/30 23:59:09");
/// ```
impl fmt::Display for ParsiDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the Display implementation of the inner ParsiDate for the date part ("YYYY/MM/DD")
        // Then, append the time part, ensuring zero-padding for H, M, S.
        write!(
            f,
            "{} {:02}:{:02}:{:02}",
            self.date, // ParsiDate's Display impl produces "YYYY/MM/DD"
            self.hour,
            self.minute,
            self.second
        )
    }
}

// --- Operator Overloads for Duration ---

/// Implements the `Add` trait for `ParsiDateTime` and `chrono::Duration`.
///
/// This allows using the `+` operator to add a `Duration` to a `ParsiDateTime`.
/// The operation returns a `Result<ParsiDateTime, DateError>` because the addition
/// might fail (e.g., due to overflow or invalid initial state).
///
/// This operation delegates to the [`ParsiDateTime::add_duration`] method.
///
/// # Examples
///
/// ```rust
/// use parsidate::{ParsiDateTime, DateError};
/// use chrono::Duration;
///
/// let dt = ParsiDateTime::new(1403, 1, 1, 10, 0, 0).unwrap();
/// let duration = Duration::hours(25); // 1 day and 1 hour
///
/// // Use the '+' operator
/// let result: Result<ParsiDateTime, DateError> = dt + duration;
///
/// assert!(result.is_ok());
/// let new_dt = result.unwrap();
/// assert_eq!(new_dt.year(), 1403);
/// assert_eq!(new_dt.month(), 1);
/// assert_eq!(new_dt.day(), 2); // Day advanced
/// assert_eq!(new_dt.hour(), 11); // Hour is 10 + 1 = 11 (after day rollover)
/// ```
impl Add<Duration> for ParsiDateTime {
    /// The result type of the addition, which includes potential errors.
    type Output = Result<ParsiDateTime, DateError>;

    /// Adds a `chrono::Duration` to this `ParsiDateTime` instance using the `+` operator.
    ///
    /// Delegates to [`ParsiDateTime::add_duration`]. See its documentation for details on
    /// the calculation process and potential errors.
    #[inline]
    fn add(self, duration: Duration) -> Self::Output {
        self.add_duration(duration)
    }
}

/// Implements the `Sub` trait for `ParsiDateTime` and `chrono::Duration`.
///
/// This allows using the `-` operator to subtract a `Duration` from a `ParsiDateTime`.
/// The operation returns a `Result<ParsiDateTime, DateError>` because the subtraction
/// might fail (e.g., due to underflow or invalid initial state).
///
/// This operation delegates to the [`ParsiDateTime::sub_duration`] method.
///
/// # Examples
///
/// ```rust
/// use parsidate::{ParsiDateTime, DateError};
/// use chrono::Duration;
///
/// let dt = ParsiDateTime::new(1403, 1, 1, 1, 0, 0).unwrap(); // 1 AM on Farvardin 1st
/// let duration = Duration::hours(2); // Subtract 2 hours
///
/// // Use the '-' operator
/// let result: Result<ParsiDateTime, DateError> = dt - duration;
///
/// assert!(result.is_ok());
/// let new_dt = result.unwrap();
/// // Should go back to the last day of the previous year (1402/12/29, non-leap)
/// assert_eq!(new_dt.year(), 1402);
/// assert_eq!(new_dt.month(), 12);
/// assert_eq!(new_dt.day(), 29); // Esfand 29th
/// assert_eq!(new_dt.hour(), 23); // 1 AM - 2 hours = 11 PM previous day
/// ```
impl Sub<Duration> for ParsiDateTime {
    /// The result type of the subtraction, which includes potential errors.
    type Output = Result<ParsiDateTime, DateError>;

    /// Subtracts a `chrono::Duration` from this `ParsiDateTime` instance using the `-` operator.
    ///
    /// Delegates to [`ParsiDateTime::sub_duration`]. See its documentation for details on
    /// the calculation process and potential errors.
    #[inline]
    fn sub(self, duration: Duration) -> Self::Output {
        self.sub_duration(duration)
    }
}

/// Implements the `Sub` trait for subtracting one `ParsiDateTime` from another.
///
/// This allows using the `-` operator between two `ParsiDateTime` instances to calculate
/// the `chrono::Duration` representing the time difference (`self` - `other`).
///
/// The calculation involves:
/// 1. Converting both `self` and `other` to their Gregorian `NaiveDateTime` equivalents.
/// 2. Calculating the duration between the two `NaiveDateTime` instances using `chrono`.
///
/// The operation returns a `Result<Duration, DateError>` because the initial conversion
/// of either `ParsiDateTime` to `NaiveDateTime` might fail.
///
/// # Errors
///
/// Returns `Err` if either `self` or `other` is an invalid `ParsiDateTime` or if their
/// conversion to `NaiveDateTime` fails (see [`ParsiDateTime::to_gregorian`] for error conditions).
///
/// # Examples
///
/// ```rust
/// use parsidate::{ParsiDateTime, DateError};
/// use chrono::Duration;
///
/// let dt1 = ParsiDateTime::new(1403, 5, 2, 15, 30, 0).unwrap(); // 1403/05/02 15:30:00
/// let dt2 = ParsiDateTime::new(1403, 5, 1, 14, 30, 0).unwrap(); // 1403/05/01 14:30:00
/// let dt3 = ParsiDateTime::new(1403, 5, 2, 15, 30, 45).unwrap(); // 1403/05/02 15:30:45
///
/// // Calculate dt1 - dt2
/// let diff1: Result<Duration, DateError> = dt1 - dt2;
/// assert!(diff1.is_ok());
/// // Expecting 1 day and 1 hour = (24 + 1) * 3600 seconds = 90000 seconds
/// assert_eq!(diff1.unwrap(), Duration::seconds(25 * 3600));
///
/// // Calculate dt2 - dt1
/// let diff2: Result<Duration, DateError> = dt2 - dt1;
/// assert!(diff2.is_ok());
/// assert_eq!(diff2.unwrap(), Duration::seconds(-25 * 3600)); // Negative duration
///
/// // Calculate dt3 - dt1
/// let diff3: Result<Duration, DateError> = dt3 - dt1;
/// assert!(diff3.is_ok());
/// assert_eq!(diff3.unwrap(), Duration::seconds(45)); // 45 seconds difference
///
/// // Example with potential error (invalid input)
/// let invalid_dt = unsafe { ParsiDateTime::new_unchecked(1404, 12, 30, 0, 0, 0) };
/// let diff_err: Result<Duration, DateError> = dt1 - invalid_dt;
/// assert!(diff_err.is_err());
/// assert!(matches!(diff_err, Err(DateError::InvalidDate))); // Error from converting invalid_dt
/// ```
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
