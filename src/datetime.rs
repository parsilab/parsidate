//! src/datetime.rs
//!
//! Copyright (C) Mohammad (Sina) Jalalvandi (parsidate) 2024-2025 <jalalvandi.sina@gmail.com>
//! Version : 1.4.0
//! eb1f0cae-a178-41e5-b109-47f208e77913
//!
//! Contains the `ParsiDateTime` struct definition and its implementation for handling

use crate::constants::{MONTH_NAMES_PERSIAN, WEEKDAY_NAMES_PERSIAN}; // Reuse constants
use crate::date::ParsiDate;
use crate::error::{DateError, ParseErrorKind};
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
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
            date: ParsiDate::new_unchecked(year, month, day),
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
        // Alternative using the validating constructor (slightly less direct):
        // Self::from_date_and_time(parsi_date, hour, minute, second)
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
                // If the date part is invalid (e.g., month 13, invalid day for month)
                return Err(DateError::InvalidDate);
            } else {
                // If the date part is valid, the time part must be invalid (e.g., hour 24)
                return Err(DateError::InvalidTime);
            }
        }

        // If self is valid, convert the ParsiDate part to NaiveDate.
        // We use the internal method `to_gregorian_internal` which skips redundant validation.
        let gregorian_date = self.date.to_gregorian_internal()?; // Propagates GregorianConversionError if ParsiDate conversion fails

        // Combine the resulting NaiveDate with the (now known to be valid) time components.
        // Since we validated hour (0-23), minute (0-59), and second (0-59),
        // `and_hms_opt` should always return Some. We use `ok_or` as a safeguard
        // against unexpected chrono behavior, mapping a potential None to our error type.
        gregorian_date
            .and_hms_opt(self.hour, self.minute, self.second)
            .ok_or(DateError::GregorianConversionError) // This path is highly unlikely if is_valid passed.
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
    /// Gregorian date/time provided by the system fails. This could potentially happen if the
    /// system clock is set to a date before the Persian epoch or encounters other issues during
    /// the conversion process handled by [`from_gregorian`].
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
    ///         // This might indicate a problem with the system clock or the conversion logic
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
    /// Specifiers requiring calculation (like `%A`, `%w`, `%j`) might return error indicators
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
    /// let dt = ParsiDateTime::new(1403, 5, 2, 8, 5, 30).unwrap(); // 1403/Mordad/02 08:05:30 (Tuesday)
    ///
    /// // Common ISO-like format (Persian date)
    /// assert_eq!(dt.format("%Y-%m-%d %H:%M:%S"), "1403-05-02 08:05:30");
    ///
    /// // Format with Persian names
    /// assert_eq!(dt.format("%d %B %Y ساعت %H:%M"), "02 مرداد 1403 ساعت 08:05");
    ///
    /// // Using %T for time
    /// assert_eq!(dt.format("%Y-%m-%dT%T"), "1403-05-02T08:05:30");
    ///
    /// // Including weekday and day of year
    /// // ParsiDate::new(1403, 5, 2) corresponds to Gregorian 2024-07-23 (Tuesday)
    /// // Parsi weekday: 1=Shanbeh, ..., 4=Seshanbeh (Tuesday) -> %w should be 3 (Sat=0)
    /// // Day of year: 31 (Far) + 31 (Ord) + 31 (Kho) + 31 (Tir) + 2 (Mor) = 126
    /// assert_eq!(dt.format("%A، %d %B %Y - %T (روز %j سال، روز هفته %w)"),
    ///              "سه‌شنبه، 02 مرداد 1403 - 08:05:30 (روز 126 سال، روز هفته 3)");
    ///
    /// // Literal percent sign
    /// assert_eq!(dt.format("Time is %H:%M %% %S seconds"), "Time is 08:05 % 30 seconds");
    ///
    /// // Formatting an invalid time (created unsafely)
    /// let invalid_dt = unsafe { ParsiDateTime::new_unchecked(1403, 1, 1, 25, 61, 99) };
    /// // Formatting behavior for invalid components is defined but might not be ideal:
    /// assert_eq!(invalid_dt.format("%H:%M:%S"), "25:61:99"); // Prints the invalid numbers
    /// // Note: Date parts would format normally if the date itself is valid.
    /// ```
    pub fn format(&self, pattern: &str) -> String {
        // Preallocate string with a reasonable estimate capacity to reduce reallocations.
        let mut result = String::with_capacity(pattern.len() + 20); // Estimate extra space needed
        // Use a character iterator for correct handling of multi-byte UTF-8 characters in the pattern.
        let mut chars = pattern.chars().peekable();

        // Caching results for potentially expensive date calculations if used multiple times
        // in the same format string (e.g., %A and %w).
        // Use Option<Result<...>> to store the result (Ok or Err) once calculated.
        let mut weekday_name_cache: Option<Result<String, DateError>> = None;
        let mut ordinal_day_cache: Option<Result<u32, DateError>> = None;
        let mut weekday_num_cache: Option<Result<u32, DateError>> = None; // Saturday = 0

        while let Some(c) = chars.next() {
            if c == '%' {
                // Check the character immediately following the '%'
                match chars.next() {
                    // --- Time Specifiers ---
                    Some('H') => result.push_str(&format!("{:02}", self.hour)), // Hour (00-23)
                    Some('M') => result.push_str(&format!("{:02}", self.minute)), // Minute (00-59)
                    Some('S') => result.push_str(&format!("{:02}", self.second)), // Second (00-59)
                    Some('T') => {
                        // Equivalent to %H:%M:%S
                        result.push_str(&format!(
                            "{:02}:{:02}:{:02}",
                            self.hour, self.minute, self.second
                        ))
                    }

                    // --- Date Specifiers (using self.date() or direct access) ---
                    Some('%') => result.push('%'), // Literal '%'
                    Some('Y') => result.push_str(&self.year().to_string()), // Year (e.g., 1403)
                    Some('m') => result.push_str(&format!("{:02}", self.month())), // Month (01-12)
                    Some('d') => result.push_str(&format!("{:02}", self.day())), // Day (01-31)
                    Some('B') => {
                        // Full Persian month name
                        // Get month index (0-11)
                        let month_index = self.month().saturating_sub(1) as usize;
                        if let Some(name) = MONTH_NAMES_PERSIAN.get(month_index) {
                            result.push_str(name);
                        } else {
                            // Handle case where month number is invalid (e.g., 0 or > 12)
                            result.push_str("?InvalidMonth?");
                        }
                    }
                    Some('A') => {
                        // Full Persian weekday name
                        // Calculate (or retrieve from cache) the weekday name
                        if weekday_name_cache.is_none() {
                            // Call the internal method on the date part which returns Result<String, _>
                            weekday_name_cache = Some(self.date.weekday_internal());
                        }
                        // Use the cached result
                        match weekday_name_cache.as_ref().unwrap() {
                            Ok(name) => result.push_str(name),
                            Err(_) => result.push_str("?WeekdayError?"), // Error during calculation (e.g., invalid date)
                        }
                    }
                    Some('w') => {
                        // Weekday number (Saturday=0)
                        // Calculate (or retrieve from cache) the weekday number
                        if weekday_num_cache.is_none() {
                            // Call the method on the date part which returns Result<u32, _>
                            weekday_num_cache = Some(self.date.weekday_num_sat_0());
                        }
                        // Use the cached result
                        match weekday_num_cache.as_ref().unwrap() {
                            Ok(num) => result.push_str(&num.to_string()),
                            Err(_) => result.push('?'), // Error during calculation
                        }
                    }
                    Some('j') => {
                        // Day of the year (001-366)
                        // Calculate (or retrieve from cache) the ordinal day
                        if ordinal_day_cache.is_none() {
                            // Call the internal method on the date part which returns Result<u32, _>
                            ordinal_day_cache = Some(self.date.ordinal_internal());
                        }
                        // Use the cached result
                        match ordinal_day_cache.as_ref().unwrap() {
                            Ok(ord) => result.push_str(&format!("{:03}", ord)), // Zero-pad to 3 digits
                            Err(_) => result.push_str("???"), // Error during calculation
                        }
                    }

                    // --- Unrecognized or Unsupported Specifier ---
                    Some(other) => {
                        // If % is followed by an unrecognized character, output % and the character literally
                        result.push('%');
                        result.push(other);
                    }
                    // --- Dangling '%' at the end of the format string ---
                    None => {
                        // Output the '%' literally if it's the last character
                        result.push('%');
                        // The loop will terminate as chars.next() returned None
                        break;
                    }
                }
            } else {
                // It's a literal character, just append it to the result.
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
    /// **Unsupported Specifiers:** Specifiers like `%A`, `%w`, `%j` are *not* supported for parsing
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
    /// *   `ParseErrorKind::UnsupportedSpecifier`: The `format` string contained a specifier not supported for parsing (e.g., `%A`, `%j`).
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
        // We will need to convert back to &str slices temporarily for UTF-8 parts like %B.
        let mut s_bytes = s.as_bytes();
        let mut fmt_bytes = format.as_bytes();

        // Iterate through the format string bytes
        while !fmt_bytes.is_empty() {
            // Check if the current format byte is '%' indicating a specifier
            if fmt_bytes[0] == b'%' {
                // Ensure there's a character after '%'
                if fmt_bytes.len() < 2 {
                    // Dangling '%' at the end of the format string
                    return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                }

                // Match the specifier character (fmt_bytes[1])
                match fmt_bytes[1] {
                    // --- Time Specifiers ---
                    b'H' | b'M' | b'S' => {
                        // Expect exactly 2 digits in the input string
                        if s_bytes.len() < 2 || !s_bytes[0..2].iter().all(|b| b.is_ascii_digit()) {
                            // Not enough characters or non-digit characters found
                            return Err(DateError::ParseError(ParseErrorKind::InvalidNumber));
                        }
                        // Safely parse the 2-digit number
                        // `from_utf8_unchecked` is safe here because we checked for ASCII digits.
                        let num_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..2]) };
                        let val: u32 = num_str
                            .parse()
                            // Map potential parsing error (e.g., overflow, though unlikely for 2 digits)
                            .map_err(|_| DateError::ParseError(ParseErrorKind::InvalidNumber))?;

                        // Store the parsed value in the correct Option
                        match fmt_bytes[1] {
                            b'H' => parsed_hour = Some(val),
                            b'M' => parsed_minute = Some(val),
                            b'S' => parsed_second = Some(val),
                            _ => unreachable!(), // Already handled by the outer match condition
                        }
                        // Consume the 2 digits from input and 2 bytes (%H/M/S) from format
                        s_bytes = &s_bytes[2..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    b'T' => {
                        // Expects "HH:MM:SS" (8 bytes total)
                        // Check length and structure (digits and colons)
                        if s_bytes.len() < 8 ||
                           !s_bytes[0..2].iter().all(|b| b.is_ascii_digit()) || s_bytes[2] != b':' || // HH:
                           !s_bytes[3..5].iter().all(|b| b.is_ascii_digit()) || s_bytes[5] != b':' || // MM:
                           !s_bytes[6..8].iter().all(|b| b.is_ascii_digit())
                        {
                            // SS
                            // Incorrect format structure or non-digit where digit expected
                            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                        }
                        // Parse HH, MM, SS parts. `from_utf8_unchecked` is safe due to digit checks.
                        let h_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..2]) };
                        let m_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[3..5]) };
                        let s_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[6..8]) };

                        // Parse each part to u32 and store
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

                        // Consume the 8 characters ("HH:MM:SS") from input and 2 bytes ("%T") from format
                        s_bytes = &s_bytes[8..];
                        fmt_bytes = &fmt_bytes[2..];
                    }

                    // --- Date Specifiers (adapted from ParsiDate::parse logic) ---
                    b'%' => {
                        // Literal '%%'
                        // Expect a literal '%' in the input
                        if s_bytes.is_empty() || s_bytes[0] != b'%' {
                            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                        }
                        // Consume '%' from input and '%%' from format
                        s_bytes = &s_bytes[1..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    b'Y' => {
                        // Year (%Y - 4 digits)
                        // Expect exactly 4 digits
                        if s_bytes.len() < 4 || !s_bytes[0..4].iter().all(|b| b.is_ascii_digit()) {
                            return Err(DateError::ParseError(ParseErrorKind::InvalidNumber));
                        }
                        let year_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..4]) };
                        parsed_year =
                            Some(year_str.parse().map_err(|_| {
                                DateError::ParseError(ParseErrorKind::InvalidNumber)
                            })?);
                        // Consume 4 digits from input and 2 bytes (%Y) from format
                        s_bytes = &s_bytes[4..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    b'm' | b'd' => {
                        // Month (%m - 2 digits) or Day (%d - 2 digits)
                        // Expect exactly 2 digits
                        if s_bytes.len() < 2 || !s_bytes[0..2].iter().all(|b| b.is_ascii_digit()) {
                            return Err(DateError::ParseError(ParseErrorKind::InvalidNumber));
                        }
                        let num_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..2]) };
                        let val: u32 = num_str
                            .parse()
                            .map_err(|_| DateError::ParseError(ParseErrorKind::InvalidNumber))?;
                        // Store in the correct Option based on the specifier
                        if fmt_bytes[1] == b'm' {
                            parsed_month = Some(val);
                        } else {
                            // fmt_bytes[1] == b'd'
                            parsed_day = Some(val);
                        }
                        // Consume 2 digits from input and 2 bytes (%m or %d) from format
                        s_bytes = &s_bytes[2..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    b'B' => {
                        // Month Name (%B - Persian full name)
                        // Consume the %B specifier from the format string first
                        fmt_bytes = &fmt_bytes[2..];
                        let mut found_month = false;
                        let mut best_match_len = 0; // Length of the matched month name in bytes
                        let mut matched_month_idx: usize = 0; // 0-based index of the matched month
                        // Convert the remaining input bytes to a &str slice for string matching
                        // This is safe because month names are valid UTF-8.
                        let current_s_str = match std::str::from_utf8(s_bytes) {
                            Ok(s_str) => s_str,
                            // If input is not valid UTF-8 here, it cannot match a month name
                            Err(_) => {
                                return Err(DateError::ParseError(
                                    ParseErrorKind::InvalidMonthName,
                                ));
                            }
                        };

                        // Iterate through the known Persian month names
                        for (idx, month_name) in MONTH_NAMES_PERSIAN.iter().enumerate() {
                            // Check if the input string starts with the current month name
                            if current_s_str.starts_with(month_name) {
                                // Found a match
                                best_match_len = month_name.as_bytes().len(); // Use byte length for slicing s_bytes
                                matched_month_idx = idx; // Store the 0-based index
                                found_month = true;
                                break; // Stop searching once a match is found
                            }
                        }

                        if !found_month {
                            // No month name matched at the current input position
                            return Err(DateError::ParseError(ParseErrorKind::InvalidMonthName));
                        }
                        // Store the parsed month number (1-based index)
                        parsed_month = Some((matched_month_idx + 1) as u32);
                        // Consume the matched month name (by its byte length) from the input byte slice
                        s_bytes = &s_bytes[best_match_len..];
                        // fmt_bytes was already advanced before the loop
                    }

                    // --- Unsupported Specifiers for Parsing ---
                    b'A' | b'w' | b'j' | _ => {
                        // Any other specifier is not supported for parsing
                        return Err(DateError::ParseError(ParseErrorKind::UnsupportedSpecifier));
                    }
                }
            } else {
                // Literal character in the format string
                // Expect the same literal character at the start of the input string
                if s_bytes.is_empty() || s_bytes[0] != fmt_bytes[0] {
                    // Input is shorter than format or characters don't match
                    return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                }
                // Consume the matching literal character from both input and format
                s_bytes = &s_bytes[1..];
                fmt_bytes = &fmt_bytes[1..];
            }
        } // End while loop over format bytes

        // After processing the entire format string, check if there are any remaining characters in the input.
        if !s_bytes.is_empty() {
            // Input string has extra characters not consumed by the format pattern
            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
        }

        // Check if all required components (Y, m, d, H, M, S) were successfully parsed
        match (
            parsed_year,
            parsed_month,
            parsed_day,
            parsed_hour,
            parsed_minute,
            parsed_second,
        ) {
            (Some(y), Some(m), Some(d), Some(h), Some(min), Some(s)) => {
                // All components were extracted. Now, use the standard `ParsiDateTime::new`
                // constructor to perform final validation (logical date validity and time ranges).
                ParsiDateTime::new(y, m, d, h, min, s).map_err(|e| {
                    // Map the validation errors from `new` to the appropriate ParseErrorKind
                    match e {
                        DateError::InvalidDate => {
                            DateError::ParseError(ParseErrorKind::InvalidDateValue)
                        }
                        DateError::InvalidTime => {
                            DateError::ParseError(ParseErrorKind::InvalidTimeValue)
                        }
                        // Propagate any other unexpected errors (though less likely here)
                        other_error => other_error,
                    }
                })
            }
            _ => {
                // Not all required components were found in the input string based on the format
                Err(DateError::ParseError(ParseErrorKind::FormatMismatch))
            }
        }
    }

    // --- Arithmetic ---
    // Note: Performing arithmetic using chrono::Duration is generally the most robust way
    // as it correctly handles rollovers across seconds, minutes, hours, and days,
    // leveraging chrono's well-tested logic via Gregorian conversion.

    /// Adds a `chrono::Duration` to this `ParsiDateTime`.
    ///
    /// This operation is performed by:
    /// 1. Validating the current `ParsiDateTime`.
    /// 2. Converting the current `ParsiDateTime` to its Gregorian `NaiveDateTime` equivalent.
    /// 3. Adding the `Duration` to the `NaiveDateTime` using `chrono`'s arithmetic (which handles rollovers).
    /// 4. Converting the resulting `NaiveDateTime` back to `ParsiDateTime`.
    ///
    /// This approach ensures accurate handling of time and date rollovers, including crossing month and year boundaries.
    /// The `duration` can be positive (moving forward in time) or negative (moving backward).
    ///
    /// # Arguments
    ///
    /// * `duration`: The `chrono::Duration` to add. Examples: `Duration::seconds(10)`, `Duration::minutes(-5)`, `Duration::days(1)`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// *   The initial `ParsiDateTime` instance (`self`) is invalid ([`DateError::InvalidDate`] or [`DateError::InvalidTime`]).
    /// *   The conversion to `NaiveDateTime` fails ([`DateError::GregorianConversionError`]).
    /// *   The addition using `chrono` results in an overflow or underflow (e.g., goes beyond the representable date range for `NaiveDateTime`), returning [`DateError::ArithmeticOverflow`].
    /// *   The conversion of the result back from `NaiveDateTime` to `ParsiDateTime` fails (e.g., resulting Gregorian date is outside the supported range for `ParsiDateTime`), returning [`DateError::GregorianConversionError`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    /// use chrono::Duration;
    ///
    /// // --- Basic Addition ---
    /// let dt = ParsiDateTime::new(1403, 5, 2, 10, 30, 0).unwrap();
    /// let dt_plus_90s = dt.add_duration(Duration::seconds(90));
    /// assert!(dt_plus_90s.is_ok());
    /// assert_eq!(dt_plus_90s.unwrap().time(), (10, 31, 30)); // 10:30:00 + 90s = 10:31:30
    ///
    /// // --- Crossing Midnight ---
    /// let dt_near_midnight = ParsiDateTime::new(1403, 1, 1, 23, 59, 58).unwrap();
    /// let dt_next_day = dt_near_midnight.add_duration(Duration::seconds(3)); // Add 3 seconds
    /// assert!(dt_next_day.is_ok());
    /// let dt_next_day = dt_next_day.unwrap();
    /// assert_eq!(dt_next_day.date(), ParsiDate::new(1403, 1, 2).unwrap()); // Date advances
    /// assert_eq!(dt_next_day.time(), (0, 0, 1)); // Time resets
    ///
    /// // --- Crossing Year Boundary (End of Leap Year 1403) ---
    /// let dt_leap_end = ParsiDateTime::new(1403, 12, 30, 23, 59, 50).unwrap();
    /// let dt_new_year = dt_leap_end.add_duration(Duration::seconds(15)); // Add 15 seconds
    /// assert!(dt_new_year.is_ok());
    /// let dt_new_year = dt_new_year.unwrap();
    /// assert_eq!(dt_new_year.date(), ParsiDate::new(1404, 1, 1).unwrap()); // New year 1404
    /// assert_eq!(dt_new_year.time(), (0, 0, 5));
    ///
    /// // --- Subtraction using negative duration ---
    /// let dt_start_of_day = ParsiDateTime::new(1403, 6, 10, 0, 0, 5).unwrap();
    /// let dt_prev_day = dt_start_of_day.add_duration(Duration::seconds(-10)); // Subtract 10 seconds
    /// assert!(dt_prev_day.is_ok());
    /// let dt_prev_day = dt_prev_day.unwrap();
    /// assert_eq!(dt_prev_day.date(), ParsiDate::new(1403, 6, 9).unwrap()); // Previous day
    /// assert_eq!(dt_prev_day.time(), (23, 59, 55));
    ///
    /// // --- Adding Days ---
    /// let dt_add_days = dt.add_duration(Duration::days(40)); // Add 40 days
    /// assert!(dt_add_days.is_ok());
    /// let expected_date = ParsiDate::new(1403, 6, 11).unwrap(); // 1403/05/02 + 40 days = 1403/06/11
    /// assert_eq!(dt_add_days.unwrap().date(), expected_date);
    /// assert_eq!(dt_add_days.unwrap().time(), (10, 30, 0)); // Time is preserved
    ///
    /// // --- Invalid Input Date ---
    /// let invalid_dt = unsafe { ParsiDateTime::new_unchecked(1404, 12, 30, 10, 0, 0) }; // Invalid date
    /// assert_eq!(invalid_dt.add_duration(Duration::days(1)), Err(DateError::InvalidDate));
    /// ```
    pub fn add_duration(&self, duration: Duration) -> Result<Self, DateError> {
        // 1. Validate the starting ParsiDateTime.
        if !self.is_valid() {
            // Determine if the date or time part is invalid.
            if !self.date.is_valid() {
                return Err(DateError::InvalidDate);
            } else {
                return Err(DateError::InvalidTime);
            }
        }

        // 2. Convert self to Gregorian NaiveDateTime. This can return GregorianConversionError.
        let gregorian_dt = self.to_gregorian()?; // Note: to_gregorian implicitly calls is_valid again, but that's okay.

        // 3. Add the duration using chrono's checked addition.
        // `checked_add_signed` takes a Duration and returns Option<NaiveDateTime>.
        // It returns None if the addition results in overflow/underflow.
        let new_gregorian_dt = gregorian_dt
            .checked_add_signed(duration)
            .ok_or(DateError::ArithmeticOverflow)?; // Map None to our specific overflow error

        // 4. Convert the resulting Gregorian NaiveDateTime back to ParsiDateTime.
        // This can also return GregorianConversionError if the result is out of ParsiDateTime's range.
        Self::from_gregorian(new_gregorian_dt)
    }

    /// Subtracts a `chrono::Duration` from this `ParsiDateTime`.
    ///
    /// This is a convenience method equivalent to calling `add_duration` with the negated `duration`.
    /// It follows the same conversion process (Parsi -> Gregorian -> Subtract -> Gregorian -> Parsi)
    /// to ensure accurate handling of date and time rollovers.
    ///
    /// # Arguments
    ///
    /// * `duration`: The `chrono::Duration` to subtract. Must be non-negative if considered as a magnitude,
    ///   but `chrono::Duration` itself handles positive/negative values internally. Examples: `Duration::seconds(10)`, `Duration::hours(1)`, `Duration::days(7)`.
    ///
    /// # Errors
    ///
    /// Returns `Err` under the same conditions as [`add_duration`]:
    /// *   Invalid initial `ParsiDateTime` ([`DateError::InvalidDate`], [`DateError::InvalidTime`]).
    /// *   Conversion failure ([`DateError::GregorianConversionError`]).
    /// *   Arithmetic overflow/underflow during subtraction ([`DateError::ArithmeticOverflow`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    /// use chrono::Duration;
    ///
    /// // --- Basic Subtraction ---
    /// let dt = ParsiDateTime::new(1403, 5, 2, 10, 30, 15).unwrap();
    /// let dt_minus_20s = dt.sub_duration(Duration::seconds(20));
    /// assert!(dt_minus_20s.is_ok());
    /// assert_eq!(dt_minus_20s.unwrap().time(), (10, 29, 55)); // 10:30:15 - 20s = 10:29:55
    ///
    /// // --- Crossing Midnight Backwards ---
    /// let dt_start_of_day = ParsiDateTime::new(1403, 1, 2, 0, 0, 5).unwrap();
    /// let dt_prev_day = dt_start_of_day.sub_duration(Duration::seconds(10)); // Subtract 10 seconds
    /// assert!(dt_prev_day.is_ok());
    /// let dt_prev_day = dt_prev_day.unwrap();
    /// assert_eq!(dt_prev_day.date(), ParsiDate::new(1403, 1, 1).unwrap()); // Date goes back
    /// assert_eq!(dt_prev_day.time(), (23, 59, 55)); // Time wraps around
    ///
    /// // --- Crossing Year Boundary Backwards (Start of Non-Leap Year 1404) ---
    /// let dt_new_year = ParsiDateTime::new(1404, 1, 1, 0, 0, 5).unwrap(); // Start of 1404
    /// let dt_leap_end = dt_new_year.sub_duration(Duration::seconds(10)); // Subtract 10 seconds
    /// assert!(dt_leap_end.is_ok());
    /// let dt_leap_end = dt_leap_end.unwrap();
    /// // Goes back to the end of the leap year 1403 (Esfand 30th)
    /// assert_eq!(dt_leap_end.date(), ParsiDate::new(1403, 12, 30).unwrap());
    /// assert_eq!(dt_leap_end.time(), (23, 59, 55));
    ///
    /// // --- Subtracting Days ---
    /// let dt = ParsiDateTime::new(1403, 2, 5, 12, 0, 0).unwrap(); // Ordibehesht 5
    /// let dt_minus_days = dt.sub_duration(Duration::days(10));
    /// assert!(dt_minus_days.is_ok());
    /// // 1403/02/05 - 10 days = 1403/01/26 (Farvardin has 31 days)
    /// assert_eq!(dt_minus_days.unwrap().date(), ParsiDate::new(1403, 1, 26).unwrap());
    /// assert_eq!(dt_minus_days.unwrap().time(), (12, 0, 0)); // Time preserved
    ///
    /// // --- Invalid Input Date ---
    /// let invalid_dt = unsafe { ParsiDateTime::new_unchecked(1403, 1, 1, 25, 0, 0) }; // Invalid time
    /// assert_eq!(invalid_dt.sub_duration(Duration::days(1)), Err(DateError::InvalidTime));
    /// ```
    pub fn sub_duration(&self, duration: Duration) -> Result<Self, DateError> {
        // Subtracting a duration is the same as adding its negation.
        // chrono::Duration handles negation correctly (e.g., negating Duration::seconds(10) gives Duration::seconds(-10)).
        self.add_duration(-duration)
    }

    /// Adds a specified number of days to the date part of this `ParsiDateTime`, preserving the time component.
    ///
    /// This method delegates the date calculation to [`ParsiDate::add_days`] and keeps the
    /// `hour`, `minute`, and `second` components unchanged. Use [`add_duration`] if you need
    /// calculations that might roll over time components (e.g., adding 25 hours).
    ///
    /// # Arguments
    ///
    /// * `days`: The number of days to add. Can be positive to move forward or negative to move backward.
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// *   The initial `ParsiDateTime` instance (`self`) is invalid ([`DateError::InvalidDate`] or [`DateError::InvalidTime`]).
    /// *   The underlying date calculation via `ParsiDate::add_days` fails (e.g., results in a date outside the supported year range), returning the error from `ParsiDate` (likely [`DateError::GregorianConversionError`] or [`DateError::ArithmeticOverflow`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    ///
    /// let dt = ParsiDateTime::new(1403, 1, 15, 10, 30, 0).unwrap();
    ///
    /// // Add 20 days
    /// let dt_plus_20d = dt.add_days(20);
    /// assert!(dt_plus_20d.is_ok());
    /// let dt_plus_20d = dt_plus_20d.unwrap();
    /// // 1403/01/15 + 20 days = 1403/02/04 (Farvardin 31 days)
    /// assert_eq!(dt_plus_20d.date(), ParsiDate::new(1403, 2, 4).unwrap());
    /// assert_eq!(dt_plus_20d.time(), (10, 30, 0)); // Time is unchanged
    ///
    /// // Subtract 20 days (using negative input)
    /// let dt_minus_20d = dt.add_days(-20);
    /// assert!(dt_minus_20d.is_ok());
    /// let dt_minus_20d = dt_minus_20d.unwrap();
    /// // 1403/01/15 - 20 days = 1402/12/26 (Year 1402, Esfand has 29 days)
    /// assert_eq!(dt_minus_20d.date(), ParsiDate::new(1402, 12, 24).unwrap());
    /// assert_eq!(dt_minus_20d.time(), (10, 30, 0)); // Time is unchanged
    /// ```
    pub fn add_days(&self, days: i64) -> Result<Self, DateError> {
        // Validate self first
        if !self.is_valid() {
            if !self.date.is_valid() {
                return Err(DateError::InvalidDate);
            } else {
                return Err(DateError::InvalidTime);
            }
        }
        // Delegate date addition to ParsiDate::add_days
        let new_date = self.date.add_days(days)?;
        // Recombine the new date with the original, known-valid time components.
        // No need to call ParsiDateTime::new, we can construct directly.
        Ok(ParsiDateTime {
            date: new_date,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
        })
    }

    /// Subtracts a specified number of days from the date part, preserving the time component.
    ///
    /// This is a convenience method equivalent to `add_days(-days)`.
    /// It delegates the date calculation to [`ParsiDate::sub_days`].
    ///
    /// # Arguments
    ///
    /// * `days`: The non-negative number of days to subtract.
    ///
    /// # Errors
    ///
    /// Returns `Err` under the same conditions as [`add_days`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    ///
    /// let dt = ParsiDateTime::new(1403, 1, 15, 10, 30, 0).unwrap();
    ///
    /// // Subtract 20 days
    /// let dt_minus_20d = dt.sub_days(20);
    /// assert!(dt_minus_20d.is_ok());
    /// let dt_minus_20d = dt_minus_20d.unwrap();
    /// // 1403/01/15 - 20 days = 1402/12/26 (Year 1402, Esfand has 29 days)
    /// assert_eq!(dt_minus_20d.date(), ParsiDate::new(1402, 12, 24).unwrap());
    /// assert_eq!(dt_minus_20d.time(), (10, 30, 0)); // Time is unchanged
    /// ```
    pub fn sub_days(&self, days: u64) -> Result<Self, DateError> {
        // Validate self first
        if !self.is_valid() {
            if !self.date.is_valid() {
                return Err(DateError::InvalidDate);
            } else {
                return Err(DateError::InvalidTime);
            }
        }
        // Delegate date subtraction to ParsiDate::sub_days
        let new_date = self.date.sub_days(days)?;
        // Recombine with original time
        Ok(ParsiDateTime {
            date: new_date,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
        })
    }

    /// Adds a specified number of months to the date part, preserving the time component and clamping the day if necessary.
    ///
    /// This method delegates the date calculation to [`ParsiDate::add_months`]. If the resulting month
    /// has fewer days than the original day, the day will be clamped to the last day of the new month
    /// (e.g., adding 1 month to 1403/06/31 results in 1403/07/30). The time component remains unchanged.
    ///
    /// # Arguments
    ///
    /// * `months`: The number of months to add. Can be positive or negative.
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// *   The initial `ParsiDateTime` instance (`self`) is invalid ([`DateError::InvalidDate`] or [`DateError::InvalidTime`]).
    /// *   The underlying date calculation via `ParsiDate::add_months` fails (e.g., results in a date outside the supported year range), returning the error from `ParsiDate`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    ///
    /// let dt = ParsiDateTime::new(1403, 6, 31, 12, 0, 0).unwrap(); // Shahrivar 31st
    ///
    /// // Add 1 month (Shahrivar 31 -> Mehr 30, as Mehr has 30 days)
    /// let dt_plus_1m = dt.add_months(1);
    /// assert!(dt_plus_1m.is_ok());
    /// let dt_plus_1m = dt_plus_1m.unwrap();
    /// assert_eq!(dt_plus_1m.date(), ParsiDate::new(1403, 7, 30).unwrap()); // Clamped to day 30
    /// assert_eq!(dt_plus_1m.time(), (12, 0, 0)); // Time unchanged
    ///
    /// // Add 7 months (Shahrivar 31 -> Farvardin 30 of next year, clamped)
    /// let dt_plus_7m = dt.add_months(7);
    /// assert!(dt_plus_7m.is_ok());
    /// let dt_plus_7m = dt_plus_7m.unwrap();
    /// assert_eq!(dt_plus_7m.date(), ParsiDate::new(1404, 1, 31).unwrap()); // Clamped to 30 (Farvardin has 31) - Note: ParsiDate::add_months clamps here. Check ParsiDate logic. Actually Farvardin has 31 days, so it should be 1404/01/31. Re-testing...
    /// assert_eq!(ParsiDate::new(1403, 6, 31).unwrap().add_months(7).unwrap(), ParsiDate::new(1404, 1, 31).unwrap()); // Yes, it becomes Farvardin 30.
    /// assert_eq!(dt_plus_7m.date(), ParsiDate::new(1404, 1, 31).unwrap());
    /// assert_eq!(dt_plus_7m.time(), (12, 0, 0));
    ///
    /// // Subtract 7 months (Shahrivar 31 -> Esfand 29 of previous year, 1402 is non-leap)
    /// let dt_minus_7m = dt.add_months(-7);
    /// assert!(dt_minus_7m.is_ok());
    /// let dt_minus_7m = dt_minus_7m.unwrap();
    /// assert_eq!(dt_minus_7m.date(), ParsiDate::new(1402, 11, 30).unwrap()); // Bahman has 30 days. Let's recheck ParsiDate logic.
    /// assert_eq!(ParsiDate::new(1403, 6, 31).unwrap().add_months(-7).unwrap(), ParsiDate::new(1402, 11, 30).unwrap()); // Correct, Bahman 30th.
    /// assert_eq!(dt_minus_7m.date(), ParsiDate::new(1402, 11, 30).unwrap());
    /// assert_eq!(dt_minus_7m.time(), (12, 0, 0));
    /// ```
    pub fn add_months(&self, months: i32) -> Result<Self, DateError> {
        // Validate self first
        if !self.is_valid() {
            if !self.date.is_valid() {
                return Err(DateError::InvalidDate);
            } else {
                return Err(DateError::InvalidTime);
            }
        }
        // Delegate date addition to ParsiDate::add_months
        let new_date = self.date.add_months(months)?;
        // Recombine with original time
        Ok(ParsiDateTime {
            date: new_date,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
        })
    }

    /// Subtracts a specified number of months, preserving the time component and clamping the day if necessary.
    ///
    /// This is a convenience method equivalent to `add_months(-months)`.
    /// It delegates the date calculation to [`ParsiDate::sub_months`].
    ///
    /// # Arguments
    ///
    /// * `months`: The non-negative number of months to subtract.
    ///
    /// # Errors
    ///
    /// Returns `Err` under the same conditions as [`add_months`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    ///
    /// let dt = ParsiDateTime::new(1403, 1, 31, 9, 0, 0).unwrap(); // Farvardin 31st
    ///
    /// // Subtract 11 months (Farvardin 31 -> Ordibehesht 30 of previous year)
    /// let dt_minus_11m = dt.sub_months(11);
    /// assert!(dt_minus_11m.is_ok());
    /// let dt_minus_11m = dt_minus_11m.unwrap();
    /// // 1403/01/31 -> 1402/02/31 -> clamps to 1402/02/31 (Ordibehesht has 31 days)
    /// assert_eq!(dt_minus_11m.date(), ParsiDate::new(1402, 2, 31).unwrap());
    /// assert_eq!(dt_minus_11m.time(), (9, 0, 0));
    ///
    /// // Subtract 7 months (Farvardin 31 -> Shahrivar 30 of previous year)
    /// let dt_minus_7m = dt.sub_months(7);
    /// assert!(dt_minus_7m.is_ok());
    /// let dt_minus_7m = dt_minus_7m.unwrap();
    /// // 1403/01/31 -> 1402/06/31 -> clamps to 1402/06/31 (Shahrivar has 31 days)
    /// assert_eq!(dt_minus_7m.date(), ParsiDate::new(1402, 6, 31).unwrap());
    /// assert_eq!(dt_minus_7m.time(), (9, 0, 0));
    /// ```
    pub fn sub_months(&self, months: u32) -> Result<Self, DateError> {
        // Validate self first
        if !self.is_valid() {
            if !self.date.is_valid() {
                return Err(DateError::InvalidDate);
            } else {
                return Err(DateError::InvalidTime);
            }
        }
        // Delegate date subtraction to ParsiDate::sub_months
        let new_date = self.date.sub_months(months)?;
        // Recombine with original time
        Ok(ParsiDateTime {
            date: new_date,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
        })
    }

    /// Adds a specified number of years to the date part, preserving the time component and adjusting for leap days if necessary.
    ///
    /// This method delegates the date calculation to [`ParsiDate::add_years`]. If the original date
    /// is Esfand 30th in a leap year, adding years might result in Esfand 29th if the target year
    /// is not a leap year. The time component remains unchanged.
    ///
    /// # Arguments
    ///
    /// * `years`: The number of years to add. Can be positive or negative.
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// *   The initial `ParsiDateTime` instance (`self`) is invalid ([`DateError::InvalidDate`] or [`DateError::InvalidTime`]).
    /// *   The underlying date calculation via `ParsiDate::add_years` fails (e.g., results in a year outside the supported range), returning the error from `ParsiDate`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    ///
    /// // --- Adding years ---
    /// let dt = ParsiDateTime::new(1400, 5, 10, 14, 0, 0).unwrap();
    /// let dt_plus_5y = dt.add_years(5);
    /// assert!(dt_plus_5y.is_ok());
    /// let dt_plus_5y = dt_plus_5y.unwrap();
    /// assert_eq!(dt_plus_5y.date(), ParsiDate::new(1405, 5, 10).unwrap());
    /// assert_eq!(dt_plus_5y.time(), (14, 0, 0));
    ///
    /// // --- Subtracting years (using negative input) ---
    /// let dt_minus_10y = dt.add_years(-10);
    /// assert!(dt_minus_10y.is_ok());
    /// let dt_minus_10y = dt_minus_10y.unwrap();
    /// assert_eq!(dt_minus_10y.date(), ParsiDate::new(1390, 5, 10).unwrap());
    /// assert_eq!(dt_minus_10y.time(), (14, 0, 0));
    ///
    /// // --- Leap day adjustment ---
    /// let dt_leap_day = ParsiDateTime::new(1403, 12, 30, 10, 0, 0).unwrap(); // Esfand 30th, 1403 (leap)
    /// // Add 1 year to a non-leap year (1404)
    /// let dt_next_year = dt_leap_day.add_years(1);
    /// assert!(dt_next_year.is_ok());
    /// let dt_next_year = dt_next_year.unwrap();
    /// assert_eq!(dt_next_year.date(), ParsiDate::new(1404, 12, 29).unwrap()); // Becomes Esfand 29th
    /// assert_eq!(dt_next_year.time(), (10, 0, 0));
    ///
    /// // Add 4 years to another leap year (1407)
    /// let dt_plus_4y = dt_leap_day.add_years(4);
    /// assert!(dt_plus_4y.is_ok());
    /// let dt_plus_4y = dt_plus_4y.unwrap();
    /// assert_eq!(dt_plus_4y.date(), ParsiDate::new(1407, 12, 29).unwrap()); // Remains Esfand 29th
    /// assert_eq!(dt_plus_4y.time(), (10, 0, 0));
    /// ```
    pub fn add_years(&self, years: i32) -> Result<Self, DateError> {
        // Validate self first
        if !self.is_valid() {
            if !self.date.is_valid() {
                return Err(DateError::InvalidDate);
            } else {
                return Err(DateError::InvalidTime);
            }
        }
        // Delegate date addition to ParsiDate::add_years
        let new_date = self.date.add_years(years)?;
        // Recombine with original time
        Ok(ParsiDateTime {
            date: new_date,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
        })
    }

    /// Subtracts a specified number of years, preserving the time component and adjusting for leap days.
    ///
    /// This is a convenience method equivalent to `add_years(-years)`.
    /// It delegates the date calculation to [`ParsiDate::sub_years`].
    ///
    /// # Arguments
    ///
    /// * `years`: The non-negative number of years to subtract.
    ///
    /// # Errors
    ///
    /// Returns `Err` under the same conditions as [`add_years`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    ///
    /// let dt_leap_day = ParsiDateTime::new(1403, 12, 30, 10, 0, 0).unwrap(); // Esfand 30th, 1403 (leap)
    ///
    /// // Subtract 1 year -> 1402 (non-leap)
    /// let dt_minus_1y = dt_leap_day.sub_years(1);
    /// assert!(dt_minus_1y.is_ok());
    /// let dt_minus_1y = dt_minus_1y.unwrap();
    /// assert_eq!(dt_minus_1y.date(), ParsiDate::new(1402, 12, 29).unwrap()); // Becomes Esfand 29th
    /// assert_eq!(dt_minus_1y.time(), (10, 0, 0));
    ///
    /// // Subtract 4 years -> 1399 (leap)
    /// let dt_minus_4y = dt_leap_day.sub_years(4);
    /// assert!(dt_minus_4y.is_ok());
    /// let dt_minus_4y = dt_minus_4y.unwrap();
    /// assert_eq!(dt_minus_4y.date(), ParsiDate::new(1399, 12, 30).unwrap()); // Remains Esfand 30th
    /// assert_eq!(dt_minus_4y.time(), (10, 0, 0));
    /// ```
    pub fn sub_years(&self, years: u32) -> Result<Self, DateError> {
        // Validate self first
        if !self.is_valid() {
            if !self.date.is_valid() {
                return Err(DateError::InvalidDate);
            } else {
                return Err(DateError::InvalidTime);
            }
        }
        // Delegate date subtraction to ParsiDate::sub_years
        let new_date = self.date.sub_years(years)?;
        // Recombine with original time
        Ok(ParsiDateTime {
            date: new_date,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
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
    /// let dt_evening = dt_evening.unwrap();
    /// assert_eq!(dt_evening.hour(), 18);
    /// assert_eq!(dt_evening.minute(), 30); // Minute unchanged
    /// assert_eq!(dt_evening.date(), dt.date()); // Date unchanged
    ///
    /// // Try to set an invalid hour
    /// assert_eq!(dt.with_hour(24), Err(DateError::InvalidTime));
    ///
    /// // Using an invalid starting date
    /// let invalid_date_dt = unsafe { ParsiDateTime::new_unchecked(1404, 12, 30, 10, 0, 0) };
    /// assert_eq!(invalid_date_dt.with_hour(11), Err(DateError::InvalidDate));
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
        // Create new instance with updated hour
        Ok(ParsiDateTime {
            date: self.date,     // Keep original date
            hour,                // Use the new hour
            minute: self.minute, // Keep original minute
            second: self.second, // Keep original second
        })
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
    /// let dt_new_min = dt_new_min.unwrap();
    /// assert_eq!(dt_new_min.minute(), 55);
    /// assert_eq!(dt_new_min.hour(), 10); // Hour unchanged
    /// assert_eq!(dt_new_min.second(), 45); // Second unchanged
    /// assert_eq!(dt_new_min.date(), dt.date()); // Date unchanged
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
        Ok(ParsiDateTime {
            date: self.date,
            hour: self.hour,
            minute, // Use new minute
            second: self.second,
        })
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
    /// let dt_new_sec = dt_new_sec.unwrap();
    /// assert_eq!(dt_new_sec.second(), 0);
    /// assert_eq!(dt_new_sec.hour(), 10); // Hour unchanged
    /// assert_eq!(dt_new_sec.minute(), 30); // Minute unchanged
    /// assert_eq!(dt_new_sec.date(), dt.date()); // Date unchanged
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
        Ok(ParsiDateTime {
            date: self.date,
            hour: self.hour,
            minute: self.minute,
            second, // Use new second
        })
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
    /// let dt_new_time = dt_new_time.unwrap();
    /// assert_eq!(dt_new_time.time(), (23, 59, 59));
    /// assert_eq!(dt_new_time.date(), dt.date()); // Date unchanged
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
            date: self.date, // Keep original date
            hour,            // Use new hour
            minute,          // Use new minute
            second,          // Use new second
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
    /// Returns `Err` if:
    /// *   The underlying call to `ParsiDate::with_year` fails. This can happen if the original date was invalid,
    ///     if the combination of the new `year` with the existing month/day is invalid (e.g., setting year
    ///     to a non-leap year when the day is Esfand 30th), or if the new year is outside the supported range.
    ///     The specific error ([`DateError::InvalidDate`], [`DateError::YearOutOfRange`]) is propagated.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    ///
    /// let dt = ParsiDateTime::new(1403, 5, 2, 10, 30, 0).unwrap(); // 1403/05/02
    ///
    /// // Change year to 1399
    /// let dt_new_year = dt.with_year(1399);
    /// assert!(dt_new_year.is_ok());
    /// let dt_new_year = dt_new_year.unwrap();
    /// assert_eq!(dt_new_year.year(), 1399);
    /// assert_eq!(dt_new_year.month(), 5); // Month unchanged
    /// assert_eq!(dt_new_year.day(), 2); // Day unchanged
    /// assert_eq!(dt_new_year.time(), (10, 30, 0)); // Time unchanged
    ///
    /// // Example with leap day adjustment
    /// let dt_leap = ParsiDateTime::new(1403, 12, 30, 11, 0, 0).unwrap(); // Leap year
    /// let dt_non_leap_year = dt_leap.with_year(1404); // 1404 is not leap
    /// assert!(dt_non_leap_year.is_ok());
    /// // The day gets adjusted to 29, as 1404/12/30 is invalid
    /// assert_eq!(dt_non_leap_year.unwrap().date(), ParsiDate::new(1404, 12, 29).unwrap());
    ///
    /// // Example resulting in invalid date (original day invalid for new year/month combo)
    /// let dt_invalid_combo = ParsiDateTime::new(1404, 12, 29, 11, 0, 0).unwrap(); // Non-leap year
    /// // Try setting year back to a leap year (1403) when day is 29 (which is valid)
    /// let dt_valid_again = dt_invalid_combo.with_year(1403);
    /// assert!(dt_valid_again.is_ok()); // This should work
    /// assert_eq!(dt_valid_again.unwrap().date(), ParsiDate::new(1403, 12, 29).unwrap());
    /// // Note: A case where `with_year` itself fails might involve setting a year < 1 or > 9999 if those checks are in ParsiDate::with_year.
    /// ```
    pub fn with_year(&self, year: i32) -> Result<Self, DateError> {
        // Delegate the date modification and validation to ParsiDate
        let new_date = self.date.with_year(year)?;
        // If ParsiDate::with_year succeeded, the new date is valid.
        // The original time components were already valid (or checked by earlier calls),
        // so we can safely recombine them.
        Ok(ParsiDateTime {
            date: new_date,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
        })
    }

    /// Creates a new `ParsiDateTime` instance with only the month component of the date changed.
    ///
    /// The year, day, and all time components remain the same. This method delegates the
    /// month change and associated validation (e.g., clamping the day if it exceeds the number
    /// of days in the new month) to [`ParsiDate::with_month`].
    ///
    /// # Arguments
    ///
    /// * `month`: The desired new Persian month (1-12).
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// *   The underlying call to `ParsiDate::with_month` fails. This can happen if the original date was invalid,
    ///     if the `month` value is invalid (e.g., 0 or 13), or if the combination of the existing day
    ///     with the new `month` is invalid (after potential clamping). The specific error ([`DateError::InvalidDate`],
    ///     [`DateError::InvalidMonth`]) is propagated.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    ///
    /// let dt = ParsiDateTime::new(1403, 1, 31, 10, 0, 0).unwrap(); // Farvardin 31st
    ///
    /// // Change month to Shahrivar (month 6, also 31 days)
    /// let dt_new_month = dt.with_month(6);
    /// assert!(dt_new_month.is_ok());
    /// let dt_new_month = dt_new_month.unwrap();
    /// assert_eq!(dt_new_month.month(), 6);
    /// assert_eq!(dt_new_month.day(), 31); // Day remains 31
    /// assert_eq!(dt_new_month.year(), 1403); // Year unchanged
    /// assert_eq!(dt_new_month.time(), (10, 0, 0)); // Time unchanged
    ///
    /// // Change month to Mehr (month 7, only 30 days)
    /// let dt_clamped_month = dt.with_month(7);
    /// assert!(dt_clamped_month.is_ok());
    /// let dt_clamped_month = dt_clamped_month.unwrap();
    /// assert_eq!(dt_clamped_month.month(), 7);
    /// assert_eq!(dt_clamped_month.day(), 30); // Day clamped to 30
    ///
    /// // Try setting an invalid month
    /// assert!(matches!(dt.with_month(13), Err(DateError::InvalidDate))); // Check error type
    /// ```
    pub fn with_month(&self, month: u32) -> Result<Self, DateError> {
        // Delegate the date modification and validation to ParsiDate
        let new_date = self.date.with_month(month)?;
        // Recombine the validated new date with the original time
        Ok(ParsiDateTime {
            date: new_date,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
        })
    }

    /// Creates a new `ParsiDateTime` instance with only the day component of the date changed.
    ///
    /// The year, month, and all time components remain the same. This method delegates the
    /// day change and associated validation (ensuring the day is valid for the existing year and month)
    /// to [`ParsiDate::with_day`].
    ///
    /// # Arguments
    ///
    /// * `day`: The desired new day of the month (1-31).
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// *   The underlying call to `ParsiDate::with_day` fails. This can happen if the original date was invalid,
    ///     or if the new `day` value is invalid for the current year and month (e.g., day 31 in Mehr,
    ///     or day 30 in Esfand of a non-leap year). The specific error ([`DateError::InvalidDate`],
    ///     [`DateError::InvalidDay`]) is propagated.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    ///
    /// let dt = ParsiDateTime::new(1403, 7, 15, 12, 0, 0).unwrap(); // Mehr 15th (Mehr has 30 days)
    ///
    /// // Change day to 30
    /// let dt_new_day = dt.with_day(30);
    /// assert!(dt_new_day.is_ok());
    /// let dt_new_day = dt_new_day.unwrap();
    /// assert_eq!(dt_new_day.day(), 30);
    /// assert_eq!(dt_new_day.month(), 7); // Month unchanged
    /// assert_eq!(dt_new_day.year(), 1403); // Year unchanged
    /// assert_eq!(dt_new_day.time(), (12, 0, 0)); // Time unchanged
    ///
    /// // Try setting an invalid day (31 in Mehr)
    /// assert!(matches!(dt.with_day(31), Err(DateError::InvalidDate)));
    ///
    /// // Try setting day 0
    /// assert!(matches!(dt.with_day(0), Err(DateError::InvalidDate)));
    ///
    /// // Example with leap year (Esfand)
    /// let dt_leap = ParsiDateTime::new(1403, 12, 1, 11, 0, 0).unwrap(); // 1403 is leap
    /// assert!(dt_leap.with_day(30).is_ok()); // Day 30 is valid in Esfand 1403
    ///
    /// let dt_non_leap = ParsiDateTime::new(1404, 12, 1, 11, 0, 0).unwrap(); // 1404 not leap
    /// assert!(matches!(dt_non_leap.with_day(30), Err(DateError::InvalidDate))); // Day 30 invalid in Esfand 1404
    /// ```
    pub fn with_day(&self, day: u32) -> Result<Self, DateError> {
        // Delegate the date modification and validation to ParsiDate
        let new_date = self.date.with_day(day)?;
        // Recombine the validated new date with the original time
        Ok(ParsiDateTime {
            date: new_date,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
        })
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
