// ~/src/season.rs
//
//  * Copyright (C) Mohammad (Sina) Jalalvandi 2024-2025 <jalalvandi.sina@gmail.com>
//  * Package : parsidate
//  * License : Apache-2.0
//  * Version : 1.6.0
//  * URL     : https://github.com/jalalvandi/parsidate
//  * Sign: parsidate-20250415-a7a78013d25e-f7c1ad27b18ba6d800f915500eda993f
//
//! Defines the Persian seasons.

use crate::constants::{SEASON_NAMES_ENGLISH, SEASON_NAMES_PERSIAN};
use std::fmt;

/// Represents the four seasons in the Persian calendar.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Season {
    /// بهار (Bahar): Months 1-3 (Farvardin, Ordibehesht, Khordad)
    Bahar,
    /// تابستان (Tabestan): Months 4-6 (Tir, Mordad, Shahrivar)
    Tabestan,
    /// پاییز (Paeez): Months 7-9 (Mehr, Aban, Azar)
    Paeez,
    /// زمستان (Zemestan): Months 10-12 (Dey, Bahman, Esfand)
    Zemestan,
}

impl Season {
    /// Returns the full Persian name of the season.
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::Season;
    /// assert_eq!(Season::Bahar.name_persian(), "بهار");
    /// assert_eq!(Season::Zemestan.name_persian(), "زمستان");
    /// ```
    pub fn name_persian(&self) -> &'static str {
        SEASON_NAMES_PERSIAN[*self as usize]
    }

    /// Returns the English name of the season.
    ///
    /// # Examples
    /// ```rust
    /// use parsidate::Season;
    /// assert_eq!(Season::Bahar.name_english(), "Spring");
    /// assert_eq!(Season::Zemestan.name_english(), "Winter");
    /// ```
    pub fn name_english(&self) -> &'static str {
        SEASON_NAMES_ENGLISH[*self as usize]
    }

    /// Returns the starting month number (1-12) of the season.
    pub fn start_month(&self) -> u32 {
        match self {
            Season::Bahar => 1,     // Farvardin
            Season::Tabestan => 4,  // Tir
            Season::Paeez => 7,     // Mehr
            Season::Zemestan => 10, // Dey
        }
    }

    /// Returns the ending month number (1-12) of the season.
    pub fn end_month(&self) -> u32 {
        match self {
            Season::Bahar => 3,     // Khordad
            Season::Tabestan => 6,  // Shahrivar
            Season::Paeez => 9,     // Azar
            Season::Zemestan => 12, // Esfand
        }
    }
}

impl fmt::Display for Season {
    /// Formats the season using its Persian name.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_persian())
    }
}
