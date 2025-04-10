//! ~/src/date.rs
//
//  * Copyright (C) Mohammad (Sina) Jalalvandi 2024-2025 <jalalvandi.sina@gmail.com>
//  * Package : parsidate
//  * License : Apache-2.0
//  * Version : 1.4.0
//  * URL     : https://github.com/jalalvandi/parsidate
//  * Sign    : eb1f0cae-a178-41e5-b109-47f208e77913
//
//! Contains the `ParsiDate` struct definition and its implementation for handling

//! dates within the Persian (Jalali or Shamsi) calendar system.

// Use necessary items from other modules and external crates
use crate::constants::{
    MAX_PARSI_DATE, MIN_PARSI_DATE, MONTH_NAMES_PERSIAN, WEEKDAY_NAMES_PERSIAN,
};
use crate::error::{DateError, ParseErrorKind};
use chrono::{Datelike, /* Days , */ NaiveDate}; // Added Days for arithmetic
use std::fmt;
// use std::ops::{Add, Sub}; // For potential future Duration addition
// use std::str::FromStr; // For potential future direct FromStr impl

// --- Data Structures ---

/// Represents a specific date in the Persian (Jalali or Shamsi) calendar system.
///
/// This struct stores the `year`, `month` (1-12), and `day` (1-31) components.
/// It provides a range of functionalities including:
/// *   Validation of date components.
/// *   Conversion to and from Gregorian [`NaiveDate`].
/// *   Formatting the date into various string representations.
/// *   Parsing strings into `ParsiDate` instances.
/// *   Date arithmetic (adding/subtracting days, months, years).
/// *   Querying date properties like weekday, ordinal day, leap year status, etc.
///
/// **Supported Range:** The struct supports Persian years from 1 up to 9999, inclusive.
/// Operations resulting in dates outside this range will typically return an error.
///
/// **Serialization:** If the `serde` feature is enabled, this struct derives `Serialize` and `Deserialize`.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParsiDate {
    /// The year component of the Persian date (e.g., 1403). Must be between 1 and 9999 inclusive.
    pub(crate) year: i32,
    /// The month component of the Persian date (1 = Farvardin, ..., 12 = Esfand). Must be between 1 and 12 inclusive.
    pub(crate) month: u32,
    /// The day component of the Persian date (1-29/30/31). Must be valid for the given month and year, considering leap years.
    pub(crate) day: u32,
}

// --- Core Implementation ---

impl ParsiDate {
    // --- Constructors and Converters ---

    /// Creates a new `ParsiDate` instance from individual year, month, and day components.
    ///
    /// This constructor performs validation to ensure the provided components form a valid
    /// date within the Persian calendar system and the supported range of this library.
    /// Checks include:
    /// *   Year is between 1 and 9999.
    /// *   Month is between 1 and 12.
    /// *   Day is between 1 and the number of days in the specified month and year (e.g., 29, 30, or 31).
    ///     This correctly handles the length of Esfand (month 12) in leap and common years.
    ///
    /// # Arguments
    ///
    /// * `year`: The Persian year (must be 1-9999).
    /// * `month`: The Persian month (1 for Farvardin, 12 for Esfand).
    /// * `day`: The day of the month (must be valid for the given `month` and `year`).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the combination of `year`, `month`, and `day`
    /// does not represent a valid Persian date within the supported range [1, 9999].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// // Create a valid date
    /// let date_result = ParsiDate::new(1403, 5, 2); // Mordad 2nd, 1403
    /// assert!(date_result.is_ok());
    /// assert_eq!(date_result.unwrap().month(), 5);
    ///
    /// // Create a valid leap day date
    /// let leap_day_result = ParsiDate::new(1403, 12, 30); // 1403 is a leap year
    /// assert!(leap_day_result.is_ok());
    ///
    /// // Example of an invalid month
    /// assert_eq!(ParsiDate::new(1403, 13, 1), Err(DateError::InvalidDate));
    ///
    /// // Example of an invalid day (Esfand 30th in a non-leap year)
    /// assert_eq!(ParsiDate::new(1404, 12, 30), Err(DateError::InvalidDate)); // 1404 is not a leap year
    ///
    /// // Example of an invalid day (too large for month)
    /// assert_eq!(ParsiDate::new(1403, 7, 31), Err(DateError::InvalidDate)); // Mehr (month 7) only has 30 days
    ///
    /// // Example of an invalid year (outside supported range)
    /// assert_eq!(ParsiDate::new(0, 1, 1), Err(DateError::InvalidDate));
    /// assert_eq!(ParsiDate::new(10000, 1, 1), Err(DateError::InvalidDate));
    /// ```
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self, DateError> {
        // Create a temporary ParsiDate instance.
        let date = ParsiDate { year, month, day };
        // Use the comprehensive validation method.
        if date.is_valid() {
            Ok(date) // Return the valid date if all checks pass.
        } else {
            Err(DateError::InvalidDate) // Return error if any check fails.
        }
    }

    /// Creates a `ParsiDate` from year, month, and day components **without** validation.
    ///
    /// **Warning:** This function is marked `unsafe` because it completely bypasses the
    /// validation checks performed by [`ParsiDate::new`]. If you provide invalid components
    /// (e.g., `month = 13`, `day = 32`, `year = 0`), this function will still create a
    /// `ParsiDate` instance containing that invalid data. Subsequent operations on such an
    /// invalid instance (like formatting, conversion, or arithmetic) can lead to unpredictable
    /// behavior, incorrect results, or runtime panics.
    ///
    /// **Only use this function if you have already rigorously validated the date components
    /// through external means and need to avoid the validation overhead for performance reasons.**
    /// In almost all scenarios, the safe [`ParsiDate::new`] constructor is preferred.
    ///
    /// # Safety
    ///
    /// The caller *must* guarantee that the provided `year`, `month`, and `day` combination
    /// represents a logically valid date in the Persian calendar system according to its rules
    /// (month lengths, leap years) and is within the supported year range (1-9999). Failure to
    /// uphold this guarantee invokes undefined behavior from the perspective of this library's
    /// date logic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// // Assume these components have been pre-validated elsewhere
    /// let p_year = 1403;
    /// let p_month = 10; // Dey
    /// let p_day = 11;
    ///
    /// // --- Incorrect Usage (creating an invalid date) ---
    /// // let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
    /// // assert!(!invalid_date.is_valid()); // This date is invalid
    /// // Using invalid_date might lead to errors later.
    ///
    /// // --- Correct Usage (inputs are guaranteed valid) ---
    /// // Example external validation (simplified):
    /// fn is_known_valid(y: i32, m: u32, d: u32) -> bool {
    ///     // Replace with actual robust validation if needed
    ///     m >= 1 && m <= 12 && d >= 1 && d <= ParsiDate::days_in_month(y, m) && y >= 1 && y <= 9999
    /// }
    ///
    /// if is_known_valid(p_year, p_month, p_day) {
    ///     // It's safe to use new_unchecked because validation passed
    ///     let date = unsafe { ParsiDate::new_unchecked(p_year, p_month, p_day) };
    ///     assert_eq!(date.year(), 1403);
    ///     assert_eq!(date.month(), 10);
    ///     assert_eq!(date.day(), 11);
    /// } else {
    ///     eprintln!("Cannot use new_unchecked with inputs that failed validation!");
    ///     // Handle the error, perhaps by returning Err or panicking
    /// }
    /// ```
    pub const unsafe fn new_unchecked(year: i32, month: u32, day: u32) -> Self {
        ParsiDate { year, month, day }
    }

    /// Creates a `ParsiDate` from the day number within a given Persian year (the ordinal day).
    ///
    /// The ordinal day counts from the beginning of the year, where `ordinal = 1` corresponds
    /// to Farvardin 1st, `ordinal = 2` to Farvardin 2nd, and so on. The maximum valid ordinal
    /// day is 365 for a common Persian year and 366 for a leap year.
    ///
    /// # Arguments
    ///
    /// * `year`: The Persian year (must be 1-9999).
    /// * `ordinal`: The day number within the year (1 to 365 or 1 to 366).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidOrdinal)` if the `ordinal` value is 0 or greater than
    /// the number of days actually present in the specified `year`.
    /// Returns `Err(DateError::InvalidDate)` if the provided `year` is outside the supported
    /// range [1, 9999] (this check is performed by the final internal call to `ParsiDate::new`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// // First day of the year
    /// assert_eq!(ParsiDate::from_ordinal(1403, 1), Ok(ParsiDate::new(1403, 1, 1).unwrap()));
    ///
    /// // A day mid-year (e.g., day 100)
    /// // 31(Far) + 31(Ord) + 31(Kho) = 93. Day 100 is 7th of Tir.
    /// assert_eq!(ParsiDate::from_ordinal(1403, 100), Ok(ParsiDate::new(1403, 4, 7).unwrap()));
    ///
    /// // Last day of a leap year (1403 is leap)
    /// assert_eq!(ParsiDate::from_ordinal(1403, 366), Ok(ParsiDate::new(1403, 12, 30).unwrap()));
    ///
    /// // Last day of a common year (1404 is common)
    /// assert_eq!(ParsiDate::from_ordinal(1404, 365), Ok(ParsiDate::new(1404, 12, 29).unwrap()));
    ///
    /// // Error: Ordinal too large for a common year
    /// assert_eq!(ParsiDate::from_ordinal(1404, 366), Err(DateError::InvalidOrdinal));
    ///
    /// // Error: Ordinal too large for a leap year
    /// assert_eq!(ParsiDate::from_ordinal(1403, 367), Err(DateError::InvalidOrdinal));
    ///
    /// // Error: Ordinal is zero
    /// assert_eq!(ParsiDate::from_ordinal(1403, 0), Err(DateError::InvalidOrdinal));
    ///
    /// // Error: Invalid year
    /// assert!(matches!(ParsiDate::from_ordinal(0, 100), Err(DateError::InvalidDate))); // Final validation fails
    /// ```
    pub fn from_ordinal(year: i32, ordinal: u32) -> Result<Self, DateError> {
        // Basic validation: ordinal must be positive.
        if ordinal == 0 {
            return Err(DateError::InvalidOrdinal);
        }
        // Determine the total number of days in the specified year.
        let days_in_year = if Self::is_persian_leap_year(year) {
            366
        } else {
            365
        };

        // Validate ordinal against the calculated number of days in the year.
        if ordinal > days_in_year {
            return Err(DateError::InvalidOrdinal);
        }

        // Calculate the month and day corresponding to the ordinal day.
        let mut month = 1u32;
        let mut day = ordinal; // Start with day = ordinal
        let month_lengths = Self::month_lengths(year);

        // Iterate through the months, subtracting month lengths until the correct month is found.
        for (m_idx, length) in month_lengths.iter().enumerate() {
            if day <= *length {
                // The remaining 'day' value falls within this month's length.
                month = (m_idx + 1) as u32; // Found the month (m_idx is 0-based, month is 1-based)
                // The 'day' value at this point is the correct day of the month.
                break; // Exit the loop
            }
            // Subtract the full length of the current month and proceed to the next.
            day -= *length;
            // Note: 'month' variable is implicitly updated in the next iteration or keeps the last value if loop ends.
        }

        // Use the safe ParsiDate::new() constructor for final validation.
        // This ensures the calculated year/month/day are valid and checks the year range.
        // While the logic above should produce valid month/day if ordinal was valid,
        // this provides robustness and handles the year range check [1, 9999].
        ParsiDate::new(year, month, day)
    }

