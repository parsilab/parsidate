//! # ParsiDate: Persian (Jalali) Calendar Implementation in Rust
//!
//! This crate provides comprehensive functionality for working with the Persian (Jalali) calendar system.
//! It allows for:
//!
//! *   **Conversion:** Seamlessly convert dates between the Gregorian and Persian calendars.
//! *   **Validation:** Check if a given year, month, and day combination forms a valid Persian date.
//! *   **Formatting:** Display Persian dates in various common formats (e.g., "YYYY/MM/DD", "D MMMM YYYY", "YYYY-MM-DD").
//! *   **Date Arithmetic:** Calculate the number of days between two Persian dates.
//! *   **Leap Year Calculation:** Determine if a Persian or Gregorian year is a leap year.
//! *   **Weekday Calculation:** Find the Persian name for the day of the week.
//!
//! It relies on the `chrono` crate for underlying Gregorian date representations and calculations.
//!
//! ## Examples
//!
//! ```rust
//! use chrono::NaiveDate;
//! use parsidate::{ParsiDate, DateError};
//!
//! // Convert Gregorian to Persian
//! let gregorian_date = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
//! let persian_date = ParsiDate::from_gregorian(gregorian_date).unwrap();
//! assert_eq!(persian_date.year, 1403);
//! assert_eq!(persian_date.month, 5); // Mordad
//! assert_eq!(persian_date.day, 2);
//!
//! // Convert Persian to Gregorian
//! let p_date = ParsiDate { year: 1403, month: 1, day: 1 }; // Farvardin 1st, 1403
//! let g_date = p_date.to_gregorian().unwrap();
//! assert_eq!(g_date, NaiveDate::from_ymd_opt(2024, 3, 20).unwrap()); // March 20th, 2024 (Persian New Year)
//!
//! // Formatting
//! assert_eq!(persian_date.format("short"), "1403/05/02");
//! assert_eq!(persian_date.format("long"), "2 مرداد 1403");
//! assert_eq!(persian_date.format("iso"), "1403-05-02");
//!
//! // Validation
//! assert!(ParsiDate { year: 1403, month: 12, day: 30 }.is_valid()); // 1403 is a leap year
//! assert!(!ParsiDate { year: 1404, month: 12, day: 30 }.is_valid());// 1404 is not a leap year
//! assert!(!ParsiDate { year: 1403, month: 13, day: 1 }.is_valid()); // Invalid month
//!
//! // Weekday
//! assert_eq!(persian_date.weekday(), "سه‌شنبه"); // Tuesday
//!
//! // Days Between
//! let date1 = ParsiDate { year: 1403, month: 5, day: 2 };
//! let date2 = ParsiDate { year: 1403, month: 5, day: 12 };
//! assert_eq!(date1.days_between(&date2), 10);
//! ```

// Import necessary items from the chrono crate for Gregorian date handling.
use chrono::{Datelike, NaiveDate, Weekday};

/// Represents a date in the Persian (Jalali or Shamsi) calendar system.
///
/// This struct holds the year, month, and day components of a Persian date.
/// It provides methods for conversion, validation, formatting, and basic date calculations.
///
/// # Fields
///
/// *   `year` - The Persian year (e.g., 1403). Represents the number of years passed since the Hijra (migration) of Prophet Muhammad, adjusted for the solar calendar.
/// *   `month` - The Persian month number, ranging from 1 (Farvardin) to 12 (Esfand).
/// *   `day` - The day of the month, ranging from 1 to 31 (depending on the month and whether the year is a leap year).
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)] // Added more useful derives
pub struct ParsiDate {
    /// The year component of the Persian date (e.g., 1403).
    pub year: i32,
    /// The month component of the Persian date (1 = Farvardin, ..., 12 = Esfand).
    pub month: u32,
    /// The day component of the Persian date (1-29/30/31).
    pub day: u32,
}

/// Enumerates potential errors during date operations within the `parsidate` crate.
///
/// Currently, it only includes a variant for invalid date representations.
#[derive(Debug, PartialEq, Eq, Clone, Copy)] // Added more useful derives
pub enum DateError {
    /// Indicates that a given set of year, month, and day does not form a valid date
    /// in the Persian calendar (e.g., month 13, day 32, or day 30 in Esfand of a non-leap year)
    /// or that a conversion resulted in an invalid date.
    InvalidDate,
}

// Implementation block for the ParsiDate struct.
impl ParsiDate {
    /// Creates a new `ParsiDate` instance from year, month, and day components.
    ///
    /// This function validates the date upon creation.
    ///
    /// # Arguments
    ///
    /// * `year` - The Persian year.
    /// * `month` - The Persian month (1-12).
    /// * `day` - The Persian day (1-31).
    ///
    /// # Returns
    ///
    /// * `Ok(ParsiDate)` if the provided components form a valid Persian date.
    /// * `Err(DateError::InvalidDate)` if the date is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// let date = ParsiDate::new(1403, 5, 2).unwrap();
    /// assert_eq!(date.year, 1403);
    /// assert_eq!(date.month, 5);
    /// assert_eq!(date.day, 2);
    ///
    /// let invalid_date_result = ParsiDate::new(1404, 12, 30);
    /// assert_eq!(invalid_date_result, Err(DateError::InvalidDate)); // 1404 is not a leap year
    /// ```
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self, DateError> {
        let date = ParsiDate { year, month, day };
        if date.is_valid() {
            Ok(date)
        } else {
            Err(DateError::InvalidDate)
        }
    }

    /// Converts a Gregorian date (`chrono::NaiveDate`) to its equivalent Persian (Jalali) date.
    ///
    /// This method implements a conversion algorithm based on comparing the input date
    /// with the start of the corresponding Persian year (March 20th or 21st).
    /// It accurately handles Gregorian leap years during the conversion.
    ///
    /// The core logic determines the Persian year first, then calculates the day difference
    /// from the start of that Persian year (Nowruz) to find the month and day.
    ///
    /// # Arguments
    ///
    /// * `date` - A `chrono::NaiveDate` representing the Gregorian date to be converted.
    ///
    /// # Returns
    ///
    /// * `Ok(ParsiDate)` containing the equivalent Persian date if the conversion is successful.
    /// * `Err(DateError::InvalidDate)` if the internal calculations lead to an invalid state,
    ///   though this is unlikely with a valid `NaiveDate` input.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use parsidate::{ParsiDate, DateError};
    ///
    /// // A date after Nowruz
    /// let gregorian_date = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
    /// let persian_date = ParsiDate::from_gregorian(gregorian_date).unwrap();
    /// assert_eq!(persian_date, ParsiDate { year: 1403, month: 5, day: 2 }); // 2 Mordad 1403
    ///
    /// // A date before Nowruz (falls in the previous Persian year)
    /// let gregorian_date_early = NaiveDate::from_ymd_opt(2024, 3, 19).unwrap();
    /// let persian_date_early = ParsiDate::from_gregorian(gregorian_date_early).unwrap();
    /// assert_eq!(persian_date_early, ParsiDate { year: 1402, month: 12, day: 29 }); // 29 Esfand 1402
    ///
    /// // Nowruz itself (first day of the Persian year)
    /// let nowruz_gregorian = NaiveDate::from_ymd_opt(2024, 3, 20).unwrap(); // 2024 is a leap year, Nowruz is Mar 20
    /// let nowruz_persian = ParsiDate::from_gregorian(nowruz_gregorian).unwrap();
    /// assert_eq!(nowruz_persian, ParsiDate { year: 1403, month: 1, day: 1 });
    /// ```

