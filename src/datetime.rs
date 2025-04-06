//  * src/datetime.rs
//
//  * Copyright (C) Mohammad (Sina) Jalalvandi (parsidate) 2024-2025 <jalalvandi.sina@gmail.com>
//  * Version : 1.4.0 
//  * eb1f0cae-a178-41e5-b109-47f208e77913
//
//! Contains the `ParsiDateTime` struct definition and its implementation for handling Persian date and time.

use crate::constants::{MONTH_NAMES_PERSIAN, WEEKDAY_NAMES_PERSIAN}; // Reuse constants
use crate::date::ParsiDate;
use crate::error::{DateError, ParseErrorKind};
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use std::fmt;
use std::ops::{Add, Sub};

// --- Data Structures ---

/// Represents a specific date and time in the Persian (Jalali or Shamsi) calendar system.
///
/// Stores a `ParsiDate` along with hour, minute, and second components.
/// Provides methods for validation, conversion, formatting, parsing, and arithmetic operations
/// involving both date and time. Nanosecond precision is currently not stored but handled during conversions.
///
/// Note on Range: Supports the same date range as `ParsiDate` (Years 1-9999).
/// Time components must be valid (0-23 for hour, 0-59 for minute/second).
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParsiDateTime {
    /// The Persian date component.
    date: ParsiDate,
    /// The hour component (0-23).
    hour: u32,
    /// The minute component (0-59).
    minute: u32,
    /// The second component (0-59).
    second: u32,
    // Nanoseconds are not stored directly to keep the struct simple,
    // but can be handled during chrono conversions if needed.
}

// --- Core Implementation ---

impl ParsiDateTime {
    // --- Constructors and Converters ---

    /// Creates a new `ParsiDateTime` instance from individual date and time components.
    ///
    /// This function validates both the date and time components upon creation.
    /// Year, month, and day must form a valid `ParsiDate`.
    /// Hour must be between 0 and 23, minute and second between 0 and 59.
    ///
    /// # Arguments
    ///
    /// * `year`, `month`, `day`: Components for the `ParsiDate` part.
    /// * `hour`: Hour of the day (0-23).
    /// * `minute`: Minute of the hour (0-59).
    /// * `second`: Second of the minute (0-59).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the date components are invalid.
    /// Returns `Err(DateError::InvalidTime)` if the time components are invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, DateError};
    ///
    /// let dt = ParsiDateTime::new(1403, 5, 2, 15, 30, 45);
    /// assert!(dt.is_ok());
    /// assert_eq!(dt.unwrap().hour(), 15);
    ///
    /// assert_eq!(ParsiDateTime::new(1404, 12, 30, 10, 0, 0), Err(DateError::InvalidDate)); // Invalid day
    /// assert_eq!(ParsiDateTime::new(1403, 5, 2, 24, 0, 0), Err(DateError::InvalidTime)); // Invalid hour
    /// assert_eq!(ParsiDateTime::new(1403, 5, 2, 10, 60, 0), Err(DateError::InvalidTime)); // Invalid minute
    /// assert_eq!(ParsiDateTime::new(1403, 5, 2, 10, 0, 60), Err(DateError::InvalidTime)); // Invalid second
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

