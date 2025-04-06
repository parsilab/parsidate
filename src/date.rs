//  * Copyright (C) Mohammad (Sina) Jalalvandi (parsidate) 2024-2025 <jalalvandi.sina@gmail.com>
//  * Version : 1.3.3
//  * 128558ad-c066-4c4a-9b93-bca896bf4465
//  * src/date.rs
//
//! Contains the `ParsiDate` struct definition and its implementation.

// Use necessary items from other modules and external crates
use crate::constants::{MAX_PARSI_DATE, MIN_PARSI_DATE, MONTH_NAMES_PERSIAN, WEEKDAY_NAMES_PERSIAN};
use crate::error::{DateError, ParseErrorKind};
use chrono::{Datelike, Days, NaiveDate, Timelike}; // Added Days for arithmetic
use std::fmt;
use std::ops::{Add, Sub}; // For potential future Duration addition
use std::str::FromStr; // For potential future direct FromStr impl

// --- Data Structures ---

/// Represents a date in the Persian (Jalali or Shamsi) calendar system.
///
/// Stores the year, month (1-12), and day (1-31) components.
/// Provides methods for validation, conversion, formatting, parsing, and arithmetic.
///
/// Note on Range: Supports years from 1 up to 9999.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParsiDate {
    /// The year component of the Persian date (e.g., 1403). Must be between 1 and 9999 inclusive.
   pub(crate) year: i32,
    /// The month component of the Persian date (1 = Farvardin, ..., 12 = Esfand). Must be between 1 and 12 inclusive.
   pub(crate) month: u32,
    /// The day component of the Persian date (1-29/30/31). Must be valid for the given month and year.
   pub(crate) day: u32,
}

// --- Core Implementation ---

impl ParsiDate {
    // --- Constructors and Converters ---

    /// Creates a new `ParsiDate` instance from year, month, and day components.
    ///
    /// This function validates the date upon creation. The year must be between 1 and 9999,
    /// the month between 1 and 12, and the day must be valid for the given month and year
    /// (considering leap years for Esfand).
    ///
    /// # Arguments
    ///
    /// * `year`: The Persian year (1-9999).
    /// * `month`: The Persian month (1-12).
    /// * `day`: The Persian day (1-31, depending on month and leap year).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the provided components do not form a valid
    /// Persian date within the supported range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// assert!(ParsiDate::new(1403, 5, 2).is_ok());
    /// assert_eq!(ParsiDate::new(1403, 13, 1), Err(DateError::InvalidDate)); // Invalid month
    /// assert_eq!(ParsiDate::new(1404, 12, 30), Err(DateError::InvalidDate)); // Invalid day (1404 not leap)
    /// assert_eq!(ParsiDate::new(0, 1, 1), Err(DateError::InvalidDate)); // Invalid year
    /// ```
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self, DateError> {
        // Initial struct creation
        let date = ParsiDate { year, month, day };
        // Validate the components
        if date.is_valid() {
            Ok(date)
        } else {
            Err(DateError::InvalidDate)
        }
    }

    /// Creates a `ParsiDate` from year, month, and day without validation.
    ///
    /// **Warning:** This function is `unsafe` because it bypasses the validation checks
    /// performed by `ParsiDate::new`. Creating a `ParsiDate` with invalid components
    /// (e.g., month 13, day 32, year 0) using this function can lead to undefined behavior,
    /// incorrect results, or panics when other methods are called on the invalid date object.
    ///
    /// This should only be used in performance-critical situations where the date components
    /// are already known to be valid through external means. Prefer `ParsiDate::new()`
    /// for safe construction in most cases.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided `year`, `month`, and `day` combination
    /// represents a logically valid Persian date according to the calendar rules and
    /// within the supported year range (1-9999).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// // Assume year, month, day are validated elsewhere
    /// let year = 1403;
    /// let month = 5;
    /// let day = 2;
    ///
    /// if year >= 1 && year <= 9999 && month >= 1 && month <= 12 && day >= 1 && day <= ParsiDate::days_in_month(year, month) {
    ///     let date = unsafe { ParsiDate::new_unchecked(year, month, day) };
    ///     assert_eq!(date.year(), 1403);
    /// } else {
    ///     // Handle invalid input case
    /// }
    /// ```
    pub const unsafe fn new_unchecked(year: i32, month: u32, day: u32) -> Self {
        ParsiDate { year, month, day }
    }

    /// Creates a `ParsiDate` from the day number within a given Persian year (ordinal day).
    ///
    /// The ordinal day is 1-based, where 1 corresponds to Farvardin 1st.
    /// The valid range for `ordinal` is 1 to 365 for common years, and 1 to 366 for leap years.
    ///
    /// # Arguments
    ///
    /// * `year`: The Persian year (1-9999).
    /// * `ordinal`: The day number within the year (1-365 or 1-366).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidOrdinal)` if the `ordinal` value is 0 or greater than
    /// the number of days in the specified `year`.
    /// Returns `Err(DateError::InvalidDate)` if the `year` is outside the supported range (1-9999),
    /// although this check happens during the final `ParsiDate::new` call.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// assert_eq!(ParsiDate::from_ordinal(1403, 1), Ok(ParsiDate::new(1403, 1, 1).unwrap())); // Farvardin 1st
    /// assert_eq!(ParsiDate::from_ordinal(1403, 366), Ok(ParsiDate::new(1403, 12, 30).unwrap())); // Last day of leap year 1403
    /// assert_eq!(ParsiDate::from_ordinal(1404, 365), Ok(ParsiDate::new(1404, 12, 29).unwrap())); // Last day of common year 1404
    /// assert_eq!(ParsiDate::from_ordinal(1404, 366), Err(DateError::InvalidOrdinal)); // Too large for common year
    /// assert_eq!(ParsiDate::from_ordinal(1403, 0), Err(DateError::InvalidOrdinal)); // Zero is invalid
    /// ```
    pub fn from_ordinal(year: i32, ordinal: u32) -> Result<Self, DateError> {
        // Basic validation of ordinal
        if ordinal == 0 {
            return Err(DateError::InvalidOrdinal);
        }
        // Determine days in the target year
        let is_leap = Self::is_persian_leap_year(year);
        let days_in_year = if is_leap { 366 } else { 365 };

        // Validate ordinal against year length
        if ordinal > days_in_year {
            return Err(DateError::InvalidOrdinal);
        }

        // Iterate through months to find the correct month and day
        let mut month = 1u32;
        let mut day = ordinal;
        let month_lengths = Self::month_lengths(year);

        for (m_idx, length) in month_lengths.iter().enumerate() {
            if day <= *length {
                month = (m_idx + 1) as u32; // Found the month (m_idx is 0-based)
                break; // Exit loop once month is found
            }
            // Subtract days of the current month and move to the next
            day -= *length;
            // Update month number for the next iteration (or if loop ends)
            // This ensures month is correct even if `day` becomes exactly 0 after subtraction
            month = (m_idx + 2) as u32;
        }

        // Use new() for final validation (including year range check)
        // The logic above should guarantee month/day are valid if ordinal was valid,
        // but `new` provides an extra safety layer and handles the year check.
        ParsiDate::new(year, month, day)
    }