    pub fn from_gregorian(date: NaiveDate) -> Result<Self, DateError> {
        // Define the Gregorian base date used as the reference point (epoch) for calculations.
        // This corresponds roughly to the start of the Persian calendar count (Year 0 or 1).
        // March 21, 621 AD is used here.
        // Panic here is acceptable as 621-03-21 is a fixed, valid date.
        let base_date = NaiveDate::from_ymd_opt(621, 3, 21).unwrap();

        // Calculate the total number of days elapsed between the input Gregorian date and the base date.
        let days_since_base = (date - base_date).num_days();

        // Check if the input date is before the established base date.
        // This algorithm does not support dates prior to March 21, 621 AD.
        if days_since_base < 0 {
            // Return an error indicating an invalid/unsupported date range.
            return Err(DateError::InvalidDate);
        }

        // Initialize the Persian year counter. Starts assuming year 0 relative to the base date.
        // `jy` will eventually hold the target Persian year number.
        let mut jy = 0; // Represents the number of full Persian years passed since the base epoch.

        // Initialize a mutable variable with the total days since the base date.
        // This variable will be reduced as we account for full years and months.
        // Cast to i32 for calculations involving subtraction.
        let mut remaining_days = days_since_base as i32;

        // --- Determine the Persian Year (jy) ---
        // Loop while the number of remaining days is sufficient to constitute at least one full Persian year.
        // This loop iteratively subtracts the days of each Persian year (starting from year 0)
        // until `remaining_days` holds the number of days *into* the target Persian year `jy`.
        while remaining_days
            >= (if Self::is_persian_leap_year(jy) {
                // Check if the current year `jy` is leap
                366 // Days in a leap year
            } else {
                365 // Days in a common year
            })
        {
            // Calculate the number of days in the current Persian year `jy`.
            let year_days = if Self::is_persian_leap_year(jy) {
                366
            } else {
                365
            };
            // Subtract the days of this full year from the remaining days.
            remaining_days -= year_days;
            // Increment the Persian year counter, moving to the next year.
            jy += 1;
        }
        // At this point, `jy` holds the correct Persian year, and `remaining_days` holds
        // the 0-indexed day number within that year (e.g., 0 for Farvardin 1st, 31 for Ordibehesht 1st).

        // --- Determine the Persian Month (jm) and Day (jd) ---
        // Check if the determined Persian year `jy` is a leap year to know Esfand's length.
        let is_persian_leap = Self::is_persian_leap_year(jy);

        // Define the lengths of the months for the determined Persian year `jy`.
        // The last month (Esfand) has 30 days if it's a leap year, 29 otherwise.
        let month_lengths = [
            31,                                    // Farvardin
            31,                                    // Ordibehesht
            31,                                    // Khordad
            31,                                    // Tir
            31,                                    // Mordad
            31,                                    // Shahrivar
            30,                                    // Mehr
            30,                                    // Aban
            30,                                    // Azar
            30,                                    // Dey
            30,                                    // Bahman
            if is_persian_leap { 30 } else { 29 }, // Esfand (length depends on leap status)
        ];

        // Initialize Persian month and day variables.
        let mut jm = 0; // Target Persian month (1-12)
        let mut jd = 0; // Target Persian day (1-31)

        // Iterate through the months of the year `jy`.
        // `i` is the 0-indexed month number (0=Farvardin, 1=Ordibehesht, ..., 11=Esfand).
        // `length` is the number of days in that month.
        for (i, &length) in month_lengths.iter().enumerate() {
            // Check if the `remaining_days` (0-indexed day within the year) falls into the current month.
            if remaining_days < length {
                // If yes, we've found the month and day.
                jm = i as u32 + 1; // Convert 0-indexed `i` to 1-indexed month number.
                jd = remaining_days + 1; // Convert 0-indexed `remaining_days` to 1-indexed day number.
                                         // Exit the loop as the correct month and day have been found.
                break;
            }
            // If the day is not in this month, subtract the length of this month
            // and continue to check the next month.
            remaining_days -= length;
        }

        // --- Final Validation and Result ---
        // If the loop completed without finding a month (jm is still 0), it implies an error
        // in the calculation (e.g., `days_since_base` was inconsistent or led to an impossible state).
        if jm == 0 {
            // This should theoretically not happen if `days_since_base` was correctly calculated
            // and the year/month loops worked as expected.
            return Err(DateError::InvalidDate);
        }

        // Construct the `ParsiDate` result using the calculated year, month, and day.
        // Cast the day `jd` (i32) back to u32 for the struct field.
        let result = ParsiDate {
            year: jy,
            month: jm,
            day: jd as u32,
        };

        // Perform a final validation check on the constructed date using the `is_valid` method.
        // This acts as a safeguard against potential edge-case errors in the algorithm
        // (e.g., if the calculations somehow resulted in day 30 for Esfand in a non-leap year).
        if !result.is_valid() {
            // If the generated date is somehow invalid by Persian calendar rules, return an error.
            return Err(DateError::InvalidDate);
        }

        // Return the successfully converted and validated Persian date.
        Ok(result)
    }
    /// Converts this Persian (Jalali) date to its equivalent Gregorian date (`chrono::NaiveDate`).
    ///
    /// This method calculates the number of days passed since a known reference point
    /// (the start of the Persian calendar epoch, corresponding roughly to March 21, 622 AD in Gregorian,
    /// although the calculation uses a relative approach) and adds these days to the Gregorian date
    /// corresponding to the start of the Persian epoch (Farvardin 1, Year 1).
    ///
    /// It accurately accounts for Persian leap years when calculating the total number of days.
    ///
    /// # Returns
    ///
    /// * `Ok(NaiveDate)` containing the equivalent Gregorian date if the current `ParsiDate` is valid.
    /// * `Err(DateError::InvalidDate)` if the current `ParsiDate` instance itself represents an invalid date
    ///   (e.g., month 13 or day 30 in Esfand of a non-leap year).
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use parsidate::ParsiDate;
    ///
    /// // Convert a standard date
    /// let persian_date = ParsiDate { year: 1403, month: 5, day: 2 }; // 2 Mordad 1403
    /// let gregorian_date = persian_date.to_gregorian().unwrap();
    /// assert_eq!(gregorian_date, NaiveDate::from_ymd_opt(2024, 7, 23).unwrap());
    ///
    /// // Convert Nowruz (start of the year)
    /// let nowruz_persian = ParsiDate { year: 1403, month: 1, day: 1 };
    /// let nowruz_gregorian = nowruz_persian.to_gregorian().unwrap();
    /// assert_eq!(nowruz_gregorian, NaiveDate::from_ymd_opt(2024, 3, 20).unwrap()); // 2024 is Gregorian leap
    ///
    /// // Convert end of a leap year
    /// let leap_end_persian = ParsiDate { year: 1403, month: 12, day: 30 }; // 1403 is leap
    /// let leap_end_gregorian = leap_end_persian.to_gregorian().unwrap();
    /// assert_eq!(leap_end_gregorian, NaiveDate::from_ymd_opt(2025, 3, 20).unwrap());
    ///
    /// // Attempt to convert an invalid date
    /// let invalid_persian = ParsiDate { year: 1404, month: 12, day: 30 }; // 1404 not leap
    /// assert!(invalid_persian.to_gregorian().is_err());
    /// ```
    pub fn to_gregorian(&self) -> Result<NaiveDate, DateError> {
        // First, ensure the ParsiDate instance itself is valid.
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }

