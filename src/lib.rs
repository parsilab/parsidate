use chrono::{NaiveDate, Datelike, Weekday};

/// Represents a Persian (Jalali) date with year, month, and day.
#[derive(Debug, PartialEq)]
pub struct ParsiDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

/// Error type for invalid date operations.
#[derive(Debug)]
pub enum DateError {
    InvalidDate,
}

impl ParsiDate {
    /// Creates a new `ParsiDate` from a Gregorian date.
    ///
    /// # Arguments
    /// * `date` - A `NaiveDate` representing the Gregorian date.
    ///
    /// # Examples
    /// ```
    /// use chrono::NaiveDate;
    /// use parsidate::{ParsiDate, DateError};
    /// let gregorian = NaiveDate::from_ymd_opt(2025, 3, 27).unwrap();
    /// let persian = ParsiDate::from_gregorian(gregorian).unwrap();
    /// assert_eq!(persian.to_string(), "1404/01/07");
    /// ```
    pub fn from_gregorian(date: NaiveDate) -> Result<Self, DateError> {
        let gregorian_year = date.year();
        let mut jy = gregorian_year - 621;
        let mut jm = 0;
        let mut jd = 0;

        let march_21 = NaiveDate::from_ymd_opt(jy + 621, 3, 21).unwrap();
        let days_diff = (date - march_21).num_days();

        if days_diff >= 0 {
            let mut days = days_diff as i32;
            let leap = if Self::is_gregorian_leap_year(gregorian_year) { 1 } else { 0 };
            let year_days = if days < 186 { 365 + leap } else { 366 + leap };

            if days >= year_days {
                jy += 1;
                days -= year_days;
            }

            let month_lengths = [31, 31, 31, 31, 31, 31, 30, 30, 30, 30, 30, 29];
            for (i, &length) in month_lengths.iter().enumerate() {
                if days < length {
                    jm = i as u32 + 1;
                    jd = days + 1;
                    break;
                }
                days -= length;
            }
        } else {
            jy -= 1;
            let leap = if Self::is_gregorian_leap_year(gregorian_year - 1) { 1 } else { 0 };
            let mut days = (date - NaiveDate::from_ymd_opt(jy + 621, 3, 20).unwrap()).num_days() as i32;
            let month_lengths = [31, 31, 31, 31, 31, 31, 30, 30, 30, 30, 30, 29 + leap];
            for (i, &length) in month_lengths.iter().enumerate() {
                if days <= length {
                    jm = i as u32 + 1;
                    jd = days;
                    break;
                }
                days -= length;
            }
        }

        if jm == 0 || jd == 0 {
            return Err(DateError::InvalidDate);
        }

        let result = ParsiDate {
            year: jy,
            month: jm,
            day: jd as u32,
        };
        if !result.is_valid() {
            return Err(DateError::InvalidDate);
        }
        Ok(result)
    }

    /// Converts a Persian date to its Gregorian equivalent.
    ///
    /// # Examples
    /// ```
    /// use chrono::NaiveDate;
    /// use parsidate::ParsiDate;
    /// let persian = ParsiDate { year: 1404, month: 1, day: 7 };
    /// let gregorian = persian.to_gregorian().unwrap();
    /// assert_eq!(gregorian, NaiveDate::from_ymd_opt(2025, 3, 27).unwrap());
    /// ```
    pub fn to_gregorian(&self) -> Result<NaiveDate, DateError> {
        if !self.is_valid() {
            return Err(DateError::InvalidDate);
        }

        let mut days = 0;
        let mut year = self.year + 621;

        for y in 0..self.year {
            days += if Self::is_persian_leap_year(y) { 366 } else { 365 };
        }
        for m in 1..self.month {
            days += match m {
                1..=6 => 31,
                7..=11 => 30,
                12 => if Self::is_persian_leap_year(self.year) { 30 } else { 29 },
                _ => unreachable!(),
            };
        }
        days += self.day - 1;

        let base_date = NaiveDate::from_ymd_opt(621, 3, 21).unwrap();
        Ok(base_date + chrono::Duration::days(days as i64))
    }

