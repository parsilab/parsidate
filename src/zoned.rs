// ~/src/zoned.rs
//
//  * Copyright (C) ParsiCore (parsidate) 2024-2025 <parsicore.dev@gmail.com>
//  * Package : parsidate
//  * License : Apache-2.0
//  * Version : 1.7.1
//  * URL     : https://github.com/parsicore/parsidate
//  * Sign: parsidate-20250607-fea13e856dcd-459c6e73c83e49e10162ee28b26ac7cd
//
//! Defines a timezone-aware Jalali (Parsi) date and time.
//!
//! This module is available when the **`timezone`** feature is enabled.
//! It provides the [`ZonedParsiDateTime`] struct, which is essential for representing
//! an exact moment in time in the Persian calendar, associated with a specific timezone.
//!
//! # Overview
//!
//! In programming, it's crucial to distinguish between:
//! 1.  **Naive Time**: A "wall-clock" time without timezone information (e.g., "14:30").
//!     This is ambiguous; "14:30" can mean different things in Tehran, London, or New York.
//!     [`ParsiDateTime`](crate::ParsiDateTime) represents this.
//! 2.  **Aware Time**: An exact, unambiguous instant in time (e.g., "December 31st, 1403 at
//!     14:30 in Asia/Tehran"). This corresponds to a single point on the global timeline.
//!
//! [`ZonedParsiDateTime`] represents this aware time. It is a robust wrapper around
//! `chrono::DateTime<Tz>` and is the recommended type for any application that
//! needs to handle dates and times across different regions, store timestamps, or perform
//! timezone-sensitive calculations. It correctly handles complex scenarios like
//! Daylight Saving Time (DST).
//!
//! # Usage
//!
//! To use `ZonedParsiDateTime`, you need a `TimeZone` provider. The most common choice
//! in the Rust ecosystem is the `chrono-tz` crate.
//!
//! First, add `parsidate` with the `timezone` feature and `chrono-tz` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! parsidate = { version = "1.7.1", features = ["timezone"] }
//! chrono-tz = "0.8"
//! ```
//!
//! Then, you can create instances of `ZonedParsiDateTime`:
//!
//! ```
//! # #[cfg(feature = "timezone")] {
//! use parsidate::ZonedParsiDateTime;
//! use chrono_tz::Asia::Tehran;
//! use chrono_tz::America::New_York;
//!
//! // Create a specific moment in time in the Tehran timezone.
//! let pdt_tehran = ZonedParsiDateTime::new(1403, 9, 21, 12, 0, 0, Tehran).unwrap();
//!
//! // Convert this exact moment to the New York timezone.
//! let pdt_new_york = pdt_tehran.with_timezone(&New_York);
//!
//! println!("Tehran Time: {}", pdt_tehran);      // Outputs: 1403/09/21 12:00:00 +0330
//! println!("New York Time: {}", pdt_new_york);  // Outputs: 1403/09/21 03:30:00 -0500
//!
//! // Despite having different local times, they represent the same instant.
//! assert_eq!(pdt_tehran, pdt_new_york);
//! # }
//! ```

use crate::{DateError, ParsiDate, ParsiDateTime};
use chrono::{DateTime, Duration, TimeZone};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub};

