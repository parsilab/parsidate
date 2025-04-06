//  * Copyright (C) Mohammad (Sina) Jalalvandi (parsidate) 2024-2025 <jalalvandi.sina@gmail.com>
//  * Version : 1.3.3
//  * 128558ad-c066-4c4a-9b93-bca896bf4465
//  * src/constants.rs
//
//! Contains constant definitions used throughout the parsidate library.

// We need ParsiDate for the MIN/MAX constants.
// Use `crate::date::ParsiDate` to refer to the struct in date.rs.
use crate::date::ParsiDate;

// --- Constants ---

/// Minimum supported ParsiDate: Year 1, Month 1 (Farvardin), Day 1.
///
/// This corresponds approximately to the Gregorian date 622-03-21 (using proleptic Gregorian calendar for calculations),
/// representing the epoch start of the Persian calendar.
pub const MIN_PARSI_DATE: ParsiDate = ParsiDate {
    year: 1,
    month: 1,
    day: 1,
};

/// Maximum supported ParsiDate: Year 9999, Month 12 (Esfand), Day 29.
///
/// The year 9999 is chosen as a practical upper limit. According to the 33-year cycle approximation
/// used in this library (`9999 % 33 == 3`), it is *not* a leap year, hence the last day is the 29th.
pub const MAX_PARSI_DATE: ParsiDate = ParsiDate {
    year: 9999,
    month: 12,
    day: 29,
};

// --- Internal Helper Constants ---

/// Persian month names (index 0 = Farvardin, ..., 11 = Esfand).
// Keep internal constants non-pub unless they need to be part of the public API.
// These are used internally by ParsiDate methods.
pub(crate) const MONTH_NAMES_PERSIAN: [&str; 12] = [
    "فروردین",
    "اردیبهشت",
    "خرداد",
    "تیر",
    "مرداد",
    "شهریور",
    "مهر",
    "آبان",
    "آذر",
    "دی",
    "بهمن",
    "اسفند",
];

/// Persian weekday names (index 0 = Shanbeh/Saturday, ..., 6 = Jomeh/Friday).
// Keep internal
pub(crate) const WEEKDAY_NAMES_PERSIAN: [&str; 7] = [
    "شنبه",     // 0
    "یکشنبه",   // 1
    "دوشنبه",   // 2
    "سه‌شنبه",   // 3
    "چهارشنبه", // 4
    "پنجشنبه",  // 5
    "جمعه",     // 6
];
