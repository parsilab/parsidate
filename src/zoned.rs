// ~/src/zoned.rs
//
//  * Copyright (C) Mohammad (Sina) Jalalvandi 2024-2025 <jalalvandi.sina@gmail.com>
//  * Package : parsidate
//  * License : Apache-2.0
//  * Version : 1.7.0
//  * URL     : https://github.com/jalalvandi/parsidate
//  * Sign: parsidate-20250607-fea13e856dcd-459c6e73c83e49e10162ee28b26ac7cd
//
//! Defines a timezone-aware Parsi date and time object.
//!
//! This module provides the `ZonedParsiDateTime` struct, which represents a specific
//! moment in time within a given timezone. It is a feature-gated module,
//! available only when the `timezone` feature is enabled.

use crate::{DateError, ParsiDate, ParsiDateTime};
use chrono::{DateTime, Duration, TimeZone};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub};

/// Represents a specific date and time in the Persian (Jalali) calendar
/// within a specific timezone.
///
/// This struct is the timezone-aware counterpart to [`ParsiDateTime`]. It handles
/// complexities like Daylight Saving Time (DST) and UTC offsets by wrapping a
/// `chrono::DateTime<Tz>`. It unambiguously represents a single, absolute moment
/// in time and is the recommended type for all timezone-sensitive operations.
///
/// This struct is only available when the `timezone` feature is enabled.
/// To use it, you'll also need a `chrono::TimeZone` implementation, typically
/// from the `chrono-tz` crate.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "timezone")] {
/// use parsidate::ZonedParsiDateTime;
/// use chrono_tz::Asia::Tehran;
///
/// let tehran_time = ZonedParsiDateTime::new(1403, 8, 15, 14, 30, 0, Tehran).unwrap();
/// assert_eq!(tehran_time.hour(), 14);
/// assert_eq!(tehran_time.timezone(), Tehran);
/// # }
/// ```
#[derive(Clone)]
pub struct ZonedParsiDateTime<Tz: TimeZone> {
    /// The underlying `chrono::DateTime` object, which serves as the source of truth
    /// for all time-related calculations.
    inner: DateTime<Tz>,
}

// --- Core Implementation ---

impl<Tz: TimeZone> ZonedParsiDateTime<Tz> {
    /// Creates a new `ZonedParsiDateTime` from an existing `chrono::DateTime`.
    ///
    /// This is the primary internal constructor and is not intended for public use.
    fn from_chrono_datetime(dt: DateTime<Tz>) -> Self {
        Self { inner: dt }
    }

    /// Returns the current date and time in the specified timezone.
    ///
    /// This method gets the current system time (as a UTC timestamp) and converts
    /// it to the desired timezone.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "timezone")] {
    /// # use parsidate::ZonedParsiDateTime;
    /// # use chrono_tz::Asia::Tehran;
    /// # use chrono_tz::America::New_York;
    /// let now_in_tehran = ZonedParsiDateTime::now(Tehran);
    /// println!("Current time in Tehran: {}", now_in_tehran);
    ///
    /// let now_in_new_york = ZonedParsiDateTime::now(New_York);
    /// println!("Current time in New York: {}", now_in_new_york);
    ///
    /// // Both represent the same instant in time.
    /// assert_eq!(now_in_tehran.clone() - now_in_new_york.clone(), Duration::zero());
    /// # }
    /// ```
    #[must_use]
    pub fn now(tz: Tz) -> Self {
        let utc_now = chrono::Utc::now();
        Self {
            inner: utc_now.with_timezone(&tz),
        }
    }

