// ~/src/zoned.rs

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
/// This struct is timezone-aware and handles complexities like Daylight Saving Time (DST)
/// by wrapping a `chrono::DateTime<Tz>`. It unambiguously represents a single, absolute
/// moment in time and is the recommended type for such use cases.
///
/// It is only available when the `timezone` feature is enabled.
// We remove `Copy` and `Eq` from derive, as they are not universally available.
#[derive(Clone)]
pub struct ZonedParsiDateTime<Tz: TimeZone> {
    /// The underlying chrono `DateTime` object, which serves as the source of truth.
    inner: DateTime<Tz>,
}

// --- Core Implementation ---

impl<Tz: TimeZone> ZonedParsiDateTime<Tz> {
    // ... (new, now methods remain the same) ...

    /// Creates a new `ZonedParsiDateTime` from a `chrono::DateTime`.
    /// This is the primary internal constructor.
    fn from_chrono_datetime(dt: DateTime<Tz>) -> Self {
        Self { inner: dt }
    }

    /// Returns the current date and time in the specified timezone.
    #[must_use]
    pub fn now(tz: Tz) -> Self {
        let utc_now = chrono::Utc::now();
        Self {
            inner: utc_now.with_timezone(&tz),
        }
    }

    /// Creates a `ZonedParsiDateTime` from Persian date and time components in a specific timezone.
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

    // ... (date, datetime, year, month, etc. methods remain the same) ...

    /// Returns the naive `ParsiDate` component of this zoned datetime.
    #[must_use]
    pub fn date(&self) -> ParsiDate {
        self.datetime().date()
    }

    /// Returns the naive `ParsiDateTime` (local time) component of this zoned datetime.
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

    /// Returns the hour component of the time (0-23).
    #[must_use]
    pub fn hour(&self) -> u32 {
        self.datetime().hour()
    }

    /// Returns the minute component of the time (0-59).
    #[must_use]
    pub fn minute(&self) -> u32 {
        self.datetime().minute()
    }

    /// Returns the second component of the time (0-59).
    #[must_use]
    pub fn second(&self) -> u32 {
        self.datetime().second()
    }

    /// Returns the timezone of this `ZonedParsiDateTime`.
    #[must_use]
    pub fn timezone(&self) -> Tz {
        self.inner.timezone()
    }

    /// Returns the UTC offset of this `ZonedParsiDateTime`.
    #[must_use]
    pub fn offset(&self) -> Tz::Offset {
        self.inner.offset().clone()
    }

    /// Changes the timezone of this `ZonedParsiDateTime`.
    #[must_use]
    pub fn with_timezone<NewTz: TimeZone>(&self, new_tz: &NewTz) -> ZonedParsiDateTime<NewTz> {
        ZonedParsiDateTime {
            inner: self.inner.with_timezone(new_tz),
        }
    }

    // --- Arithmetic ---

    /// Adds a `chrono::Duration` to this `ZonedParsiDateTime`, returning a new instance.
    #[must_use]
    pub fn add_duration(&self, duration: Duration) -> Self {
        Self {
            // Re-add .clone() because we are no longer using Copy
            inner: self.inner.clone() + duration,
        }
    }

    /// Subtracts a `chrono::Duration` from this `ZonedParsiDateTime`, returning a new instance.
    #[must_use]
    pub fn sub_duration(&self, duration: Duration) -> Self {
        Self {
            // Re-add .clone() because we are no longer using Copy
            inner: self.inner.clone() - duration,
        }
    }
}

// --- Trait Implementations ---

// PartialEq is always available for `DateTime<Tz>`.
impl<Tz: TimeZone> PartialEq for ZonedParsiDateTime<Tz> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

// Implement Eq manually for clarity. It follows from PartialEq.
impl<Tz: TimeZone> Eq for ZonedParsiDateTime<Tz> where DateTime<Tz>: Eq {}

// PartialOrd is always available.
impl<Tz: TimeZone> PartialOrd for ZonedParsiDateTime<Tz> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

// Ord requires Eq, so we add the same bound.
impl<Tz: TimeZone> Ord for ZonedParsiDateTime<Tz>
where
    DateTime<Tz>: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

// For arithmetic, we now operate on `self` (an owned value), not `&self`.
// The caller's value will be moved, which is standard for operator traits.
impl<Tz: TimeZone> Add<Duration> for ZonedParsiDateTime<Tz> {
    type Output = Self;
    fn add(self, rhs: Duration) -> Self::Output {
        Self {
            inner: self.inner + rhs,
        }
    }
}

impl<Tz: TimeZone> Sub<Duration> for ZonedParsiDateTime<Tz> {
    type Output = Self;
    fn sub(self, rhs: Duration) -> Self::Output {
        Self {
            inner: self.inner - rhs,
        }
    }
}

impl<Tz: TimeZone, OtherTz: TimeZone> Sub<ZonedParsiDateTime<OtherTz>> for ZonedParsiDateTime<Tz> {
    type Output = Duration;
    fn sub(self, rhs: ZonedParsiDateTime<OtherTz>) -> Self::Output {
        self.inner.signed_duration_since(rhs.inner)
    }
}

// Display and Debug implementations remain mostly the same.
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