    /// Checks if the Persian date is valid.
    ///
    /// # Examples
    /// ```
    /// use parsidate::ParsiDate;
    /// let valid_date = ParsiDate { year: 1404, month: 1, day: 7 };
    /// assert!(valid_date.is_valid());
    /// let invalid_date = ParsiDate { year: 1404, month: 1, day: 32 };
    /// assert!(!invalid_date.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        if self.month > 12 || self.day == 0 {
            return false;
        }
        let max_days = match self.month {
            1..=6 => 31,
            7..=11 => 30,
            12 => if Self::is_persian_leap_year(self.year) { 30 } else { 29 },
            _ => return false,
        };
        self.day <= max_days
    }

    /// Checks if a Persian year is a leap year (simplified algorithm).
    fn is_persian_leap_year(year: i32) -> bool {
        let cycle = year % 33;
        matches!(cycle, 1 | 5 | 9 | 13 | 17 | 21 | 25 | 30)
    }

    /// Checks if a Gregorian year is a leap year.
    fn is_gregorian_leap_year(year: i32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }

    /// Formats the Persian date in various styles.
    ///
    /// # Arguments
    /// * `style` - The desired format ("short", "long", "iso").
    ///
    /// # Examples
    /// ```
    /// use parsidate::ParsiDate;
    /// let date = ParsiDate { year: 1404, month: 1, day: 7 };
    /// assert_eq!(date.format("short"), "1404/01/07");
    /// assert_eq!(date.format("long"), "7 Farvardin 1404");
    /// assert_eq!(date.format("iso"), "1404-01-07");
    /// ```
    pub fn format(&self, style: &str) -> String {
        let month_names = [
            "Farvardin", "Ordibehesht", "Khordad", "Tir", "Mordad", "Shahrivar",
            "Mehr", "Aban", "Azar", "Dey", "Bahman", "Esfand",
        ];
        match style {
            "long" => format!("{} {} {}", self.day, month_names[(self.month - 1) as usize], self.year),
            "iso" => format!("{}-{:02}-{:02}", self.year, self.month, self.day),
            _ => self.to_string(),
        }
    }

    /// Returns the weekday name in Persian.
    ///
    /// # Examples
    /// ```
    /// use parsidate::ParsiDate;
    /// let date = ParsiDate { year: 1404, month: 1, day: 7 };
    /// assert_eq!(date.weekday(), "Panjshanbe");
    /// ```
    pub fn weekday(&self) -> String {
        let gregorian = self.to_gregorian().unwrap();
        let day_names = [
            "Yekshanbe", "Doshanbe", "Seshanbe", "Chaharshanbe",
            "Panjshanbe", "Jome", "Shanbe",
        ];
        day_names[gregorian.weekday().num_days_from_sunday() as usize].to_string()
    }

    /// Calculates the absolute number of days between two Persian dates.
    ///
    /// # Arguments
    /// * `other` - The other `ParsiDate` to compare with.
    ///
    /// # Examples
    /// ```
    /// use parsidate::ParsiDate;
    /// let date1 = ParsiDate { year: 1404, month: 1, day: 7 };
    /// let date2 = ParsiDate { year: 1404, month: 1, day: 10 };
    /// assert_eq!(date1.days_between(&date2), 3);
    /// ```
    pub fn days_between(&self, other: &ParsiDate) -> i64 {
        let g1 = self.to_gregorian().unwrap();
        let g2 = other.to_gregorian().unwrap();
        (g1 - g2).num_days().abs()
    }

    /// Returns the date as a string in the format "YYYY/MM/DD".
    pub fn to_string(&self) -> String {
        format!("{}/{:02}/{:02}", self.year, self.month, self.day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gregorian_to_persian() {
        let gregorian = NaiveDate::from_ymd_opt(2025, 3, 27).unwrap();
        let persian = ParsiDate::from_gregorian(gregorian).unwrap();
        assert_eq!(persian, ParsiDate { year: 1404, month: 1, day: 7 });
    }

    #[test]
    fn test_persian_to_gregorian() {
        let persian = ParsiDate { year: 1404, month: 1, day: 7 };
        let gregorian = persian.to_gregorian().unwrap();
        assert_eq!(gregorian, NaiveDate::from_ymd_opt(2025, 3, 27).unwrap());
    }

    #[test]
    fn test_is_valid() {
        let valid = ParsiDate { year: 1404, month: 1, day: 7 };
        let invalid = ParsiDate { year: 1404, month: 1, day: 32 };
        assert!(valid.is_valid());
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_format() {
        let date = ParsiDate { year: 1404, month: 1, day: 7 };
        assert_eq!(date.format("short"), "1404/01/07");
        assert_eq!(date.format("long"), "7 Farvardin 1404");
        assert_eq!(date.format("iso"), "1404-01-07");
    }

    #[test]
    fn test_weekday() {
        let date = ParsiDate { year: 1404, month: 1, day: 7 };
        assert_eq!(date.weekday(), "Panjshanbe");
    }

    #[test]
    fn test_days_between() {
        let date1 = ParsiDate { year: 1404, month: 1, day: 7 };
        let date2 = ParsiDate { year: 1404, month: 1, day: 10 };
        assert_eq!(date1.days_between(&date2), 3);
    }
}