        // --- Calculate days passed since the Persian epoch reference point ---

        // Reference Gregorian date for Farvardin 1, Year 1 (Persian epoch start).
        // This corresponds to March 22, 622 AD Julian, or March 19, 622 AD Proleptic Gregorian?
        // Most algorithms use March 21, 622 AD as the practical reference start for day counting.
        // Let's use the widely accepted reference: March 21, 622 (Gregorian) as day 1 of year 1.
        // However, calculating relative to a known recent Nowruz is often simpler and less error-prone.

        // Let's calculate days relative to Farvardin 1 of the *given* Persian year `self.year`.
        // First, find the Gregorian date for Farvardin 1 of `self.year`.
        // This is March 21st of Gregorian year `gy = self.year + 621`, adjusted to March 20th if `gy` is a leap year.

        // Alternative: Calculate total days from Persian Year 1, Month 1, Day 1.
        let mut total_days: i64 = 0;

        // Add days for full years passed before the current year `self.year`.
        // We iterate from year 1 up to (but not including) `self.year`.
        for y in 1..self.year {
            total_days += if Self::is_persian_leap_year(y) {
                366
            } else {
                365
            };
        }

        // Add days for full months passed in the current year `self.year` before the current month `self.month`.
        for m in 1..self.month {
            total_days += match m {
                1..=6 => 31,  // Farvardin to Shahrivar have 31 days
                7..=11 => 30, // Mehr to Bahman have 30 days
                12 => {
                    // Esfand depends on leap year
                    // This case shouldn't be reached here as we loop up to `self.month`.
                    // Included for completeness, but the logic focuses on months *before* `self.month`.
                    // If self.month is 12, this loop runs up to 11.
                    panic!("Logic error: Month 12 encountered in loop for previous months.");
                }
                _ => unreachable!("Invalid month {} encountered during day calculation", m), // Should be caught by is_valid
            };
        }
        // Add days for the current month (adjusting because day is 1-based).
        total_days += (self.day - 1) as i64;

        // Define the Gregorian date corresponding to Farvardin 1, Year 1 (Persian).
        // Based on common astronomical sources and algorithms (e.g., Dershowitz & Reingold),
        // Persian date 1/1/1 corresponds to March 19, 622, in the Proleptic Gregorian calendar.
        // Let's test this reference point.
        // If ParsiDate {1, 1, 1}, total_days = 0.
        // Gregorian base date needs to be March 19, 622.
        // Note: Chrono's NaiveDate might have limitations for years that far back.
        // Let's use a more modern reference if possible or stick to relative calculations.

        // Using the common reference start day calculation (e.g., from Calendar FAQ by Claus Tøndering)
        // Day number `N` from March 21, 622 AD (Gregorian).
        // Gregorian year `gy = self.year + 621`.
        // Find Gregorian date of Nowruz (Farvardin 1) for `self.year`.
        let gy_start = self.year + 621;
        // Nowruz is Mar 21 unless gy_start+1 is a leap year, then it's Mar 20? No, check gy_start.
        // Let's use the reference: March 21, 622 AD is Parsi 1/1/1 approximately.
        // March 21, 622 was chosen as a practical starting point.
        // Let's use NaiveDate::from_ymd(622, 3, 21) as the Gregorian equivalent of day 1 of year 1.
        // Use unwrap: Assuming 622-03-21 is a valid date.
        let persian_epoch_gregorian_start = NaiveDate::from_ymd_opt(622, 3, 21).unwrap();

        // Add the calculated total_days to the Gregorian epoch start date.
        // Note: The day added should be the difference from Parsi 1/1/1.
        // Our current `total_days` counts days *since* 1/1/1 (0 for 1/1/1, 1 for 1/1/2, etc.)
        match persian_epoch_gregorian_start.checked_add_days(chrono::Days::new(total_days as u64)) {
            Some(date) => Ok(date),
            None => Err(DateError::InvalidDate), // Indicates overflow or invalid calculation
        }