    /// Creates a `ParsiDateTime` from components without validation.
    ///
    /// **Warning:** This function is `unsafe`. It bypasses validation for both date and time.
    /// Using invalid components can lead to undefined behavior or panics. Only use when
    /// components are guaranteed to be valid by external means. Prefer `ParsiDateTime::new`.
    ///
    /// # Safety
    ///
    /// Caller must ensure date components form a valid `ParsiDate` and time components
    /// are within their valid ranges (H: 0-23, M/S: 0-59).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, ParsiDateTime};
    ///
    /// // Assume components are validated elsewhere
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// let hour = 10; let minute = 30; let second = 0;
    /// if hour <= 23 && minute <= 59 && second <= 59 {
    ///    let dt = unsafe { ParsiDateTime::new_unchecked(date.year(), date.month(), date.day(), hour, minute, second) };
    ///    assert_eq!(dt.hour(), 10);
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
            // Creates the inner ParsiDate unsafely as well
            date: ParsiDate::new_unchecked(year, month, day),
            hour,
            minute,
            second,
        }
    }

    /// Creates a `ParsiDateTime` from a valid `ParsiDate` and time components.
    ///
    /// Validates only the time components.
    ///
    /// # Arguments
    ///
    /// * `date`: A valid `ParsiDate` object.
    /// * `hour`: Hour of the day (0-23).
    /// * `minute`: Minute of the hour (0-59).
    /// * `second`: Second of the minute (0-59).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidTime)` if the time components are invalid.
    /// Note: It assumes the provided `date` is already valid. If an invalid `ParsiDate`
    /// is passed (e.g., via `new_unchecked`), the resulting `ParsiDateTime` might
    /// be invalid despite this function returning `Ok`. Use `is_valid()` afterwards if needed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, ParsiDateTime, DateError};
    ///
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// let dt = ParsiDateTime::from_date_and_time(date, 16, 0, 0);
    /// assert!(dt.is_ok());
    /// assert_eq!(dt.unwrap().date(), date);
    ///
    /// assert_eq!(ParsiDateTime::from_date_and_time(date, 25, 0, 0), Err(DateError::InvalidTime));
    /// ```
    pub fn from_date_and_time(
        date: ParsiDate,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Result<Self, DateError> {
        // Validate time components
        if hour > 23 || minute > 59 || second > 59 {
            return Err(DateError::InvalidTime);
        }
        // Assume date is valid as per function contract
        Ok(ParsiDateTime {
            date,
            hour,
            minute,
            second,
        })
    }

    /// Converts a Gregorian `NaiveDateTime` to its equivalent `ParsiDateTime`.
    ///
    /// # Arguments
    ///
    /// * `gregorian_dt`: The `chrono::NaiveDateTime` to convert.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::GregorianConversionError)` if the date part conversion fails
    /// (e.g., date is before Persian epoch or out of supported range).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    ///
    /// let g_dt = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap().and_hms_opt(15, 30, 45).unwrap();
    /// let pd_dt = ParsiDateTime::from_gregorian(g_dt);
    /// assert!(pd_dt.is_ok());
    /// let pd_dt = pd_dt.unwrap();
    /// assert_eq!(pd_dt.date(), ParsiDate::new(1403, 5, 2).unwrap());
    /// assert_eq!(pd_dt.hour(), 15);
    /// assert_eq!(pd_dt.minute(), 30);
    /// assert_eq!(pd_dt.second(), 45);
    /// ```
    pub fn from_gregorian(gregorian_dt: NaiveDateTime) -> Result<Self, DateError> {
        // Convert the date part
        let parsi_date = ParsiDate::from_gregorian(gregorian_dt.date())?;
        // Extract time components
        let hour = gregorian_dt.hour();
        let minute = gregorian_dt.minute();
        let second = gregorian_dt.second();
        // Nanoseconds are ignored for now

        // Combine using from_date_and_time (time validation is technically redundant here)
        Self::from_date_and_time(parsi_date, hour, minute, second)
        // Optimization: directly construct since time components from chrono are always valid
        // Ok(ParsiDateTime { date: parsi_date, hour, minute, second })
    }

    /// Converts this `ParsiDateTime` to its equivalent Gregorian `NaiveDateTime`.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the date part of `self` is invalid.
    /// Returns `Err(DateError::GregorianConversionError)` if the date conversion fails.
    /// Returns `Err(DateError::InvalidTime)` if the time part of `self` is invalid (though should be caught earlier).
    /// Note: `chrono::NaiveDate::and_hms_opt` used internally should not fail if time components are valid (0-23, 0-59, 0-59).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use parsidate::ParsiDateTime;
    ///
    /// let pd_dt = ParsiDateTime::new(1403, 5, 2, 15, 30, 45).unwrap();
    /// let expected_g_dt = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap().and_hms_opt(15, 30, 45).unwrap();
    /// assert_eq!(pd_dt.to_gregorian(), Ok(expected_g_dt));
    /// ```
    pub fn to_gregorian(&self) -> Result<NaiveDateTime, DateError> {
        // First, validate the entire ParsiDateTime object
        if !self.is_valid() {
            // Determine if the date or time part is invalid
            if !self.date.is_valid() {
                return Err(DateError::InvalidDate);
            } else {
                // Date is valid, so time must be invalid
                return Err(DateError::InvalidTime);
            }
        }

        // Convert the ParsiDate part to NaiveDate (use internal method as validation is done)
        let gregorian_date = self.date.to_gregorian_internal()?;

        // Combine with time components.
        // Since we validated time components (0-23, 0-59, 0-59), and_hms_opt should succeed.
        gregorian_date
            .and_hms_opt(self.hour, self.minute, self.second)
            .ok_or(DateError::GregorianConversionError) // Should not happen with valid H,M,S
    }

    /// Returns the current system date and time as a `ParsiDateTime`.
    ///
    /// Obtains the current local time and converts it.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::GregorianConversionError)` if the conversion from the current
    /// Gregorian date/time fails (e.g., system clock is wildly inaccurate).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// match ParsiDateTime::now() {
    ///     Ok(now) => println!("Current Persian date and time: {}", now),
    ///     Err(e) => eprintln!("Failed to get current Persian date/time: {}", e),
    /// }
    /// ```
    pub fn now() -> Result<Self, DateError> {
        let now_local = Local::now();
        let naive_local = now_local.naive_local(); // Get NaiveDateTime
        Self::from_gregorian(naive_local)
    }

    // --- Accessors ---

    /// Returns the `ParsiDate` component of this date-time.
    #[inline]
    pub const fn date(&self) -> ParsiDate {
        self.date
    }

    /// Returns the year component.
    #[inline]
    pub const fn year(&self) -> i32 {
        self.date.year()
    }

    /// Returns the month component (1-12).
    #[inline]
    pub const fn month(&self) -> u32 {
        self.date.month()
    }

    /// Returns the day component (1-31).
    #[inline]
    pub const fn day(&self) -> u32 {
        self.date.day()
    }

    /// Returns the hour component (0-23).
    #[inline]
    pub const fn hour(&self) -> u32 {
        self.hour
    }

    /// Returns the minute component (0-59).
    #[inline]
    pub const fn minute(&self) -> u32 {
        self.minute
    }

    /// Returns the second component (0-59).
    #[inline]
    pub const fn second(&self) -> u32 {
        self.second
    }

    /// Returns the time as a tuple `(hour, minute, second)`.
    #[inline]
    pub const fn time(&self) -> (u32, u32, u32) {
        (self.hour, self.minute, self.second)
    }

    // --- Validation ---

    /// Checks if the current `ParsiDateTime` instance represents a valid date and time.
    ///
    /// Validates both the `ParsiDate` part and the time components (H: 0-23, M/S: 0-59).
    ///
    /// # Returns
    ///
    /// `true` if the date and time are valid, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// assert!(ParsiDateTime::new(1403, 5, 2, 23, 59, 59).unwrap().is_valid());
    ///
    /// let invalid_time = unsafe { ParsiDateTime::new_unchecked(1403, 5, 2, 24, 0, 0) };
    /// assert!(!invalid_time.is_valid());
    ///
    /// let invalid_date = unsafe { ParsiDateTime::new_unchecked(1404, 12, 30, 10, 0, 0) };
    /// assert!(!invalid_date.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.date.is_valid() && self.hour <= 23 && self.minute <= 59 && self.second <= 59
    }

    // --- Formatting ---

    /// Formats the `ParsiDateTime` into a string based on a format pattern.
    ///
    /// This extends the `ParsiDate::format_strftime` logic with time specifiers.
    ///
    /// # Supported Format Specifiers (in addition to `ParsiDate` specifiers):
    ///
    /// *   `%H`: Hour (24-hour clock), zero-padded (00-23).
    /// *   `%M`: Minute, zero-padded (00-59).
    /// *   `%S`: Second, zero-padded (00-59).
    /// *   `%T`: Equivalent to `%H:%M:%S`.
    /// *   (Potentially add `%I` for 12-hour clock, `%p` for AM/PM later if needed)
    ///
    /// See [`ParsiDate::format_strftime`](../date/struct.ParsiDate.html#method.format_strftime) for date specifiers.
    ///
    /// # Arguments
    ///
    /// * `pattern`: The format string containing literal characters and format specifiers.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted date and time. Invalid components might result
    /// in error indicators (e.g., "??" for invalid time parts).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDateTime;
    ///
    /// let dt = ParsiDateTime::new(1403, 5, 2, 8, 5, 30).unwrap();
    /// assert_eq!(dt.format("%Y/%m/%d %H:%M:%S"), "1403/05/02 08:05:30");
    /// assert_eq!(dt.format("%d %B %Y ساعت %H:%M"), "02 مرداد 1403 ساعت 08:05");
    /// assert_eq!(dt.format("%Y-%m-%dT%T"), "1403-05-02T08:05:30"); // ISO-like format
    /// assert_eq!(dt.format("%A، %d %B %Y - %T"), "سه‌شنبه، 02 مرداد 1403 - 08:05:30");
    /// ```
    pub fn format(&self, pattern: &str) -> String {
        // Preallocate string with a reasonable estimate capacity.
        let mut result = String::with_capacity(pattern.len() + 20);
        // Use iterator over characters for Unicode safety.
        let mut chars = pattern.chars().peekable();

        // Caching results from date part if needed multiple times
        let mut weekday_cache: Option<Result<String, DateError>> = None;
        let mut ordinal_cache: Option<Result<u32, DateError>> = None;
        let mut weekday_num_cache: Option<Result<u32, DateError>> = None;

        while let Some(c) = chars.next() {
            if c == '%' {
                match chars.next() {
                    // --- Time Specifiers ---
                    Some('H') => result.push_str(&format!("{:02}", self.hour)), // 24-hour
                    Some('M') => result.push_str(&format!("{:02}", self.minute)),
                    Some('S') => result.push_str(&format!("{:02}", self.second)),
                    Some('T') => result.push_str(&format!( // %H:%M:%S
                        "{:02}:{:02}:{:02}",
                        self.hour, self.minute, self.second
                    )),

                    // --- Date Specifiers (delegated or direct access) ---
                    Some('%') => result.push('%'),
                    Some('Y') => result.push_str(&self.year().to_string()),
                    Some('m') => result.push_str(&format!("{:02}", self.month())),
                    Some('d') => result.push_str(&format!("{:02}", self.day())),
                    Some('B') => {
                        if let Some(name) = MONTH_NAMES_PERSIAN
                            .get((self.month().saturating_sub(1)) as usize)
                        {
                            result.push_str(name);
                        } else {
                            result.push_str("?InvalidMonth?");
                        }
                    }
                    Some('A') => {
                        if weekday_cache.is_none() {
                            // Need to call weekday_internal on the date part
                            weekday_cache = Some(self.date.weekday_internal());
                        }
                        match weekday_cache.as_ref().unwrap() {
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
                         if ordinal_cache.is_none() {
                            ordinal_cache = Some(self.date.ordinal_internal());
                        }
                        match ordinal_cache.as_ref().unwrap() {
                            Ok(ord) => result.push_str(&format!("{:03}", ord)),
                            Err(_) => result.push_str("???"),
                        }
                    }
                    // Unrecognized specifier
                    Some(other) => {
                        result.push('%');
                        result.push(other);
                    }
                    // Dangling '%'
                    None => {
                        result.push('%');
                        break;
                    }
                }
            } else {
                // Literal character
                result.push(c);
            }
        }
        result
    }

    // --- Parsing ---

    /// Parses a string into a `ParsiDateTime` using a specified format pattern.
    ///
    /// Extends `ParsiDate::parse` to handle time specifiers (`%H`, `%M`, `%S`, `%T`).
    /// Requires an exact match, including separators and padding.
    ///
    /// # Supported Format Specifiers for Parsing:
    ///
    /// *   Date: `%Y`, `%m`, `%d`, `%B`, `%%` (See `ParsiDate::parse`)
    /// *   Time:
    ///     *   `%H`: Parses a 2-digit hour (00-23).
    ///     *   `%M`: Parses a 2-digit minute (00-59).
    ///     *   `%S`: Parses a 2-digit second (00-59).
    ///     *   `%T`: Parses time in "HH:MM:SS" format.
    ///
    /// # Arguments
    ///
    /// * `s`: The input string slice to parse.
    /// * `format`: The format string containing literals and specifiers.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::ParseError(kind))` similar to `ParsiDate::parse`, with potential `kind` values including:
    /// * `ParseErrorKind::FormatMismatch`: Input doesn't match format structure/literals.
    /// * `ParseErrorKind::InvalidNumber`: Failed to parse numeric component (Y/m/d/H/M/S) or wrong digit count.
    /// * `ParseErrorKind::InvalidMonthName`: Failed to parse `%B`.
    /// * `ParseErrorKind::UnsupportedSpecifier`: Used an unsupported specifier (e.g., `%j`, `%A`).
    /// * `ParseErrorKind::InvalidDateValue`: Parsed date components are logically invalid.
    /// * `ParseErrorKind::InvalidTimeValue`: Parsed time components are logically invalid (e.g., hour 24).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError, ParseErrorKind};
    ///
    /// let s = "1403/05/02 15:30:45";
    /// let fmt = "%Y/%m/%d %H:%M:%S";
    /// let expected = ParsiDateTime::new(1403, 5, 2, 15, 30, 45).unwrap();
    /// assert_eq!(ParsiDateTime::parse(s, fmt), Ok(expected));
    ///
    /// let s_t = "1403-05-02T09:05:00";
    /// let fmt_t = "%Y-%m-%dT%T";
    /// assert_eq!(ParsiDateTime::parse(s_t, fmt_t), Ok(ParsiDateTime::new(1403, 5, 2, 9, 5, 0).unwrap()));
    ///
    /// // --- Error Cases ---
    /// // Invalid hour value
    /// assert_eq!(ParsiDateTime::parse("1403/05/02 24:00:00", fmt), Err(DateError::ParseError(ParseErrorKind::InvalidTimeValue)));
    /// // Invalid minute format (single digit)
    /// assert_eq!(ParsiDateTime::parse("1403/05/02 15:3:45", fmt), Err(DateError::ParseError(ParseErrorKind::InvalidNumber)));
    /// // Format mismatch in time separator
    /// assert_eq!(ParsiDateTime::parse("1403/05/02 15-30-45", fmt), Err(DateError::ParseError(ParseErrorKind::FormatMismatch)));
    /// // Missing time part completely
    /// assert_eq!(ParsiDateTime::parse("1403/05/02", fmt), Err(DateError::ParseError(ParseErrorKind::FormatMismatch)));
    /// ```
    pub fn parse(s: &str, format: &str) -> Result<Self, DateError> {
        let mut parsed_year: Option<i32> = None;
        let mut parsed_month: Option<u32> = None;
        let mut parsed_day: Option<u32> = None;
        let mut parsed_hour: Option<u32> = None;
        let mut parsed_minute: Option<u32> = None;
        let mut parsed_second: Option<u32> = None;

        // Use byte slices where possible, fallback to str for UTF-8 (%B)
        let mut s_bytes = s.as_bytes();
        let mut fmt_bytes = format.as_bytes();

        while !fmt_bytes.is_empty() {
            if fmt_bytes[0] == b'%' {
                if fmt_bytes.len() < 2 {
                    return Err(DateError::ParseError(ParseErrorKind::FormatMismatch)); // Dangling %
                }

                match fmt_bytes[1] {
                    // --- Time Specifiers ---
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
                            _ => unreachable!(), // Handled by outer match
                        }
                        s_bytes = &s_bytes[2..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    b'T' => { // Expects HH:MM:SS
                         if s_bytes.len() < 8 ||
                           !s_bytes[0..2].iter().all(|b| b.is_ascii_digit()) || s_bytes[2] != b':' ||
                           !s_bytes[3..5].iter().all(|b| b.is_ascii_digit()) || s_bytes[5] != b':' ||
                           !s_bytes[6..8].iter().all(|b| b.is_ascii_digit()) {
                            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch)); // Structure or non-digit
                         }
                        let h_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..2]) };
                        let m_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[3..5]) };
                        let s_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[6..8]) };

                        parsed_hour = Some(h_str.parse().map_err(|_| DateError::ParseError(ParseErrorKind::InvalidNumber))?);
                        parsed_minute = Some(m_str.parse().map_err(|_| DateError::ParseError(ParseErrorKind::InvalidNumber))?);
                        parsed_second = Some(s_str.parse().map_err(|_| DateError::ParseError(ParseErrorKind::InvalidNumber))?);

                        s_bytes = &s_bytes[8..];
                        fmt_bytes = &fmt_bytes[2..]; // Consume %T
                    }

                    // --- Date Specifiers (copied & adapted from ParsiDate::parse) ---
                    b'%' => {
                         if s_bytes.is_empty() || s_bytes[0] != b'%' { return Err(DateError::ParseError(ParseErrorKind::FormatMismatch)); }
                         s_bytes = &s_bytes[1..];
                         fmt_bytes = &fmt_bytes[2..];
                    }
                    b'Y' => {
                        if s_bytes.len() < 4 || !s_bytes[0..4].iter().all(|b| b.is_ascii_digit()) { return Err(DateError::ParseError(ParseErrorKind::InvalidNumber)); }
                        let year_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..4]) };
                        parsed_year = Some(year_str.parse().map_err(|_| DateError::ParseError(ParseErrorKind::InvalidNumber))?);
                        s_bytes = &s_bytes[4..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    b'm' | b'd' => {
                        if s_bytes.len() < 2 || !s_bytes[0..2].iter().all(|b| b.is_ascii_digit()) { return Err(DateError::ParseError(ParseErrorKind::InvalidNumber)); }
                        let num_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..2]) };
                        let val: u32 = num_str.parse().map_err(|_| DateError::ParseError(ParseErrorKind::InvalidNumber))?;
                        if fmt_bytes[1] == b'm' { parsed_month = Some(val); }
                        else { parsed_day = Some(val); }
                        s_bytes = &s_bytes[2..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                     b'B' => {
                        fmt_bytes = &fmt_bytes[2..]; // Consume %B format specifier first
                        let mut found_month = false;
                        let mut best_match_len = 0;
                        let mut matched_month_idx = 0;
                        let current_s = unsafe { std::str::from_utf8_unchecked(s_bytes) };

                        for (idx, month_name) in MONTH_NAMES_PERSIAN.iter().enumerate() {
                            if current_s.starts_with(month_name) {
                                best_match_len = month_name.as_bytes().len();
                                matched_month_idx = idx;
                                found_month = true;
                                break;
                            }
                        }
                        if !found_month { return Err(DateError::ParseError(ParseErrorKind::InvalidMonthName)); }
                        parsed_month = Some((matched_month_idx + 1) as u32);
                        s_bytes = &s_bytes[best_match_len..];
                        // fmt_bytes already advanced
                    }

                    // --- Unsupported Specifiers ---
                     _ => return Err(DateError::ParseError(ParseErrorKind::UnsupportedSpecifier)),
                }
            } else { // Literal character
                if s_bytes.is_empty() || s_bytes[0] != fmt_bytes[0] {
                    return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                }
                s_bytes = &s_bytes[1..];
                fmt_bytes = &fmt_bytes[1..];
            }
        } // End while loop

        if !s_bytes.is_empty() {
            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch)); // Trailing chars
        }

        // Check if all components were parsed
        match ( parsed_year, parsed_month, parsed_day, parsed_hour, parsed_minute, parsed_second)
        {
            (Some(y), Some(m), Some(d), Some(h), Some(min), Some(s)) => {
                // All components found, use ParsiDateTime::new for final validation
                 ParsiDateTime::new(y, m, d, h, min, s).map_err(|e| match e {
                     // Map validation errors to specific parse error kinds
                     DateError::InvalidDate => DateError::ParseError(ParseErrorKind::InvalidDateValue),
                     DateError::InvalidTime => DateError::ParseError(ParseErrorKind::InvalidTimeValue),
                     other => other, // Propagate unexpected errors
                 })
            }
            _ => Err(DateError::ParseError(ParseErrorKind::FormatMismatch)), // Missing components
        }
    }

    // --- Arithmetic ---
    // Note: Arithmetic using chrono::Duration is often the most robust way.

    /// Adds a `chrono::Duration` to this `ParsiDateTime`.
    ///
    /// Converts to Gregorian `NaiveDateTime`, adds the duration, and converts back.
    /// Handles date and time rollovers correctly.
    ///
    /// # Arguments
    ///
    /// * `duration`: The `chrono::Duration` to add (can be positive or negative).
    ///
    /// # Errors
    ///
    /// Returns `Err` if the initial `ParsiDateTime` is invalid, if conversion to/from
    /// Gregorian fails, or if the resulting date/time is outside the supported range
    /// or causes an overflow during `chrono`'s arithmetic. Specific errors can be
    /// `InvalidDate`, `InvalidTime`, `GregorianConversionError`, or `ArithmeticOverflow`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    /// use chrono::Duration;
    ///
    /// let dt = ParsiDateTime::new(1403, 12, 30, 23, 59, 58).unwrap(); // Leap year end
    ///
    /// // Add 3 seconds -> crosses into next year
    /// let dt_plus_3s = dt.add_duration(Duration::seconds(3));
    /// assert!(dt_plus_3s.is_ok());
    /// let dt_plus_3s = dt_plus_3s.unwrap();
    /// assert_eq!(dt_plus_3s.date(), ParsiDate::new(1404, 1, 1).unwrap());
    /// assert_eq!(dt_plus_3s.time(), (0, 0, 1));
    ///
    /// // Subtract 1 day and 1 second
    /// let dt_minus_day_sec = dt.add_duration(Duration::days(-1) + Duration::seconds(-1));
    /// assert!(dt_minus_day_sec.is_ok());
    /// let dt_minus_day_sec = dt_minus_day_sec.unwrap();
    /// assert_eq!(dt_minus_day_sec.date(), ParsiDate::new(1403, 12, 29).unwrap());
    /// assert_eq!(dt_minus_day_sec.time(), (23, 59, 57));
    /// ```
    pub fn add_duration(&self, duration: Duration) -> Result<Self, DateError> {
        // Validate self first
        if !self.is_valid() {
            if !self.date.is_valid() { return Err(DateError::InvalidDate); }
            else { return Err(DateError::InvalidTime); }
        }

        // Convert self to Gregorian NaiveDateTime
        let gregorian_dt = self.to_gregorian()?; // Handles validation implicitly

        // Add duration using chrono's checked_add
        let new_gregorian_dt = gregorian_dt.checked_add_signed(duration)
            .ok_or(DateError::ArithmeticOverflow)?;

        // Convert back to ParsiDateTime
        Self::from_gregorian(new_gregorian_dt)
    }

    /// Subtracts a `chrono::Duration` from this `ParsiDateTime`.
    ///
    /// Equivalent to `add_duration(-duration)`.
    ///
    /// # Errors
    ///
    /// See `add_duration` for potential errors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDateTime, ParsiDate, DateError};
    /// use chrono::Duration;
    ///
    /// let dt = ParsiDateTime::new(1404, 1, 1, 0, 0, 1).unwrap(); // Start of common year
    ///
    /// // Subtract 3 seconds -> crosses into previous year (leap)
    /// let dt_minus_3s = dt.sub_duration(Duration::seconds(3));
    /// assert!(dt_minus_3s.is_ok());
    /// let dt_minus_3s = dt_minus_3s.unwrap();
    /// assert_eq!(dt_minus_3s.date(), ParsiDate::new(1403, 12, 30).unwrap());
    /// assert_eq!(dt_minus_3s.time(), (23, 59, 58));
    /// ```
    pub fn sub_duration(&self, duration: Duration) -> Result<Self, DateError> {
        // Add the negated duration. chrono::Duration handles negation safely.
        self.add_duration(-duration)
    }

     /// Adds a specified number of days, preserving the time component.
     ///
     /// Uses `ParsiDate::add_days` for the date part calculation.
     ///
     /// # Arguments
     /// * `days`: Number of days to add (can be negative).
     /// # Errors
     /// Returns `Err` if the initial date/time is invalid, or if the date arithmetic fails (e.g., overflow, out of range). See `ParsiDate::add_days`.
    pub fn add_days(&self, days: i64) -> Result<Self, DateError> {
        if !self.is_valid() {
            if !self.date.is_valid() { return Err(DateError::InvalidDate); }
            else { return Err(DateError::InvalidTime); }
        }
        let new_date = self.date.add_days(days)?;
        // Recombine with original time. Time components are known valid.
        Ok(ParsiDateTime { date: new_date, hour: self.hour, minute: self.minute, second: self.second })
    }

     /// Subtracts a specified number of days, preserving the time component.
     /// Equivalent to `add_days(-days)`.
    pub fn sub_days(&self, days: u64) -> Result<Self, DateError> {
        if !self.is_valid() {
            if !self.date.is_valid() { return Err(DateError::InvalidDate); }
            else { return Err(DateError::InvalidTime); }
        }
         let new_date = self.date.sub_days(days)?;
         Ok(ParsiDateTime { date: new_date, hour: self.hour, minute: self.minute, second: self.second })
    }

     /// Adds a specified number of months, preserving the time component and clamping the day if necessary.
     /// Uses `ParsiDate::add_months` for the date part calculation.
     /// # Arguments
     /// * `months`: Number of months to add (can be negative).
     /// # Errors
     /// Returns `Err` if the initial date/time is invalid, or if the date arithmetic fails. See `ParsiDate::add_months`.
    pub fn add_months(&self, months: i32) -> Result<Self, DateError> {
        if !self.is_valid() {
            if !self.date.is_valid() { return Err(DateError::InvalidDate); }
            else { return Err(DateError::InvalidTime); }
        }
         let new_date = self.date.add_months(months)?;
         Ok(ParsiDateTime { date: new_date, hour: self.hour, minute: self.minute, second: self.second })
     }

    /// Subtracts a specified number of months, preserving the time component and clamping the day.
    /// Equivalent to `add_months(-months)`.
    pub fn sub_months(&self, months: u32) -> Result<Self, DateError> {
        if !self.is_valid() {
            if !self.date.is_valid() { return Err(DateError::InvalidDate); }
            else { return Err(DateError::InvalidTime); }
        }
         let new_date = self.date.sub_months(months)?;
         Ok(ParsiDateTime { date: new_date, hour: self.hour, minute: self.minute, second: self.second })
    }

    /// Adds a specified number of years, preserving the time component and adjusting leap day if necessary.
    /// Uses `ParsiDate::add_years` for the date part calculation.
    /// # Arguments
    /// * `years`: Number of years to add (can be negative).
    /// # Errors
    /// Returns `Err` if the initial date/time is invalid, or if the date arithmetic fails. See `ParsiDate::add_years`.
    pub fn add_years(&self, years: i32) -> Result<Self, DateError> {
        if !self.is_valid() {
            if !self.date.is_valid() { return Err(DateError::InvalidDate); }
            else { return Err(DateError::InvalidTime); }
        }
         let new_date = self.date.add_years(years)?;
         Ok(ParsiDateTime { date: new_date, hour: self.hour, minute: self.minute, second: self.second })
    }

    /// Subtracts a specified number of years, preserving the time component and adjusting leap day.
    /// Equivalent to `add_years(-years)`.
     pub fn sub_years(&self, years: u32) -> Result<Self, DateError> {
        if !self.is_valid() {
            if !self.date.is_valid() { return Err(DateError::InvalidDate); }
            else { return Err(DateError::InvalidTime); }
        }
         let new_date = self.date.sub_years(years)?;
         Ok(ParsiDateTime { date: new_date, hour: self.hour, minute: self.minute, second: self.second })
     }


    // --- Helper Methods ---

    /// Creates a new `ParsiDateTime` with the hour component modified.
    ///
    /// # Arguments
    /// * `hour`: The desired hour (0-23).
    /// # Errors
    /// Returns `Err(DateError::InvalidTime)` if the target `hour` is invalid.
    /// Returns `Err(DateError::InvalidDate)` if `self`'s date part was invalid.
    pub fn with_hour(&self, hour: u32) -> Result<Self, DateError> {
        if !self.date.is_valid() { return Err(DateError::InvalidDate); } // Check date part first
        if hour > 23 { return Err(DateError::InvalidTime); }
        Ok(ParsiDateTime { date: self.date, hour, minute: self.minute, second: self.second })
    }

    /// Creates a new `ParsiDateTime` with the minute component modified.
    ///
    /// # Arguments
    /// * `minute`: The desired minute (0-59).
    /// # Errors
    /// Returns `Err(DateError::InvalidTime)` if the target `minute` is invalid.
    /// Returns `Err(DateError::InvalidDate)` if `self`'s date part was invalid.
     pub fn with_minute(&self, minute: u32) -> Result<Self, DateError> {
        if !self.date.is_valid() { return Err(DateError::InvalidDate); }
        if minute > 59 { return Err(DateError::InvalidTime); }
        Ok(ParsiDateTime { date: self.date, hour: self.hour, minute, second: self.second })
    }

    /// Creates a new `ParsiDateTime` with the second component modified.
    ///
    /// # Arguments
    /// * `second`: The desired second (0-59).
    /// # Errors
    /// Returns `Err(DateError::InvalidTime)` if the target `second` is invalid.
    /// Returns `Err(DateError::InvalidDate)` if `self`'s date part was invalid.
    pub fn with_second(&self, second: u32) -> Result<Self, DateError> {
        if !self.date.is_valid() { return Err(DateError::InvalidDate); }
        if second > 59 { return Err(DateError::InvalidTime); }
        Ok(ParsiDateTime { date: self.date, hour: self.hour, minute: self.minute, second })
    }

    /// Creates a new `ParsiDateTime` with the time components modified.
    ///
    /// # Arguments
    /// * `hour`: The desired hour (0-23).
    /// * `minute`: The desired minute (0-59).
    /// * `second`: The desired second (0-59).
    /// # Errors
    /// Returns `Err(DateError::InvalidTime)` if any time component is invalid.
    /// Returns `Err(DateError::InvalidDate)` if `self`'s date part was invalid.
    pub fn with_time(&self, hour: u32, minute: u32, second: u32) -> Result<Self, DateError> {
        if !self.date.is_valid() { return Err(DateError::InvalidDate); }
        if hour > 23 || minute > 59 || second > 59 {
            return Err(DateError::InvalidTime);
        }
        Ok(ParsiDateTime { date: self.date, hour, minute, second })
    }

     /// Creates a new `ParsiDateTime` with the date component modified using `ParsiDate::with_year`.
     /// Time component remains unchanged. See `ParsiDate::with_year` for details and errors.
     pub fn with_year(&self, year: i32) -> Result<Self, DateError> {
         let new_date = self.date.with_year(year)?;
         // Time component validation is not needed again if `self` was valid initially.
         Ok(ParsiDateTime { date: new_date, hour: self.hour, minute: self.minute, second: self.second })
     }

     /// Creates a new `ParsiDateTime` with the date component modified using `ParsiDate::with_month`.
     /// Time component remains unchanged. See `ParsiDate::with_month` for details and errors.
     pub fn with_month(&self, month: u32) -> Result<Self, DateError> {
         let new_date = self.date.with_month(month)?;
         Ok(ParsiDateTime { date: new_date, hour: self.hour, minute: self.minute, second: self.second })
     }

     /// Creates a new `ParsiDateTime` with the date component modified using `ParsiDate::with_day`.
     /// Time component remains unchanged. See `ParsiDate::with_day` for details and errors.
    pub fn with_day(&self, day: u32) -> Result<Self, DateError> {
         let new_date = self.date.with_day(day)?;
         Ok(ParsiDateTime { date: new_date, hour: self.hour, minute: self.minute, second: self.second })
    }

} // <<<=== End impl ParsiDateTime ===>>>