    /// Converts a Gregorian date (`chrono::NaiveDate`) to its equivalent Persian (Jalali) date.
    ///
    /// This function implements the conversion algorithm, determining the corresponding
    /// Persian year, month, and day for the given Gregorian date.
    ///
    /// # Arguments
    ///
    /// * `gregorian_date`: The Gregorian date to convert.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::GregorianConversionError)` if:
    /// * The `gregorian_date` is before the Persian epoch start (approximately 622-03-21).
    /// * The calculation results in a Persian year outside the supported range (1-9999).
    /// * An internal `chrono` operation fails (e.g., creating the epoch date).
    /// * An internal inconsistency is detected during calculation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// let g_date = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
    /// assert_eq!(ParsiDate::from_gregorian(g_date), Ok(ParsiDate::new(1403, 5, 2).unwrap()));
    ///
    /// let epoch_gregorian = NaiveDate::from_ymd_opt(622, 3, 21).unwrap();
    /// assert_eq!(ParsiDate::from_gregorian(epoch_gregorian), Ok(ParsiDate::new(1, 1, 1).unwrap()));
    ///
    /// let before_epoch = NaiveDate::from_ymd_opt(622, 3, 20).unwrap();
    /// assert_eq!(ParsiDate::from_gregorian(before_epoch), Err(DateError::GregorianConversionError));
    /// ```
    pub fn from_gregorian(gregorian_date: NaiveDate) -> Result<Self, DateError> {
        // Define the start of the Persian epoch in Gregorian terms
        let persian_epoch_gregorian_start =
            NaiveDate::from_ymd_opt(622, 3, 21).ok_or(DateError::GregorianConversionError)?;

        // Check if the date is before the epoch
        if gregorian_date < persian_epoch_gregorian_start {
            return Err(DateError::GregorianConversionError); // Date is before epoch start
        }

        // --- Calculate Persian Year ---
        // Estimate the number of days passed since the epoch start
        let days_since_epoch_day1 = gregorian_date
            .signed_duration_since(persian_epoch_gregorian_start)
            .num_days();

        // Make an initial guess for the Persian year.
        // Average ~365.25 days/year. Dividing by 366 provides a conservative initial guess.
        let mut p_year_guess = MIN_PARSI_DATE.year + (days_since_epoch_day1 / 366) as i32;
        if p_year_guess < MIN_PARSI_DATE.year {
            p_year_guess = MIN_PARSI_DATE.year; // Ensure guess is at least 1
        }

        // Loop to find the correct Persian year by checking the Gregorian date of Farvardin 1st
        // for the guessed year and the next year.
        let p_year = loop {
            // Calculate Gregorian date for Farvardin 1 of the guessed year
            // Use unsafe new_unchecked + internal conversion for performance inside the loop
            let temp_start_date = unsafe { ParsiDate::new_unchecked(p_year_guess, 1, 1) };
            let gy_start_of_pyear = match temp_start_date.to_gregorian_internal() {
                Ok(gd) => gd,
                // If conversion fails for the guess (e.g., year too high), we need to adjust down.
                // However, the logic should generally converge before hitting extreme limits.
                // If it *does* fail, it implies an issue, likely out of range.
                Err(e) => return Err(e),
            };

            // If Farvardin 1st of the guess is *after* the target date, the guess is too high.
            if gy_start_of_pyear > gregorian_date {
                p_year_guess -= 1; // Adjust guess down
                                   // If the adjusted guess is now the correct year, break.
                                   // We need to re-check the start date for the new guess if we continue looping,
                                   // but if `gy_start_of_pyear` was only slightly too high, `p_year_guess - 1` is likely correct.
                                   // Re-evaluating in the next loop iteration is safer. Let's refine this.

                // Let's test the *new* guess immediately.
                let temp_prev_start_date = unsafe { ParsiDate::new_unchecked(p_year_guess, 1, 1) };
                match temp_prev_start_date.to_gregorian_internal() {
                    Ok(gd_prev) => {
                        if gd_prev <= gregorian_date {
                            // The previous year starts on or before the target date.
                            // And we know the original guess year started *after*.
                            // So, the correct year is `p_year_guess`.
                            break p_year_guess;
                        } else {
                            // Still too high? Continue loop to decrement further. Should be rare.
                            continue;
                        }
                    }
                    Err(e) => return Err(e), // Error converting decremented year start
                }
            }

            // If Farvardin 1st of the guess is on or before the target date,
            // check if Farvardin 1st of the *next* year is *after* the target date.
            let next_year = match p_year_guess.checked_add(1) {
                Some(y) => y,
                None => return Err(DateError::GregorianConversionError), // Year overflow
            };
            let temp_start_date_next = unsafe { ParsiDate::new_unchecked(next_year, 1, 1) };
            match temp_start_date_next.to_gregorian_internal() {
                Ok(gd_next) => {
                    if gd_next > gregorian_date {
                        // Found the correct year range! `p_year_guess` is the year.
                        break p_year_guess;
                    } else {
                        // Target date is in a later year, increment guess and loop again.
                        p_year_guess += 1;
                    }
                }
                Err(_) => {
                    // If converting the start of the *next* year fails (e.g., out of range like year 10000+),
                    // it implies the current guess (`p_year_guess`) might be the last possible valid year
                    // containing the date.
                    if gy_start_of_pyear <= gregorian_date {
                        // The current guess starts on/before the target, and the next year is invalid/too far.
                        break p_year_guess;
                    } else {
                        // This case (current guess starts *after* target AND next year fails) seems unlikely
                        // given the earlier check. If it happens, return error.
                        return Err(DateError::GregorianConversionError);
                    }
                }
            }

            // Safety break to prevent potential infinite loops with very large dates or logic errors.
            if p_year_guess > MAX_PARSI_DATE.year + 5 || p_year_guess < MIN_PARSI_DATE.year {
                return Err(DateError::GregorianConversionError); // Likely out of range or issue
            }
        }; // End of year-finding loop

        // --- Calculate Persian Month and Day ---
        // Now `p_year` holds the correct Persian year.
        // Find the Gregorian date corresponding to Farvardin 1st of this correct year.
        let correct_pyear_start_gregorian =
            unsafe { ParsiDate::new_unchecked(p_year, 1, 1) }.to_gregorian_internal()?;

        // Calculate how many days into the Persian year the target Gregorian date falls (0-based index).
        let days_into_year = gregorian_date
            .signed_duration_since(correct_pyear_start_gregorian)
            .num_days();

        // This should not be negative if the year-finding logic is correct.
        if days_into_year < 0 {
            return Err(DateError::GregorianConversionError); // Internal calculation error state
        }

        // Determine month and day from the 0-based `days_into_year`.
        let month_lengths = Self::month_lengths(p_year);
        let mut remaining_days_in_year = days_into_year as u32; // Now 0-indexed day number within year
        let mut p_month = 1u32;
        let mut p_day = 1u32; // Will be overwritten

        for (m_idx, length) in month_lengths.iter().enumerate() {
            // Ensure length is not zero to avoid infinite loop (shouldn't happen)
            if *length == 0 {
                return Err(DateError::InvalidDate); // Should not happen with valid month_lengths
            }
            // Check if the day falls within the current month (length)
            if remaining_days_in_year < *length {
                p_month = (m_idx + 1) as u32; // Month is 1-based index + 1
                p_day = remaining_days_in_year + 1; // Day is 1-based remaining days + 1
                break; // Found the month and day
            }
            // Subtract the days of this month and continue to the next
            remaining_days_in_year -= *length;

            // Handle case where the date is the very last day of the year
            if m_idx == 11 && remaining_days_in_year == 0 {
                // This occurs *after* subtracting the last month's length.
                // It means the target day was the last day of Esfand.
                p_month = 12;
                p_day = *length; // Day is the length of Esfand
                break;
            }
        }

        // Use new() for final validation of the calculated date (year, month, day).
        // This ensures consistency and catches potential edge cases in the logic above.
        ParsiDate::new(p_year, p_month, p_day)
    }

    /// Converts this Persian (Jalali) date to its equivalent Gregorian date (`chrono::NaiveDate`).
    ///
    /// Performs validation before attempting the conversion.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the `ParsiDate` instance itself holds invalid data
    /// (e.g., created via `unsafe fn new_unchecked` with bad values).
    /// Returns `Err(DateError::GregorianConversionError)` if the conversion results in a Gregorian
    /// date outside the range supported by `chrono::NaiveDate` or if an internal arithmetic
    /// overflow occurs during calculation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use parsidate::ParsiDate;
    ///
    /// let pd = ParsiDate::new(1403, 5, 2).unwrap();
    /// let expected_gregorian = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
    /// assert_eq!(pd.to_gregorian(), Ok(expected_gregorian));
    ///
    /// let pd_epoch = ParsiDate::new(1, 1, 1).unwrap();
    /// let expected_epoch_gregorian = NaiveDate::from_ymd_opt(622, 3, 21).unwrap();
    /// assert_eq!(pd_epoch.to_gregorian(), Ok(expected_epoch_gregorian));
    /// ```
    pub fn to_gregorian(&self) -> Result<NaiveDate, DateError> {
        // Ensure the ParsiDate object itself is valid before converting.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Call the internal conversion logic.
        self.to_gregorian_internal()
    }

    /// Internal conversion logic: Persian to Gregorian.
    /// Assumes `self` represents a valid ParsiDate.
    /// Calculates days since the Persian epoch and adds them to the Gregorian epoch start date.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::GregorianConversionError)` if chrono fails to create the epoch date,
    /// if integer overflow occurs during day summation, or if adding the final day offset
    /// to the Gregorian epoch date fails (e.g., results in a date out of chrono's range).
    fn to_gregorian_internal(&self) -> Result<NaiveDate, DateError> {
        // Gregorian start date corresponding to Persian epoch (1/1/1).
        let persian_epoch_gregorian_start =
            NaiveDate::from_ymd_opt(622, 3, 21).ok_or(DateError::GregorianConversionError)?;

        // Calculate the total number of days from Persian epoch start (1/1/1) up to the day *before* self.
        // Sum days in full years prior to self.year.
        let mut total_days_offset: i64 = 0;
        // Loop from year 1 up to (but not including) self.year.
        for y in MIN_PARSI_DATE.year..self.year {
            let days_in_year: i64 = if Self::is_persian_leap_year(y) {
                366
            } else {
                365
            };
            // Add days, checking for potential integer overflow.
            total_days_offset = total_days_offset
                .checked_add(days_in_year)
                .ok_or(DateError::GregorianConversionError)?;
        }

        // Sum days in full months prior to self.month within self.year.
        let month_lengths_current_year = Self::month_lengths(self.year);
        // month is 1-based, loop from month 1 up to (but not including) self.month.
        if self.month > 1 {
            // self.month is guaranteed to be <= 12 because to_gregorian checks is_valid first.
            for m in 1..self.month {
                // Get month length (m-1 is the 0-based index).
                // This indexing is safe due to the is_valid check.
                let days_in_month = month_lengths_current_year[(m - 1) as usize] as i64;
                // Add days, checking for potential integer overflow.
                total_days_offset = total_days_offset
                    .checked_add(days_in_month)
                    .ok_or(DateError::GregorianConversionError)?;
            }
        }
        // If self.month is 1, this loop doesn't run, which is correct.

        // Add the day of the month (minus 1, since we want offset from the start of the month).
        // self.day is guaranteed to be >= 1.
        total_days_offset = total_days_offset
            .checked_add((self.day - 1) as i64)
            .ok_or(DateError::GregorianConversionError)?;

        // The total_days_offset now represents the number of days elapsed since 1/1/1.
        // Add this offset to the Gregorian date corresponding to 1/1/1.
        if total_days_offset < 0 {
            // This should not happen if year >= 1 and day >= 1.
            return Err(DateError::GregorianConversionError); // Indicates an internal logic error
        }

        // Use chrono's checked_add_days for safe addition.
        persian_epoch_gregorian_start
            .checked_add_days(chrono::Days::new(total_days_offset as u64))
            .ok_or(DateError::GregorianConversionError) // Return error if chrono addition fails (e.g., out of range)
    }