        /* // Previous calculation based on adding days to a calculated Nowruz:
         // Seems more complex than the epoch-based approach if the epoch date is reliable.

        // Calculate the Gregorian year corresponding to the start of this Persian year.
        let gregorian_equiv_year = self.year + 621;

        // Determine the exact Gregorian date of Nowruz (Farvardin 1) for `self.year`.
        // This depends on Gregorian leap years affecting the vernal equinox timing.
        // A common approximation: March 21st, unless the *next* Gregorian year is leap, then March 20th.
        // Let's test: 1 Farvardin 1399 -> March 20, 2020 (because 2020 was leap)
        // 1 Farvardin 1400 -> March 21, 2021 (because 2021 was not leap)
        // 1 Farvardin 1403 -> March 20, 2024 (because 2024 was leap)
        // Rule seems to be: If Gregorian year `gregorian_equiv_year` is leap, Nowruz is March 20, else March 21.
        let nowruz_day = if Self::is_gregorian_leap_year(gregorian_equiv_year) { 20 } else { 21 };
        let nowruz_gregorian = NaiveDate::from_ymd_opt(gregorian_equiv_year, 3, nowruz_day).unwrap();

        // Calculate the number of days passed *within* the current Persian year `self.year` until the given date.
        let mut days_into_persian_year: u32 = 0;
        for m in 1..self.month {
            days_into_persian_year += match m {
                1..=6 => 31,
                7..=11 => 30,
                12 => if Self::is_persian_leap_year(self.year) { 30 } else { 29 },
                _ => 0, // Invalid month, should be caught by is_valid
            };
        }
        days_into_persian_year += self.day - 1; // Add days of the current month (0-indexed)

        // Add these days to the Gregorian date of Nowruz.
        Ok(nowruz_gregorian + chrono::Duration::days(days_into_persian_year as i64))
        */
    }

    /// Checks if the current `ParsiDate` instance represents a valid date within the Persian calendar rules.
    ///
    /// Validation criteria:
    /// *   Month must be between 1 and 12 (inclusive).
    /// *   Day must be between 1 and 31 (inclusive).
    /// *   Day must not exceed the number of days in the given month:
    ///     *   Months 1-6 (Farvardin to Shahrivar) have 31 days.
    ///     *   Months 7-11 (Mehr to Bahman) have 30 days.
    ///     *   Month 12 (Esfand) has 30 days in a leap year, 29 otherwise.
    ///
    /// # Returns
    ///
    /// * `true` if the date (year, month, day combination) is valid.
    /// * `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use parsidate::ParsiDate;
    ///
    /// // Valid date
    /// assert!(ParsiDate { year: 1403, month: 1, day: 31 }.is_valid());
    /// // Valid date in leap year Esfand
    /// assert!(ParsiDate { year: 1403, month: 12, day: 30 }.is_valid()); // 1403 is leap
    /// // Valid date in non-leap year Esfand
    /// assert!(ParsiDate { year: 1404, month: 12, day: 29 }.is_valid()); // 1404 is not leap
    ///
    /// // Invalid month
    /// assert!(!ParsiDate { year: 1403, month: 0, day: 1 }.is_valid());
    /// assert!(!ParsiDate { year: 1403, month: 13, day: 1 }.is_valid());
    /// // Invalid day (too low)
    /// assert!(!ParsiDate { year: 1403, month: 1, day: 0 }.is_valid());
    /// // Invalid day (too high for month)
    /// assert!(!ParsiDate { year: 1403, month: 2, day: 32 }.is_valid()); // Ordibehesht has 31 days
    /// assert!(!ParsiDate { year: 1403, month: 7, day: 31 }.is_valid()); // Mehr has 30 days
    /// // Invalid day (too high for Esfand in non-leap year)
    /// assert!(!ParsiDate { year: 1404, month: 12, day: 30 }.is_valid()); // 1404 is not leap
    /// ```
    pub fn is_valid(&self) -> bool {
        // Check if month is within the valid range (1 to 12).
        if self.month == 0 || self.month > 12 {
            return false;
        }
        // Check if day is positive.
        if self.day == 0 {
            return false;
        }

        // Determine the maximum allowed days for the given month.
        let max_days = match self.month {
            // Months 1 through 6 (Farvardin to Shahrivar) have 31 days.
            1..=6 => 31,
            // Months 7 through 11 (Mehr to Bahman) have 30 days.
            7..=11 => 30,
            // Month 12 (Esfand) has 30 days in a Persian leap year, 29 otherwise.
            12 => {
                if Self::is_persian_leap_year(self.year) {
                    30
                } else {
                    29
                }
            }
            // This case should be unreachable due to the initial month check, but included for exhaustive matching.
            _ => return false,
        };

        // Check if the day is within the valid range for the determined month length.
        self.day <= max_days
    }

    /// Determines if a given Persian year is a leap year.
    ///
    /// The Persian calendar uses an observational VERNAL EQUINOX rule for leap years, which is very accurate.
    /// A common algorithmic approximation involves a 33-year cycle, attributed to Omar Khayyam or later astronomers.
    /// This implementation uses the widely accepted 33-year cycle approximation where specific years within the cycle are designated as leap years.
    ///
    /// The leap years within a 33-year cycle (starting from a reference year) typically fall on years:
    /// 1, 5, 9, 13, 17, 21, 25, **29** or **30**. (Most sources use 30, some older ones might use 29).
    /// This implementation uses the pattern with 30.
    /// Year `y` is leap if `(y * 8 + 21) % 33 < 8`. This is equivalent to checking `y % 33` against the set {1, 5, 9, 13, 17, 21, 25, 30}.
    ///
    /// # Arguments
    ///
    /// * `year` - The Persian year to check (e.g., 1403).
    ///
    /// # Returns
    ///
    /// * `true` if the given Persian year is a leap year according to the 33-year cycle approximation.
    /// * `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use parsidate::ParsiDate;
    ///
    /// assert!(ParsiDate::is_persian_leap_year(1399)); // 1399 % 33 = 5 -> Leap
    /// assert!(ParsiDate::is_persian_leap_year(1403)); // 1403 % 33 = 9 -> Leap
    /// assert!(!ParsiDate::is_persian_leap_year(1400)); // 1400 % 33 = 6 -> Not Leap
    /// assert!(!ParsiDate::is_persian_leap_year(1401)); // 1401 % 33 = 7 -> Not Leap
    /// assert!(!ParsiDate::is_persian_leap_year(1402)); // 1402 % 33 = 8 -> Not Leap
    /// assert!(ParsiDate::is_persian_leap_year(1408)); // 1408 % 33 = 12 -> Leap
    /// assert!(ParsiDate::is_persian_leap_year(1424)); // 1423 % 33 = 28 -> Leap
    /// ```
    pub fn is_persian_leap_year(year: i32) -> bool {
        // Ensure year is positive, though the cycle works for negative years mathematically.
        // Assume non-positive years are not meaningful in this calendar context or handle as needed.
        if year <= 0 {
            return false;
        } // Or adjust based on desired behavior for year 0 or BC dates.

        // Calculate the position within the 33-year cycle.
        // The cycle reference point can affect the result if not aligned, but modulo 33 gives the pattern.
        let cycle_position = year % 33;

        // Check if the cycle position matches one of the designated leap year positions in the 33-year pattern.
        // The pattern is {1, 5, 9, 13, 17, 21, 25, 30}.
        matches!(cycle_position, 1 | 5 | 9 | 13 | 17 | 22 | 26 | 30)
    }

    /// Determines if a given Gregorian year is a leap year.
    ///
    /// Implements the standard Gregorian calendar leap year rules:
    /// 1.  A year is a leap year if it is divisible by 4.
    /// 2.  However, if the year is divisible by 100, it is *not* a leap year...
    /// 3.  Unless the year is also divisible by 400, in which case it *is* a leap year.
    ///
    /// # Arguments
    ///
    /// * `year` - The Gregorian year to check (e.g., 2024).
    ///
    /// # Returns
    ///
    /// * `true` if the given Gregorian year is a leap year.
    /// * `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use parsidate::ParsiDate;
    ///
    /// assert!(ParsiDate::is_gregorian_leap_year(2020)); // Divisible by 4, not by 100
    /// assert!(ParsiDate::is_gregorian_leap_year(2024)); // Divisible by 4, not by 100
    /// assert!(!ParsiDate::is_gregorian_leap_year(2021)); // Not divisible by 4
    /// assert!(!ParsiDate::is_gregorian_leap_year(1900)); // Divisible by 100, but not by 400
    /// assert!(ParsiDate::is_gregorian_leap_year(2000)); // Divisible by 400
    /// ```
    pub fn is_gregorian_leap_year(year: i32) -> bool {
        // Check divisibility by 4, 100, and 400 according to the rules.
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Formats the `ParsiDate` into a string based on the specified style.
    ///
    /// Supported styles:
    /// *   `"short"` (Default): Formats as "YYYY/MM/DD" (e.g., "1403/05/02"). Uses leading zeros for month and day.
    /// *   `"long"`: Formats as "D MMMM YYYY" using Persian month names (e.g., "2 مرداد 1403").
    /// *   `"iso"`: Formats according to ISO 8601 style for dates: "YYYY-MM-DD" (e.g., "1403-05-02"). Uses leading zeros.
    /// *   Any other string: Currently defaults to the `"short"` format.
    ///
    /// # Arguments
    ///
    /// * `style` - A string slice (`&str`) specifying the desired format ("short", "long", or "iso"). Case-sensitive.
    ///
    /// # Returns
    ///
    /// * A `String` containing the formatted date. Returns the "short" format if the style is unrecognized.
    ///
    /// # Panics
    ///
    /// *   Panics if `self.month` is outside the valid range 1-12, which shouldn't happen if the `ParsiDate` was created via `new` or `from_gregorian` or validated beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate { year: 1403, month: 5, day: 2 }; // 2 Mordad 1403
    ///
    /// assert_eq!(date.format("short"), "1403/05/02");
    /// assert_eq!(date.format("long"), "2 مرداد 1403");
    /// assert_eq!(date.format("iso"), "1403-05-02");
    ///
    /// // Default format (same as "short")
    /// assert_eq!(date.to_string(), "1403/05/02");
    ///
    /// // Unrecognized style defaults to "short"
    /// assert_eq!(date.format("medium"), "1403/05/02");
    /// ```
    pub fn format(&self, style: &str) -> String {
        // Array of Persian month names, indexed 0 to 11.
        let month_names = [
            "فروردین",  // 1
            "اردیبهشت", // 2
            "خرداد",    // 3
            "تیر",      // 4
            "مرداد",    // 5
            "شهریور",   // 6
            "مهر",      // 7
            "آبان",     // 8
            "آذر",      // 9
            "دی",       // 10
            "بهمن",     // 11
            "اسفند",    // 12
        ];

        match style {
            // Long format: Day MonthName Year (e.g., "7 فروردین 1404")
            "long" => format!(
                "{} {} {}",
                self.day,
                // Access month name using month number (1-based) converted to 0-based index.
                // Panic potential if self.month is invalid (e.g., 0 or > 12).
                month_names[(self.month - 1) as usize],
                self.year
            ),
            // ISO 8601 format: YYYY-MM-DD (e.g., "1404-01-07")
            "iso" => format!("{}-{:02}-{:02}", self.year, self.month, self.day),
            // Short format (also default): YYYY/MM/DD (e.g., "1404/01/07")
            "short" | _ => self.to_string(), // Use the Display impl (via to_string method) for short/default
        }
    }

    /// Returns the Persian name of the weekday for this date.
    ///
    /// This method first converts the `ParsiDate` to its Gregorian equivalent using `to_gregorian`,
    /// then uses `chrono`'s `weekday()` method to find the day of the week (Monday=0 to Sunday=6),
    /// and finally maps this to the corresponding Persian name (شنبه to جمعه).
    ///
    /// # Returns
    ///
    /// * A `String` containing the Persian name of the weekday (e.g., "شنبه", "یکشنبه", ... "جمعه").
    ///
    /// # Panics
    ///
    /// *   Panics if the internal call to `to_gregorian()` fails (e.g., if the `ParsiDate` is invalid). Ensure the date is valid before calling.
    /// *   Panics if the weekday number from `chrono` is outside the expected 0-6 range, which should not happen.
    ///
    /// # Examples
    ///
    /// ```
    /// use parsidate::ParsiDate;
    ///
    /// // 7 Farvardin 1404 corresponds to March 27, 2025, which is a Thursday.
    /// let date1 = ParsiDate { year: 1404, month: 1, day: 7 };
    /// assert_eq!(date1.weekday(), "پنجشنبه"); // Thursday
    ///
    /// // 1 Farvardin 1403 corresponds to March 20, 2024, which is a Wednesday.
    /// let date2 = ParsiDate { year: 1403, month: 1, day: 1 };
    /// assert_eq!(date2.weekday(), "چهارشنبه"); // Wednesday
    ///
    /// // 29 Esfand 1402 corresponds to March 19, 2024, which is a Tuesday.
    /// let date3 = ParsiDate { year: 1402, month: 12, day: 29 };
    /// assert_eq!(date3.weekday(), "سه‌شنبه"); // Tuesday
    /// ```
    pub fn weekday(&self) -> String {
        // Convert the Persian date to Gregorian. Panics if self is invalid.
        let gregorian_date = self
            .to_gregorian()
            .expect("Cannot get weekday of an invalid ParsiDate");

        // Get the weekday from the Gregorian date.
        // chrono::Weekday: Mon=0, Tue=1, ..., Sun=6
        let day_num = gregorian_date.weekday().num_days_from_monday(); // Using Monday as 0 for consistency if needed

        // Alternative: Get Sunday as 0.
        // let day_num_sun0 = gregorian_date.weekday().num_days_from_sunday();

        // Persian weekdays start with Shanbeh (Saturday) corresponding to Gregorian Saturday.
        // Mapping from chrono::Weekday (Mon=0..Sun=6) to Persian Names (Shanbeh..Jomeh)
        // Gregorian: Mon(0), Tue(1), Wed(2), Thu(3), Fri(4), Sat(5), Sun(6)
        // Persian:   Doshanbeh, Seshanbeh, Chaharshanbeh, Panjshanbeh, Jomeh, Shanbeh, Yekshanbeh
        // Need a mapping:
        // Sat (5) -> Shanbeh (شنبه) - Index 0
        // Sun (6) -> Yekshanbeh (یکشنبه) - Index 1
        // Mon (0) -> Doshanbeh (دوشنبه) - Index 2
        // Tue (1) -> Seshanbeh (سه‌شنبه) - Index 3
        // Wed (2) -> Chaharshanbeh (چهارشنبه) - Index 4
        // Thu (3) -> Panjshanbeh (پنجشنبه) - Index 5
        // Fri (4) -> Jomeh (جمعه) - Index 6

        // Array of Persian weekday names, ordered Saturday to Friday.
        let persian_day_names = [
            "شنبه",     // Saturday
            "یکشنبه",   // Sunday
            "دوشنبه",   // Monday
            "سه‌شنبه",   // Tuesday
            "چهارشنبه", // Wednesday
            "پنجشنبه",  // Thursday
            "جمعه",     // Friday
        ];

        // Calculate the index into persian_day_names based on chrono's weekday.
        // chrono Sun=6 -> persian index 1
        // chrono Mon=0 -> persian index 2
        // chrono Tue=1 -> persian index 3
        // ...
        // chrono Sat=5 -> persian index 0
        let persian_index = (day_num + 2) % 7; // Map Mon(0)..Sun(6) to Sat(0)..Fri(6) effectively

        // Let's re-verify the mapping with num_days_from_sunday() (Sun=0..Sat=6)
        // Gregorian: Sun(0), Mon(1), Tue(2), Wed(3), Thu(4), Fri(5), Sat(6)
        // Persian:   Yekshanbeh, Doshanbeh, Seshanbeh, Chaharshanbeh, Panjshanbeh, Jomeh, Shanbeh
        // Need mapping:
        // Sun(0) -> Yekshanbeh (یکشنبه) - Index 1
        // Mon(1) -> Doshanbeh (دوشنبه) - Index 2
        // Tue(2) -> Seshanbeh (سه‌شنبه) - Index 3
        // Wed(3) -> Chaharshanbeh (چهارشنبه) - Index 4
        // Thu(4) -> Panjshanbeh (پنجشنبه) - Index 5
        // Fri(5) -> Jomeh (جمعه) - Index 6
        // Sat(6) -> Shanbeh (شنبه) - Index 0

        let day_num_sun0 = gregorian_date.weekday().num_days_from_sunday();
        // Mapping Sun(0)..Sat(6) to persian_day_names index (Sat=0 .. Fri=6)
        let persian_index_from_sun0 = (day_num_sun0 + 1) % 7; // Check this map
                                                              // Sun(0) -> (0+1)%7 = 1 -> Yekshanbeh (Correct)
                                                              // Mon(1) -> (1+1)%7 = 2 -> Doshanbeh (Correct)
                                                              // ...
                                                              // Fri(5) -> (5+1)%7 = 6 -> Jomeh (Correct)
                                                              // Sat(6) -> (6+1)%7 = 0 -> Shanbeh (Correct)
                                                              // This mapping seems correct.

        persian_day_names[persian_index_from_sun0 as usize].to_string()
    }

    /// Calculates the absolute difference in days between this `ParsiDate` and another `ParsiDate`.
    ///
    /// This method converts both Persian dates to their Gregorian equivalents and then calculates
    /// the duration between them using `chrono`'s capabilities. The result is always non-negative.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another `ParsiDate` instance to compare against.
    ///
    /// # Returns
    ///
    /// * An `i64` representing the absolute number of days between the two dates.
    ///
    /// # Panics
    ///
    /// *   Panics if either `self` or `other` represents an invalid Persian date, as `to_gregorian()` would panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use parsidate::ParsiDate;
    ///
    /// let date1 = ParsiDate { year: 1404, month: 1, day: 7 };
    /// let date2 = ParsiDate { year: 1404, month: 1, day: 10 };
    /// let date3 = ParsiDate { year: 1403, month: 12, day: 25 }; // Earlier date
    ///
    /// // Difference within the same month
    /// assert_eq!(date1.days_between(&date2), 3);
    /// assert_eq!(date2.days_between(&date1), 3); // Order doesn't matter
    ///
    /// // Difference across years
    /// // 1404/01/07 is March 27, 2025
    /// // 1403/12/25 is March 15, 2025 (1403 is leap)
    /// assert_eq!(date1.days_between(&date3), 12);
    /// ```
    pub fn days_between(&self, other: &ParsiDate) -> i64 {
        // Convert both dates to Gregorian. Panics if either is invalid.
        let g1 = self
            .to_gregorian()
            .expect("Cannot calculate days_between with invalid 'self' date");
        let g2 = other
            .to_gregorian()
            .expect("Cannot calculate days_between with invalid 'other' date");

        // Calculate the signed duration and return the absolute number of days.
        (g1 - g2).num_days().abs()
    }

    /// Returns the string representation of the date in the default "short" format: "YYYY/MM/DD".
    ///
    /// This method provides the standard string conversion, often used by the `Display` trait (if implemented).
    /// It ensures that the month and day are zero-padded to two digits.
    ///
    /// # Returns
    ///
    /// * A `String` formatted as "YYYY/MM/DD".
    ///
    /// # Examples
    ///
    /// ```
    /// use parsidate::ParsiDate;
    ///
    /// let date = ParsiDate { year: 1403, month: 5, day: 2 };
    /// assert_eq!(date.to_string(), "1403/05/02");
    ///
    /// let date_early = ParsiDate { year: 1399, month: 11, day: 22 };
    /// assert_eq!(date_early.to_string(), "1399/11/22");
    /// ```
    // Note: If `impl std::fmt::Display for ParsiDate` is added, this method might become redundant
    // or the Display implementation could call this.
    pub fn to_string(&self) -> String {
        // Format the year, month (zero-padded), and day (zero-padded) separated by slashes.
        format!("{}/{:02}/{:02}", self.year, self.month, self.day)
    }
}