/// Represents a timezone-aware date and time in the Persian (Jalali) calendar.
///
/// `ZonedParsiDateTime<Tz>` is a high-level wrapper around `chrono::DateTime<Tz>`,
/// providing a calendar-aware API for an unambiguous moment in time. It is the
/// timezone-aware counterpart to [`ParsiDateTime`].
///
/// This struct is the preferred way to handle time when correctness across different
/// geographical locations is required. It accounts for UTC offsets and Daylight
/// Saving Time (DST) rules automatically, ensuring that operations are accurate.
///
/// An instance of `ZonedParsiDateTime<Tz>` contains both the Jalali date and time
/// components and the specific `TimeZone` (`Tz`) it belongs to.
///
/// This struct is only available if the `timezone` feature is enabled. You will typically
/// use it with a `TimeZone` implementation from a crate like `chrono-tz`.
///
/// # Examples
///
/// Creating a new `ZonedParsiDateTime`:
///
/// ```
/// # #[cfg(feature = "timezone")] {
/// use parsidate::ZonedParsiDateTime;
/// use chrono_tz::Asia::Tehran;
///
/// // Create a ZonedParsiDateTime for November 5, 1403, at 2:30 PM in Tehran.
/// let tehran_time = ZonedParsiDateTime::new(1403, 8, 15, 14, 30, 0, Tehran).unwrap();
///
/// assert_eq!(tehran_time.year(), 1403);
/// assert_eq!(tehran_time.month(), 8);
/// assert_eq!(tehran_time.day(), 15);
/// assert_eq!(tehran_time.hour(), 14);
/// assert_eq!(tehran_time.timezone(), Tehran);
/// # }
/// ```
#[derive(Clone)]
pub struct ZonedParsiDateTime<Tz: TimeZone> {
    /// The underlying `chrono::DateTime` object.
    /// This serves as the single source of truth for the absolute instant in time.
    /// All Jalali calendar calculations are derived from this value.
    inner: DateTime<Tz>,
}

// --- Core Implementation ---

impl<Tz: TimeZone> ZonedParsiDateTime<Tz> {
    /// Creates a new `ZonedParsiDateTime` from an existing `chrono::DateTime`.
    ///
    /// This is an internal constructor used by other methods within the library.
    #[inline]
    fn from_chrono_datetime(dt: DateTime<Tz>) -> Self {
        Self { inner: dt }
    }