    /// Returns the Persian date for the current system date based on the local timezone.
    ///
    /// Obtains the current Gregorian date from the system and converts it to `ParsiDate`.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::GregorianConversionError)` if the conversion from the current
    /// Gregorian date fails. This could happen if the system clock is set to a date
    /// outside the range supported by this library or `chrono`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// match ParsiDate::today() {
    ///     Ok(today) => println!("Today in Persian calendar is: {}", today),
    ///     Err(e) => eprintln!("Failed to get today's Persian date: {}", e),
    /// }
    /// ```
    pub fn today() -> Result<Self, DateError> {
        // Get current local time.
        let now = chrono::Local::now();
        // Extract the naive date part (ignoring time and timezone offset after getting local date).
        let gregorian_today = now.date_naive();
        // Convert the Gregorian date to ParsiDate.
        Self::from_gregorian(gregorian_today)
    }

    // --- Accessors ---

    /// Returns the year component of the date.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// assert_eq!(date.year(), 1403);
    /// ```
    #[inline]
    pub const fn year(&self) -> i32 {
        self.year
    }

    /// Returns the month component of the date (1 = Farvardin, ..., 12 = Esfand).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// assert_eq!(date.month(), 5); // 5 corresponds to Mordad
    /// ```
    #[inline]
    pub const fn month(&self) -> u32 {
        self.month
    }

    /// Returns the day component of the date (1-31).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// assert_eq!(date.day(), 2);
    /// ```
    #[inline]
    pub const fn day(&self) -> u32 {
        self.day
    }

    // --- Validation and Leap Year ---