// --- Trait Implementations ---

/// Implements the `Display` trait for `ParsiDateTime`.
///
/// Formats the date and time using a default style: "YYYY/MM/DD HH:MM:SS".
impl fmt::Display for ParsiDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format date using ParsiDate's Display (short format) and add time
        write!(
            f,
            "{} {:02}:{:02}:{:02}",
            self.date, // Uses ParsiDate's Display impl ("YYYY/MM/DD")
            self.hour,
            self.minute,
            self.second
        )
    }
}

// --- Operator Overloads for Duration ---

impl Add<Duration> for ParsiDateTime {
    type Output = Result<ParsiDateTime, DateError>;

    /// Adds a `chrono::Duration` using the `+` operator.
    /// See [`add_duration`](#method.add_duration).
    fn add(self, duration: Duration) -> Self::Output {
        self.add_duration(duration)
    }
}

impl Sub<Duration> for ParsiDateTime {
    type Output = Result<ParsiDateTime, DateError>;

    /// Subtracts a `chrono::Duration` using the `-` operator.
    /// See [`sub_duration`](#method.sub_duration).
    fn sub(self, duration: Duration) -> Self::Output {
        self.sub_duration(duration)
    }
}

// Optional: Implement Add/Sub between two ParsiDateTime instances to get a Duration?
// This requires converting both to NaiveDateTime first.
impl Sub<ParsiDateTime> for ParsiDateTime {
    type Output = Result<Duration, DateError>;

    /// Calculates the `chrono::Duration` between two `ParsiDateTime` instances (`self` - `other`).
    ///
    /// # Errors
    /// Returns `Err` if either `ParsiDateTime` is invalid or conversion to Gregorian fails.
    fn sub(self, other: ParsiDateTime) -> Self::Output {
        let self_g = self.to_gregorian()?;
        let other_g = other.to_gregorian()?;
        Ok(self_g.signed_duration_since(other_g))
    }
}