    /// Returns the current date and time in the specified timezone.
    ///
    /// This function retrieves the current system time (as a UTC timestamp) and
    /// converts it to the requested timezone `tz`.
    ///
    /// # Note
    /// This relies on the system's clock. It can panic if the system time is earlier
    /// than the Unix epoch (1970-01-01 00:00:00 UTC).
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "timezone")] {
    /// # use parsidate::ZonedParsiDateTime;
    /// # use chrono_tz::Asia::Tehran;
    /// # use chrono_tz::America::New_York;
    /// # use chrono::Duration;
    ///
    /// // Get the current time in two different timezones.
    /// let now_in_tehran = ZonedParsiDateTime::now(Tehran);
    /// println!("Current time in Tehran: {}", now_in_tehran);
    ///
    /// let now_in_new_york = ZonedParsiDateTime::now(New_York);
    /// println!("Current time in New York: {}", now_in_new_york);
    ///
    /// // Although their wall-clock times are different, they represent the same
    /// // absolute instant. Their difference should be zero.
    /// assert!((now_in_tehran - now_in_new_york).num_seconds() == 0);
    /// # }
    /// ```
    #[must_use]
    pub fn now(tz: Tz) -> Self {
        // Get the current UTC time from the system.
        let utc_now = chrono::Utc::now();
        // Convert the UTC time to the specified timezone.
        Self {
            inner: utc_now.with_timezone(&tz),
        }
    }

    /// Creates a `ZonedParsiDateTime` from Jalali date and time components in a given timezone.
    ///
    /// This is the primary constructor for creating a specific, timezone-aware datetime.
    /// It performs a full validation of the date and time components and correctly handles
    /// Daylight Saving Time (DST) transitions, where a local time might be ambiguous or
    /// non-existent.
    ///
    /// The process is as follows:
    /// 1. The Jalali date/time is converted to a naive Gregorian `DateTime`.
    /// 2. The `TimeZone` provider attempts to resolve this naive local time.
    ///
    /// ## Timezone Resolution
    ///
    /// - **Unique Time**: If the local time exists and is unambiguous, a `ZonedParsiDateTime` is returned.
    /// - **Ambiguous Time**: During a "fall back" (e.g., when DST ends), a local time may occur twice.
    ///   This function resolves the ambiguity by choosing the *earlier* of the two possible instances.
    /// - **Non-existent Time**: During a "spring forward" (e.g., when DST begins), a gap in local time
    ///   is created. If the provided time falls within this gap, it is considered invalid.
    ///
    /// # Errors
    ///
    /// This function will return an `Err` in the following cases:
    /// - [`DateError::InvalidDate`]: If the year, month, or day do not form a valid Jalali date
    ///   (e.g., `1403-12-31` in a non-leap year).
    /// - [`DateError::InvalidTime`]: If the hour, minute, or second are out of their valid ranges.
    /// - [`DateError::InvalidTime`]: If the specified local time does not exist in the given
    ///   timezone due to a DST transition (a "spring forward" gap).
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "timezone")] {
    /// # use parsidate::{ZonedParsiDateTime, DateError};
    /// # use chrono_tz::Asia::Tehran;
    /// # use chrono_tz::America::New_York;
    ///
    /// // 1. A valid, unambiguous time in Tehran.
    /// let dt = ZonedParsiDateTime::new(1403, 10, 1, 12, 0, 0, Tehran);
    /// assert!(dt.is_ok());
    ///
    /// // 2. An invalid date component (1404 is not a leap year, so Esfand has 29 days).
    /// let invalid_date = ZonedParsiDateTime::new(1404, 12, 30, 10, 0, 0, Tehran);
    /// assert_eq!(invalid_date, Err(DateError::InvalidDate));
    ///
    /// // 3. A non-existent time in New York due to a DST spring-forward.
    /// // In 2024, clocks jumped from 1:59:59 AM to 3:00:00 AM on March 10th.
    /// // The Jalali date for this event is 1402/12/20.
    /// let non_existent_time = ZonedParsiDateTime::new(1402, 12, 20, 2, 30, 0, New_York);
    /// assert_eq!(non_existent_time, Err(DateError::InvalidTime));
    /// # }
    /// ```
    pub fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        tz: Tz,
    ) -> Result<Self, DateError> {
        // First, create a naive ParsiDateTime to validate components.
        let pdt = ParsiDateTime::new(year, month, day, hour, minute, second)?;
        // Convert the naive ParsiDateTime to its equivalent naive Gregorian DateTime.
        let naive_gregorian = pdt.to_gregorian()?;

        // Use chrono's TimeZone trait to resolve the local time. This correctly
        // handles DST ambiguities and non-existent times.
        match tz.from_local_datetime(&naive_gregorian) {
            // The local time is valid and unique.
            chrono::LocalResult::Single(dt) => Ok(Self::from_chrono_datetime(dt)),
            // The local time is ambiguous (occurs twice, e.g., DST end).
            // We follow chrono's convention and choose the earlier instance.
            chrono::LocalResult::Ambiguous(dt1, _dt2) => Ok(Self::from_chrono_datetime(dt1)),
            // The local time does not exist (e.g., during DST start).
            chrono::LocalResult::None => Err(DateError::InvalidTime),
        }
    }

    // --- Accessors ---

    /// Returns the naive [`ParsiDate`] (local date) component of this datetime.
    ///
    /// This represents the date on the "wall clock" in the object's timezone,
    /// without any timezone information attached.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "timezone")] {
    /// # use parsidate::{ParsiDate, ZonedParsiDateTime};
    /// # use chrono_tz::Asia::Tehran;
    /// let zdt = ZonedParsiDateTime::new(1403, 5, 2, 10, 0, 0, Tehran).unwrap();
    /// assert_eq!(zdt.date(), ParsiDate::new(1403, 5, 2).unwrap());
    /// # }
    /// ```
    #[must_use]
    pub fn date(&self) -> ParsiDate {
        self.datetime().date()
    }

    /// Returns the naive [`ParsiDateTime`] (local datetime) component of this datetime.
    ///
    /// This represents the full date and time on the "wall clock" in the object's
    /// timezone, stripped of its timezone context.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "timezone")] {
    /// # use parsidate::{ParsiDateTime, ZonedParsiDateTime};
    /// # use chrono_tz::Asia::Tehran;
    /// let zdt = ZonedParsiDateTime::new(1403, 5, 2, 10, 30, 45, Tehran).unwrap();
    /// let expected_pdt = ParsiDateTime::new(1403, 5, 2, 10, 30, 45).unwrap();
    /// assert_eq!(zdt.datetime(), expected_pdt);
    /// # }
    /// ```
    #[must_use]
    pub fn datetime(&self) -> ParsiDateTime {
        // Convert the inner chrono::DateTime's naive local time to a ParsiDateTime.
        // This unwrap is safe because a valid ZonedParsiDateTime always corresponds
        // to a valid Gregorian date, which in turn can be converted to Parsi.
        ParsiDateTime::from_gregorian(self.inner.naive_local()).unwrap()
    }

    /// Returns the year component of the local Persian date.
    #[must_use]
    #[inline]
    pub fn year(&self) -> i32 {
        self.datetime().year()
    }

    /// Returns the month component of the local Persian date (1-12).
    #[must_use]
    #[inline]
    pub fn month(&self) -> u32 {
        self.datetime().month()
    }

    /// Returns the day of the month component of the local Persian date (1-31).
    #[must_use]
    #[inline]
    pub fn day(&self) -> u32 {
        self.datetime().day()
    }

    /// Returns the hour component of the local time (0-23).
    #[must_use]
    #[inline]
    pub fn hour(&self) -> u32 {
        self.datetime().hour()
    }

    /// Returns the minute component of the local time (0-59).
    #[must_use]
    #[inline]
    pub fn minute(&self) -> u32 {
        self.datetime().minute()
    }

    /// Returns the second component of the local time (0-59).
    #[must_use]
    #[inline]
    pub fn second(&self) -> u32 {
        self.datetime().second()
    }

    /// Returns a copy of the timezone associated with this `ZonedParsiDateTime`.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "timezone")] {
    /// # use parsidate::ZonedParsiDateTime;
    /// # use chrono_tz::Asia::Tehran;
    /// let zdt = ZonedParsiDateTime::now(Tehran);
    /// assert_eq!(zdt.timezone(), Tehran);
    /// # }
    /// ```
    #[must_use]
    pub fn timezone(&self) -> Tz {
        self.inner.timezone()
    }

    /// Returns the UTC offset for this specific date and time.
    ///
    /// The offset represents the duration between the local time and UTC.
    /// Its value can vary for the same timezone depending on the date due to rules
    /// like Daylight Saving Time.
    ///
    /// # Example
    ///
    /// New York's UTC offset changes between summer and winter.
    /// ```
    /// # #[cfg(feature = "timezone")] {
    /// # use parsidate::ZonedParsiDateTime;
    /// # use chrono_tz::America::New_York;
    /// # use chrono::Offset; // Trait needed for `fix()`
    ///
    /// // In summer (DST), New York is UTC-4. Jalali month 4 is Tir.
    /// let summer_time = ZonedParsiDateTime::new(1403, 4, 1, 10, 0, 0, New_York).unwrap();
    /// assert_eq!(summer_time.offset().fix().local_minus_utc(), -4 * 3600); // -14400 seconds
    ///
    /// // In winter (standard time), New York is UTC-5. Jalali month 10 is Dey.
    /// let winter_time = ZonedParsiDateTime::new(1403, 10, 1, 10, 0, 0, New_York).unwrap();
    /// assert_eq!(winter_time.offset().fix().local_minus_utc(), -5 * 3600); // -18000 seconds
    /// # }
    /// ```
    #[must_use]
    pub fn offset(&self) -> Tz::Offset {
        self.inner.offset().clone()
    }

    /// Changes the timezone of this datetime.
    ///
    /// This method converts the datetime to a different timezone while preserving the
    /// absolute instant in time. The local ("wall-clock") date and time components
    /// will be adjusted to reflect the new timezone.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "timezone")] {
    /// # use parsidate::{ParsiDate, ZonedParsiDateTime};
    /// # use chrono_tz::Asia::Tehran;
    /// # use chrono_tz::Europe::London;
    ///
    /// // An event at 2:00 AM in Tehran on Dey 10th.
    /// let dt_tehran = ZonedParsiDateTime::new(1402, 10, 10, 2, 0, 0, Tehran).unwrap();
    ///
    /// // Find out what time it was in London at that same moment.
    /// let dt_london = dt_tehran.with_timezone(&London);
    ///
    /// // In winter, Tehran (UTC+3:30) is 3.5 hours ahead of London (UTC+0).
    /// // So, 2:00 AM in Tehran is 10:30 PM the *previous day* in London.
    /// assert_eq!(dt_london.date(), ParsiDate::new(1402, 10, 9).unwrap());
    /// assert_eq!(dt_london.hour(), 22);
    /// assert_eq!(dt_london.minute(), 30);
    /// # }
    /// ```
    #[must_use]
    pub fn with_timezone<NewTz: TimeZone>(&self, new_tz: &NewTz) -> ZonedParsiDateTime<NewTz> {
        ZonedParsiDateTime {
            inner: self.inner.with_timezone(new_tz),
        }
    }

    // --- Arithmetic ---

    /// Adds a `chrono::Duration` to this datetime, returning a new instance.
    ///
    /// This is a convenience method. For more idiomatic code, use the `+` operator.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "timezone")] {
    /// # use parsidate::ZonedParsiDateTime;
    /// # use chrono::Duration;
    /// # use chrono_tz::Asia::Tehran;
    ///
    /// let dt = ZonedParsiDateTime::new(1403, 1, 1, 23, 0, 0, Tehran).unwrap();
    /// let later = dt.add_duration(Duration::hours(2)); // Same as `dt + Duration::hours(2)`
    ///
    /// // Adding 2 hours crosses into the next day.
    /// assert_eq!(later.date(), dt.date().with_day(2).unwrap());
    /// assert_eq!(later.hour(), 1);
    /// # }
    /// ```
    #[must_use]
    pub fn add_duration(&self, duration: Duration) -> Self {
        Self {
            inner: self.inner.clone() + duration,
        }
    }

    /// Subtracts a `chrono::Duration` from this datetime, returning a new instance.
    ///
    /// This is a convenience method. For more idiomatic code, use the `-` operator.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "timezone")] {
    /// # use parsidate::ZonedParsiDateTime;
    /// # use chrono::Duration;
    /// # use chrono_tz::Asia::Tehran;
    ///
    /// let dt = ZonedParsiDateTime::new(1403, 1, 1, 1, 0, 0, Tehran).unwrap();
    /// let earlier = dt.sub_duration(Duration::hours(2)); // Same as `dt - Duration::hours(2)`
    ///
    /// // Subtracting 2 hours crosses into the previous day.
    /// assert_eq!(earlier.date(), dt.date().with_year(1402).unwrap().with_month(12).unwrap().with_day(29).unwrap());
    /// assert_eq!(earlier.hour(), 23);
    /// # }
    /// ```
    #[must_use]
    pub fn sub_duration(&self, duration: Duration) -> Self {
        Self {
            inner: self.inner.clone() - duration,
        }
    }
}