    /// Converts a Gregorian date (`chrono::NaiveDate`) to its equivalent Persian (Jalali) `ParsiDate`.
    ///
    /// This function implements the conversion algorithm from the Gregorian calendar to the
    /// Persian calendar, determining the corresponding Persian year, month, and day.
    /// The algorithm typically involves calculating the number of days passed since a common epoch
    /// (the start of the Persian calendar, corresponding to Gregorian March 21, 622 CE)
    /// and then mapping that day count back into the Persian calendar structure.
    ///
    /// # Arguments
    ///
    /// * `gregorian_date`: The `chrono::NaiveDate` instance representing the Gregorian date to convert.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::GregorianConversionError)` if:
    /// *   The input `gregorian_date` is earlier than the start of the Persian epoch (approx. 622-03-21 CE).
    /// *   The conversion calculation results in a Persian year outside the supported range [1, 9999].
    /// *   An internal error occurs during date calculations (e.g., `chrono` fails to create epoch date, overflow).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// // Convert a typical Gregorian date
    /// let g_date_1 = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
    /// assert_eq!(ParsiDate::from_gregorian(g_date_1), Ok(ParsiDate::new(1403, 5, 2).unwrap())); // Should be Mordad 2nd
    ///
    /// // Convert another Gregorian date
    /// let g_date_2 = NaiveDate::from_ymd_opt(2023, 3, 20).unwrap(); // Last day of Gregorian year before Nowruz
    /// assert_eq!(ParsiDate::from_gregorian(g_date_2), Ok(ParsiDate::new(1401, 12, 29).unwrap())); // Esfand 29th, 1401 (common year)
    ///
    /// // Convert Gregorian date corresponding to Nowruz (Persian New Year)
    /// let g_date_nowruz = NaiveDate::from_ymd_opt(2023, 3, 21).unwrap();
    /// assert_eq!(ParsiDate::from_gregorian(g_date_nowruz), Ok(ParsiDate::new(1402, 1, 1).unwrap())); // Farvardin 1st, 1402
    ///
    /// // Convert the Gregorian date corresponding to the Persian epoch start
    /// let epoch_gregorian = NaiveDate::from_ymd_opt(622, 3, 21).unwrap();
    /// assert_eq!(ParsiDate::from_gregorian(epoch_gregorian), Ok(ParsiDate::new(1, 1, 1).unwrap()));
    ///
    /// // Error: Date before the Persian epoch
    /// let before_epoch = NaiveDate::from_ymd_opt(622, 3, 20).unwrap();
    /// assert_eq!(ParsiDate::from_gregorian(before_epoch), Err(DateError::GregorianConversionError));
    ///
    /// // Error: Far future date likely resulting in year > 9999 (example only, actual limit depends on chrono)
    /// let far_future_g = NaiveDate::MAX; // Chrono's max date
    /// // The result depends on whether chrono's max date converts to a Persian year > 9999
    /// match ParsiDate::from_gregorian(far_future_g) {
    ///     Ok(pd) => println!("Conversion succeeded for NaiveDate::MAX: {}", pd), // Might succeed if within ParsiDate range
    ///     Err(e) => assert!(matches!(e, DateError::GregorianConversionError)), // Likely error if exceeds 9999
    /// }
    /// ```
    pub fn from_gregorian(gregorian_date: NaiveDate) -> Result<Self, DateError> {
        // Define the Gregorian start date corresponding to the Persian epoch (1/1/1 Parsi).
        let persian_epoch_gregorian_start =
            NaiveDate::from_ymd_opt(622, 3, 21).ok_or(DateError::GregorianConversionError)?; // Handle potential chrono error

        // Ensure the input Gregorian date is not before the Persian epoch start.
        if gregorian_date < persian_epoch_gregorian_start {
            // Date is too early, cannot be represented in the Persian calendar starting from year 1.
            return Err(DateError::GregorianConversionError);
        }

        // --- Calculate Persian Year ---
        // This part finds the Persian year `p_year` such that:
        // start_of_persian_year(p_year) <= gregorian_date < start_of_persian_year(p_year + 1)

        // Calculate days passed since the epoch day 1 (Gregorian 622-03-21).
        // This is a 0-based count if we consider the epoch day itself as day 0.
        let days_since_epoch_day0 = gregorian_date
            .signed_duration_since(persian_epoch_gregorian_start)
            .num_days(); // number of days *after* the start date

        // Estimate the Persian year. Average days/year is approx 365.242.
        // Dividing by 365 gives a reasonable starting guess. Add 1 because epoch is year 1.
        let mut p_year_guess = MIN_PARSI_DATE.year + (days_since_epoch_day0 / 365) as i32;
        // Ensure the guess is at least the minimum supported year.
        p_year_guess = p_year_guess.max(MIN_PARSI_DATE.year);

        // Loop to refine the year guess.
        let p_year = loop {
            // Calculate the Gregorian date for Farvardin 1st of the guessed Persian year.
            // We use `new_unchecked` + `to_gregorian_internal` for performance inside this loop,
            // assuming the year guess itself is plausible.
            let start_date_guess = unsafe { ParsiDate::new_unchecked(p_year_guess, 1, 1) };
            let gregorian_start_of_guess_year = match start_date_guess.to_gregorian_internal() {
                Ok(gd) => gd,
                Err(e) => {
                    // If conversion fails (e.g., year guess too high/low), return error.
                    // This indicates an issue, possibly the date is outside the convertible range.
                    return Err(e);
                }
            };

            // Check if the start of the guessed year is *after* the target date.
            if gregorian_start_of_guess_year > gregorian_date {
                // The guess is too high. Try the previous year.
                p_year_guess -= 1;
                // Re-check the start date for this adjusted guess in the next iteration.
                continue;
            }

            // If the start of the guessed year is on or before the target date,
            // we need to check if the *next* year starts *after* the target date.
            let next_persian_year = p_year_guess + 1;
            // Check if next year exceeds max supported year before attempting conversion
            if next_persian_year > MAX_PARSI_DATE.year {
                // If the current guess starts <= target AND the next year is out of bounds,
                // then the current guess must be the correct year.
                break p_year_guess;
            }

            let start_date_next_year = unsafe { ParsiDate::new_unchecked(next_persian_year, 1, 1) };
            match start_date_next_year.to_gregorian_internal() {
                Ok(gregorian_start_of_next_year) => {
                    if gregorian_start_of_next_year > gregorian_date {
                        // Correct year found: Starts <= target_date, Next year starts > target_date.
                        break p_year_guess;
                    } else {
                        // Target date is in a later year. Increment guess and loop again.
                        p_year_guess += 1;
                        // Add a check to prevent runaway loops, although unlikely with correct logic.
                        if p_year_guess > MAX_PARSI_DATE.year + 2 {
                            return Err(DateError::GregorianConversionError); // Protect against infinite loops
                        }
                    }
                }
                Err(_) => {
                    // If converting the start of the *next* year fails (e.g., year 10000),
                    // and the current guess starts on/before the target date, then the current
                    // guess must be the correct year (it's the last valid one containing the date).
                    if gregorian_start_of_guess_year <= gregorian_date {
                        break p_year_guess;
                    } else {
                        // This case (current guess starts *after* target AND next year fails)
                        // shouldn't be reachable due to the earlier check.
                        return Err(DateError::GregorianConversionError);
                    }
                }
            }
        }; // End of year-finding loop

        // --- Calculate Persian Month and Day ---
        // At this point, `p_year` holds the correct Persian year.
        // Find the Gregorian start date for this correct Persian year.
        let correct_pyear_start_gregorian =
            unsafe { ParsiDate::new_unchecked(p_year, 1, 1) }.to_gregorian_internal()?;

        // Calculate the 0-based day number within the Persian year.
        let days_into_year = gregorian_date
            .signed_duration_since(correct_pyear_start_gregorian)
            .num_days();

        // This should not be negative if the year-finding was correct.
        if days_into_year < 0 {
            return Err(DateError::GregorianConversionError); // Internal calculation error
        }
        let mut remaining_days_in_year = days_into_year as u32; // 0-indexed day number

        // Determine month and day by iterating through month lengths for the correct p_year.
        let month_lengths = Self::month_lengths(p_year);
        let mut p_month = 1u32;
        let mut p_day = 1u32; // Placeholder, will be set in the loop

        for (m_idx, length) in month_lengths.iter().enumerate() {
            // Check if the day falls within this month (length is number of days).
            // Since remaining_days_in_year is 0-indexed, we check if it's less than length.
            if remaining_days_in_year < *length {
                p_month = (m_idx + 1) as u32; // Month index is 0-based, month number is 1-based
                p_day = remaining_days_in_year + 1; // Day number is 0-based index + 1
                break; // Found the correct month and day
            }
            // Subtract the days of this full month and continue to the next.
            remaining_days_in_year -= *length;
        }
        // The loop should always find a month/day if days_into_year was valid for the year length.

        // Use ParsiDate::new for final construction and validation (e.g., year range).
        // The calculated p_month/p_day should be logically valid based on the derivation.
        ParsiDate::new(p_year, p_month, p_day)
    }

    /// Converts this Persian (Jalali) `ParsiDate` to its equivalent Gregorian `chrono::NaiveDate`.
    ///
    /// This function first validates the `ParsiDate` instance itself using [`is_valid`].
    /// If valid, it proceeds with the conversion algorithm, which typically involves calculating
    /// the total number of days elapsed since the Persian epoch start (1/1/1) and adding
    /// that count to the corresponding Gregorian start date (622-03-21 CE).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the `ParsiDate` instance itself holds invalid data
    /// (e.g., month 0, day 32, or created using `new_unchecked` with invalid components).
    /// Returns `Err(DateError::GregorianConversionError)` if the conversion calculation fails. This
    /// could be due to internal arithmetic overflows (extremely unlikely for valid dates within the
    /// supported range) or if the resulting Gregorian date falls outside the range supported by
    /// `chrono::NaiveDate`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// // Convert a typical Persian date
    /// let pd1 = ParsiDate::new(1403, 5, 2).unwrap(); // Mordad 2nd, 1403
    /// let expected_g1 = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
    /// assert_eq!(pd1.to_gregorian(), Ok(expected_g1));
    ///
    /// // Convert the Persian epoch start date
    /// let pd_epoch = ParsiDate::new(1, 1, 1).unwrap();
    /// let expected_epoch_gregorian = NaiveDate::from_ymd_opt(622, 3, 21).unwrap();
    /// assert_eq!(pd_epoch.to_gregorian(), Ok(expected_epoch_gregorian));
    ///
    /// // Convert end of a Persian leap year
    /// let pd_leap_end = ParsiDate::new(1403, 12, 30).unwrap(); // 1403 is leap
    /// let expected_g_leap_end = NaiveDate::from_ymd_opt(2025, 3, 20).unwrap();
    /// assert_eq!(pd_leap_end.to_gregorian(), Ok(expected_g_leap_end));
    ///
    /// // Example with an invalid ParsiDate
    /// let invalid_pd = unsafe { ParsiDate::new_unchecked(1404, 12, 30) }; // Invalid day for non-leap year
    /// assert_eq!(invalid_pd.to_gregorian(), Err(DateError::InvalidDate));
    /// ```
    pub fn to_gregorian(&self) -> Result<NaiveDate, DateError> {
        // First, ensure the ParsiDate object itself contains valid data.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // If valid, call the internal conversion logic which assumes validity.
        self.to_gregorian_internal()
    }

    /// **Internal** conversion logic: Converts a *valid* `ParsiDate` to Gregorian `NaiveDate`.
    ///
    /// This function assumes `self` represents a valid Persian date (validation should be done prior).
    /// It calculates the total number of days elapsed from the Persian epoch start (1/1/1) up to `self`
    /// and adds this offset to the Gregorian date corresponding to the epoch start (622-03-21).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::GregorianConversionError)` if:
    /// *   `chrono` fails to create the reference epoch date (622-03-21).
    /// *   Integer overflow occurs during the summation of days (highly unlikely for valid dates).
    /// *   Adding the final calculated day offset using `chrono::Days` fails, likely because the
    ///     resulting Gregorian date is outside the range supported by `chrono::NaiveDate`.
    // Marked pub(crate) as it's an internal helper assuming validity.
    pub(crate) fn to_gregorian_internal(self) -> Result<NaiveDate, DateError> {
        // Define the Gregorian start date corresponding to the Persian epoch (1/1/1 Parsi).
        let persian_epoch_gregorian_start =
            NaiveDate::from_ymd_opt(622, 3, 21).ok_or(DateError::GregorianConversionError)?;

        // --- Calculate total days elapsed since 1/1/1 ---
        // Sum days in full years preceding self.year.
        let mut total_days_offset: i64 = 0;
        // Loop from year 1 up to (but not including) self.year.
        // Assumes self.year >= MIN_PARSI_DATE.year (checked by caller via is_valid).
        for y in MIN_PARSI_DATE.year..self.year {
            let days_in_year: i64 = if Self::is_persian_leap_year(y) {
                366
            } else {
                365
            };
            // Add days, checking for potential i64 overflow.
            total_days_offset = total_days_offset
                .checked_add(days_in_year)
                .ok_or(DateError::GregorianConversionError)?; // Map overflow to conversion error
        }

        // Sum days in full months preceding self.month within self.year.
        // Assumes self.month >= 1 (checked by caller via is_valid).
        if self.month > 1 {
            let month_lengths_current_year = Self::month_lengths(self.year);
            // Loop from month 1 up to (but not including) self.month.
            // Indexing `month_lengths_current_year` with (m - 1) is safe because m <= self.month <= 12.
            for m in 1..self.month {
                let days_in_month = month_lengths_current_year[(m - 1) as usize] as i64;
                // Add days, checking for potential i64 overflow.
                total_days_offset = total_days_offset
                    .checked_add(days_in_month)
                    .ok_or(DateError::GregorianConversionError)?;
            }
        }

        // Add the day of the month (minus 1, as we need the 0-based offset from the start of the month).
        // Assumes self.day >= 1 (checked by caller via is_valid).
        total_days_offset = total_days_offset
            .checked_add((self.day - 1) as i64) // self.day is u32, safe cast to i64
            .ok_or(DateError::GregorianConversionError)?;

        // --- Add offset to Gregorian epoch start ---
        // `total_days_offset` now holds the total number of days passed since 1/1/1 (0-based).
        // This offset should be non-negative for valid dates.
        if total_days_offset < 0 {
            // This state indicates an internal logic error if self was validated.
            return Err(DateError::GregorianConversionError);
        }

        // Use chrono's `checked_add_days` for safe addition, converting the i64 offset to chrono::Days.
        // `chrono::Days` takes a u64.
        persian_epoch_gregorian_start
            .checked_add_days(chrono::Days::new(total_days_offset as u64))
            .ok_or(DateError::GregorianConversionError) // Map chrono's None result (overflow/out of range) to our error type.
    }

    /// Returns the current system date, converted to `ParsiDate`.
    ///
    /// This function determines the current date based on the system's local timezone setting,
    /// obtains the Gregorian date, and then converts it to the corresponding `ParsiDate`.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::GregorianConversionError)` if the conversion from the current
    /// system Gregorian date fails. This might occur if the system clock is set to a date
    /// before the Persian epoch (approx. 622 CE) or encounters other issues during the
    /// conversion process handled by [`from_gregorian`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// match ParsiDate::today() {
    ///     Ok(today) => {
    ///         println!("Today in the Persian calendar is: {}", today);
    ///         println!("Year: {}, Month: {}, Day: {}", today.year(), today.month(), today.day());
    ///         // Example: Check if it's Esfand (month 12)
    ///         if today.month() == 12 {
    ///             println!("We are in the last month of the Persian year!");
    ///         }
    ///     }
    ///     Err(e) => {
    ///         eprintln!("Could not determine today's Persian date: {}", e);
    ///         // This might indicate a system clock issue or a conversion range problem.
    ///     }
    /// }
    /// ```
    pub fn today() -> Result<Self, DateError> {
        // Get the current date and time in the local system timezone.
        let now: chrono::DateTime<chrono::Local> = chrono::Local::now();
        // Extract the naive date part (date without timezone information).
        let gregorian_today: NaiveDate = now.date_naive();
        // Convert this Gregorian date to ParsiDate using the existing conversion method.
        Self::from_gregorian(gregorian_today)
    }

    // --- Accessors ---

    /// Returns the year component of the Persian date.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// assert_eq!(date.year(), 1403);
    ///
    /// let date_early = ParsiDate::new(50, 1, 1).unwrap();
    /// assert_eq!(date_early.year(), 50);
    /// ```
    #[inline]
    pub const fn year(&self) -> i32 {
        self.year
    }

    /// Returns the month component of the Persian date (1 = Farvardin, ..., 12 = Esfand).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 2).unwrap(); // Month 5 = Mordad
    /// assert_eq!(date.month(), 5);
    ///
    /// let date_esfand = ParsiDate::new(1403, 12, 30).unwrap(); // Month 12 = Esfand
    /// assert_eq!(date_esfand.month(), 12);
    /// ```
    #[inline]
    pub const fn month(&self) -> u32 {
        self.month
    }