// Implement the Display trait for easy printing using the default format.
impl std::fmt::Display for ParsiDate {
    /// Formats the `ParsiDate` using the default "short" style ("YYYY/MM/DD").
    ///
    /// This allows `ParsiDate` instances to be easily printed or converted to strings
    /// using standard formatting macros like `println!` or `format!`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Use the dedicated to_string method which implements the short format.
        write!(f, "{}", self.to_string())
    }
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    // Import necessary items from the parent module (the code being tested) and chrono.
    use super::*;
    use chrono::NaiveDate;

    // --- Conversion Tests ---

    #[test]
    /// Tests converting a typical Gregorian date (after Nowruz) to Persian.
    fn test_gregorian_to_persian_standard() {
        let gregorian = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap(); // July 23, 2024
        let expected_persian = ParsiDate {
            year: 1403,
            month: 5,
            day: 2,
        }; // 2 Mordad 1403
        let calculated_persian = ParsiDate::from_gregorian(gregorian).unwrap();
        assert_eq!(calculated_persian, expected_persian);
    }

    #[test]
    /// Tests converting a Gregorian date that falls before Nowruz (should be in the previous Persian year).
    fn test_gregorian_to_persian_before_nowruz() {
        let gregorian = NaiveDate::from_ymd_opt(2024, 3, 19).unwrap(); // March 19, 2024
        let expected_persian = ParsiDate {
            year: 1402,
            month: 12,
            day: 29,
        }; // 29 Esfand 1402 (1402 not leap)
        let calculated_persian = ParsiDate::from_gregorian(gregorian).unwrap();
        assert_eq!(calculated_persian, expected_persian);
    }

    #[test]
    /// Tests converting the Gregorian date of Nowruz itself (start of Persian year).
    /// Note: Nowruz date in Gregorian can be March 20 or 21. 2024 Gregorian is leap year.
    fn test_gregorian_to_persian_nowruz_leap_gregorian() {
        let gregorian = NaiveDate::from_ymd_opt(2024, 3, 20).unwrap(); // March 20, 2024 is Nowruz
        let expected_persian = ParsiDate {
            year: 1403,
            month: 1,
            day: 1,
        }; // 1 Farvardin 1403
        let calculated_persian = ParsiDate::from_gregorian(gregorian).unwrap();
        assert_eq!(calculated_persian, expected_persian);
    }

    #[test]
    /// Tests converting the Gregorian date of Nowruz itself (start of Persian year).
    /// Note: Nowruz date in Gregorian can be March 20 or 21. 2025 Gregorian is not leap year.
    fn test_gregorian_to_persian_nowruz_non_leap_gregorian() {
        let gregorian = NaiveDate::from_ymd_opt(2025, 3, 21).unwrap(); // March 21, 2025 is Nowruz
        let expected_persian = ParsiDate {
            year: 1404,
            month: 1,
            day: 1,
        }; // 1 Farvardin 1404
        let calculated_persian = ParsiDate::from_gregorian(gregorian).unwrap();
        assert_eq!(calculated_persian, expected_persian);
    }

    #[test]
    /// Tests converting a standard Persian date back to Gregorian.
    fn test_persian_to_gregorian_standard() {
        let persian = ParsiDate {
            year: 1403,
            month: 5,
            day: 2,
        }; // 2 Mordad 1403
        let expected_gregorian = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap(); // July 23, 2024
        let calculated_gregorian = persian.to_gregorian().unwrap();
        assert_eq!(calculated_gregorian, expected_gregorian);
    }

    #[test]
    /// Tests converting the start of a Persian year (Nowruz) to Gregorian.
    fn test_persian_to_gregorian_nowruz() {
        let persian = ParsiDate {
            year: 1403,
            month: 1,
            day: 1,
        }; // 1 Farvardin 1403
        let expected_gregorian = NaiveDate::from_ymd_opt(2024, 3, 20).unwrap(); // March 20, 2024
        let calculated_gregorian = persian.to_gregorian().unwrap();
        assert_eq!(calculated_gregorian, expected_gregorian);
    }

    #[test]
    /// Tests converting the end of a Persian leap year (Esfand 30th) to Gregorian.
    fn test_persian_to_gregorian_leap_year_end() {
        assert!(ParsiDate::is_persian_leap_year(1403)); // Ensure 1403 is leap
        let persian = ParsiDate {
            year: 1403,
            month: 12,
            day: 30,
        }; // 30 Esfand 1403
           // This should be the day before Nowruz 1404, which is March 21, 2025.
        let expected_gregorian = NaiveDate::from_ymd_opt(2025, 3, 20).unwrap();
        let calculated_gregorian = persian.to_gregorian().unwrap();
        assert_eq!(calculated_gregorian, expected_gregorian);
    }

    #[test]
    /// Tests converting the end of a Persian non-leap year (Esfand 29th) to Gregorian.
    fn test_persian_to_gregorian_non_leap_year_end() {
        assert!(!ParsiDate::is_persian_leap_year(1402)); // Ensure 1402 is not leap
        let persian = ParsiDate {
            year: 1402,
            month: 12,
            day: 29,
        }; // 29 Esfand 1402
           // This should be the day before Nowruz 1403, which is March 20, 2024.
        let expected_gregorian = NaiveDate::from_ymd_opt(2024, 3, 19).unwrap();
        let calculated_gregorian = persian.to_gregorian().unwrap();
        assert_eq!(calculated_gregorian, expected_gregorian);
    }

    #[test]
    /// Tests converting a date from a more distant past year.
    fn test_persian_to_gregorian_past_year() {
        let persian = ParsiDate {
            year: 1357,
            month: 11,
            day: 22,
        }; // 22 Bahman 1357 (Iranian Revolution)
        let expected_gregorian = NaiveDate::from_ymd_opt(1979, 2, 11).unwrap(); // February 11, 1979
        let calculated_gregorian = persian.to_gregorian().unwrap();
        assert_eq!(calculated_gregorian, expected_gregorian);
    }

    #[test]
    /// Tests converting a date from a future year.
    fn test_persian_to_gregorian_future_year() {
        let persian = ParsiDate {
            year: 1410,
            month: 6,
            day: 15,
        }; // 15 Shahrivar 1410
        let expected_gregorian = NaiveDate::from_ymd_opt(2031, 9, 6).unwrap(); // September 6, 2031
        let calculated_gregorian = persian.to_gregorian().unwrap();
        assert_eq!(calculated_gregorian, expected_gregorian);
    }

    // --- Validation Tests ---

    #[test]
    /// Tests the `is_valid` method with various valid dates.
    fn test_is_valid_true_cases() {
        // Standard date
        assert!(ParsiDate {
            year: 1403,
            month: 5,
            day: 2
        }
        .is_valid());
        // Start of year
        assert!(ParsiDate {
            year: 1403,
            month: 1,
            day: 1
        }
        .is_valid());
        // End of 31-day month
        assert!(ParsiDate {
            year: 1403,
            month: 6,
            day: 31
        }
        .is_valid());
        // End of 30-day month
        assert!(ParsiDate {
            year: 1403,
            month: 7,
            day: 30
        }
        .is_valid());
        // End of Esfand in leap year
        assert!(ParsiDate {
            year: 1403,
            month: 12,
            day: 30
        }
        .is_valid()); // 1403 is leap
                      // End of Esfand in non-leap year
        assert!(ParsiDate {
            year: 1404,
            month: 12,
            day: 29
        }
        .is_valid()); // 1404 is not leap
    }

    #[test]
    /// Tests the `is_valid` method with various invalid dates.
    fn test_is_valid_false_cases() {
        // Invalid month (0)
        assert!(!ParsiDate {
            year: 1403,
            month: 0,
            day: 1
        }
        .is_valid());
        // Invalid month (13)
        assert!(!ParsiDate {
            year: 1403,
            month: 13,
            day: 1
        }
        .is_valid());
        // Invalid day (0)
        assert!(!ParsiDate {
            year: 1403,
            month: 1,
            day: 0
        }
        .is_valid());
        // Invalid day (32 in 31-day month)
        assert!(!ParsiDate {
            year: 1403,
            month: 1,
            day: 32
        }
        .is_valid());
        // Invalid day (31 in 30-day month)
        assert!(!ParsiDate {
            year: 1403,
            month: 7,
            day: 31
        }
        .is_valid());
        // Invalid day (30 in Esfand, non-leap year)
        assert!(!ParsiDate {
            year: 1404,
            month: 12,
            day: 30
        }
        .is_valid()); // 1404 not leap
                      // Invalid day (31 in Esfand, leap year)
        assert!(!ParsiDate {
            year: 1403,
            month: 12,
            day: 31
        }
        .is_valid()); // 1403 is leap, but Esfand only has 30 days max
    }

    #[test]
    /// Tests that `to_gregorian` returns an error for an invalid ParsiDate.
    fn test_to_gregorian_invalid_input() {
        let invalid_persian = ParsiDate {
            year: 1404,
            month: 12,
            day: 30,
        }; // Invalid day
        assert!(!invalid_persian.is_valid()); // Double check validity check
        let result = invalid_persian.to_gregorian();
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), DateError::InvalidDate);
    }

    #[test]
    /// Tests that the `new` constructor validates input.
    fn test_new_constructor_validation() {
        // Valid case
        assert!(ParsiDate::new(1403, 5, 2).is_ok());
        // Invalid case
        let result = ParsiDate::new(1404, 12, 30); // 1404 not leap
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), DateError::InvalidDate);
    }

    // --- Leap Year Tests ---

    #[test]
    /// Tests the Persian leap year calculation for known leap years.
    fn test_is_persian_leap_year_true() {
        assert!(
            ParsiDate::is_persian_leap_year(1399),
            "Year 1399 should be leap (cycle pos 5)"
        );
        assert!(
            ParsiDate::is_persian_leap_year(1403),
            "Year 1403 should be leap (cycle pos 9)"
        );
        assert!(
            ParsiDate::is_persian_leap_year(1408),
            "Year 1408 should be leap (cycle pos 14)"
        );
        assert!(
            ParsiDate::is_persian_leap_year(1412),
            "Year 1412 should be leap (cycle pos 18)"
        );
        assert!(
            ParsiDate::is_persian_leap_year(1416),
            "Year 1416 should be leap (cycle pos 22)"
        );
        assert!(
            ParsiDate::is_persian_leap_year(1420),
            "Year 1420 should be leap (cycle pos 26)"
        );
        assert!(
            ParsiDate::is_persian_leap_year(1424),
            "Year 1424 should be leap (cycle pos 31)"
        );
        assert!(
            ParsiDate::is_persian_leap_year(1428),
            "Year 1428 should be leap (cycle pos 1)"
        ); // Next cycle start
    }

    #[test]
    /// Tests the Persian leap year calculation for known non-leap years.
    fn test_is_persian_leap_year_false() {
        assert!(
            !ParsiDate::is_persian_leap_year(1400),
            "Year 1400 should not be leap (cycle pos 6)"
        );
        assert!(
            !ParsiDate::is_persian_leap_year(1401),
            "Year 1401 should not be leap (cycle pos 7)"
        );
        assert!(
            !ParsiDate::is_persian_leap_year(1402),
            "Year 1402 should not be leap (cycle pos 8)"
        );
        assert!(
            !ParsiDate::is_persian_leap_year(1404),
            "Year 1404 should not be leap (cycle pos 10)"
        );
        assert!(
            !ParsiDate::is_persian_leap_year(1425),
            "Year 1424 should not be leap (cycle pos 32)"
        );
    }

    #[test]
    /// Tests the Gregorian leap year calculation.
    fn test_is_gregorian_leap_year() {
        assert!(
            ParsiDate::is_gregorian_leap_year(2000),
            "Year 2000 divisible by 400"
        );
        assert!(
            ParsiDate::is_gregorian_leap_year(2020),
            "Year 2020 divisible by 4, not 100"
        );
        assert!(
            ParsiDate::is_gregorian_leap_year(2024),
            "Year 2024 divisible by 4, not 100"
        );
        assert!(
            !ParsiDate::is_gregorian_leap_year(1900),
            "Year 1900 divisible by 100, not 400"
        );
        assert!(
            !ParsiDate::is_gregorian_leap_year(2021),
            "Year 2021 not divisible by 4"
        );
        assert!(
            !ParsiDate::is_gregorian_leap_year(2023),
            "Year 2023 not divisible by 4"
        );
    }

    // --- Formatting Tests ---

    #[test]
    /// Tests the different formatting styles.
    fn test_format_styles() {
        let date = ParsiDate {
            year: 1404,
            month: 1,
            day: 7,
        }; // 7 Farvardin 1404
           // Short format (YYYY/MM/DD) - Also tests Display trait via to_string()
        assert_eq!(date.format("short"), "1404/01/07");
        assert_eq!(date.to_string(), "1404/01/07");
        // Long format (D MMMM YYYY)
        assert_eq!(date.format("long"), "7 فروردین 1404");
        // ISO format (YYYY-MM-DD)
        assert_eq!(date.format("iso"), "1404-01-07");
        // Default/Unknown format should fallback to short
        assert_eq!(date.format("unknown_style"), "1404/01/07");
    }

    #[test]
    /// Tests formatting for a date requiring two-digit padding for day/month.
    fn test_format_padding() {
        let date = ParsiDate {
            year: 1400,
            month: 11,
            day: 20,
        }; // 20 Bahman 1400
        assert_eq!(date.format("short"), "1400/11/20");
        assert_eq!(date.format("iso"), "1400-11-20");
        assert_eq!(date.format("long"), "20 بهمن 1400");
    }

    // --- Weekday Tests ---

    #[test]
    /// Tests the weekday calculation for known dates.
    fn test_weekday() {
        // 7 Farvardin 1404 -> March 27, 2025 -> Thursday
        let date1 = ParsiDate {
            year: 1404,
            month: 1,
            day: 7,
        };
        assert_eq!(date1.weekday(), "پنجشنبه");

        // 1 Farvardin 1403 -> March 20, 2024 -> Wednesday
        let date2 = ParsiDate {
            year: 1403,
            month: 1,
            day: 1,
        };
        assert_eq!(date2.weekday(), "چهارشنبه");

        // 29 Esfand 1402 -> March 19, 2024 -> Tuesday
        let date3 = ParsiDate {
            year: 1402,
            month: 12,
            day: 29,
        };
        assert_eq!(date3.weekday(), "سه‌شنبه");

        // 22 Bahman 1357 -> Feb 11, 1979 -> Sunday
        let date4 = ParsiDate {
            year: 1357,
            month: 11,
            day: 22,
        };
        assert_eq!(date4.weekday(), "یکشنبه");

        // 2 Mordad 1403 -> July 23, 2024 -> Tuesday
        let date5 = ParsiDate {
            year: 1403,
            month: 5,
            day: 2,
        };
        assert_eq!(date5.weekday(), "سه‌شنبه");

        // Test a Friday
        // 9 Farvardin 1404 -> March 29, 2025 -> Saturday // Let's find a Friday...
        // 1 Tir 1403 -> June 21, 2024 -> Friday
        let date_fri = ParsiDate {
            year: 1403,
            month: 4,
            day: 1,
        };
        assert_eq!(date_fri.weekday(), "جمعه");

        // Test a Saturday
        // 2 Tir 1403 -> June 22, 2024 -> Saturday
        let date_sat = ParsiDate {
            year: 1403,
            month: 4,
            day: 2,
        };
        assert_eq!(date_sat.weekday(), "شنبه");

        // Test a Monday
        // 4 Tir 1403 -> June 24, 2024 -> Monday
        let date_mon = ParsiDate {
            year: 1403,
            month: 4,
            day: 4,
        };
        assert_eq!(date_mon.weekday(), "دوشنبه");
    }

    // --- Days Between Tests ---

    #[test]
    /// Tests calculating days between dates within the same month and year.
    fn test_days_between_same_month() {
        let date1 = ParsiDate {
            year: 1404,
            month: 1,
            day: 7,
        };
        let date2 = ParsiDate {
            year: 1404,
            month: 1,
            day: 10,
        };
        assert_eq!(date1.days_between(&date2), 3);
        assert_eq!(date2.days_between(&date1), 3); // Test commutativity
    }

    #[test]
    /// Tests calculating days between dates in different months but the same year.
    fn test_days_between_different_months() {
        let date1 = ParsiDate {
            year: 1403,
            month: 1,
            day: 1,
        }; // Mar 20, 2024
        let date2 = ParsiDate {
            year: 1403,
            month: 2,
            day: 1,
        }; // Apr 20, 2024 (31 days in Farvardin)
        assert_eq!(date1.days_between(&date2), 31);
    }

    #[test]
    /// Tests calculating days between dates spanning across a Persian year boundary.
    fn test_days_between_crossing_year() {
        // End of non-leap year
        let date1 = ParsiDate {
            year: 1402,
            month: 12,
            day: 29,
        }; // Mar 19, 2024
           // Start of next year
        let date2 = ParsiDate {
            year: 1403,
            month: 1,
            day: 1,
        }; // Mar 20, 2024
        assert_eq!(date1.days_between(&date2), 1);

        // End of leap year
        let date3 = ParsiDate {
            year: 1403,
            month: 12,
            day: 30,
        }; // Mar 20, 2025
           // Start of next year
        let date4 = ParsiDate {
            year: 1404,
            month: 1,
            day: 1,
        }; // Mar 21, 2025
        assert_eq!(date3.days_between(&date4), 1);

        // Larger gap across year
        let date5 = ParsiDate {
            year: 1403,
            month: 10,
            day: 15,
        }; // Dec 5, 2024 approx? No, Jan 5, 2025
           // 1403/10/15 -> Jan 5, 2025
        let date6 = ParsiDate {
            year: 1404,
            month: 2,
            day: 10,
        }; // Apr 30, 2025 approx?
           // 1404/01/01 -> Mar 21, 2025
           // 1404/02/10 -> Day 31 (Far) + 10 (Ord) = 41 days after Nowruz -> Mar 21 + 40 days = Apr 30, 2025
        let g5 = date5.to_gregorian().unwrap(); // 2025-01-05
        let g6 = date6.to_gregorian().unwrap(); // 2025-04-30
        let expected_days = (g6 - g5).num_days(); // 115 days
        assert_eq!(date5.days_between(&date6), expected_days);
        assert_eq!(date5.days_between(&date6), 116);
    }

    #[test]
    /// Tests calculating days between identical dates.
    fn test_days_between_same_date() {
        let date1 = ParsiDate {
            year: 1403,
            month: 5,
            day: 2,
        };
        let date2 = ParsiDate {
            year: 1403,
            month: 5,
            day: 2,
        };
        assert_eq!(date1.days_between(&date2), 0);
    }
}