// --- Trait Implementations ---

/// Compares two `ZonedParsiDateTime` instances for equality.
///
/// Two instances are considered equal if they represent the exact same moment in
/// universal time, regardless of their timezone.
///
/// For example, `1403-01-01 12:00:00` in `Asia/Tehran` is **not equal** to
/// `1403-01-01 12:00:00` in `Europe/London`, but it **is equal** to
/// `1403-01-01 08:30:00` in `Europe/London`.
impl<Tz: TimeZone> PartialEq for ZonedParsiDateTime<Tz> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

/// Implements the `Eq` trait, allowing `ZonedParsiDateTime` to be used in
/// collections that require total equality, like `HashMap`.
impl<Tz: TimeZone> Eq for ZonedParsiDateTime<Tz> where DateTime<Tz>: Eq {}

/// Provides partial ordering for `ZonedParsiDateTime`.
///
/// The comparison is performed on the absolute instant in time, not on the
/// local "wall-clock" time.
impl<Tz: TimeZone> PartialOrd for ZonedParsiDateTime<Tz> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Provides total ordering for `ZonedParsiDateTime`.
///
/// Like `PartialEq`, the comparison is based on the absolute instant in time.
impl<Tz: TimeZone> Ord for ZonedParsiDateTime<Tz>
where
    DateTime<Tz>: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

