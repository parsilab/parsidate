// ~/src/season.rs
//
//  * Copyright (C) ParsiCore (parsidate) 2024-2025 <parsicore.dev@gmail.com>
//  * Package : parsidate
//  * License : Apache-2.0
//  * Version : 1.7.1
//  * URL     : https://github.com/parsicore/parsidate
//  * Sign: parsidate-20250607-fea13e856dcd-459c6e73c83e49e10162ee28b26ac7cd
//
//! # Persian Calendar Seasons
//!
//! This module defines the [`Season`] enum, which represents the four seasons of the year
//! according to the Persian (Jalali) calendar.
//!
//! Each variant corresponds to a three-month period:
//! - **Bahar (Spring)**: Farvardin, Ordibehesht, Khordad (Months 1-3)
//! - **Tabestan (Summer)**: Tir, Mordad, Shahrivar (Months 4-6)
//! - **Paeez (Autumn)**: Mehr, Aban, Azar (Months 7-9)
//! - **Zemestan (Winter)**: Dey, Bahman, Esfand (Months 10-12)
//!
//! The enum provides methods to get the season's name in both Persian and English, as well as its
//! start and end months. It is returned by methods like [`ParsiDate::season()`](crate::ParsiDate::season)
//! and can be used for date-based logic and formatting.

use crate::constants::{SEASON_NAMES_ENGLISH, SEASON_NAMES_PERSIAN};
use std::fmt;

/// Represents one of the four seasons in the Persian calendar.
///
/// This enum is `Copy`, `Clone`, `Debug`, `PartialEq`, `Eq`, and `Hash`. It can also be serialized
/// and deserialized with `serde` if the `serde` feature is enabled.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Season {
    /// **Bahar** (بهار), or Spring. Corresponds to months 1, 2, and 3.
    Bahar,
    /// **Tabestan** (تابستان), or Summer. Corresponds to months 4, 5, and 6.
    Tabestan,
    /// **Paeez** (پاییز), or Autumn/Fall. Corresponds to months 7, 8, and 9.
    Paeez,
    /// **Zemestan** (زمستان), or Winter. Corresponds to months 10, 11, and 12.
    Zemestan,
}

impl Season {
    /// Returns the full Persian name of the season as a static string slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::Season;
    ///
    /// assert_eq!(Season::Bahar.name_persian(), "بهار");
    /// assert_eq!(Season::Tabestan.name_persian(), "تابستان");
    /// assert_eq!(Season::Paeez.name_persian(), "پاییز");
    /// assert_eq!(Season::Zemestan.name_persian(), "زمستان");
    /// ```
    #[inline]
    pub fn name_persian(&self) -> &'static str {
        // The enum variants are ordered to match the constants array.
        // Casting `*self as usize` safely maps each variant to its index (0-3).
        SEASON_NAMES_PERSIAN[*self as usize]
    }

    /// Returns the English name of the season as a static string slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::Season;
    ///
    /// assert_eq!(Season::Bahar.name_english(), "Spring");
    /// assert_eq!(Season::Tabestan.name_english(), "Summer");
    /// assert_eq!(Season::Paeez.name_english(), "Autumn");
    /// assert_eq!(Season::Zemestan.name_english(), "Winter");
    /// ```
    #[inline]
    pub fn name_english(&self) -> &'static str {
        // The enum variants are ordered to match the constants array.
        SEASON_NAMES_ENGLISH[*self as usize]
    }

    /// Returns the starting month number (1-12) of the season.
    ///
    /// - `Bahar` starts in month 1 (Farvardin).
    /// - `Tabestan` starts in month 4 (Tir).
    /// - `Paeez` starts in month 7 (Mehr).
    /// - `Zemestan` starts in month 10 (Dey).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::Season;
    ///
    /// assert_eq!(Season::Bahar.start_month(), 1);
    /// assert_eq!(Season::Paeez.start_month(), 7);
    /// ```
    #[inline]
    pub fn start_month(&self) -> u32 {
        match self {
            Season::Bahar => 1,     // Farvardin
            Season::Tabestan => 4,  // Tir
            Season::Paeez => 7,     // Mehr
            Season::Zemestan => 10, // Dey
        }
    }

    /// Returns the ending month number (1-12) of the season.
    ///
    /// - `Bahar` ends in month 3 (Khordad).
    /// - `Tabestan` ends in month 6 (Shahrivar).
    /// - `Paeez` ends in month 9 (Azar).
    /// - `Zemestan` ends in month 12 (Esfand).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parsidate::Season;
    ///
    /// assert_eq!(Season::Tabestan.end_month(), 6);
    /// assert_eq!(Season::Zemestan.end_month(), 12);
    /// ```
    #[inline]
    pub fn end_month(&self) -> u32 {
        match self {
            Season::Bahar => 3,
            Season::Tabestan => 6,
            Season::Paeez => 9,
            Season::Zemestan => 12,
        }
    }
}

/// Implements the `Display` trait for `Season`.
///
/// This provides a default string representation for a `Season` instance, which is its
/// full Persian name. It allows `Season` to be used seamlessly with macros like `println!`
/// and `format!`.
///
/// # Examples
///
/// ```rust
/// use parsidate::Season;
///
/// let season = Season::Paeez;
///
/// // Using format! or to_string()
/// assert_eq!(season.to_string(), "پاییز");
///
/// // Using in println!
/// println!("The current season is {}.", season); // Prints: The current season is پاییز.
/// ```
impl fmt::Display for Season {
    /// Formats the season using its Persian name.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate formatting to the name_persian() method.
        write!(f, "{}", self.name_persian())
    }
}