    /// Checks if the current `ParsiDate` instance represents a valid date according to the
    /// Persian calendar rules and the supported range of this library.
    ///
    /// Validation checks include:
    /// * Year is within the range [1, 9999].
    /// * Month is within the range [1, 12].
    /// * Day is within the range [1, days_in_month(year, month)].
    ///
    /// # Returns
    ///
    /// `true` if the date is valid, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let valid_date = ParsiDate::new(1403, 12, 30).unwrap(); // 1403 is leap
    /// assert!(valid_date.is_valid());
    ///
    /// let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) }; // 1404 not leap
    /// assert!(!invalid_date.is_valid());
    ///
    /// let invalid_month = unsafe { ParsiDate::new_unchecked(1403, 13, 1) };
    /// assert!(!invalid_month.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        // Check year range
        if self.year < MIN_PARSI_DATE.year || self.year > MAX_PARSI_DATE.year {
            return false;
        }
        // Check month range
        if self.month == 0 || self.month > 12 {
            return false;
        }
        // Check day range (day must be at least 1)
        if self.day == 0 {
            return false;
        }
        // Check day against the maximum days for the given month and year
        let max_days = Self::days_in_month(self.year, self.month);
        // Note: days_in_month returns 0 for invalid months, handled by the month check above.
        // If max_days were 0 here, it would imply an invalid month passed the earlier check,
        // which shouldn't happen. The final check ensures day <= max_days.
        self.day <= max_days
    }

    /// Determines if a given Persian year is a leap year.
    ///
    /// This implementation uses a common algorithmic approximation based on a 33-year cycle.
    /// A year `y` is considered leap if `y % 33` results in one of the specific remainders:
    /// 1, 5, 9, 13, 17, 22, 26, or 30.
    ///
    /// Note: Astronomical calculations provide the most accurate determination, but this
    /// 33-year cycle is widely used and accurate for a very long period around the present.
    ///
    /// Years less than 1 are considered non-leap.
    ///
    /// # Arguments
    ///
    /// * `year`: The Persian year to check.
    ///
    /// # Returns
    ///
    /// `true` if the year is a leap year according to the 33-year cycle, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// assert!(ParsiDate::is_persian_leap_year(1403)); // 1403 % 33 = 5
    /// assert!(!ParsiDate::is_persian_leap_year(1404)); // 1404 % 33 = 6
    /// assert!(ParsiDate::is_persian_leap_year(1399)); // 1399 % 33 = 30
    /// assert!(!ParsiDate::is_persian_leap_year(1400)); // 1400 % 33 = 1
    /// assert!(!ParsiDate::is_persian_leap_year(1411)); // 1411 % 33 = 13
    /// assert!(!ParsiDate::is_persian_leap_year(0));
    /// ```
    pub fn is_persian_leap_year(year: i32) -> bool {
        // Years <= 0 are not valid Persian years in this context.
        if year <= 0 {
            return false;
        }
        // Check the remainder when the year is divided by 33 using Euclidean remainder.
        match year.rem_euclid(33) {
            // These remainders correspond to leap years in the cycle.
            1 | 5 | 9 | 13 | 17 | 22 | 26 | 30 => true,
            // All other remainders correspond to common years.
            _ => false,
        }
    }

    /// Determines if a given Gregorian year is a leap year.
    ///
    /// Uses the standard Gregorian calendar rules:
    /// * Divisible by 4, but not by 100, unless also divisible by 400.
    ///
    /// # Arguments
    ///
    /// * `year`: The Gregorian year to check.
    ///
    /// # Returns
    ///
    /// `true` if the year is a leap year, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// assert!(ParsiDate::is_gregorian_leap_year(2000)); // Divisible by 400
    /// assert!(ParsiDate::is_gregorian_leap_year(2024)); // Divisible by 4, not by 100
    /// assert!(!ParsiDate::is_gregorian_leap_year(1900)); // Divisible by 100, not by 400
    /// assert!(!ParsiDate::is_gregorian_leap_year(2023)); // Not divisible by 4
    /// ```
    pub fn is_gregorian_leap_year(year: i32) -> bool {
        // Standard Gregorian leap year rule implementation.
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Returns the number of days in a specific month of a given Persian year.
    ///
    /// Takes into account whether the year is a leap year for the month of Esfand (12).
    /// * Months 1-6 (Farvardin to Shahrivar) have 31 days.
    /// * Months 7-11 (Mehr to Bahman) have 30 days.
    /// * Month 12 (Esfand) has 30 days in a leap year, and 29 days in a common year.
    ///
    /// # Arguments
    ///
    /// * `year`: The Persian year (used to check for leap year if month is 12).
    /// * `month`: The Persian month (1-12).
    ///
    /// # Returns
    ///
    /// The number of days (29, 30, or 31) in the specified month and year.
    /// Returns 0 if the `month` number is invalid (outside 1-12).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// assert_eq!(ParsiDate::days_in_month(1403, 1), 31); // Farvardin
    /// assert_eq!(ParsiDate::days_in_month(1403, 7), 30); // Mehr
    /// assert_eq!(ParsiDate::days_in_month(1403, 12), 30); // Esfand (1403 is leap)
    /// assert_eq!(ParsiDate::days_in_month(1404, 12), 29); // Esfand (1404 is common)
    /// assert_eq!(ParsiDate::days_in_month(1403, 13), 0); // Invalid month
    /// ```
    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1..=6 => 31,  // Farvardin to Shahrivar
            7..=11 => 30, // Mehr to Bahman
            12 => {
                // Esfand: depends on leap year status
                if Self::is_persian_leap_year(year) {
                    30
                } else {
                    29
                }
            }
            // Invalid month number specified
            _ => 0,
        }
    }

    /// Returns an array containing the lengths of the 12 months for a given Persian year.
    ///
    /// This is an internal helper function used by other methods like `from_ordinal`
    /// and `to_gregorian_internal`. The length of the 12th month (Esfand) depends
    /// on whether the given `year` is a leap year.
    ///
    /// # Arguments
    ///
    /// * `year`: The Persian year.
    ///
    /// # Returns
    ///
    /// An array `[u32; 12]` where index 0 is the length of Farvardin, ..., index 11 is the length of Esfand.
    // Keep internal? Yes, seems like an implementation detail. Make pub(crate).
    pub(crate) fn month_lengths(year: i32) -> [u32; 12] {
        [
            31, // 1: Farvardin
            31, // 2: Ordibehesht
            31, // 3: Khordad
            31, // 4: Tir
            31, // 5: Mordad
            31, // 6: Shahrivar
            30, // 7: Mehr
            30, // 8: Aban
            30, // 9: Azar
            30, // 10: Dey
            30, // 11: Bahman
            // 12: Esfand - length depends on whether the year is leap
            if Self::is_persian_leap_year(year) {
                30
            } else {
                29
            },
        ]
    }

    // --- Formatting ---

    /// Formats the `ParsiDate` into a string based on predefined styles or a custom pattern.
    ///
    /// # Arguments
    ///
    /// * `style_or_pattern`: A string specifying the desired format. Can be one of:
    ///     * `"short"`: Formats as "YYYY/MM/DD" (e.g., "1403/05/02"). This is the default for `Display`.
    ///     * `"long"`: Formats as "D Month YYYY" (e.g., "2 مرداد 1403"). Note: Day is *not* zero-padded.
    ///     * `"iso"`: Formats as "YYYY-MM-DD" (e.g., "1403-05-02").
    ///     * Custom `strftime`-like pattern: Any other string is treated as a format pattern.
    ///       See [`format_strftime`](#method.format_strftime) for supported specifiers.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted date. If the `ParsiDate` instance is invalid
    /// (e.g., created via `unsafe`), the output for some format specifiers might indicate
    /// an error (like "?InvalidMonth?").
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// assert_eq!(date.format("short"), "1403/05/02");
    /// assert_eq!(date.format("long"), "2 مرداد 1403");
    /// assert_eq!(date.format("iso"), "1403-05-02");
    /// assert_eq!(date.format("%Y, %B %d"), "1403, مرداد 02"); // Custom pattern
    /// assert_eq!(date.to_string(), "1403/05/02"); // Display trait uses "short"
    /// ```
    pub fn format(&self, style_or_pattern: &str) -> String {
        match style_or_pattern {
            "short" => format!("{}/{:02}/{:02}", self.year, self.month, self.day),
            "long" => format!(
                // Note: Day 'D' is NOT zero-padded in the "long" format specification.
                "{} {} {}",
                self.day,
                // Get month name safely, using saturating_sub and get to handle potential invalid month values gracefully.
                MONTH_NAMES_PERSIAN
                    .get((self.month.saturating_sub(1)) as usize)
                    .unwrap_or(&"?InvalidMonth?"), // Provide fallback for invalid index
                self.year
            ),
            // ISO 8601 format: YYYY-MM-DD (e.g., "1404-01-07")
            "iso" => format!("{}-{:02}-{:02}", self.year, self.month, self.day),
            // Any other string is treated as a custom format pattern.
            pattern => self.format_strftime(pattern),
        }
    }

    /// Formats the date according to `strftime`-like specifiers.
    ///
    /// This method is called internally by `format` when a custom pattern is provided.
    ///
    /// # Supported Format Specifiers:
    ///
    /// *   `%Y`: The full Persian year (e.g., 1403).
    /// *   `%m`: The Persian month number, zero-padded (01-12).
    /// *   `%d`: The Persian day of the month, zero-padded (01-31).
    /// *   `%B`: The full Persian month name (e.g., "فروردین", "مرداد").
    /// *   `%A`: The full Persian weekday name (e.g., "شنبه", "سه‌شنبه").
    /// *   `%w`: The weekday number (Saturday=0, Sunday=1, ..., Friday=6).
    /// *   `%j`: The day of the year (ordinal day), zero-padded (001-366).
    /// *   `%%`: A literal percent sign (`%`).
    ///
    /// Any characters in the pattern that are not part of a recognized specifier are included literally
    /// in the output string. Unrecognized specifiers (e.g., `%x`) are also output literally.
    ///
    /// # Arguments
    ///
    /// * `pattern`: The format string containing literal characters and format specifiers.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted date according to the pattern.
    /// If the `ParsiDate` instance is invalid, or if calculations like weekday/ordinal fail,
    /// placeholders like "?InvalidMonth?", "?", "???" may appear in the output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 1, 7).unwrap(); // Tue 2024-03-26
    /// assert_eq!(date.format_strftime("%Y-%m-%d is a %A (day %j)"), "1403-01-07 is a سه‌شنبه (day 007)");
    /// assert_eq!(date.format_strftime("Date: %d %B, %Y %%"), "Date: 07 فروردین, 1403 %");
    /// ```
    pub fn format_strftime(&self, pattern: &str) -> String {
        // Preallocate string with a reasonable estimate capacity.
        let mut result = String::with_capacity(pattern.len() + 10);
        // Use iterator over characters for Unicode safety.
        let mut chars = pattern.chars().peekable();

        // Caching results for performance if the same specifier is used multiple times in one pattern.
        // Cache the Result to handle potential errors during calculation once.
        let mut weekday_cache: Option<Result<String, DateError>> = None;
        let mut ordinal_cache: Option<Result<u32, DateError>> = None;
        let mut weekday_num_cache: Option<Result<u32, DateError>> = None;

        while let Some(c) = chars.next() {
            if c == '%' {
                // Check the character following the '%'
                match chars.next() {
                    // Literal '%'
                    Some('%') => result.push('%'),
                    // Year
                    Some('Y') => result.push_str(&self.year.to_string()),
                    // Month (zero-padded)
                    Some('m') => result.push_str(&format!("{:02}", self.month)),
                    // Day (zero-padded)
                    Some('d') => result.push_str(&format!("{:02}", self.day)),
                    // Month name
                    Some('B') => {
                        // Access month name safely using get() with 0-based index.
                        if let Some(name) =
                            MONTH_NAMES_PERSIAN.get((self.month.saturating_sub(1)) as usize)
                        {
                            result.push_str(name);
                        } else {
                            result.push_str("?InvalidMonth?"); // Handle invalid month value in date
                        }
                    }
                    // Weekday name
                    Some('A') => {
                        // Compute (or retrieve from cache) the weekday name.
                        if weekday_cache.is_none() {
                            weekday_cache = Some(self.weekday_internal()); // Use internal fn returning Result
                        }
                        // Handle the cached Result.
                        match weekday_cache.as_ref().unwrap() {
                            // Safe unwrap as we just set it if None
                            Ok(name) => result.push_str(name),
                            Err(_) => result.push_str("?WeekdayError?"), // Indicate calculation error
                        }
                    }
                    // Weekday number (Sat=0..Fri=6)
                    Some('w') => {
                        if weekday_num_cache.is_none() {
                            weekday_num_cache = Some(self.weekday_num_sat_0()); // Calculate if not cached
                        }
                        match weekday_num_cache.as_ref().unwrap() {
                            Ok(num) => result.push_str(&num.to_string()),
                            Err(_) => result.push('?'), // Indicate calculation error
                        }
                    }
                    // Ordinal day (zero-padded)
                    Some('j') => {
                        if ordinal_cache.is_none() {
                            ordinal_cache = Some(self.ordinal_internal()); // Calculate if not cached
                        }
                        match ordinal_cache.as_ref().unwrap() {
                            Ok(ord) => result.push_str(&format!("{:03}", ord)),
                            Err(_) => result.push_str("???"), // Indicate calculation error
                        }
                    }
                    // Optional: Add %e for space-padded day (common in some strftime implementations)
                    // Some('e') => result.push_str(&format!("{:>2}", self.day)), // Right-align with space padding

                    // Unrecognized specifier - output literally
                    Some(other) => {
                        result.push('%');
                        result.push(other);
                    }
                    // Dangling '%' at the end of the pattern string
                    None => {
                        result.push('%');
                        break; // End of pattern reached unexpectedly after %
                    }
                }
            } else {
                // Literal character, push directly to result
                result.push(c);
            }
        }
        result
    }

    // --- Parsing ---

    /// Parses a string into a `ParsiDate` using a specified format pattern.
    ///
    /// This function attempts to match the input string `s` against the `format` pattern.
    /// It requires an exact match, including separators and padding as specified.
    /// Whitespace in the format string matches literal whitespace in the input.
    ///
    /// # Supported Format Specifiers for Parsing:
    ///
    /// *   `%Y`: Parses a 4-digit Persian year.
    /// *   `%m`: Parses a 2-digit Persian month (01-12).
    /// *   `%d`: Parses a 2-digit Persian day (01-31).
    /// *   `%B`: Parses a full Persian month name (e.g., "مرداد", "اسفند"). Case-sensitive.
    /// *   `%%`: Matches a literal percent sign (`%`) in the input.
    ///
    /// # Arguments
    ///
    /// * `s`: The input string slice to parse.
    /// * `format`: The format string containing literal characters and supported specifiers.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::ParseError(kind))` where `kind` indicates the failure:
    /// * `ParseErrorKind::FormatMismatch`: Input string doesn't match the format structure,
    ///   separators, has trailing characters, or is shorter than expected.
    /// * `ParseErrorKind::InvalidNumber`: Failed to parse `%Y`, `%m`, or `%d` as a number,
    ///   or they didn't have the required number of digits (4 for `%Y`, 2 for `%m`/`%d`).
    /// * `ParseErrorKind::InvalidMonthName`: Failed to match a known Persian month name for `%B`.
    /// * `ParseErrorKind::UnsupportedSpecifier`: An unsupported specifier (like `%A` or `%j`)
    ///   was used in the `format` string.
    /// * `ParseErrorKind::InvalidDateValue`: The components were parsed successfully but formed
    ///   an invalid date (e.g., month 13, day 31 in Mehr, day 30 in Esfand of a common year).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, DateError, ParseErrorKind};
    ///
    /// // Simple parsing
    /// assert_eq!(ParsiDate::parse("1403/05/02", "%Y/%m/%d"), Ok(ParsiDate::new(1403, 5, 2).unwrap()));
    /// assert_eq!(ParsiDate::parse("1399-12-30", "%Y-%m-%d"), Ok(ParsiDate::new(1399, 12, 30).unwrap()));
    ///
    /// // Parsing with month name (%B requires exact match)
    /// assert_eq!(ParsiDate::parse("02 مرداد 1403", "%d %B %Y"), Ok(ParsiDate::new(1403, 5, 2).unwrap()));
    /// assert_eq!(ParsiDate::parse("10 دی 1400", "%d %B %Y"), Ok(ParsiDate::new(1400, 10, 10).unwrap()));
    ///
    /// // --- Error Cases ---
    /// // Wrong format (separator)
    /// assert_eq!(ParsiDate::parse("1403 05 02", "%Y/%m/%d"), Err(DateError::ParseError(ParseErrorKind::FormatMismatch)));
    /// // Invalid number (single digit day for %d)
    /// assert_eq!(ParsiDate::parse("1403/05/2", "%Y/%m/%d"), Err(DateError::ParseError(ParseErrorKind::InvalidNumber)));
    /// // Invalid month name
    /// assert_eq!(ParsiDate::parse("02 Tirr 1403", "%d %B %Y"), Err(DateError::ParseError(ParseErrorKind::InvalidMonthName)));
    /// // Invalid date value (Esfand 30 in common year)
    /// assert_eq!(ParsiDate::parse("1404/12/30", "%Y/%m/%d"), Err(DateError::ParseError(ParseErrorKind::InvalidDateValue)));
    /// // Unsupported specifier
    /// assert_eq!(ParsiDate::parse("Tuesday 1403", "%A %Y"), Err(DateError::ParseError(ParseErrorKind::UnsupportedSpecifier)));
    /// ```
    pub fn parse(s: &str, format: &str) -> Result<Self, DateError> {
        let mut parsed_year: Option<i32> = None;
        let mut parsed_month: Option<u32> = None;
        let mut parsed_day: Option<u32> = None;

        // Use byte slices for efficient processing, assuming ASCII for format specifiers and digits.
        // Input string `s` can contain UTF-8 (for %B), handled specifically.
        let mut s_bytes = s.as_bytes();
        let mut fmt_bytes = format.as_bytes();

        // Iterate through the format string
        while !fmt_bytes.is_empty() {
            // Check if current format char is '%'
            if fmt_bytes[0] == b'%' {
                // Check for specifier character after '%'
                if fmt_bytes.len() < 2 {
                    // Dangling '%' at end of format string
                    return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                }

                // Match the specifier byte
                match fmt_bytes[1] {
                    // Literal '%%'
                    b'%' => {
                        // Input must also start with '%'
                        if s_bytes.is_empty() || s_bytes[0] != b'%' {
                            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                        }
                        // Consume '%' from both input and format
                        s_bytes = &s_bytes[1..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    // Year '%Y' (expect 4 digits)
                    b'Y' => {
                        if s_bytes.len() < 4 || !s_bytes[0..4].iter().all(|b| b.is_ascii_digit()) {
                            // Not enough chars or not all digits
                            return Err(DateError::ParseError(ParseErrorKind::InvalidNumber));
                        }
                        // Parse the 4 digits as year (unsafe from_utf8_unchecked is safe due to ASCII digit check)
                        let year_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..4]) };
                        parsed_year = Some(year_str.parse().map_err(|_| {
                            // This parse should not fail if the digits were validated, but handle defensively.
                            DateError::ParseError(ParseErrorKind::InvalidNumber)
                        })?);
                        // Consume 4 digits from input and '%Y' from format
                        s_bytes = &s_bytes[4..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    // Month '%m' or Day '%d' (expect exactly 2 digits)
                    b'm' | b'd' => {
                        if s_bytes.len() < 2 || !s_bytes[0..2].iter().all(|b| b.is_ascii_digit()) {
                            // Not enough chars or not exactly 2 digits
                            return Err(DateError::ParseError(ParseErrorKind::InvalidNumber));
                        }
                        // Parse the 2 digits (unsafe from_utf8_unchecked is safe)
                        let num_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..2]) };
                        let val: u32 = num_str
                            .parse()
                            .map_err(|_| DateError::ParseError(ParseErrorKind::InvalidNumber))?;

                        // Store the value in the correct Option
                        if fmt_bytes[1] == b'm' {
                            parsed_month = Some(val);
                        } else {
                            parsed_day = Some(val);
                        }
                        // Consume 2 digits from input and '%m' or '%d' from format
                        s_bytes = &s_bytes[2..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    // Month Name '%B' (expects Persian name)
                    b'B' => {
                        // Consume '%B' from format first
                        fmt_bytes = &fmt_bytes[2..];
                        let mut found_month = false;
                        let mut best_match_len = 0; // Length of the matched month name in bytes
                        let mut matched_month_idx = 0; // 0-based index of the matched month

                        // Need to work with the original string slice for UTF-8 month names
                        // Safety: We operate on byte indices derived from UTF-8 string lengths, which is safe.
                        let current_s = unsafe { std::str::from_utf8_unchecked(s_bytes) };

                        // Iterate through known Persian month names
                        for (idx, month_name) in MONTH_NAMES_PERSIAN.iter().enumerate() {
                            if current_s.starts_with(month_name) {
                                // Found a potential match. Assume it's the correct one for now.
                                best_match_len = month_name.as_bytes().len();
                                matched_month_idx = idx;
                                found_month = true;
                                break; // Use the first match found
                            }
                        }

                        // If no month name was matched at the current position
                        if !found_month {
                            return Err(DateError::ParseError(ParseErrorKind::InvalidMonthName));
                        }

                        // Consume the matched month name (best_match_len bytes) from input `s_bytes`.
                        parsed_month = Some((matched_month_idx + 1) as u32); // Store 1-based month index
                        s_bytes = &s_bytes[best_match_len..];
                        // `fmt_bytes` was already advanced past '%B' above.
                    }
                    // Any other specifier after '%' is unsupported for parsing
                    _ => return Err(DateError::ParseError(ParseErrorKind::UnsupportedSpecifier)),
                }
            } else {
                // Literal character in format string
                // Input must match this literal character
                if s_bytes.is_empty() || s_bytes[0] != fmt_bytes[0] {
                    // Mismatch between input and literal format character
                    return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                }
                // Consume the matched literal character from both input and format
                s_bytes = &s_bytes[1..];
                fmt_bytes = &fmt_bytes[1..];
            }
        } // End while loop through format string

        // After processing the entire format string, check if there's any remaining input.
        // If yes, the input string was longer than the format expected.
        if !s_bytes.is_empty() {
            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
        }

        // Check if all required components (year, month, day) were successfully parsed
        match (parsed_year, parsed_month, parsed_day) {
            (Some(y), Some(m), Some(d)) => {
                // All components parsed, now validate the resulting date logically.
                // Use ParsiDate::new() for this final validation step.
                ParsiDate::new(y, m, d).map_err(|e| {
                    // Distinguish between InvalidDate from ParsiDate::new and other internal errors.
                    match e {
                         DateError::InvalidDate => DateError::ParseError(ParseErrorKind::InvalidDateValue),
                         // Propagate other potential errors from new, though unlikely here.
                         other_err => other_err,
                    }
                })
            }
            // If any component is missing, the input string didn't fully match the format.
            _ => Err(DateError::ParseError(ParseErrorKind::FormatMismatch)),
        }
    }

    // --- Date Information ---

    /// Returns the Persian name of the weekday for this date (e.g., "شنبه", "یکشنبه").
    ///
    /// Calculates the weekday based on the Gregorian equivalent date.
    /// Saturday is considered the first day of the week in the Persian calendar.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the `ParsiDate` instance holds invalid data.
    /// Returns `Err(DateError::GregorianConversionError)` if the conversion to Gregorian fails
    /// during the calculation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// // 2024-07-23 is a Tuesday (سه‌شنبه) -> 1403-05-02
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// assert_eq!(date.weekday(), Ok("سه‌شنبه".to_string()));
    ///
    /// // 2024-03-23 is a Saturday (شنبه) -> 1403-01-04
    /// let date_sat = ParsiDate::new(1403, 1, 4).unwrap();
    /// assert_eq!(date_sat.weekday(), Ok("شنبه".to_string()));
    /// ```
    pub fn weekday(&self) -> Result<String, DateError> {
        self.weekday_internal() // Call the internal implementation
    }

    /// Internal helper for weekday calculation, returns Result.
    /// Assumes self might be invalid, performs check.
    fn weekday_internal(&self) -> Result<String, DateError> {
        // Ensure the date is valid before proceeding.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Get the numerical weekday (Sat=0..Fri=6).
        let day_num_sat_0 = self.weekday_num_sat_0()?;
        // Get the corresponding name from the constant array.
        // The index should be valid (0-6) if weekday_num_sat_0 is correct.
        WEEKDAY_NAMES_PERSIAN
            .get(day_num_sat_0 as usize)
            .map(|s| s.to_string())
            // If get fails (shouldn't happen), map it to a conversion error.
            .ok_or(DateError::GregorianConversionError)
    }

    /// Returns the weekday as a number, where Saturday=0, Sunday=1, ..., Friday=6.
    /// Internal helper function.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if `self` is invalid.
    /// Returns `Err(DateError::GregorianConversionError)` if `to_gregorian_internal` fails.
    fn weekday_num_sat_0(&self) -> Result<u32, DateError> {
        // Ensure the date is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Convert to Gregorian to use chrono's weekday calculation. Use internal to avoid double validation.
        let gregorian_date = self.to_gregorian_internal()?;

        // chrono::Weekday provides Sunday=0, ..., Saturday=6 via num_days_from_sunday().
        let day_num_sun0 = gregorian_date.weekday().num_days_from_sunday(); // 0=Sun, 1=Mon, ..., 6=Sat

        // Map Sunday=0..Saturday=6 to Persian Saturday=0..Friday=6.
        // Sun (0) -> Ekshanbe (1)  => (0 + 1) % 7 = 1
        // Mon (1) -> Doshanbe (2)  => (1 + 1) % 7 = 2
        // Tue (2) -> Seshanbe (3)  => (2 + 1) % 7 = 3
        // Wed (3) -> Chaharshanbe(4)=> (3 + 1) % 7 = 4
        // Thu (4) -> Panjshanbe(5) => (4 + 1) % 7 = 5
        // Fri (5) -> Jomeh (6)     => (5 + 1) % 7 = 6
        // Sat (6) -> Shanbeh (0)   => (6 + 1) % 7 = 0
        let day_num_sat0 = (day_num_sun0 + 1) % 7;

        Ok(day_num_sat0)
    }

    /// Calculates the day number within the year (ordinal day).
    ///
    /// Farvardin 1st is day 1. The result ranges from 1 to 365 (common year)
    /// or 1 to 366 (leap year).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the `ParsiDate` instance holds invalid data.
    /// Returns `Err(DateError::ArithmeticOverflow)` if an internal overflow occurs during summation
    /// (very unlikely with u32 for days in a year).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// assert_eq!(ParsiDate::new(1403, 1, 1).unwrap().ordinal(), Ok(1));
    /// assert_eq!(ParsiDate::new(1403, 2, 1).unwrap().ordinal(), Ok(32)); // After 31 days in Farvardin
    /// assert_eq!(ParsiDate::new(1403, 12, 30).unwrap().ordinal(), Ok(366)); // Last day of leap year
    /// assert_eq!(ParsiDate::new(1404, 12, 29).unwrap().ordinal(), Ok(365)); // Last day of common year
    /// ```
    pub fn ordinal(&self) -> Result<u32, DateError> {
        self.ordinal_internal() // Call the internal implementation
    }

    /// Internal helper for ordinal calculation, returns Result.
    /// Assumes self might be invalid, performs check.
    fn ordinal_internal(&self) -> Result<u32, DateError> {
        // Ensure the date is valid before calculating.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }

        // Get lengths of months for the current year.
        let month_lengths = Self::month_lengths(self.year);
        let mut days: u32 = 0;

        // Sum the lengths of all full months preceding the current month.
        // month is 1-based, loop goes from 0 up to month-2 (inclusive index).
        if self.month > 1 {
            let mut current_sum: u32 = 0;
            // Slice includes months from index 0 up to self.month - 2.
            for m_len in &month_lengths[0..(self.month - 1) as usize] {
                // Use checked_add for safety against potential overflow (unlikely here).
                current_sum = current_sum
                    .checked_add(*m_len)
                    .ok_or(DateError::ArithmeticOverflow)?;
            }
            days = current_sum;
        }

        // Add the day of the current month.
        // day is 1-based, so this gives the correct total ordinal day.
        days = days
            .checked_add(self.day)
            .ok_or(DateError::ArithmeticOverflow)?; // Safety check

        // Result should always be >= 1 since self.day >= 1.
        Ok(days)
    }

    // --- Arithmetic ---

    /// Adds a specified number of days to the date. Handles positive and negative `days`.
    ///
    /// This operation is performed by converting the `ParsiDate` to its Gregorian equivalent,
    /// adding the days using `chrono`, and then converting back to `ParsiDate`.
    ///
    /// # Arguments
    ///
    /// * `days`: The number of days to add. Can be positive to move forward in time,
    ///           or negative to move backward.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid.
    /// Returns `Err(DateError::ArithmeticOverflow)` if the addition/subtraction results in a
    /// Gregorian date outside `chrono`'s representable range, or if the resulting date, when
    /// converted back to Persian, falls outside the supported year range (1-9999).
    /// Returns `Err(DateError::GregorianConversionError)` if the initial conversion to Gregorian
    /// or the final conversion back to Persian fails for reasons other than overflow (e.g., epoch issues).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 12, 28).unwrap(); // Leap year
    /// assert_eq!(date.add_days(1), Ok(ParsiDate::new(1403, 12, 29).unwrap()));
    /// assert_eq!(date.add_days(2), Ok(ParsiDate::new(1403, 12, 30).unwrap()));
    /// assert_eq!(date.add_days(3), Ok(ParsiDate::new(1404, 1, 1).unwrap())); // Cross year boundary
    ///
    /// let date2 = ParsiDate::new(1404, 1, 1).unwrap();
    /// assert_eq!(date2.add_days(-1), Ok(ParsiDate::new(1403, 12, 30).unwrap())); // Subtract day
    /// assert_eq!(date2.add_days(-366), Ok(ParsiDate::new(1403, 1, 1).unwrap())); // Subtract leap year days
    /// ```
    pub fn add_days(&self, days: i64) -> Result<Self, DateError> {
        // Ensure the starting date is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }

        // Convert to Gregorian to perform arithmetic.
        let gregorian_equiv = self.to_gregorian_internal()?;

        // Use chrono's checked arithmetic for adding/subtracting days.
        let new_gregorian = if days >= 0 {
            // Add positive number of days.
            gregorian_equiv.checked_add_days(chrono::Days::new(days as u64))
        } else {
            // Subtract days (add negative). Convert negative i64 to positive u64 for subtraction.
            // Use checked_abs to handle potential i64::MIN overflow if needed, although days=i64::MIN is extreme.
            let days_to_sub = days.checked_abs().ok_or(DateError::ArithmeticOverflow)? as u64;
            gregorian_equiv.checked_sub_days(chrono::Days::new(days_to_sub))
        }
        .ok_or(DateError::ArithmeticOverflow)?; // Map chrono's None result (overflow/invalid) to our error type.

        // Convert the resulting Gregorian date back to ParsiDate.
        // This also handles checks for the supported Persian year range.
        Self::from_gregorian(new_gregorian)
    }

    /// Subtracts a specified number of days from the date.
    ///
    /// Equivalent to `add_days(-days)`. `days` must be non-negative.
    ///
    /// # Arguments
    ///
    /// * `days`: The non-negative number of days to subtract.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid.
    /// Returns `Err(DateError::ArithmeticOverflow)` if `days` is too large to be represented
    /// as a negative `i64`, or if the subtraction results in a date outside the representable range.
    /// Returns `Err(DateError::GregorianConversionError)` if conversion issues occur.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1404, 1, 3).unwrap();
    /// assert_eq!(date.sub_days(1), Ok(ParsiDate::new(1404, 1, 2).unwrap()));
    /// assert_eq!(date.sub_days(2), Ok(ParsiDate::new(1404, 1, 1).unwrap()));
    /// assert_eq!(date.sub_days(3), Ok(ParsiDate::new(1403, 12, 30).unwrap())); // Cross year boundary (1403 leap)
    /// ```
    pub fn sub_days(&self, days: u64) -> Result<Self, DateError> {
        // Convert u64 to negative i64 for add_days.
        // Check if the u64 value fits within the positive range of i64 before negating.
        // Negating i64::MIN is undefined behavior, but u64 can represent i64::MAX + 1 up to u64::MAX.
        if days > i64::MAX as u64 {
            // A number of days larger than i64::MAX is practically astronomical and likely leads to overflow anyway.
            return Err(DateError::ArithmeticOverflow);
        }
        // Safely negate the value (which is now known to be <= i64::MAX).
        let days_neg = -(days as i64);
        // Call add_days with the negative value.
        self.add_days(days_neg)
    }

    /// Adds a specified number of months to the date. Handles positive and negative `months_to_add`.
    ///
    /// If the resulting month has fewer days than the original day component,
    /// the day is clamped to the last day of the target month.
    ///
    /// # Arguments
    ///
    /// * `months_to_add`: The number of months to add. Can be positive or negative.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid.
    /// Returns `Err(DateError::ArithmeticOverflow)` if the calculation results in a year
    /// outside the supported range (1-9999) or causes internal integer overflow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 1, 31).unwrap(); // End of Farvardin (31 days)
    /// // Add 1 month -> Ordibehesht (31 days) -> 1403/02/31
    /// assert_eq!(date.add_months(1), Ok(ParsiDate::new(1403, 2, 31).unwrap()));
    /// // Add 6 months -> Mehr (30 days) -> Day clamped from 31 to 30 -> 1403/07/30
    /// assert_eq!(date.add_months(6), Ok(ParsiDate::new(1403, 7, 30).unwrap()));
    /// // Add 12 months -> Farvardin next year -> 1404/01/31
    /// assert_eq!(date.add_months(12), Ok(ParsiDate::new(1404, 1, 31).unwrap()));
    ///
    /// let date2 = ParsiDate::new(1404, 1, 1).unwrap();
    /// // Subtract 1 month -> Esfand previous year (1403 is leap, 30 days) -> 1403/12/01
    /// assert_eq!(date2.add_months(-1), Ok(ParsiDate::new(1403, 12, 1).unwrap()));
    ///
    /// let date3 = ParsiDate::new(1403, 12, 30).unwrap(); // End of Esfand (leap)
    /// // Subtract 1 month -> Bahman (30 days) -> Day remains 30 -> 1403/11/30
    /// assert_eq!(date3.sub_months(1), Ok(ParsiDate::new(1403, 11, 30).unwrap()));
    /// ```
    pub fn add_months(&self, months_to_add: i32) -> Result<Self, DateError> {
        // Ensure the starting date is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // If adding zero months, return the original date.
        if months_to_add == 0 {
            return Ok(*self);
        }

        // Calculate the total number of months from the start of the era (or a relative baseline).
        // Work with 0-indexed months (0=Farvardin..11=Esfand) for easier modulo arithmetic.
        let current_year = self.year;
        let current_month0 = self.month as i32 - 1; // 0..11

        // Calculate the absolute month index if we flattened the calendar from year 0.
        let total_months_abs =
            (current_year as i64 * 12) + current_month0 as i64 + months_to_add as i64;
        // Check if this absolute month count could lead to year overflow (e.g., year > 9999 or < 1).
        // Target Year = floor(total_months_abs / 12)
        // Target Month Index = total_months_abs % 12
        let target_year_abs = total_months_abs.div_euclid(12);
        let target_month0 = total_months_abs.rem_euclid(12); // Resulting month index (0..11)

        // Check if target year is within our i32 and supported range.
        if target_year_abs < MIN_PARSI_DATE.year as i64
            || target_year_abs > MAX_PARSI_DATE.year as i64
        {
            return Err(DateError::ArithmeticOverflow);
        }
        let target_year = target_year_abs as i32;
        let target_month = (target_month0 + 1) as u32; // Convert back to 1-based month (1..12)

        // Determine the maximum valid day in the target month and year.
        let max_days_in_target_month = Self::days_in_month(target_year, target_month);
        // This check should be redundant if target_month calculation is correct (always 1-12).
        if max_days_in_target_month == 0 {
            // This could happen if days_in_month received an invalid month (e.g., 0 or 13)
            // due to a logic error in the calculation above. Treat as internal error -> InvalidDate.
            return Err(DateError::InvalidDate); // Should not happen ideally
        }

        // Clamp the day: use the original day or the max valid day, whichever is smaller.
        let target_day = self.day.min(max_days_in_target_month);

        // Use new() for final validation (primarily year range, month/day should be valid by logic).
        ParsiDate::new(target_year, target_month, target_day)
    }

    /// Subtracts a specified number of months from the date.
    ///
    /// Equivalent to `add_months(-months)`. `months_to_sub` must be non-negative.
    /// Clamps day if necessary.
    ///
    /// # Arguments
    ///
    /// * `months_to_sub`: The non-negative number of months to subtract.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid.
    /// Returns `Err(DateError::ArithmeticOverflow)` if `months_to_sub` is too large (exceeds `i32::MAX`)
    /// or if the calculation results in a year outside the supported range (1-9999).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 7, 30).unwrap(); // End of Mehr (30 days)
    /// // Subtract 1 month -> Shahrivar (31 days) -> Day remains 30 -> 1403/06/30
    /// assert_eq!(date.sub_months(1), Ok(ParsiDate::new(1403, 6, 30).unwrap()));
    ///
    /// let date2 = ParsiDate::new(1404, 2, 29).unwrap(); // Ordibehesht (31 days) in common year
    /// // Subtract 2 months -> Esfand previous year (1403 is leap, 30 days) -> Day remains 29 -> 1403/12/29
    /// assert_eq!(date2.sub_months(2), Ok(ParsiDate::new(1403, 12, 29).unwrap()));
    /// ```
    pub fn sub_months(&self, months_to_sub: u32) -> Result<Self, DateError> {
        // Check for potential overflow before negation: u32 max > i32 max.
        if months_to_sub > i32::MAX as u32 {
            return Err(DateError::ArithmeticOverflow);
        }
        // Negate and call add_months.
        self.add_months(-(months_to_sub as i32))
    }

    /// Adds a specified number of years to the date. Handles positive and negative `years_to_add`.
    ///
    /// Special handling for leap day: If the original date is Esfand 30th (a leap day),
    /// and the target year is not a leap year, the resulting date will be clamped to Esfand 29th.
    ///
    /// # Arguments
    ///
    /// * `years_to_add`: The number of years to add. Can be positive or negative.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid.
    /// Returns `Err(DateError::ArithmeticOverflow)` if the calculation results in a year
    /// outside the supported range (1-9999) or causes internal integer overflow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1400, 5, 10).unwrap(); // 1400 is common
    /// assert_eq!(date.add_years(3), Ok(ParsiDate::new(1403, 5, 10).unwrap())); // 1403 is leap
    /// assert_eq!(date.add_years(-1), Ok(ParsiDate::new(1399, 5, 10).unwrap())); // 1399 is leap
    ///
    /// let leap_day = ParsiDate::new(1403, 12, 30).unwrap(); // Esfand 30 on leap year
    /// // Add 1 year -> 1404 (common year) -> Day clamped to 29 -> 1404/12/29
    /// assert_eq!(leap_day.add_years(1), Ok(ParsiDate::new(1404, 12, 29).unwrap()));
    /// // Add 4 years -> 1407 (common year) -> Day remains 29 -> 1407/12/29
    /// assert_eq!(leap_day.add_years(4), Ok(ParsiDate::new(1407, 12, 29).unwrap())); // 1407 is common
    ///
    /// // Subtract 1 year from leap day -> 1402 (common year) -> Day clamped to 29 -> 1402/12/29
    /// assert_eq!(leap_day.sub_years(1), Ok(ParsiDate::new(1402, 12, 29).unwrap()));
    /// ```
    pub fn add_years(&self, years_to_add: i32) -> Result<Self, DateError> {
        // Ensure the starting date is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // If adding zero years, return the original date.
        if years_to_add == 0 {
            return Ok(*self);
        }

        // Calculate the target year using checked addition.
        let target_year = self
            .year
            .checked_add(years_to_add)
            .ok_or(DateError::ArithmeticOverflow)?;

        // Validate the target year is within the supported range.
        if target_year < MIN_PARSI_DATE.year || target_year > MAX_PARSI_DATE.year {
            return Err(DateError::ArithmeticOverflow); // Year out of range [1, 9999]
        }

        // Handle the leap day adjustment:
        // If the original date is Esfand 30 (only possible in a leap year),
        // and the target year is *not* a leap year, clamp the day to 29.
        let mut target_day = self.day;
        if self.month == 12 && self.day == 30 && !Self::is_persian_leap_year(target_year) {
            target_day = 29;
        }

        // Use new() for final validation. Month remains the same. Day might be adjusted.
        // new() will ensure the adjusted day (29) is valid for Esfand in the target year.
        ParsiDate::new(target_year, self.month, target_day)
    }

    /// Subtracts a specified number of years from the date.
    ///
    /// Equivalent to `add_years(-years)`. `years_to_sub` must be non-negative.
    /// Adjusts day for leap day (Esfand 30th) if necessary.
    ///
    /// # Arguments
    ///
    /// * `years_to_sub`: The non-negative number of years to subtract.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid.
    /// Returns `Err(DateError::ArithmeticOverflow)` if `years_to_sub` is too large (exceeds `i32::MAX`)
    /// or if the calculation results in a year outside the supported range (1-9999).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 10).unwrap(); // Leap year
    /// assert_eq!(date.sub_years(1), Ok(ParsiDate::new(1402, 5, 10).unwrap())); // To common year
    /// assert_eq!(date.sub_years(4), Ok(ParsiDate::new(1399, 5, 10).unwrap())); // To leap year
    ///
    /// let leap_day = ParsiDate::new(1403, 12, 30).unwrap();
    /// // Subtract 1 year -> 1402 (common) -> Clamp day to 29 -> 1402/12/29
    /// assert_eq!(leap_day.sub_years(1), Ok(ParsiDate::new(1402, 12, 29).unwrap()));
    /// ```
    pub fn sub_years(&self, years_to_sub: u32) -> Result<Self, DateError> {
        // Check for potential overflow before negation.
        if years_to_sub > i32::MAX as u32 {
            return Err(DateError::ArithmeticOverflow);
        }
        // Negate and call add_years.
        self.add_years(-(years_to_sub as i32))
    }

    /// Calculates the absolute difference in days between this `ParsiDate` and another `ParsiDate`.
    ///
    /// This is done by converting both dates to their Gregorian equivalents and calculating
    /// the difference between them.
    ///
    /// # Arguments
    ///
    /// * `other`: The other `ParsiDate` to compare against.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if either `self` or `other` is invalid.
    /// Returns `Err(DateError::GregorianConversionError)` if the conversion of either date
    /// to Gregorian fails.
    ///
    /// # Returns
    ///
    /// The absolute number of days between the two dates as an `i64`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let d1 = ParsiDate::new(1403, 1, 1).unwrap();
    /// let d2 = ParsiDate::new(1403, 1, 11).unwrap();
    /// assert_eq!(d1.days_between(&d2), Ok(10));
    /// assert_eq!(d2.days_between(&d1), Ok(10)); // Absolute difference
    ///
    /// let d3 = ParsiDate::new(1404, 1, 1).unwrap(); // Next year (1403 is leap)
    /// assert_eq!(d1.days_between(&d3), Ok(366));
    /// ```
    pub fn days_between(&self, other: &ParsiDate) -> Result<i64, DateError> {
        // Ensure both dates are valid before proceeding.
        if !self.is_valid() || !other.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Convert both dates to Gregorian.
        let g1 = self.to_gregorian_internal()?; // Use internal conversion as validity is checked.
        let g2 = other.to_gregorian_internal()?;
        // Calculate the signed duration using chrono and return the absolute number of days.
        Ok(g1.signed_duration_since(g2).num_days().abs())
    }

    // --- Helper Methods ---

    /// Creates a new `ParsiDate` with the year component modified.
    ///
    /// Adjusts the day to 29 if the original date was Esfand 30th (leap day)
    /// and the target `year` is not a leap year.
    ///
    /// # Arguments
    ///
    /// * `year`: The desired year for the new date (1-9999).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid,
    /// or if the target `year` is outside the supported range (1-9999), or if the
    /// resulting date (after potential day clamping) is somehow invalid (should not happen
    /// if target year is valid).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// assert_eq!(date.with_year(1404), Ok(ParsiDate::new(1404, 5, 2).unwrap()));
    ///
    /// let leap_day = ParsiDate::new(1403, 12, 30).unwrap();
    /// assert_eq!(leap_day.with_year(1404), Ok(ParsiDate::new(1404, 12, 29).unwrap())); // Day clamped
    /// assert_eq!(leap_day.with_year(1407), Ok(ParsiDate::new(1407, 12, 29).unwrap())); // Leap to leap (1407 is common)
    ///
    /// assert!(date.with_year(0).is_err()); // Invalid target year
    /// ```
    pub fn with_year(&self, year: i32) -> Result<Self, DateError> {
        // Ensure the starting date is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Validate the target year range immediately.
        if year < MIN_PARSI_DATE.year || year > MAX_PARSI_DATE.year {
            // Using InvalidDate error for out-of-range years for simplicity.
            return Err(DateError::InvalidDate);
        }

        // Check if leap day adjustment is needed.
        let mut day = self.day;
        if self.month == 12 && self.day == 30 && !Self::is_persian_leap_year(year) {
            // Original is Esfand 30 (must be leap), target year is not leap. Clamp day.
            day = 29;
        }

        // Use new() for final validation. It ensures the combination is valid.
        ParsiDate::new(year, self.month, day)
    }

    /// Creates a new `ParsiDate` with the month component modified.
    ///
    /// If the original day component is invalid for the target `month` in the same year
    /// (e.g., changing from Farvardin 31st to Mehr), the day is clamped to the
    /// last valid day of the target `month`.
    ///
    /// # Arguments
    ///
    /// * `month`: The desired month for the new date (1-12).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid,
    /// or if the target `month` is outside the range (1-12).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 1, 31).unwrap(); // Farvardin 31
    /// assert_eq!(date.with_month(2), Ok(ParsiDate::new(1403, 2, 31).unwrap())); // To Ordibehesht (31 days)
    /// assert_eq!(date.with_month(7), Ok(ParsiDate::new(1403, 7, 30).unwrap())); // To Mehr (30 days, clamped)
    ///
    /// let date2 = ParsiDate::new(1404, 7, 15).unwrap(); // Mehr 15 (common year)
    /// assert_eq!(date2.with_month(12), Ok(ParsiDate::new(1404, 12, 15).unwrap())); // To Esfand (29 days)
    ///
    /// assert!(date.with_month(0).is_err()); // Invalid target month
    /// assert!(date.with_month(13).is_err());
    /// ```
    pub fn with_month(&self, month: u32) -> Result<Self, DateError> {
        // Ensure the starting date is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Validate the target month range immediately.
        if month == 0 || month > 12 {
            return Err(DateError::InvalidDate); // Invalid target month number
        }

        // Determine the maximum valid day for the target month in the current year.
        let max_days = Self::days_in_month(self.year, month);
        // This check should be redundant if month is 1-12, as days_in_month returns > 0.
        if max_days == 0 {
            return Err(DateError::InvalidDate); // Should not happen
        }

        // Clamp the original day to the maximum allowed day of the target month.
        let day = self.day.min(max_days);

        // Use new() for final validation. Year remains the same.
        ParsiDate::new(self.year, month, day)
    }

    /// Creates a new `ParsiDate` with the day component modified.
    ///
    /// # Arguments
    ///
    /// * `day`: The desired day for the new date (1-31).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the starting `ParsiDate` is invalid,
    /// or if the target `day` is 0 or greater than the number of days in the
    /// current month and year.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// let date = ParsiDate::new(1403, 7, 1).unwrap(); // Mehr 1st (30 days)
    /// assert_eq!(date.with_day(15), Ok(ParsiDate::new(1403, 7, 15).unwrap()));
    /// assert_eq!(date.with_day(30), Ok(ParsiDate::new(1403, 7, 30).unwrap()));
    /// assert_eq!(date.with_day(31), Err(DateError::InvalidDate)); // Invalid day for Mehr
    /// assert_eq!(date.with_day(0), Err(DateError::InvalidDate)); // Day 0 is invalid
    /// ```
    pub fn with_day(&self, day: u32) -> Result<Self, DateError> {
        // Ensure the starting date is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Basic check for day > 0. The upper bound check is handled by ParsiDate::new.
        if day == 0 {
            return Err(DateError::InvalidDate);
        }

        // Let ParsiDate::new perform the full validation (checks day <= days_in_month).
        ParsiDate::new(self.year, self.month, day)
    }

    /// Returns the date of the first day of the month for the current date.
    ///
    /// Creates a new `ParsiDate` with the same year and month, but with the day set to 1.
    /// Assumes `self` is already a valid date.
    ///
    /// # Safety
    ///
    /// This method uses `unsafe { ParsiDate::new_unchecked }` internally for performance,
    /// relying on the assumption that `self` is valid. Day 1 is always valid for any valid
    /// month (1-12) and year (1-9999). A `debug_assert!` is included.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 15).unwrap();
    /// assert_eq!(date.first_day_of_month(), ParsiDate::new(1403, 5, 1).unwrap());
    /// ```
    #[inline]
    pub fn first_day_of_month(&self) -> Self {
        // Ensure self is valid in debug builds.
        debug_assert!(self.is_valid(), "first_day_of_month called on invalid date");
        // Safety: Day 1 is always valid for the (assumed valid) self.month and self.year.
        unsafe { ParsiDate::new_unchecked(self.year, self.month, 1) }
    }

    /// Returns the date of the last day of the month for the current date.
    ///
    /// Calculates the last day based on the month and whether the year is a leap year.
    /// Assumes `self` is already a valid date.
    ///
    /// # Safety
    ///
    /// This method uses `unsafe { ParsiDate::new_unchecked }` internally for performance.
    /// It relies on `days_in_month` returning the correct last day for the valid `self.year`
    /// and `self.month`. A `debug_assert!` is included.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 15).unwrap(); // Mordad (31 days)
    /// assert_eq!(date.last_day_of_month(), ParsiDate::new(1403, 5, 31).unwrap());
    ///
    /// let date_mehr = ParsiDate::new(1403, 7, 10).unwrap(); // Mehr (30 days)
    /// assert_eq!(date_mehr.last_day_of_month(), ParsiDate::new(1403, 7, 30).unwrap());
    ///
    /// let date_esfand_leap = ParsiDate::new(1403, 12, 5).unwrap(); // Esfand (leap year, 30 days)
    /// assert_eq!(date_esfand_leap.last_day_of_month(), ParsiDate::new(1403, 12, 30).unwrap());
    ///
    /// let date_esfand_common = ParsiDate::new(1404, 12, 5).unwrap(); // Esfand (common year, 29 days)
    /// assert_eq!(date_esfand_common.last_day_of_month(), ParsiDate::new(1404, 12, 29).unwrap());
    /// ```
    #[inline]
    pub fn last_day_of_month(&self) -> Self {
        // Ensure self is valid in debug builds.
        debug_assert!(self.is_valid(), "last_day_of_month called on invalid date");
        // Calculate the last day of the current month/year.
        let last_day = Self::days_in_month(self.year, self.month);
        // Safety: days_in_month returns a valid day number (29, 30, or 31) for the valid self.month/self.year.
        unsafe { ParsiDate::new_unchecked(self.year, self.month, last_day) }
    }

    /// Returns the date of the first day of the year (Farvardin 1st) for the current date's year.
    ///
    /// Creates a new `ParsiDate` with the same year, but month set to 1 and day set to 1.
    /// Assumes `self` is already a valid date.
    ///
    /// # Safety
    ///
    /// Uses `unsafe { ParsiDate::new_unchecked }`. Relies on the assumption that `self` is valid,
    /// meaning `self.year` is valid. Month 1, Day 1 is always valid for any valid year.
    /// A `debug_assert!` is included.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 15).unwrap();
    /// assert_eq!(date.first_day_of_year(), ParsiDate::new(1403, 1, 1).unwrap());
    /// ```
    #[inline]
    pub fn first_day_of_year(&self) -> Self {
        // Ensure self is valid in debug builds.
        debug_assert!(self.is_valid(), "first_day_of_year called on invalid date");
        // Safety: Month 1, Day 1 is always valid for the (assumed valid) self.year.
        unsafe { ParsiDate::new_unchecked(self.year, 1, 1) }
    }

    /// Returns the date of the last day of the year (Esfand 29th or 30th) for the current date's year.
    ///
    /// Calculates the last day (29 or 30) based on whether the year is a leap year.
    /// Assumes `self` is already a valid date.
    ///
    /// # Safety
    ///
    /// Uses `unsafe { ParsiDate::new_unchecked }`. Relies on `is_persian_leap_year` correctly
    /// determining the last day (29 or 30), which is always valid for month 12 (Esfand)
    /// in the valid `self.year`. A `debug_assert!` is included.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// // Leap year
    /// let date_leap = ParsiDate::new(1403, 5, 15).unwrap();
    /// assert_eq!(date_leap.last_day_of_year(), ParsiDate::new(1403, 12, 30).unwrap());
    ///
    /// // Common year
    /// let date_common = ParsiDate::new(1404, 7, 10).unwrap();
    /// assert_eq!(date_common.last_day_of_year(), ParsiDate::new(1404, 12, 29).unwrap());
    /// ```
    #[inline]
    pub fn last_day_of_year(&self) -> Self {
        // Ensure self is valid in debug builds.
        debug_assert!(self.is_valid(), "last_day_of_year called on invalid date");
        // Determine the last day based on leap year status.
        let last_day = if Self::is_persian_leap_year(self.year) {
            30
        } else {
            29
        };
        // Safety: Month 12 is valid, and last_day (29 or 30) is the valid last day for month 12
        // in the (assumed valid) self.year.
        unsafe { ParsiDate::new_unchecked(self.year, 12, last_day) }
    }
} // <<<=== *** MISSING BRACE WAS HERE ***

// --- Trait Implementations ---

/// Implements the `Display` trait for `ParsiDate`.
///
/// Formats the date using the default "short" style: "YYYY/MM/DD".
///
/// Note: This formatting assumes the `ParsiDate` instance is valid. If an invalid date
/// (e.g., created via `unsafe`) is displayed, the output might show the invalid components
/// directly (e.g., "1404/12/30").
impl fmt::Display for ParsiDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the "short" format: YYYY/MM/DD with zero-padding for month and day.
        write!(f, "{}/{:02}/{:02}", self.year, self.month, self.day)
    }
}