/// Adds a `chrono::Duration` to a `ZonedParsiDateTime` using the `+` operator.
///
/// This operation performs exact time arithmetic, correctly handling DST and
/// timezone transitions.
impl<Tz: TimeZone> Add<Duration> for ZonedParsiDateTime<Tz> {
    type Output = Self;
    fn add(self, rhs: Duration) -> Self::Output {
        // This moves `self`, which is the standard convention for operator implementations.
        Self {
            inner: self.inner + rhs,
        }
    }
}

/// Subtracts a `chrono::Duration` from a `ZonedParsiDateTime` using the `-` operator.
///
/// This operation performs exact time arithmetic.
impl<Tz: TimeZone> Sub<Duration> for ZonedParsiDateTime<Tz> {
    type Output = Self;
    fn sub(self, rhs: Duration) -> Self::Output {
        // This moves `self`.
        Self {
            inner: self.inner - rhs,
        }
    }
}

/// Calculates the `chrono::Duration` between two `ZonedParsiDateTime` instances.
///
/// This operation returns the exact duration of time that has elapsed between
/// two moments, regardless of their timezones.
impl<Tz: TimeZone, OtherTz: TimeZone> Sub<ZonedParsiDateTime<OtherTz>> for ZonedParsiDateTime<Tz> {
    type Output = Duration;
    fn sub(self, rhs: ZonedParsiDateTime<OtherTz>) -> Self::Output {
        self.inner.signed_duration_since(rhs.inner)
    }
}