    /// Creates a `ZonedParsiDateTime` from Persian date and time components in a specific timezone.
    ///
    /// This method performs full validation of the date and time components and correctly
    /// handles ambiguities and non-existent times that can occur during Daylight Saving Time (DST)
    /// transitions.
    ///
    /// - If the local time is **ambiguous** (e.g., during a "fall back" when clocks are set back),
    ///   the earlier of the two possible instances is chosen by default.
    /// - If the local time is **non-existent** (e.g., during a "spring forward" when clocks jump),
    ///   an `Err(DateError::InvalidTime)` is returned.
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// - The components do not form a valid Parsi date (`DateError::InvalidDate`).
    /// - The components do not form a valid time (`DateError::InvalidTime`).
    /// - The resulting local time does not exist in the specified timezone (`DateError::InvalidTime`).
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "timezone")] {
    /// # use parsidate::{ZonedParsiDateTime, DateError};
    /// # use chrono_tz::Asia::Tehran;
    /// # use chrono_tz::America::New_York;
    ///
    /// // A valid time in Tehran
    /// let dt = ZonedParsiDateTime::new(1403, 10, 1, 12, 0, 0, Tehran);
    /// assert!(dt.is_ok());
    ///
    /// // An invalid date component
    /// let invalid_date = ZonedParsiDateTime::new(1404, 12, 30, 10, 0, 0, Tehran);
    /// assert_eq!(invalid_date, Err(DateError::InvalidDate));
    ///
    /// // A non-existent time during a DST spring-forward in New York
    /// // On 2024-03-10 (1402-12-20), 2:30 AM did not exist.
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
        let pdt = ParsiDateTime::new(year, month, day, hour, minute, second)?;
        let naive_gregorian = pdt.to_gregorian()?;
        match tz.from_local_datetime(&naive_gregorian) {
            chrono::LocalResult::Single(dt) => Ok(Self::from_chrono_datetime(dt)),
            chrono::LocalResult::Ambiguous(dt1, _dt2) => Ok(Self::from_chrono_datetime(dt1)),
            chrono::LocalResult::None => Err(DateError::InvalidTime),
        }
    }

    // --- Accessors ---

    /// Returns the naive [`ParsiDate`] component of this zoned datetime.
    ///
    /// This represents the local date in the object's timezone.
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

    /// Returns the naive [`ParsiDateTime`] (local time) component of this zoned datetime.
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
        ParsiDateTime::from_gregorian(self.inner.naive_local()).unwrap()
    }

    /// Returns the year component of the Persian date.
    #[must_use]
    pub fn year(&self) -> i32 {
        self.date().year()
    }

    /// Returns the month component of the Persian date (1-12).
    #[must_use]
    pub fn month(&self) -> u32 {
        self.date().month()
    }

    /// Returns the day component of the Persian date (1-31).
    #[must_use]
    pub fn day(&self) -> u32 {
        self.date().day()
    }

    /// Returns the hour component of the local time (0-23).
    #[must_use]
    pub fn hour(&self) -> u32 {
        self.datetime().hour()
    }

    /// Returns the minute component of the local time (0-59).
    #[must_use]
    pub fn minute(&self) -> u32 {
        self.datetime().minute()
    }

    /// Returns the second component of the local time (0-59).
    #[must_use]
    pub fn second(&self) -> u32 {
        self.datetime().second()
    }

    /// Returns the timezone associated with this `ZonedParsiDateTime`.
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
    /// The offset can vary for the same timezone depending on the date
    /// (e.g., due to Daylight Saving Time).
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "timezone")] {
    /// # use parsidate::ZonedParsiDateTime;
    /// # use chrono_tz::America::New_York;
    /// # use chrono::Offset; // Trait needed for .fix()
    /// // In summer (DST), New York is UTC-4.
    /// let summer_time = ZonedParsiDateTime::new(1403, 4, 1, 10, 0, 0, New_York).unwrap();
    /// assert_eq!(summer_time.offset().fix().local_minus_utc(), -4 * 3600);
    ///
    /// // In winter (standard time), New York is UTC-5.
    /// let winter_time = ZonedParsiDateTime::new(1403, 10, 1, 10, 0, 0, New_York).unwrap();
    /// assert_eq!(winter_time.offset().fix().local_minus_utc(), -5 * 3600);
    /// # }
    /// ```
    #[must_use]
    pub fn offset(&self) -> Tz::Offset {
        self.inner.offset().clone()
    }

    /// Changes the timezone of this `ZonedParsiDateTime`.
    ///
    /// This operation preserves the absolute instant in time but may change the
    /// local date and time components to reflect the new timezone.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "timezone")] {
    /// # use parsidate::{ParsiDate, ZonedParsiDateTime};
    /// # use chrono_tz::Asia::Tehran;
    /// # use chrono_tz::Europe::London;
    /// // 2:00 AM in Tehran on Dey 10th.
    /// let dt_tehran = ZonedParsiDateTime::new(1402, 10, 10, 2, 0, 0, Tehran).unwrap();
    ///
    /// // Convert to London time.
    /// let dt_london = dt_tehran.with_timezone(&London);
    ///
    /// // Tehran (UTC+3:30) is 3.5 hours ahead of London (UTC+0) in winter.
    /// // So, 2:00 AM in Tehran is 10:30 PM the *previous day* in London.
    /// assert_eq!(dt_london.hour(), 22);
    /// assert_eq!(dt_london.minute(), 30);
    /// assert_eq!(dt_london.date(), ParsiDate::new(1402, 10, 9).unwrap());
    /// # }
    /// ```
    #[must_use]
    pub fn with_timezone<NewTz: TimeZone>(&self, new_tz: &NewTz) -> ZonedParsiDateTime<NewTz> {
        ZonedParsiDateTime {
            inner: self.inner.with_timezone(new_tz),
        }
    }

    // --- Arithmetic ---

    /// Adds a `chrono::Duration` to this `ZonedParsiDateTime`, returning a new instance.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "timezone")] {
    /// # use parsidate::ZonedParsiDateTime;
    /// # use chrono::Duration;
    /// # use chrono_tz::Asia::Tehran;
    /// let dt = ZonedParsiDateTime::new(1403, 1, 1, 23, 0, 0, Tehran).unwrap();
    /// let two_hours_later = dt.add_duration(Duration::hours(2));
    ///
    /// assert_eq!(two_hours_later.date(), dt.date().add_days(1).unwrap());
    /// assert_eq!(two_hours_later.hour(), 1);
    /// # }
    /// ```
    #[must_use]
    pub fn add_duration(&self, duration: Duration) -> Self {
        Self {
            inner: self.inner.clone() + duration,
        }
    }

    /// Subtracts a `chrono::Duration` from this `ZonedParsiDateTime`, returning a new instance.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "timezone")] {
    /// # use parsidate::ZonedParsiDateTime;
    /// # use chrono::Duration;
    /// # use chrono_tz::Asia::Tehran;
    /// let dt = ZonedParsiDateTime::new(1403, 1, 1, 1, 0, 0, Tehran).unwrap();
    /// let two_hours_earlier = dt.sub_duration(Duration::hours(2));
    ///
    /// assert_eq!(two_hours_earlier.date(), dt.date().add_days(-1).unwrap());
    /// assert_eq!(two_hours_earlier.hour(), 23);
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
/// Two instances are equal if they represent the exact same moment in time,
/// regardless of their timezone.
impl<Tz: TimeZone> PartialEq for ZonedParsiDateTime<Tz> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

/// Implements the `Eq` trait, allowing for hashing and other equality-based operations.
impl<Tz: TimeZone> Eq for ZonedParsiDateTime<Tz> where DateTime<Tz>: Eq {}

/// Partially compares two `ZonedParsiDateTime` instances.
///
/// The comparison is based on the absolute instant in time.
impl<Tz: TimeZone> PartialOrd for ZonedParsiDateTime<Tz> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

/// Totally compares two `ZonedParsiDateTime` instances.
impl<Tz: TimeZone> Ord for ZonedParsiDateTime<Tz>
where
    DateTime<Tz>: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

/// Adds a `chrono::Duration` using the `+` operator.
impl<Tz: TimeZone> Add<Duration> for ZonedParsiDateTime<Tz> {
    type Output = Self;
    fn add(self, rhs: Duration) -> Self::Output {
        // This moves `self`, which is standard for operator implementations.
        Self {
            inner: self.inner + rhs,
        }
    }
}

/// Subtracts a `chrono::Duration` using the `-` operator.
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
impl<Tz: TimeZone, OtherTz: TimeZone> Sub<ZonedParsiDateTime<OtherTz>> for ZonedParsiDateTime<Tz> {
    type Output = Duration;
    fn sub(self, rhs: ZonedParsiDateTime<OtherTz>) -> Self::Output {
        self.inner.signed_duration_since(rhs.inner)
    }
}

/// Formats the `ZonedParsiDateTime` for display.
///
/// The default format is `YYYY/MM/DD HH:MM:SS OFFSET`, for example,
/// `1403/08/15 14:30:00 +0330`.
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
impl<Tz: TimeZone> fmt::Debug for ZonedParsiDateTime<Tz>
where
    Tz: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ZonedParsiDateTime")
            .field("datetime", &self.datetime())
            .field("timezone", &self.inner.timezone())
            .finish()
    }
}