    /// Returns the day component of the Persian date (typically 1-31).
    ///
    /// The actual maximum value depends on the month and whether the year is a leap year.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// assert_eq!(date.day(), 2);
    ///
    /// let date_end_month = ParsiDate::new(1403, 1, 31).unwrap(); // Farvardin 31st
    /// assert_eq!(date_end_month.day(), 31);
    ///
    /// let date_leap_day = ParsiDate::new(1403, 12, 30).unwrap(); // Esfand 30th (leap year)
    /// assert_eq!(date_leap_day.day(), 30);
    /// ```
    #[inline]
    pub const fn day(&self) -> u32 {
        self.day
    }

    // --- Validation and Leap Year ---

    /// Checks if the current `ParsiDate` instance represents a valid date.
    ///
    /// Performs a comprehensive check based on the rules of the Persian calendar and the
    /// supported range of this library:
    /// *   Year must be in the range [1, 9999].
    /// *   Month must be in the range [1, 12].
    /// *   Day must be in the range [1, N], where N is the number of days in the specified
    ///     month (`self.month`) of the specified year (`self.year`), considering leap years.
    ///
    /// This method is used internally by constructors like `new` and should be used to verify
    /// instances created with `unsafe new_unchecked`.
    ///
    /// # Returns
    ///
    /// *   `true` if the date (year, month, day combination) is valid.
    /// *   `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// // Valid dates
    /// assert!(ParsiDate::new(1403, 1, 1).unwrap().is_valid());
    /// assert!(ParsiDate::new(1403, 12, 30).unwrap().is_valid()); // Leap day in leap year 1403
    /// assert!(ParsiDate::new(1404, 12, 29).unwrap().is_valid()); // Last day of common year 1404
    /// assert!(ParsiDate::new(9999, 12, 29).unwrap().is_valid()); // Max supported year (common)
    ///
    /// // Invalid dates created unsafely
    /// let invalid_day = unsafe { ParsiDate::new_unchecked(1404, 12, 30) }; // Esfand 30 in common year
    /// assert!(!invalid_day.is_valid());
    ///
    /// let invalid_month = unsafe { ParsiDate::new_unchecked(1403, 13, 1) }; // Month 13
    /// assert!(!invalid_month.is_valid());
    ///
    /// let invalid_day_zero = unsafe { ParsiDate::new_unchecked(1403, 1, 0) }; // Day 0
    /// assert!(!invalid_day_zero.is_valid());
    ///
    /// let invalid_year_zero = unsafe { ParsiDate::new_unchecked(0, 1, 1) }; // Year 0
    /// assert!(!invalid_year_zero.is_valid());
    ///
    /// let invalid_year_high = unsafe { ParsiDate::new_unchecked(10000, 1, 1) }; // Year 10000
    /// assert!(!invalid_year_high.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        // Check year validity (must be within the supported range)
        if self.year < MIN_PARSI_DATE.year || self.year > MAX_PARSI_DATE.year {
            return false;
        }
        // Check month validity (must be between 1 and 12)
        if self.month < 1 || self.month > 12 {
            return false;
        }
        // Check day validity (must be at least 1 and not exceed the number of days in the month)
        if self.day < 1 || self.day > Self::days_in_month(self.year, self.month) {
            // days_in_month handles the leap year logic for month 12 correctly.
            // If days_in_month returns 0 (e.g., for invalid month), this check correctly fails if day >= 1.
            return false;
        }
        // If all checks passed, the date is valid.
        true
    }

    /// Determines if a given Persian year is a leap year based on a common algorithm.
    ///
    /// The Persian calendar's leap year rule is astronomically determined (vernal equinox timing).
    /// However, a highly accurate algorithmic approximation based on a 33-year cycle is widely used.
    /// This function implements that approximation: A year `y` is considered leap if the remainder
    /// of `y` divided by 33 is one of the following values: 1, 5, 9, 13, 17, 22, 26, or 30.
    ///
    /// Years less than 1 are considered non-leap by this function.
    ///
    /// **Note:** While extremely accurate for historical and near-future dates, this is still an
    /// approximation of the true astronomical rule.
    ///
    /// # Arguments
    ///
    /// * `year`: The Persian year to check.
    ///
    /// # Returns
    ///
    /// *   `true` if the year is determined to be a leap year by the 33-year cycle algorithm.
    /// *   `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// assert!(ParsiDate::is_persian_leap_year(1399)); // 1399 % 33 = 30 -> Leap
    /// assert!(!ParsiDate::is_persian_leap_year(1400)); // 1400 % 33 = 1 -> Common
    /// assert!(!ParsiDate::is_persian_leap_year(1400));  // 1400 % 33 = 1 -> Common
    /// assert!(!ParsiDate::is_persian_leap_year(1401)); // 1401 % 33 = 2 -> Common
    /// assert!(!ParsiDate::is_persian_leap_year(1402)); // 1402 % 33 = 3 -> Common
    /// assert!(ParsiDate::is_persian_leap_year(1403));  // 1403 % 33 = 5 -> Leap
    /// assert!(!ParsiDate::is_persian_leap_year(1404)); // 1404 % 33 = 6 -> Common
    /// assert!(!ParsiDate::is_persian_leap_year(1405)); // 1405 % 33 = 7 -> Common
    /// assert!(!ParsiDate::is_persian_leap_year(1406)); // 1406 % 33 = 8 -> Common
    /// assert!(!ParsiDate::is_persian_leap_year(1407)); // 1407 % 33 = 9 -> Common
    /// assert!(!ParsiDate::is_persian_leap_year(1407));  // 1407 % 33 = 9 -> Common
    /// assert!(ParsiDate::is_persian_leap_year(1408)); // 1408 % 33 = 10 -> leap
    ///
    /// assert!(!ParsiDate::is_persian_leap_year(0));    // Year 0 is not considered leap
    /// assert!(!ParsiDate::is_persian_leap_year(-5));   // Negative years are not considered leap
    /// ```
    pub fn is_persian_leap_year(year: i32) -> bool {
        // Persian years are positive; years <= 0 are treated as non-leap.
        if year <= 0 {
            return false;
        }
        // Apply the 33-year cycle rule using Euclidean remainder.
        match year.rem_euclid(33) {
            // These specific remainders indicate a leap year in the cycle.
            1 | 5 | 9 | 13 | 17 | 22 | 26 | 30 => true,
            // All other remainders (0, 2, 3, 4, 6, etc.) indicate a common year.
            _ => false,
        }
    }

    /// Determines if a given Gregorian year is a leap year.
    ///
    /// Implements the standard Gregorian calendar leap year rules:
    /// 1.  A year is a leap year if it is divisible by 4.
    /// 2.  However, if a year is divisible by 100, it is *not* a leap year...
    /// 3.  ...unless that year is also divisible by 400.
    ///
    /// # Arguments
    ///
    /// * `year`: The Gregorian calendar year to check.
    ///
    /// # Returns
    ///
    /// *   `true` if the year is a Gregorian leap year.
    /// *   `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// assert!(ParsiDate::is_gregorian_leap_year(2000)); // Divisible by 400 -> Leap
    /// assert!(ParsiDate::is_gregorian_leap_year(2024)); // Divisible by 4, not by 100 -> Leap
    /// assert!(ParsiDate::is_gregorian_leap_year(2020)); // Divisible by 4, not by 100 -> Leap
    /// assert!(!ParsiDate::is_gregorian_leap_year(1900)); // Divisible by 100, not by 400 -> Common
    /// assert!(!ParsiDate::is_gregorian_leap_year(2021)); // Not divisible by 4 -> Common
    /// assert!(!ParsiDate::is_gregorian_leap_year(2023)); // Not divisible by 4 -> Common
    /// ```
    pub fn is_gregorian_leap_year(year: i32) -> bool {
        // Combine the Gregorian rules: (divisible by 4 AND not divisible by 100) OR (divisible by 400)
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Returns the number of days in a specific month of a given Persian year.
    ///
    /// This function accounts for the standard lengths of Persian months and the leap year
    /// rule affecting the length of Esfand (the 12th month).
    /// *   Months 1-6 (Farvardin to Shahrivar) always have 31 days.
    /// *   Months 7-11 (Mehr to Bahman) always have 30 days.
    /// *   Month 12 (Esfand) has 30 days if the `year` is a Persian leap year, otherwise it has 29 days.
    ///
    /// # Arguments
    ///
    /// * `year`: The Persian year (used to determine if Esfand has 29 or 30 days).
    /// * `month`: The Persian month number (1-12).
    ///
    /// # Returns
    ///
    /// The number of days (29, 30, or 31) in the specified month and year.
    /// Returns `0` if the provided `month` number is invalid (outside the range 1-12).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// // Months 1-6 have 31 days
    /// assert_eq!(ParsiDate::days_in_month(1403, 1), 31); // Farvardin
    /// assert_eq!(ParsiDate::days_in_month(1403, 6), 31); // Shahrivar
    ///
    /// // Months 7-11 have 30 days
    /// assert_eq!(ParsiDate::days_in_month(1403, 7), 30); // Mehr
    /// assert_eq!(ParsiDate::days_in_month(1403, 11), 30); // Bahman
    ///
    /// // Month 12 (Esfand) depends on leap year
    /// assert_eq!(ParsiDate::days_in_month(1403, 12), 30); // 1403 is leap
    /// assert_eq!(ParsiDate::days_in_month(1404, 12), 29); // 1404 is common
    /// assert_eq!(ParsiDate::days_in_month(1399, 12), 30); // 1399 is leap
    ///
    /// // Invalid month returns 0
    /// assert_eq!(ParsiDate::days_in_month(1403, 0), 0);
    /// assert_eq!(ParsiDate::days_in_month(1403, 13), 0);
    /// ```
    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1..=6 => 31,  // First 6 months have 31 days
            7..=11 => 30, // Next 5 months have 30 days
            12 => {
                // 12th month (Esfand) depends on leap year status
                if Self::is_persian_leap_year(year) {
                    30 // 30 days in a leap year
                } else {
                    29 // 29 days in a common year
                }
            }
            _ => 0, // Invalid month number results in 0 days
        }
    }

    /// **Internal**: Returns an array containing the lengths of the 12 months for a given Persian year.
    ///
    /// This is primarily a helper function used internally by methods like `from_ordinal`
    /// and `to_gregorian_internal` that need quick access to the length of each month.
    /// The length of the 12th month (Esfand, index 11) depends on whether the `year` is leap.
    ///
    /// # Arguments
    ///
    /// * `year`: The Persian year for which to get month lengths.
    ///
    /// # Returns
    ///
    /// An array `[u32; 12]` where `array[0]` is the length of Farvardin (month 1),
    /// `array[1]` is the length of Ordibehesht (month 2), ..., and `array[11]` is the
    /// length of Esfand (month 12).
    // Marked pub(crate) as it's an implementation detail.
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
            // 12: Esfand - length depends on leap year status
            Self::days_in_month(year, 12), // Reuse the logic from days_in_month
        ]
    }

    // --- Formatting ---

    /// Formats the `ParsiDate` into a string using predefined styles or a custom pattern.
    ///
    /// This provides convenient ways to represent the date as a string.
    ///
    /// # Arguments
    ///
    /// * `style_or_pattern`: A string slice (`&str`) specifying the desired format. It can be:
    ///     *   `"short"`: Formats as "YYYY/MM/DD" (e.g., "1403/05/02"). This is the default style used by the `Display` trait implementation (`.to_string()`).
    ///     *   `"long"`: Formats as "D MonthName YYYY" using the full Persian month name (e.g., "2 مرداد 1403"). Note: The day `D` is *not* zero-padded in this style.
    ///     *   `"iso"`: Formats according to ISO 8601 style for dates: "YYYY-MM-DD" (e.g., "1403-05-02").
    ///     *   **Custom Pattern**: If the string does not match "short", "long", or "iso", it is treated as a custom format pattern string to be processed by [`format_strftime`](#method.format_strftime). See that method's documentation for supported specifiers like `%Y`, `%m`, `%d`, `%B`, etc.
    ///
    /// # Returns
    ///
    /// A `String` containing the date formatted according to the specified style or pattern.
    /// If the `ParsiDate` instance itself contains invalid data (e.g., created via `unsafe new_unchecked`),
    /// the output for certain format specifiers might show error indicators (e.g., "?InvalidMonth?").
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 2).unwrap(); // Mordad 2nd, 1403
    ///
    /// // Predefined styles
    /// assert_eq!(date.format("short"), "1403/05/02");
    /// assert_eq!(date.format("long"), "2 مرداد 1403"); // Day '2' is not zero-padded
    /// assert_eq!(date.format("iso"), "1403-05-02");
    ///
    /// // Default display format (same as "short")
    /// assert_eq!(date.to_string(), "1403/05/02");
    ///
    /// // Custom pattern (delegates to format_strftime)
    /// assert_eq!(date.format("%d %B %Y"), "02 مرداد 1403"); // Day '%d' *is* zero-padded
    /// assert_eq!(date.format("%Y-%j"), "1403-126"); // ISO date with ordinal day
    /// ```
    pub fn format(&self, style_or_pattern: &str) -> String {
        match style_or_pattern {
            "short" => format!("{}/{:02}/{:02}", self.year, self.month, self.day),
            "long" => format!(
                // Day is NOT zero-padded in the "long" style. Month name is used.
                "{} {} {}",
                self.day,
                // Safely get month name, handling potential invalid month in self.
                MONTH_NAMES_PERSIAN
                    .get((self.month.saturating_sub(1)) as usize)
                    .unwrap_or(&"?InvalidMonth?"), // Fallback if month index is out of bounds
                self.year
            ),
            "iso" => format!("{}-{:02}-{:02}", self.year, self.month, self.day),
            // If not a predefined style, treat as a custom strftime pattern.
            pattern => self.format_strftime(pattern),
        }
    }

    /// Formats the `ParsiDate` into a string according to `strftime`-like format specifiers.
    ///
    /// This method allows for flexible date formatting by interpreting a pattern string containing
    /// special percent-prefixed sequences (specifiers). Each specifier is replaced with the
    /// corresponding part of the date. Characters in the pattern that are not part of a specifier
    /// are included literally in the output.
    ///
    /// This method is called internally by [`format`](#method.format) when a custom pattern is provided.
    ///
    /// # Supported Format Specifiers:
    ///
    /// | Specifier | Replaced By                                        | Example (for 1403/05/02) |
    /// | :-------- | :------------------------------------------------- | :----------------------- |
    /// | `%Y`      | Year with century (4 digits)                       | `1403`                   |
    /// | `%m`      | Month as a zero-padded number                      | `05`                     |
    /// | `%d`      | Day of the month as a zero-padded number           | `02`                     |
    /// | `%B`      | Full Persian month name                            | `مرداد`                  |
    /// | `%A`      | Full Persian weekday name (Saturday to Friday)     | `سه‌شنبه`               |
    /// | `%w`      | Weekday as a number (Saturday=0, ..., Friday=6)   | `3`                      |
    /// | `%j`      | Day of the year as a zero-padded number (001-366) | `126`                    |
    /// | `%%`      | A literal percent sign (`%`)                       | `%`                      |
    ///
    /// **Note:** Unrecognized specifiers (e.g., `%x`, `%y`) are treated as literal characters
    /// and will appear in the output string as `%x`, `%y`, etc.
    ///
    /// # Arguments
    ///
    /// * `pattern`: The format string containing literal characters and supported format specifiers.
    ///
    /// # Returns
    ///
    /// A `String` containing the date formatted according to the `pattern`.
    /// If the `ParsiDate` instance contains invalid data (e.g., created via `unsafe new_unchecked`),
    /// or if calculations required for specifiers like `%A`, `%w`, `%j` fail (due to conversion errors),
    /// placeholder values like "?InvalidMonth?", "?WeekdayError?", "?", "???" may appear in the output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// // Example date: 1403/01/07 (Farvardin 7th, 1403), which corresponds to Tuesday, March 26, 2024.
    /// let date = ParsiDate::new(1403, 1, 7).unwrap();
    ///
    /// // ISO format
    /// assert_eq!(date.format_strftime("%Y-%m-%d"), "1403-01-07");
    ///
    /// // Format with names
    /// assert_eq!(date.format_strftime("%A، %d %B %Y"), "سه‌شنبه، 07 فروردین 1403");
    ///
    /// // Format with day/weekday numbers
    /// assert_eq!(date.format_strftime("Year %Y, Day %j (Weekday %w)"), "Year 1403, Day 007 (Weekday 3)"); // Tuesday is 3 (Sat=0)
    ///
    /// // Including literal percent sign
    /// assert_eq!(date.format_strftime("Discount %d%% off on %m/%d!"), "Discount 07% off on 01/07!");
    ///
    /// // Unrecognized specifier is output literally
    /// assert_eq!(date.format_strftime("%Y %x %m"), "1403 %x 01");
    /// ```
    pub fn format_strftime(&self, pattern: &str) -> String {
        // Preallocate string capacity for potentially better performance.
        let mut result = String::with_capacity(pattern.len() + 10); // Estimate a bit extra
        // Use a character iterator to handle multi-byte characters in the pattern correctly.
        let mut chars = pattern.chars().peekable();

        // --- Caching Results ---
        // Cache results of potentially expensive calculations (weekday, ordinal)
        // if they are requested multiple times in the same format pattern.
        // Store the Result to handle potential errors during calculation only once.
        let mut weekday_name_cache: Option<Result<String, DateError>> = None;
        let mut ordinal_day_cache: Option<Result<u32, DateError>> = None;
        let mut weekday_num_cache: Option<Result<u32, DateError>> = None; // Saturday = 0

        // Iterate through the format pattern characters
        while let Some(c) = chars.next() {
            if c == '%' {
                // Found a potential specifier, look at the next character.
                match chars.next() {
                    // %% -> Literal percent sign
                    Some('%') => result.push('%'),
                    // %Y -> Year with century
                    Some('Y') => result.push_str(&self.year.to_string()),
                    // %m -> Month number (01-12)
                    Some('m') => result.push_str(&format!("{:02}", self.month)),
                    // %d -> Day number (01-31)
                    Some('d') => result.push_str(&format!("{:02}", self.day)),
                    // %B -> Full Persian month name
                    Some('B') => {
                        // Safely access the month name using 0-based index.
                        // `saturating_sub(1)` prevents underflow if month is 0 (invalid).
                        // `get` returns None if index is out of bounds (e.g., month > 12).
                        if let Some(name) =
                            MONTH_NAMES_PERSIAN.get((self.month.saturating_sub(1)) as usize)
                        {
                            result.push_str(name);
                        } else {
                            // Handle case where self.month is invalid (e.g., 0 or 13)
                            result.push_str("?InvalidMonth?");
                        }
                    }
                    // %A -> Full Persian weekday name
                    Some('A') => {
                        // Calculate or retrieve cached weekday name.
                        if weekday_name_cache.is_none() {
                            // Calculate only if not already cached. Store the Result.
                            weekday_name_cache = Some(self.weekday_internal());
                        }
                        // Use the cached Result.
                        match weekday_name_cache.as_ref().unwrap() {
                            // Safe unwrap as we set it if None
                            Ok(name) => result.push_str(name),
                            Err(_) => result.push_str("?WeekdayError?"), // Indicate calculation error
                        }
                    }
                    // %w -> Weekday number (Saturday=0)
                    Some('w') => {
                        // Calculate or retrieve cached weekday number.
                        if weekday_num_cache.is_none() {
                            weekday_num_cache = Some(self.weekday_num_sat_0());
                        }
                        // Use the cached Result.
                        match weekday_num_cache.as_ref().unwrap() {
                            Ok(num) => result.push_str(&num.to_string()),
                            Err(_) => result.push('?'), // Indicate calculation error
                        }
                    }
                    // %j -> Day of the year (001-366)
                    Some('j') => {
                        // Calculate or retrieve cached ordinal day.
                        if ordinal_day_cache.is_none() {
                            ordinal_day_cache = Some(self.ordinal_internal());
                        }
                        // Use the cached Result.
                        match ordinal_day_cache.as_ref().unwrap() {
                            Ok(ord) => result.push_str(&format!("{:03}", ord)), // Pad to 3 digits
                            Err(_) => result.push_str("???"), // Indicate calculation error
                        }
                    }
                    // Unrecognized Specifier (e.g., %x)
                    Some(other) => {
                        // Output the '%' and the unrecognized character literally.
                        result.push('%');
                        result.push(other);
                    }
                    // Dangling '%' at the end of the format string
                    None => {
                        // Output the '%' literally. The loop will terminate.
                        result.push('%');
                        break;
                    }
                }
            } else {
                // Not a '%', so it's a literal character. Append it directly.
                result.push(c);
            }
        }
        result // Return the final formatted string
    }

    // --- Parsing ---

    /// Parses a string representation of a Persian date into a `ParsiDate` instance,
    /// based on a provided format pattern.
    ///
    /// This function attempts to match the input string `s` against the structure defined
    /// by the `format` string. It requires an *exact* match between the literal characters
    /// (like `/`, `-`, spaces) in the format string and the input string. It also expects
    /// the date components in the input string to correspond precisely to the format specifiers
    /// used (e.g., `%Y` expects 4 digits, `%m` expects 2 digits).
    ///
    /// After successfully extracting year, month, and day values based on the specifiers,
    /// it validates these values using [`ParsiDate::new`] to ensure they form a logically
    /// valid date in the Persian calendar.
    ///
    /// # Supported Format Specifiers for Parsing:
    ///
    /// *   `%Y`: Parses exactly 4 digits as the Persian year.
    /// *   `%m`: Parses exactly 2 digits as the Persian month (01-12).
    /// *   `%d`: Parses exactly 2 digits as the Persian day (01-31).
    /// *   `%B`: Parses a full Persian month name (case-sensitive, must match one of the names in `MONTH_NAMES_PERSIAN`, e.g., "فروردین", "مرداد").
    /// *   `%%`: Matches a literal percent sign (`%`) character in the input string.
    ///
    /// **Unsupported Specifiers:** Specifiers representing calculated values like `%A` (weekday name),
    /// `%w` (weekday number), and `%j` (ordinal day) are *not* supported for parsing. Using them
    /// in the `format` string will result in a `ParseErrorKind::UnsupportedSpecifier` error.
    ///
    /// # Arguments
    ///
    /// * `s`: The input string slice (`&str`) containing the date representation to be parsed.
    /// * `format`: The format string slice (`&str`) describing the expected structure and specifiers of the input `s`.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::ParseError(kind))` if parsing fails. The `kind` ([`ParseErrorKind`]) provides details:
    /// *   `ParseErrorKind::FormatMismatch`: The input string `s` does not structurally match the `format` string (e.g., wrong separators, missing components, extra trailing characters).
    /// *   `ParseErrorKind::InvalidNumber`: A numeric component (`%Y`, `%m`, `%d`) could not be parsed as a number, or it did not contain the required number of digits (4 for `%Y`, 2 for `%m`/`%d`).
    /// *   `ParseErrorKind::InvalidMonthName`: The input string did not contain a valid, recognized Persian month name where `%B` was expected in the format.
    /// *   `ParseErrorKind::UnsupportedSpecifier`: The `format` string included a specifier not supported for parsing (e.g., `%A`, `%j`).
    /// *   `ParseErrorKind::InvalidDateValue`: The year, month, and day values were successfully extracted according to the format, but they do not form a logically valid Persian date (e.g., "1404/12/30" where 1404 is not a leap year; "1403/07/31" where Mehr has only 30 days). This is checked by the final internal call to `ParsiDate::new`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, DateError, ParseErrorKind};
    ///
    /// // --- Success Cases ---
    /// // Standard YYYY/MM/DD format
    /// assert_eq!(ParsiDate::parse("1403/05/02", "%Y/%m/%d"), Ok(ParsiDate::new(1403, 5, 2).unwrap()));
    ///
    /// // ISO format YYYY-MM-DD
    /// assert_eq!(ParsiDate::parse("1399-12-30", "%Y-%m-%d"), Ok(ParsiDate::new(1399, 12, 30).unwrap())); // Leap year
    ///
    /// // Parsing with Persian month name (%B)s
    /// assert_eq!(ParsiDate::parse("02 مرداد 1403", "%d %B %Y"), Ok(ParsiDate::new(1403, 5, 2).unwrap())); // Note: %d parsed ' ' padding okay here
    /// // Let's re-evaluate the previous example. %d expects 2 digits. Need to fix the example or parsing logic.
    /// // Correcting the example to match strict parsing:
    /// assert_eq!(ParsiDate::parse("02 مرداد 1403", "%d %B %Y"), Ok(ParsiDate::new(1403, 5, 2).unwrap()));
    /// assert_eq!(ParsiDate::parse("10 دی 1400", "%d %B %Y"), Ok(ParsiDate::new(1400, 10, 10).unwrap()));
    /// assert_eq!(ParsiDate::parse("30 اسفند 1399", "%d %B %Y"), Ok(ParsiDate::new(1399, 12, 30).unwrap()));
    ///
    /// // Parsing with literal percent
    /// assert!(matches!(ParsiDate::parse("Year%%1403", "Year%%%%%Y"), Err(DateError::ParseError(ParseErrorKind::FormatMismatch)))); // Assuming default month/day if missing? No, parse should fail.
    /// // Better %% example:
    /// // This requires Year, Month, Day to be present for a valid date result.
    /// // Let's assume a format like "Data%1403/01/01"
    /// // assert_eq!(ParsiDate::parse("Data%1403/01/01", "Data%%%Y/%m/%d"), Ok(ParsiDate::new(1403, 1, 1).unwrap())); // This parse won't work as written. Let's simplify.
    /// // Parsing just the date part after a literal prefix.
    /// // Assume format requires a literal prefix like "Date: "
    ///
    /// // --- Error Cases ---
    /// // Format mismatch (wrong separator)
    /// assert_eq!(ParsiDate::parse("1403-05-02", "%Y/%m/%d"), Err(DateError::ParseError(ParseErrorKind::FormatMismatch)));
    ///
    /// // Format mismatch (extra characters)
    /// assert_eq!(ParsiDate::parse("1403/05/02 extra", "%Y/%m/%d"), Err(DateError::ParseError(ParseErrorKind::FormatMismatch)));
    ///
    /// // Invalid number (too few digits for day %d)
    /// assert_eq!(ParsiDate::parse("1403/05/2", "%Y/%m/%d"), Err(DateError::ParseError(ParseErrorKind::InvalidNumber)));
    ///
    /// // Invalid number (non-digits for year %Y)
    /// assert_eq!(ParsiDate::parse("abcd/05/02", "%Y/%m/%d"), Err(DateError::ParseError(ParseErrorKind::InvalidNumber)));
    ///
    /// // Invalid date value (parsed ok, but logically invalid date)
    /// assert_eq!(ParsiDate::parse("1404/12/30", "%Y/%m/%d"), Err(DateError::ParseError(ParseErrorKind::InvalidDateValue))); // Esfand 30 in common year
    /// assert_eq!(ParsiDate::parse("1403/07/31", "%Y/%m/%d"), Err(DateError::ParseError(ParseErrorKind::InvalidDateValue))); // Day 31 in Mehr
    /// assert_eq!(ParsiDate::parse("1403/13/01", "%Y/%m/%d"), Err(DateError::ParseError(ParseErrorKind::InvalidDateValue))); // Month 13
    ///
    /// // Unsupported specifier used in format string
    /// assert_eq!(ParsiDate::parse("Tuesday 1403", "%A %Y"), Err(DateError::ParseError(ParseErrorKind::UnsupportedSpecifier)));
    /// assert_eq!(ParsiDate::parse("1403 126", "%Y %j"), Err(DateError::ParseError(ParseErrorKind::UnsupportedSpecifier)));
    /// ```
    pub fn parse(s: &str, format: &str) -> Result<Self, DateError> {
        // Options to store the parsed components. They start as None.
        let mut parsed_year: Option<i32> = None;
        let mut parsed_month: Option<u32> = None;
        let mut parsed_day: Option<u32> = None;

        // Use byte slices for efficient processing where possible (ASCII parts).
        // We need to handle the input string `s` as potentially UTF-8 when parsing %B.
        let mut s_bytes = s.as_bytes();
        let mut fmt_bytes = format.as_bytes();

        // Iterate through the format string bytes
        while !fmt_bytes.is_empty() {
            // Check if the current format byte is '%' indicating a specifier
            if fmt_bytes[0] == b'%' {
                // Ensure there's a character after '%'
                if fmt_bytes.len() < 2 {
                    return Err(DateError::ParseError(ParseErrorKind::FormatMismatch)); // Dangling %
                }

                // Match the specifier character (fmt_bytes[1])
                match fmt_bytes[1] {
                    // --- Literal '%%' ---
                    b'%' => {
                        // Input must also start with '%'
                        if s_bytes.is_empty() || s_bytes[0] != b'%' {
                            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                        }
                        // Consume '%' from input and '%%' from format
                        s_bytes = &s_bytes[1..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    // --- Year '%Y' (expects 4 digits) ---
                    b'Y' => {
                        // Check for 4 ASCII digits
                        if s_bytes.len() < 4 || !s_bytes[0..4].iter().all(|b| b.is_ascii_digit()) {
                            return Err(DateError::ParseError(ParseErrorKind::InvalidNumber));
                        }
                        // Parse the 4 digits (unsafe from_utf8 is safe here)
                        let year_str = unsafe { std::str::from_utf8_unchecked(&s_bytes[0..4]) };
                        parsed_year = Some(year_str.parse().map_err(|_| {
                            DateError::ParseError(ParseErrorKind::InvalidNumber) // Should not fail, but handle defensively
                        })?);
                        // Consume 4 digits from input and '%Y' from format
                        s_bytes = &s_bytes[4..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    // --- Month '%m' or Day '%d' (expects 2 digits) ---
                    b'm' | b'd' => {
                        // Check for 2 ASCII digits
                        if s_bytes.len() < 2 || !s_bytes[0..2].iter().all(|b| b.is_ascii_digit()) {
                            return Err(DateError::ParseError(ParseErrorKind::InvalidNumber));
                        }
                        // Parse the 2 digits (unsafe from_utf8 is safe)
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
                        // Consume 2 digits from input and '%m' or '%d' from format
                        s_bytes = &s_bytes[2..];
                        fmt_bytes = &fmt_bytes[2..];
                    }
                    // --- Month Name '%B' (expects Persian name) ---
                    b'B' => {
                        // Consume '%B' from format first
                        fmt_bytes = &fmt_bytes[2..];
                        let mut found_month = false;
                        let mut best_match_len = 0; // Length in *bytes* of the matched name
                        let mut matched_month_idx = 0; // 0-based index

                        // Need to compare against the input string slice `s` for UTF-8 names.
                        // Convert the *remaining* input bytes slice `s_bytes` to `&str` for matching.
                        let current_s_str = match std::str::from_utf8(s_bytes) {
                            Ok(s_str) => s_str,
                            // If remaining input isn't valid UTF-8, it can't match a Persian name.
                            Err(_) => {
                                return Err(DateError::ParseError(
                                    ParseErrorKind::InvalidMonthName,
                                ));
                            }
                        };

                        // Iterate through the known Persian month names
                        for (idx, month_name) in MONTH_NAMES_PERSIAN.iter().enumerate() {
                            // Check if the input string starts with this month name (case-sensitive)
                            if current_s_str.starts_with(month_name) {
                                // Found a match. Store its details.
                                best_match_len = month_name.len(); // Get byte length for slicing
                                matched_month_idx = idx;
                                found_month = true;
                                break; // Stop searching after the first match
                            }
                        }

                        if !found_month {
                            // No month name matched at the current input position.
                            return Err(DateError::ParseError(ParseErrorKind::InvalidMonthName));
                        }

                        // Store the parsed month number (1-based index)
                        parsed_month = Some((matched_month_idx + 1) as u32);
                        // Consume the matched month name (by its byte length) from the input byte slice.
                        s_bytes = &s_bytes[best_match_len..];
                        // `fmt_bytes` was already advanced past '%B'.
                    }
                    // --- Unsupported Specifiers for Parsing ---
                    b'A' | b'w' | b'j' => {
                        // Includes any other byte
                        // Specifiers like weekday, ordinal day are not supported for parsing.
                        return Err(DateError::ParseError(ParseErrorKind::UnsupportedSpecifier));
                    }
                    _ => {
                        return Err(DateError::ParseError(ParseErrorKind::UnsupportedSpecifier));
                    }
                }
            } else {
                // Literal character in the format string
                // Input must have the same literal character at the current position.
                if s_bytes.is_empty() || s_bytes[0] != fmt_bytes[0] {
                    // Input is shorter, or characters don't match.
                    return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
                }
                // Consume the matching literal character from both input and format.
                s_bytes = &s_bytes[1..];
                fmt_bytes = &fmt_bytes[1..];
            }
        } // End while loop over format bytes

        // After processing the entire format string, check if there are any unconsumed characters left in the input.
        if !s_bytes.is_empty() {
            // Input string has extra characters not accounted for by the format.
            return Err(DateError::ParseError(ParseErrorKind::FormatMismatch));
        }

        // Check if all necessary components (year, month, day) were successfully parsed from the input.
        match (parsed_year, parsed_month, parsed_day) {
            (Some(y), Some(m), Some(d)) => {
                // All components were extracted. Now, use the standard `ParsiDate::new` constructor
                // to perform final validation (logical date validity, e.g., day 31 in Mehr).
                ParsiDate::new(y, m, d).map_err(|e| {
                    // Map the validation error from `new` to the appropriate ParseErrorKind.
                    match e {
                        DateError::InvalidDate => {
                            DateError::ParseError(ParseErrorKind::InvalidDateValue)
                        }
                        // Propagate any other unexpected errors (less likely here).
                        other_error => other_error,
                    }
                })
            }
            // If any component is still None, the input string didn't provide all required parts matching the format.
            _ => Err(DateError::ParseError(ParseErrorKind::FormatMismatch)),
        }
    }

    // --- Date Information ---

    /// Returns the full Persian name of the weekday for this date (e.g., "شنبه", "یکشنبه", "دوشنبه", ...).
    ///
    /// This function calculates the weekday by converting the `ParsiDate` to its Gregorian equivalent
    /// and then using `chrono`'s weekday calculation. The result is mapped to the corresponding
    /// Persian weekday name, considering Saturday ("شنبه") as the first day of the week (index 0).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the `ParsiDate` instance itself holds invalid data
    /// (e.g., created via `unsafe new_unchecked` with bad values).
    /// Returns `Err(DateError::GregorianConversionError)` if the necessary conversion to a Gregorian
    /// date fails during the calculation process.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// // 1403/05/02 corresponds to Gregorian 2024-07-23 (Tuesday)
    /// let date_tue = ParsiDate::new(1403, 5, 2).unwrap();
    /// assert_eq!(date_tue.weekday(), Ok("سه‌شنبه".to_string()));
    ///
    /// // 1403/01/04 corresponds to Gregorian 2024-03-23 (Saturday)
    /// let date_sat = ParsiDate::new(1403, 1, 4).unwrap();
    /// assert_eq!(date_sat.weekday(), Ok("شنبه".to_string()));
    ///
    /// // 1403/01/10 corresponds to Gregorian 2024-03-29 (Friday)
    /// let date_fri = ParsiDate::new(1403, 1, 10).unwrap();
    /// assert_eq!(date_fri.weekday(), Ok("جمعه".to_string()));
    ///
    /// // Example with invalid date
    /// let invalid_date = unsafe { ParsiDate::new_unchecked(1404, 12, 30) };
    /// assert!(invalid_date.weekday().is_err());
    /// ```
    pub fn weekday(&self) -> Result<String, DateError> {
        // Delegate to the internal implementation which includes validation.
        self.weekday_internal()
    }

    /// **Internal**: Calculates and returns the Persian weekday name. Includes validation.
    ///
    /// This helper exists to share logic and ensures validation occurs before calculation.
    /// Returns `Result` to propagate errors from validation or calculation.
    // Marked pub(crate) as it's primarily internal logic.
    pub(crate) fn weekday_internal(&self) -> Result<String, DateError> {
        // 1. Ensure the date itself is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // 2. Calculate the numerical weekday (Saturday=0, ..., Friday=6).
        // This can return an error if Gregorian conversion fails.
        let day_num_sat_0 = self.weekday_num_sat_0()?;
        // 3. Get the name from the constant array using the calculated index.
        // This indexing should be safe (0-6) if weekday_num_sat_0 returned Ok.
        WEEKDAY_NAMES_PERSIAN
            .get(day_num_sat_0 as usize) // Convert u32 index to usize
            .map(|s| s.to_string()) // Convert the found &str to String
            // If `get` somehow fails (e.g., index out of bounds, which shouldn't happen here),
            // map it to a relevant error type. GregorianConversionError implies something went wrong in the process.
            .ok_or(DateError::GregorianConversionError)
    }

    /// **Internal**: Calculates the weekday as a number (Saturday=0, ..., Friday=6). Includes validation.
    ///
    /// This helper converts to Gregorian, gets chrono's weekday number (Sun=0..Sat=6),
    /// and remaps it to the Persian convention (Sat=0..Fri=6).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if `self` is invalid.
    /// Returns `Err(DateError::GregorianConversionError)` if the `to_gregorian_internal` conversion fails.
    // Marked pub(crate) as it's primarily internal logic.
    pub(crate) fn weekday_num_sat_0(&self) -> Result<u32, DateError> {
        // 1. Ensure the date is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // 2. Convert to Gregorian date using the internal method (avoids double validation).
        // This can return GregorianConversionError.
        let gregorian_date = self.to_gregorian_internal()?;

        // 3. Get chrono's weekday number (Sunday=0, Monday=1, ..., Saturday=6).
        let day_num_sun0 = gregorian_date.weekday().num_days_from_sunday();

        // 4. Remap chrono's Sunday=0..Saturday=6 to Persian Saturday=0..Friday=6.
        // The mapping is: (chrono_num + 1) % 7
        // Sun (0) -> (0+1)%7 = 1 (EkShanbe)
        // Mon (1) -> (1+1)%7 = 2 (DoShanbe)
        // ...
        // Fri (5) -> (5+1)%7 = 6 (Jomeh)
        // Sat (6) -> (6+1)%7 = 0 (Shanbeh)
        let day_num_sat0 = (day_num_sun0 + 1) % 7;

        Ok(day_num_sat0)
    }

    /// Calculates the day number within the year, also known as the ordinal day.
    ///
    /// Counts days starting from 1 for Farvardin 1st. The result will be between 1 and 365
    /// for a common Persian year, or between 1 and 366 for a leap year.
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if the `ParsiDate` instance holds invalid data
    /// (e.g., created via `unsafe new_unchecked` with month 0 or day 0).
    /// Returns `Err(DateError::ArithmeticOverflow)` if an internal overflow occurs during the
    /// summation of days (highly unlikely for days within a single year using u32).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// // First day of the year
    /// assert_eq!(ParsiDate::new(1403, 1, 1).unwrap().ordinal(), Ok(1));
    ///
    /// // Second day of the year
    /// assert_eq!(ParsiDate::new(1403, 1, 2).unwrap().ordinal(), Ok(2));
    ///
    /// // First day of the second month (Ordibehesht)
    /// // Comes after Farvardin (31 days)
    /// assert_eq!(ParsiDate::new(1403, 2, 1).unwrap().ordinal(), Ok(32));
    ///
    /// // Last day of a leap year (1403 is leap)
    /// assert_eq!(ParsiDate::new(1403, 12, 30).unwrap().ordinal(), Ok(366));
    ///
    /// // Last day of a common year (1404 is common)
    /// assert_eq!(ParsiDate::new(1404, 12, 29).unwrap().ordinal(), Ok(365));
    ///
    /// // Example with invalid date
    /// let invalid_date = unsafe { ParsiDate::new_unchecked(1403, 0, 1) }; // Invalid month
    /// assert!(invalid_date.ordinal().is_err());
    /// ```
    pub fn ordinal(&self) -> Result<u32, DateError> {
        // Delegate to the internal implementation which includes validation.
        self.ordinal_internal()
    }

    /// **Internal**: Calculates the ordinal day (day number within the year). Includes validation.
    ///
    /// Assumes `self` might be invalid and performs checks before calculation.
    /// Returns `Result` to propagate errors from validation or potential (though unlikely) overflow.
    // Marked pub(crate) as it's primarily internal logic.
    pub(crate) fn ordinal_internal(&self) -> Result<u32, DateError> {
        // 1. Ensure the date itself is valid before starting calculations.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }

        // 2. Get the lengths of all months for the current year.
        let month_lengths = Self::month_lengths(self.year);
        let mut accumulated_days: u32 = 0;

        // 3. Sum the lengths of all full months *preceding* the current month.
        // `self.month` is 1-based. Loop from index 0 up to `self.month - 2`.
        if self.month > 1 {
            // Iterate through the lengths of months from Farvardin up to the one before self.month.
            // Example: If self.month is 3 (Khordad), loop runs for indices 0 (Far) and 1 (Ord).
            for days in month_lengths.iter().take((self.month - 1) as usize) {
                // Use checked_add for safety against potential u32 overflow (very unlikely here).
                accumulated_days = accumulated_days
                    .checked_add(*days)
                    .ok_or(DateError::ArithmeticOverflow)?;
            }
        }

        // 4. Add the day of the current month to the accumulated total.
        // `self.day` is 1-based, so adding it directly gives the correct 1-based ordinal day.
        accumulated_days = accumulated_days
            .checked_add(self.day)
            .ok_or(DateError::ArithmeticOverflow)?; // Safety check

        // The result is the 1-based ordinal day number.
        Ok(accumulated_days)
    }

    // --- Arithmetic ---

    /// Adds a specified number of days to this `ParsiDate`, returning a new `ParsiDate`.
    ///
    /// This operation correctly handles crossing month and year boundaries, including leap years.
    /// It works by converting the `ParsiDate` to its Gregorian equivalent (`NaiveDate`),
    /// performing the day addition using `chrono`'s reliable arithmetic, and then converting
    /// the resulting Gregorian date back to `ParsiDate`.
    ///
    /// The input `days` can be positive to move forward in time or negative to move backward.
    ///
    /// # Arguments
    ///
    /// * `days`: The number of days to add. A positive value moves the date forward,
    ///   a negative value moves it backward.
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// *   `DateError::InvalidDate`: The starting `ParsiDate` (`self`) is invalid.
    /// *   `DateError::GregorianConversionError`: The initial conversion to Gregorian or the final
    ///     conversion back to Persian fails (e.g., due to epoch issues, though unlikely for valid dates).
    /// *   `DateError::ArithmeticOverflow`: The date arithmetic results in a Gregorian date outside
    ///     the range supported by `chrono::NaiveDate`, or the final Persian date falls outside the
    ///     supported year range [1, 9999].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 12, 28).unwrap(); // 1403 is a leap year
    ///
    /// // Add a few days within the same month/year
    /// assert_eq!(date.add_days(1), Ok(ParsiDate::new(1403, 12, 29).unwrap()));
    /// assert_eq!(date.add_days(2), Ok(ParsiDate::new(1403, 12, 30).unwrap())); // Hit leap day
    ///
    /// // Add days to cross into the next year
    /// assert_eq!(date.add_days(3), Ok(ParsiDate::new(1404, 1, 1).unwrap())); // 1403/12/30 + 1 day
    /// assert_eq!(date.add_days(10), Ok(ParsiDate::new(1404, 1, 8).unwrap()));
    ///
    /// // Subtract days (using negative input)
    /// let date_start_year = ParsiDate::new(1404, 1, 1).unwrap();
    /// assert_eq!(date_start_year.add_days(-1), Ok(ParsiDate::new(1403, 12, 30).unwrap())); // Back to leap day
    /// assert_eq!(date_start_year.add_days(-2), Ok(ParsiDate::new(1403, 12, 29).unwrap()));
    ///
    /// // Subtract a larger number of days
    /// assert_eq!(date_start_year.add_days(-366), Ok(ParsiDate::new(1403, 1, 1).unwrap())); // Subtract full leap year
    ///
    /// // Example resulting in error (e.g., going before year 1)
    /// let early_date = ParsiDate::new(1, 1, 1).unwrap();
    /// assert!(early_date.add_days(-1).is_err()); // Cannot go before 1/1/1
    /// ```
    pub fn add_days(&self, days: i64) -> Result<Self, DateError> {
        // 1. Validate the starting date.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }

        // 2. Convert the ParsiDate to its Gregorian equivalent. Use internal method for efficiency.
        let gregorian_equiv = self.to_gregorian_internal()?;

        // 3. Perform the day addition/subtraction using chrono's checked arithmetic.
        // `checked_add_days` and `checked_sub_days` return Option<NaiveDate>, None on overflow/underflow.
        let new_gregorian = if days >= 0 {
            // Adding days: Convert positive i64 to u64 for chrono::Days.
            gregorian_equiv.checked_add_days(chrono::Days::new(days as u64))
        } else {
            // Subtracting days: Convert negative i64 to positive u64 magnitude.
            // `checked_abs` handles potential i64::MIN. `abs()` is usually sufficient.
            // `try_into()` can also safely convert positive i64 to u64.
            let days_to_sub = days.checked_abs().ok_or(DateError::ArithmeticOverflow)? as u64;
            gregorian_equiv.checked_sub_days(chrono::Days::new(days_to_sub))
        };

        // Map chrono's Option result: None -> ArithmeticOverflow error.
        let new_gregorian = new_gregorian.ok_or(DateError::ArithmeticOverflow)?;

        // 4. Convert the resulting Gregorian date back to ParsiDate.
        // This step implicitly checks if the result falls within the supported ParsiDate range [1, 9999].
        Self::from_gregorian(new_gregorian)
    }

    /// Subtracts a specified number of days from this `ParsiDate`, returning a new `ParsiDate`.
    ///
    /// This is a convenience method equivalent to calling `add_days` with a negative value (`-days`).
    /// It uses the same underlying conversion and `chrono` arithmetic process as `add_days`.
    ///
    /// # Arguments
    ///
    /// * `days`: The non-negative number of days to subtract.
    ///
    /// # Errors
    ///
    /// Returns `Err` under the same conditions as [`add_days`]:
    /// *   `DateError::InvalidDate`: The starting `ParsiDate` (`self`) is invalid.
    /// *   `DateError::GregorianConversionError`: Conversion to/from Gregorian fails.
    /// *   `DateError::ArithmeticOverflow`: The subtraction results in a date outside the representable
    ///     range (either in `chrono` or the final `ParsiDate` year range), or if `days` exceeds `i64::MAX`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1404, 1, 3).unwrap(); // Farvardin 3rd, 1404 (common year)
    ///
    /// // Subtract a few days
    /// assert_eq!(date.sub_days(1), Ok(ParsiDate::new(1404, 1, 2).unwrap()));
    /// assert_eq!(date.sub_days(2), Ok(ParsiDate::new(1404, 1, 1).unwrap()));
    ///
    /// // Subtract to cross into the previous year (1403 was leap)
    /// assert_eq!(date.sub_days(3), Ok(ParsiDate::new(1403, 12, 30).unwrap())); // Lands on leap day
    /// assert_eq!(date.sub_days(4), Ok(ParsiDate::new(1403, 12, 29).unwrap()));
    ///
    /// // Subtract a larger number
    /// let date_mid_year = ParsiDate::new(1403, 6, 15).unwrap(); // Shahrivar 15th
    /// assert_eq!(date_mid_year.sub_days(100), Ok(ParsiDate::new(1403, 3, 8).unwrap())); // Back to Khordad 8th
    ///
    /// // Example resulting in error (going before year 1)
    /// let early_date = ParsiDate::new(1, 1, 1).unwrap();
    /// assert!(early_date.sub_days(1).is_err());
    /// ```
    pub fn sub_days(&self, days: u64) -> Result<Self, DateError> {
        // Convert the non-negative u64 `days` to subtract into a negative i64 value
        // suitable for the `add_days` method.
        // We need to ensure the u64 value doesn't exceed i64::MAX before negation to avoid overflow.
        if days > i64::MAX as u64 {
            // A u64 this large is practically guaranteed to cause overflow in date calculation anyway.
            return Err(DateError::ArithmeticOverflow);
        }
        // Safely cast to i64 and negate.
        let days_as_neg_i64 = -(days as i64);
        // Delegate the work to add_days with the negative value.
        self.add_days(days_as_neg_i64)
    }

    /// Adds a specified number of months to this `ParsiDate`, returning a new `ParsiDate`.
    ///
    /// This operation adjusts the month and, if necessary, the year. A crucial aspect is
    /// **day clamping**: If the original day of the month is greater than the number of days
    /// in the target month (after adding `months_to_add`), the day in the resulting `ParsiDate`
    /// will be set to the last valid day of that target month. For example, adding 1 month
    /// to Farvardin 31st (1/31) results in Ordibehesht 31st (2/31), but adding 6 months
    /// results in Mehr 30th (7/30), because Mehr only has 30 days.
    ///
    /// The input `months_to_add` can be positive or negative.
    ///
    /// # Arguments
    ///
    /// * `months_to_add`: The number of months to add. Positive moves forward, negative moves backward.
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// *   `DateError::InvalidDate`: The starting `ParsiDate` (`self`) is invalid.
    /// *   `DateError::ArithmeticOverflow`: The calculation results in a year outside the
    ///     supported range [1, 9999], or an internal integer overflow occurs during month/year calculation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 1, 31).unwrap(); // Farvardin 31st
    ///
    /// // Add 1 month: -> Ordibehesht 31st (month 2 has 31 days)
    /// assert_eq!(date.add_months(1), Ok(ParsiDate::new(1403, 2, 31).unwrap()));
    ///
    /// // Add 6 months: -> Mehr 30th (month 7 has 30 days, day clamped from 31)
    /// assert_eq!(date.add_months(6), Ok(ParsiDate::new(1403, 7, 30).unwrap()));
    ///
    /// // Add 12 months: -> Farvardin 31st of next year
    /// assert_eq!(date.add_months(12), Ok(ParsiDate::new(1404, 1, 31).unwrap()));
    ///
    /// // Add 13 months: -> Ordibehesht 31st of next year
    /// assert_eq!(date.add_months(13), Ok(ParsiDate::new(1404, 2, 31).unwrap()));
    ///
    /// // Subtract months (using negative input)
    /// let date_mid = ParsiDate::new(1403, 7, 15).unwrap(); // Mehr 15th
    /// assert_eq!(date_mid.add_months(-1), Ok(ParsiDate::new(1403, 6, 15).unwrap())); // -> Shahrivar 15th
    /// assert_eq!(date_mid.add_months(-7), Ok(ParsiDate::new(1402, 12, 15).unwrap())); // -> Esfand 15th prev year
    ///
    /// // Subtract months resulting in clamping
    /// let date_end_mehr = ParsiDate::new(1403, 7, 30).unwrap(); // Mehr 30th
    /// // Subtract 1 month -> Shahrivar (31 days). Day 30 is valid. -> 1403/06/30
    /// assert_eq!(date_end_mehr.add_months(-1), Ok(ParsiDate::new(1403, 6, 30).unwrap()));
    ///
    /// // Example resulting in year overflow
    /// let max_date = ParsiDate::new(9999, 1, 1).unwrap();
    /// assert!(max_date.add_months(12).is_err()); // Adding 1 year would exceed 9999
    /// let min_date = ParsiDate::new(1, 1, 1).unwrap();
    /// assert!(min_date.add_months(-1).is_err()); // Subtracting would go before year 1
    /// ```
    pub fn add_months(&self, months_to_add: i32) -> Result<Self, DateError> {
        // 1. Validate the starting date.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Optimization: If adding 0 months, return the original date cloned.
        if months_to_add == 0 {
            return Ok(*self);
        }

        // 2. Calculate the target year and month.
        // Use 0-based month index for easier calculation (0=Farvardin .. 11=Esfand).
        let current_year = self.year;
        let current_month0 = self.month as i32 - 1; // 0 to 11

        // Calculate the total number of months from year 0, month 0, conceptually.
        // Need i64 to avoid overflow for large year/month combinations.
        let total_months_from_origin =
            (current_year as i64 * 12) + current_month0 as i64 + months_to_add as i64;

        // Calculate the target year and 0-based month index using Euclidean division/remainder.
        let target_year_abs = total_months_from_origin.div_euclid(12);
        let target_month0 = total_months_from_origin.rem_euclid(12); // result is always 0..11

        // 3. Check if the target year is within the supported range [1, 9999].
        // Also check if it fits within i32.
        if target_year_abs < MIN_PARSI_DATE.year as i64
            || target_year_abs > MAX_PARSI_DATE.year as i64
        {
            return Err(DateError::ArithmeticOverflow); // Target year out of supported range
        }
        // Cast target year back to i32 (safe due to the check above).
        let target_year = target_year_abs as i32;
        // Convert 0-based month index back to 1-based month number (1..12).
        let target_month = (target_month0 + 1) as u32;

        // 4. Determine the maximum valid day in the target month and year.
        let max_days_in_target_month = Self::days_in_month(target_year, target_month);
        // days_in_month returns 0 for invalid months, but target_month should be 1..12 here.
        // If max_days is 0, it indicates an internal logic error.
        if max_days_in_target_month == 0 {
            return Err(DateError::InvalidDate); // Should not happen if target_month is correct.
        }

        // 5. Clamp the day: The resulting day is the minimum of the original day and the max valid day in the target month.
        let target_day = self.day.min(max_days_in_target_month);

        // 6. Use ParsiDate::new for final construction and validation.
        // This primarily re-validates the year range and ensures consistency.
        // The calculated target_month and target_day should be logically valid by this point.
        ParsiDate::new(target_year, target_month, target_day)
    }

    /// Subtracts a specified number of months from this `ParsiDate`, returning a new `ParsiDate`.
    ///
    /// This is a convenience method equivalent to calling `add_months` with a negative value (`-months_to_sub`).
    /// It handles month/year adjustments and day clamping similarly to `add_months`.
    ///
    /// # Arguments
    ///
    /// * `months_to_sub`: The non-negative number of months to subtract.
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// *   `DateError::InvalidDate`: The starting `ParsiDate` (`self`) is invalid.
    /// *   `DateError::ArithmeticOverflow`: `months_to_sub` is larger than `i32::MAX`, or the calculation
    ///     results in a year outside the supported range [1, 9999].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 3, 31).unwrap(); // Khordad 31st
    ///
    /// // Subtract 1 month -> Ordibehesht (31 days). Day 31 is valid. -> 1403/02/31
    /// assert_eq!(date.sub_months(1), Ok(ParsiDate::new(1403, 2, 31).unwrap()));
    ///
    /// // Subtract 3 months -> Esfand prev year (1402 is common, 29 days). Day clamped to 29. -> 1402/12/29
    /// assert_eq!(date.sub_months(3), Ok(ParsiDate::new(1402, 12, 29).unwrap()));
    ///
    /// // Subtract 12 months -> Khordad 31st prev year
    /// assert_eq!(date.sub_months(12), Ok(ParsiDate::new(1402, 3, 31).unwrap()));
    ///
    /// // Example resulting in year underflow
    /// let min_date = ParsiDate::new(1, 1, 1).unwrap();
    /// assert!(min_date.sub_months(1).is_err()); // Cannot go before year 1
    /// ```
    pub fn sub_months(&self, months_to_sub: u32) -> Result<Self, DateError> {
        // Check if the u32 value exceeds the maximum positive value representable by i32.
        // If it does, negating it would cause overflow.
        if months_to_sub > i32::MAX as u32 {
            return Err(DateError::ArithmeticOverflow);
        }
        // Safely cast to i32 and negate.
        let months_as_neg_i32 = -(months_to_sub as i32);
        // Delegate the work to add_months with the negative value.
        self.add_months(months_as_neg_i32)
    }

    /// Adds a specified number of years to this `ParsiDate`, returning a new `ParsiDate`.
    ///
    /// This operation adjusts the year component. It includes special handling for the
    /// Persian leap day (Esfand 30th): If the original date is Esfand 30th (which only
    /// occurs in a leap year) and the target year (after adding `years_to_add`) is *not*
    /// a leap year, the day in the resulting `ParsiDate` will be clamped to 29 (the last
    /// day of Esfand in a common year). In all other cases, the month and day remain unchanged.
    ///
    /// The input `years_to_add` can be positive or negative.
    ///
    /// # Arguments
    ///
    /// * `years_to_add`: The number of years to add. Positive moves forward, negative moves backward.
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// *   `DateError::InvalidDate`: The starting `ParsiDate` (`self`) is invalid.
    /// *   `DateError::ArithmeticOverflow`: The calculation results in a year outside the
    ///     supported range [1, 9999].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1400, 5, 10).unwrap(); // 1400 is leap year
    ///
    /// // Add years, month/day stay the same
    /// assert_eq!(date.add_years(3), Ok(ParsiDate::new(1403, 5, 10).unwrap())); // 1403 is leap
    /// assert_eq!(date.add_years(4), Ok(ParsiDate::new(1404, 5, 10).unwrap())); // 1404 is common
    ///
    /// // Subtract years (using negative input)
    /// assert_eq!(date.add_years(-1), Ok(ParsiDate::new(1399, 5, 10).unwrap())); // 1399 is leap
    ///
    /// // --- Leap Day Handling ---
    /// let leap_day = ParsiDate::new(1403, 12, 30).unwrap(); // Esfand 30th in leap year 1403
    ///
    /// // Add 1 year -> 1404 (common year). Day clamped from 30 to 29. -> 1404/12/29
    /// assert_eq!(leap_day.add_years(1), Ok(ParsiDate::new(1404, 12, 29).unwrap()));
    ///
    /// // Add 4 years -> 1407 (leap year). Day remains 30. -> 1407/12/30
    /// assert_eq!(leap_day.add_years(4), Ok(ParsiDate::new(1407, 12, 29).unwrap())); // 1407 is common
    ///
    /// // Add 5 years -> 1408 (common year). Day clamped from 30 to 29. -> 1408/12/29
    /// assert_eq!(leap_day.add_years(5), Ok(ParsiDate::new(1408, 12, 30).unwrap())); // 1408 is leap
    ///
    /// // Subtract 4 years from leap day -> 1399 (leap year). Day remains 30. -> 1399/12/30
    /// assert_eq!(leap_day.add_years(-4), Ok(ParsiDate::new(1399, 12, 30).unwrap()));
    ///
    /// // Example resulting in year range error
    /// let max_date = ParsiDate::new(9999, 1, 1).unwrap();
    /// assert!(max_date.add_years(1).is_err());
    /// let min_date = ParsiDate::new(1, 1, 1).unwrap();
    /// assert!(min_date.add_years(-1).is_err());
    /// ```
    pub fn add_years(&self, years_to_add: i32) -> Result<Self, DateError> {
        // 1. Validate the starting date.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // Optimization: Adding 0 years changes nothing.
        if years_to_add == 0 {
            return Ok(*self);
        }

        // 2. Calculate the target year using checked arithmetic to detect potential overflow.
        let target_year = self
            .year
            .checked_add(years_to_add)
            .ok_or(DateError::ArithmeticOverflow)?; // Map Option::None to our error

        // 3. Check if the calculated target year falls outside the supported range [1, 9999].
        if !(MIN_PARSI_DATE.year..=MAX_PARSI_DATE.year).contains(&target_year) {
            return Err(DateError::ArithmeticOverflow); // Year out of bounds
        }

        // 4. Handle the leap day clamping logic.
        let mut target_day = self.day;
        // Check if the original date is specifically Esfand 30th (only possible in a leap year).
        if self.month == 12 && self.day == 30 {
            // If the target year is NOT a leap year, we must clamp the day to 29.
            if !Self::is_persian_leap_year(target_year) {
                target_day = 29;
            }
            // If the target year *is* also a leap year, target_day remains 30 (no change needed).
        }
        // If the original day was not Esfand 30th, target_day remains self.day.

        // 5. Use ParsiDate::new for final construction and validation.
        // This ensures the final combination (potentially with clamped day) is valid.
        ParsiDate::new(target_year, self.month, target_day)
    }

    /// Subtracts a specified number of years from this `ParsiDate`, returning a new `ParsiDate`.
    ///
    /// This is a convenience method equivalent to calling `add_years` with a negative value (`-years_to_sub`).
    /// It includes the same leap day handling as `add_years`: if the original date is Esfand 30th
    /// and the target year (after subtraction) is not a leap year, the day is clamped to 29.
    ///
    /// # Arguments
    ///
    /// * `years_to_sub`: The non-negative number of years to subtract.
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// *   `DateError::InvalidDate`: The starting `ParsiDate` (`self`) is invalid.
    /// *   `DateError::ArithmeticOverflow`: `years_to_sub` is larger than `i32::MAX`, or the calculation
    ///     results in a year outside the supported range [1, 9999].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 10).unwrap(); // 1403 is leap
    ///
    /// // Subtract years
    /// assert_eq!(date.sub_years(1), Ok(ParsiDate::new(1402, 5, 10).unwrap())); // 1402 common
    /// assert_eq!(date.sub_years(4), Ok(ParsiDate::new(1399, 5, 10).unwrap())); // 1399 leap
    ///
    /// // --- Leap Day Handling ---
    /// let leap_day = ParsiDate::new(1403, 12, 30).unwrap(); // Esfand 30th in leap year 1403
    ///
    /// // Subtract 1 year -> 1402 (common year). Day clamped from 30 to 29. -> 1402/12/29
    /// assert_eq!(leap_day.sub_years(1), Ok(ParsiDate::new(1402, 12, 29).unwrap()));
    ///
    /// // Subtract 4 years -> 1399 (leap year). Day remains 30. -> 1399/12/30
    /// assert_eq!(leap_day.sub_years(4), Ok(ParsiDate::new(1399, 12, 30).unwrap()));
    ///
    /// // Example resulting in year range error
    /// let min_date = ParsiDate::new(1, 1, 1).unwrap();
    /// assert!(min_date.sub_years(1).is_err()); // Cannot go below year 1
    /// ```
    pub fn sub_years(&self, years_to_sub: u32) -> Result<Self, DateError> {
        // Check if the u32 value can be safely negated within i32 range.
        if years_to_sub > i32::MAX as u32 {
            return Err(DateError::ArithmeticOverflow);
        }
        // Safely cast to i32 and negate.
        let years_as_neg_i32 = -(years_to_sub as i32);
        // Delegate to add_years with the negative value.
        self.add_years(years_as_neg_i32)
    }

    /// Calculates the absolute difference in days between this `ParsiDate` and another `ParsiDate`.
    ///
    /// This method determines the number of days separating the two dates, regardless of which
    /// date comes first. The calculation is performed by converting both `ParsiDate` instances
    /// to their Gregorian `NaiveDate` equivalents and then using `chrono`'s duration calculation.
    ///
    /// # Arguments
    ///
    /// * `other`: A reference to the other `ParsiDate` instance to compare against.
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// *   `DateError::InvalidDate`: Either `self` or `other` represents an invalid date.
    /// *   `DateError::GregorianConversionError`: The conversion of either `self` or `other`
    ///     to `NaiveDate` fails.
    ///
    /// # Returns
    ///
    /// The absolute difference between the two dates, measured in days, as an `i64`. Returns
    /// `Ok(0)` if both dates are the same.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let d1 = ParsiDate::new(1403, 1, 1).unwrap();
    /// let d2 = ParsiDate::new(1403, 1, 11).unwrap();
    /// let d3 = ParsiDate::new(1404, 1, 1).unwrap(); // Next year (1403 is leap, 366 days)
    /// let d4 = ParsiDate::new(1402, 12, 29).unwrap(); // Day before d1 (1402 is common)
    ///
    /// // Difference within the same month
    /// assert_eq!(d1.days_between(&d2), Ok(10));
    /// assert_eq!(d2.days_between(&d1), Ok(10)); // Order doesn't matter for absolute difference
    ///
    /// // Difference across a year boundary (leap year)
    /// assert_eq!(d1.days_between(&d3), Ok(366));
    /// assert_eq!(d3.days_between(&d1), Ok(366));
    ///
    /// // Difference of one day
    /// assert_eq!(d1.days_between(&d4), Ok(1));
    /// assert_eq!(d4.days_between(&d1), Ok(1));
    ///
    /// // Difference with itself
    /// assert_eq!(d1.days_between(&d1), Ok(0));
    ///
    /// // Example with invalid date
    /// let invalid_date = unsafe { ParsiDate::new_unchecked(1400, 13, 1) };
    /// assert!(d1.days_between(&invalid_date).is_err());
    /// ```
    pub fn days_between(&self, other: &ParsiDate) -> Result<i64, DateError> {
        // 1. Validate both input dates first.
        if !self.is_valid() || !other.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // 2. Convert both dates to Gregorian using internal helpers (avoids re-validation).
        let gregorian_self = self.to_gregorian_internal()?;
        let gregorian_other = other.to_gregorian_internal()?;

        // 3. Calculate the signed duration between the Gregorian dates using chrono.
        let duration = gregorian_self.signed_duration_since(gregorian_other);

        // 4. Return the absolute number of days from the duration.
        Ok(duration.num_days().abs())
    }

    // --- Helper Methods ---

    /// Creates a new `ParsiDate` instance with only the year component changed.
    ///
    /// This method sets the year to the specified `year` value, keeping the original `month`
    /// and `day`. It includes the necessary check for the Esfand 30th leap day: if the
    /// original date is Esfand 30th and the target `year` is not a leap year, the day
    /// in the new `ParsiDate` is automatically adjusted to 29.
    ///
    /// # Arguments
    ///
    /// * `year`: The desired year for the new date (must be within the range 1-9999).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if:
    /// *   The starting `ParsiDate` (`self`) is invalid.
    /// *   The target `year` is outside the supported range [1, 9999].
    /// *   The resulting combination (after potential day adjustment) forms an invalid date
    ///     (this should generally not happen if the target year is valid, as `new` handles it).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// let date = ParsiDate::new(1403, 5, 2).unwrap(); // Mordad 2nd, 1403 (leap)
    ///
    /// // Change year, keeping month/day
    /// let date_next_year = date.with_year(1404); // 1404 is common
    /// assert!(date_next_year.is_ok());
    /// assert_eq!(date_next_year.unwrap(), ParsiDate::new(1404, 5, 2).unwrap());
    ///
    /// // --- Leap Day Handling ---
    /// let leap_day = ParsiDate::new(1403, 12, 30).unwrap(); // Esfand 30th, 1403 (leap)
    ///
    /// // Change to a common year -> day clamped to 29
    /// let common_year_date = leap_day.with_year(1404); // 1404 is common
    /// assert!(common_year_date.is_ok());
    /// assert_eq!(common_year_date.unwrap(), ParsiDate::new(1404, 12, 29).unwrap());
    ///
    /// // Change to another leap year -> day remains 30
    /// let another_leap_year_date = leap_day.with_year(1407); // 1407 is Common
    /// assert!(another_leap_year_date.is_ok());
    /// assert_eq!(another_leap_year_date.unwrap(), ParsiDate::new(1407, 12, 29).unwrap());
    ///
    /// // --- Error Cases ---
    /// // Target year out of range
    /// assert_eq!(date.with_year(0), Err(DateError::InvalidDate));
    /// assert_eq!(date.with_year(10000), Err(DateError::InvalidDate));
    ///
    /// // Starting date is invalid
    /// let invalid_start = unsafe { ParsiDate::new_unchecked(1400, 13, 1) };
    /// assert_eq!(invalid_start.with_year(1401), Err(DateError::InvalidDate));
    /// ```
    pub fn with_year(&self, year: i32) -> Result<Self, DateError> {
        // 1. Validate the starting date first.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // 2. Validate the target year range immediately.
        if !(MIN_PARSI_DATE.year..=MAX_PARSI_DATE.year).contains(&year) {
            // Map out-of-range year to InvalidDate for consistency.
            return Err(DateError::InvalidDate);
        }

        // 3. Check for and apply leap day adjustment if necessary.
        let mut day = self.day;
        if self.month == 12 && self.day == 30 && !Self::is_persian_leap_year(year) {
            // Original is Esfand 30th (must be leap year), target year is not leap. Clamp day to 29.
            day = 29;
        }

        // 4. Use the safe ParsiDate::new constructor.
        // This validates the final combination (target year, original month, potentially adjusted day).
        ParsiDate::new(year, self.month, day)
    }

    /// Creates a new `ParsiDate` instance with only the month component changed.
    ///
    /// This method sets the month to the specified `month` value, keeping the original `year`
    /// and `day`. It includes **day clamping**: if the original `day` is greater than the
    /// number of days in the target `month` (for the same `year`), the day in the new
    /// `ParsiDate` is adjusted downward to the last valid day of that target month.
    ///
    /// # Arguments
    ///
    /// * `month`: The desired month for the new date (must be between 1 and 12).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if:
    /// *   The starting `ParsiDate` (`self`) is invalid.
    /// *   The target `month` is outside the valid range [1, 12].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// let date = ParsiDate::new(1403, 1, 31).unwrap(); // Farvardin 31st
    ///
    /// // Change month to one with 31 days -> day remains 31
    /// let date_ordibehesht = date.with_month(2); // Ordibehesht also has 31 days
    /// assert!(date_ordibehesht.is_ok());
    /// assert_eq!(date_ordibehesht.unwrap(), ParsiDate::new(1403, 2, 31).unwrap());
    ///
    /// // Change month to one with 30 days -> day clamped to 30
    /// let date_mehr = date.with_month(7); // Mehr has 30 days
    /// assert!(date_mehr.is_ok());
    /// assert_eq!(date_mehr.unwrap(), ParsiDate::new(1403, 7, 30).unwrap());
    ///
    /// // Change month to Esfand in a leap year -> day clamped to 30
    /// let date_esfand_leap = date.with_month(12); // 1403 is leap, Esfand has 30 days
    /// assert!(date_esfand_leap.is_ok());
    /// assert_eq!(date_esfand_leap.unwrap(), ParsiDate::new(1403, 12, 30).unwrap());
    ///
    /// // Change month to Esfand in a common year
    /// let date_common_year = ParsiDate::new(1404, 1, 31).unwrap(); // Farvardin 31st, 1404 (common)
    /// let date_esfand_common = date_common_year.with_month(12); // 1404 common, Esfand has 29 days
    /// assert!(date_esfand_common.is_ok());
    /// assert_eq!(date_esfand_common.unwrap(), ParsiDate::new(1404, 12, 29).unwrap()); // Clamped to 29
    ///
    /// // --- Error Cases ---
    /// // Target month out of range
    /// assert_eq!(date.with_month(0), Err(DateError::InvalidDate));
    /// assert_eq!(date.with_month(13), Err(DateError::InvalidDate));
    ///
    /// // Starting date is invalid
    /// let invalid_start = unsafe { ParsiDate::new_unchecked(1400, 1, 32) };
    /// assert_eq!(invalid_start.with_month(2), Err(DateError::InvalidDate));
    /// ```
    pub fn with_month(&self, month: u32) -> Result<Self, DateError> {
        // 1. Validate the starting date.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // 2. Validate the target month range immediately.
        if !(1..=12).contains(&month) {
            return Err(DateError::InvalidDate); // Invalid target month number
        }

        // 3. Determine the maximum valid day for the target month in the original year.
        let max_days = Self::days_in_month(self.year, month);
        // days_in_month returns 0 for invalid months, but we checked month is 1-12.
        // If it returned 0 here, it implies an internal inconsistency.
        if max_days == 0 {
            return Err(DateError::InvalidDate); // Should not happen with valid month 1-12
        }

        // 4. Clamp the original day to the maximum allowed day of the target month.
        let day = self.day.min(max_days);

        // 5. Use the safe ParsiDate::new constructor for final validation.
        ParsiDate::new(self.year, month, day)
    }

    /// Creates a new `ParsiDate` instance with only the day component changed.
    ///
    /// This method sets the day to the specified `day` value, keeping the original `year`
    /// and `month`. It performs validation to ensure the target `day` is valid for the
    /// existing year and month.
    ///
    /// # Arguments
    ///
    /// * `day`: The desired day of the month (must be valid for the current year and month, typically 1-31).
    ///
    /// # Errors
    ///
    /// Returns `Err(DateError::InvalidDate)` if:
    /// *   The starting `ParsiDate` (`self`) is invalid.
    /// *   The target `day` is 0 or greater than the number of days allowed in the
    ///     current month and year (e.g., setting day to 31 in Mehr, or 30 in Esfand of a common year).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// let date = ParsiDate::new(1403, 7, 1).unwrap(); // Mehr 1st (30 days)
    ///
    /// // Change day within the valid range
    /// let date_mid_month = date.with_day(15);
    /// assert!(date_mid_month.is_ok());
    /// assert_eq!(date_mid_month.unwrap(), ParsiDate::new(1403, 7, 15).unwrap());
    ///
    /// let date_end_month = date.with_day(30);
    /// assert!(date_end_month.is_ok());
    /// assert_eq!(date_end_month.unwrap(), ParsiDate::new(1403, 7, 30).unwrap());
    ///
    /// // --- Error Cases ---
    /// // Try setting day to 31 in a 30-day month
    /// assert_eq!(date.with_day(31), Err(DateError::InvalidDate));
    ///
    /// // Try setting day to 0
    /// assert_eq!(date.with_day(0), Err(DateError::InvalidDate));
    ///
    /// // Example with Esfand
    /// let date_esfand_leap = ParsiDate::new(1403, 12, 1).unwrap(); // 1403 is leap (30 days)
    /// assert!(date_esfand_leap.with_day(30).is_ok());
    /// assert_eq!(date_esfand_leap.with_day(31), Err(DateError::InvalidDate));
    ///
    /// let date_esfand_common = ParsiDate::new(1404, 12, 1).unwrap(); // 1404 is common (29 days)
    /// assert!(date_esfand_common.with_day(29).is_ok());
    /// assert_eq!(date_esfand_common.with_day(30), Err(DateError::InvalidDate));
    ///
    /// // Starting date is invalid
    /// let invalid_start = unsafe { ParsiDate::new_unchecked(1400, 13, 1) };
    /// assert_eq!(invalid_start.with_day(15), Err(DateError::InvalidDate));
    /// ```
    pub fn with_day(&self, day: u32) -> Result<Self, DateError> {
        // 1. Validate the starting date.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }
        // 2. Perform a basic check: day must be positive.
        // The upper bound check (day <= days_in_month) is implicitly handled by ParsiDate::new.
        if day == 0 {
            return Err(DateError::InvalidDate);
        }

        // 3. Use the safe ParsiDate::new constructor.
        // It performs the crucial validation: is `day` valid for `self.year` and `self.month`?
        ParsiDate::new(self.year, self.month, day)
    }

    /// Returns the date of the first day of the month for the current date's year and month.
    ///
    /// Effectively creates a new `ParsiDate` instance representing the 1st of the same month and year.
    /// Assumes that the `self` instance on which it's called is already a valid `ParsiDate`.
    ///
    /// # Safety & Performance
    ///
    /// This method uses `unsafe { ParsiDate::new_unchecked }` internally for optimal performance,
    /// bypassing redundant validation. This is considered safe because:
    /// 1.  It assumes `self` is valid (checked by `debug_assert!`).
    /// 2.  If `self.year` and `self.month` are valid (which is assumed), then day `1` is *always*
    ///     a valid day for that month and year in the Persian calendar.
    ///     A `debug_assert!(self.is_valid())` is included to catch misuse in debug builds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate::new(1403, 5, 15).unwrap(); // Mordad 15th
    /// let first_day = date.first_day_of_month();
    /// assert_eq!(first_day, ParsiDate::new(1403, 5, 1).unwrap());
    ///
    /// let date_esfand = ParsiDate::new(1404, 12, 29).unwrap(); // Last day of Esfand (common year)
    /// let first_day_esfand = date_esfand.first_day_of_month();
    /// assert_eq!(first_day_esfand, ParsiDate::new(1404, 12, 1).unwrap());
    /// ```
    #[inline]
    pub fn first_day_of_month(&self) -> Self {
        // Add a debug assertion to ensure the precondition (self is valid) holds in debug builds.
        debug_assert!(
            self.is_valid(),
            "Precondition failed: first_day_of_month called on an invalid ParsiDate instance."
        );
        // Safety justification: If self.year and self.month are valid (as assumed),
        // then day 1 is guaranteed to be a valid day for that month/year.
        // Therefore, constructing with day=1 using new_unchecked is safe here.
        unsafe { ParsiDate::new_unchecked(self.year, self.month, 1) }
    }

    /// Returns the date of the last day of the month for the current date's year and month.
    ///
    /// This calculates the actual last day of the month (29, 30, or 31) based on the
    /// `self.month` and whether `self.year` is a leap year (for Esfand). It then creates
    /// a new `ParsiDate` instance representing that last day.
    /// Assumes that the `self` instance on which it's called is already a valid `ParsiDate`.
    ///
    /// # Safety & Performance
    ///
    /// This method uses `unsafe { ParsiDate::new_unchecked }` internally for performance.
    /// This is considered safe because:
    /// 1.  It assumes `self` is valid (checked by `debug_assert!`).
    /// 2.  [`ParsiDate::days_in_month`] correctly calculates the valid last day number (29, 30, or 31)
    ///     for the assumed-valid `self.year` and `self.month`.
    /// 3.  Constructing a date with this calculated last day for the same year/month is guaranteed to be valid.
    ///     A `debug_assert!(self.is_valid())` is included.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// // Month with 31 days
    /// let date_farvardin = ParsiDate::new(1403, 1, 15).unwrap();
    /// let last_day_farvardin = date_farvardin.last_day_of_month();
    /// assert_eq!(last_day_farvardin, ParsiDate::new(1403, 1, 31).unwrap());
    ///
    /// // Month with 30 days
    /// let date_mehr = ParsiDate::new(1403, 7, 10).unwrap(); // Mehr
    /// let last_day_mehr = date_mehr.last_day_of_month();
    /// assert_eq!(last_day_mehr, ParsiDate::new(1403, 7, 30).unwrap());
    ///
    /// // Esfand in a leap year (30 days)
    /// let date_esfand_leap = ParsiDate::new(1403, 12, 5).unwrap(); // 1403 is leap
    /// let last_day_esfand_leap = date_esfand_leap.last_day_of_month();
    /// assert_eq!(last_day_esfand_leap, ParsiDate::new(1403, 12, 30).unwrap());
    ///
    /// // Esfand in a common year (29 days)
    /// let date_esfand_common = ParsiDate::new(1404, 12, 5).unwrap(); // 1404 is common
    /// let last_day_esfand_common = date_esfand_common.last_day_of_month();
    /// assert_eq!(last_day_esfand_common, ParsiDate::new(1404, 12, 29).unwrap());
    /// ```
    #[inline]
    pub fn last_day_of_month(&self) -> Self {
        // Debug assertion for validity precondition.
        debug_assert!(
            self.is_valid(),
            "Precondition failed: last_day_of_month called on an invalid ParsiDate instance."
        );
        // Calculate the correct last day number for the current month and year.
        let last_day_num = Self::days_in_month(self.year, self.month);
        // Safety justification: days_in_month returns the correct, valid last day (29/30/31)
        // for the assumed-valid self.year and self.month. Constructing a date with this day is safe.
        unsafe { ParsiDate::new_unchecked(self.year, self.month, last_day_num) }
    }

    /// Returns the date of the first day of the year (Farvardin 1st) for the current date's year.
    ///
    /// Creates a new `ParsiDate` instance with the same year as `self`, but with month set to 1
    /// (Farvardin) and day set to 1.
    /// Assumes that the `self` instance on which it's called is already a valid `ParsiDate`.
    ///
    /// # Safety & Performance
    ///
    /// Uses `unsafe { ParsiDate::new_unchecked }` for performance. This is safe because:
    /// 1.  It assumes `self` is valid (checked by `debug_assert!`), meaning `self.year` is valid [1, 9999].
    /// 2.  Month 1 (Farvardin) and Day 1 are always valid components for any valid year in the Persian calendar.
    ///     A `debug_assert!(self.is_valid())` is included.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// let date_mid_year = ParsiDate::new(1403, 5, 15).unwrap();
    /// let first_day = date_mid_year.first_day_of_year();
    /// assert_eq!(first_day, ParsiDate::new(1403, 1, 1).unwrap());
    ///
    /// let date_end_year = ParsiDate::new(1404, 12, 29).unwrap();
    /// let first_day_for_end = date_end_year.first_day_of_year();
    /// assert_eq!(first_day_for_end, ParsiDate::new(1404, 1, 1).unwrap());
    /// ```
    #[inline]
    pub fn first_day_of_year(&self) -> Self {
        // Debug assertion for validity precondition.
        debug_assert!(
            self.is_valid(),
            "Precondition failed: first_day_of_year called on an invalid ParsiDate instance."
        );
        // Safety justification: If self.year is valid (assumed), then month 1 and day 1
        // always form a valid date (Farvardin 1st).
        unsafe { ParsiDate::new_unchecked(self.year, 1, 1) }
    }

    /// Returns the date of the last day of the year for the current date's year.
    ///
    /// This will be Esfand 30th if `self.year` is a leap year, or Esfand 29th if it's a common year.
    /// Creates a new `ParsiDate` instance representing that last day.
    /// Assumes that the `self` instance on which it's called is already a valid `ParsiDate`.
    ///
    /// # Safety & Performance
    ///
    /// Uses `unsafe { ParsiDate::new_unchecked }` for performance. This is safe because:
    /// 1.  It assumes `self` is valid (checked by `debug_assert!`), meaning `self.year` is valid [1, 9999].
    /// 2.  [`is_persian_leap_year`] correctly determines if the last day is 29 or 30.
    /// 3.  Month 12 (Esfand) and the calculated last day (29 or 30) always form a valid date
    ///     for the given `self.year`.
    ///     A `debug_assert!(self.is_valid())` is included.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::ParsiDate;
    ///
    /// // Date within a leap year (1403)
    /// let date_in_leap = ParsiDate::new(1403, 5, 15).unwrap();
    /// let last_day_leap = date_in_leap.last_day_of_year();
    /// assert_eq!(last_day_leap, ParsiDate::new(1403, 12, 30).unwrap()); // Esfand 30th
    ///
    /// // Date within a common year (1404)
    /// let date_in_common = ParsiDate::new(1404, 7, 10).unwrap();
    /// let last_day_common = date_in_common.last_day_of_year();
    /// assert_eq!(last_day_common, ParsiDate::new(1404, 12, 29).unwrap()); // Esfand 29th
    /// ```
    #[inline]
    pub fn last_day_of_year(&self) -> Self {
        // Debug assertion for validity precondition.
        debug_assert!(
            self.is_valid(),
            "Precondition failed: last_day_of_year called on an invalid ParsiDate instance."
        );
        // Determine the correct last day number (29 or 30) for Esfand of self.year.
        let last_day_num = if Self::is_persian_leap_year(self.year) {
            30
        } else {
            29
        };
        // Safety justification: Month 12 is valid, and last_day_num (29 or 30) is guaranteed
        // to be the valid last day for month 12 in the assumed-valid self.year.
        unsafe { ParsiDate::new_unchecked(self.year, 12, last_day_num) }
    }
} // End impl ParsiDate

// --- Trait Implementations ---

/// Implements the `Display` trait for `ParsiDate`.
///
/// This provides a default string representation when a `ParsiDate` instance is used with
/// formatting macros like `println!`, `format!`, etc.
///
/// The default format follows the `"short"` style: `"YYYY/MM/DD"`, with zero-padding for
/// the month and day components (e.g., "1403/05/02", "1399/12/30").
///
/// **Note:** If this trait method is called on an invalid `ParsiDate` instance (e.g., one
/// created using `unsafe new_unchecked` with invalid data like month 13), the output will
/// likely display those invalid components directly (e.g., "1403/13/01"), as `Display` usually
/// assumes the data it receives is well-formed.
///
/// # Examples
///
/// ```rust
/// use parsidate::ParsiDate;
///
/// let date1 = ParsiDate::new(1403, 5, 2).unwrap();
/// assert_eq!(date1.to_string(), "1403/05/02");
/// println!("Date 1: {}", date1); // Output: Date 1: 1403/05/02
///
/// let date2 = ParsiDate::new(1399, 12, 9).unwrap(); // Single digit day
/// assert_eq!(format!("{}", date2), "1399/12/09"); // Day is zero-padded
///
/// let date3 = ParsiDate::new(1400, 1, 1).unwrap(); // Single digit month
/// assert_eq!(date3.to_string(), "1400/01/01"); // Month is zero-padded
/// ```
impl fmt::Display for ParsiDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format using the "short" style: YYYY/MM/DD.
        // Use :02 format specifier to ensure month and day are zero-padded to two digits.
        write!(f, "{}/{:02}/{:02}", self.year, self.month, self.day)
    }
}