/// Formats the `ZonedParsiDateTime` for display.
///
/// The default format is `YYYY/MM/DD HH:MM:SS OFFSET`, which combines the local Parsi
/// datetime with its UTC offset.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "timezone")] {
/// # use parsidate::ZonedParsiDateTime;
/// # use chrono_tz::Asia::Tehran;
/// let dt = ZonedParsiDateTime::new(1403, 8, 15, 14, 30, 0, Tehran).unwrap();
/// assert_eq!(dt.to_string(), "1403/08/15 14:30:00 +0330");
/// # }
/// ```
impl<Tz: TimeZone> fmt::Display for ZonedParsiDateTime<Tz>
where
    Tz::Offset: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pdt = self.datetime();
        let offset = self.inner.offset();
        write!(f, "{} {}", pdt, offset)
    }
}

/// Formats the `ZonedParsiDateTime` for debugging.
///
/// The debug format provides a more detailed, developer-friendly representation
/// of the struct's contents, including the local Parsi datetime and the timezone name.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "timezone")] {
/// # use parsidate::ZonedParsiDateTime;
/// # use chrono_tz::Asia::Tehran;
/// let dt = ZonedParsiDateTime::new(1403, 8, 15, 9, 5, 0, Tehran).unwrap();
/// let debug_str = format!("{:?}", dt);
/// assert!(debug_str.contains("datetime: ParsiDateTime(1403/08/15 09:05:00)"));
/// assert!(debug_str.contains("timezone: Asia/Tehran"));
/// # }
/// ```
impl<Tz: TimeZone> fmt::Debug for ZonedParsiDateTime<Tz>
where
    // The timezone type itself must be printable for debugging.
    Tz: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ZonedParsiDateTime")
            .field("datetime", &self.datetime())
            .field("timezone", &self.inner.timezone())
            .finish()
    }
}
