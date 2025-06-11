# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.7.0] - 2025-06-07
This version introduces major new functionality for timezone handling, along with several improvements and fixes to the existing API.

### ✨ Added

-   **Timezone Support (`timezone` feature)([#23](https://github.com/parsicore/ParsiDate/issues/23)):**
    -   Introduced a new primary struct, `ZonedParsiDateTime<Tz: TimeZone>`, for handling timezone-aware date and time objects. This is available under the new `timezone` feature flag.
    -   `ZonedParsiDateTime::now(tz: Tz)`: Creates a new instance representing the current time in the specified timezone.
    -   `ZonedParsiDateTime::new(...)`: A robust constructor that correctly handles Daylight Saving Time (DST) ambiguities and non-existent times.
    -   `with_timezone(&self, new_tz: &NewTz)`: A method to convert a datetime from one timezone to another while preserving the absolute moment in time.
    -   Full implementation of accessors (`.year()`, `.hour()`, etc.), arithmetic operators (`+`, `-` with `chrono::Duration`), and comparison traits (`PartialEq`, `Ord`).
`timezone`).

Sign: parsidate-20250607-fea13e856dcd-459c6e73c83e49e10162ee28b26ac7cd


## [1.6.1] - 2025-06-04
### Changed
- **Dependency Optimization([#21](https://github.com/parsicore/ParsiDate/issues/21)):**
  - Lowered minimum required Rust version from 1.85 to 1.70 for broader compatibility
  - Made `serde_json` an optional dependency
  - Restructured feature flags:
    - `serde`: Basic serialization support (default feature)
    - `json`: JSON serialization support (includes serde)
    - `full`: All available features

Sign: parsidate-20250604-e62e50090da3-d83a3ca6effcd0c0090c02213ae867cb

## [1.6.0] - 2025-04-15 
### Added
  *   **Week of Year:** Calculate the week number within the Persian year (Saturday start).
      *   Added `.week_of_year()-> Result<u32, DateError>` method to `ParsiDate` and `ParsiDateTime` to determine the week of the year for a given date.

Sign: parsidate-20250415-a7a78013d25e-f7c1ad27b18ba6d800f915500eda993f


## [1.5.0] - 2025-04-12
### Added

*   **Persian Season Support:**
    *   Introduced a new `Season` enum (`Bahar`, `Tabestan`, `Paeez`, `Zemestan`) representing the four Persian seasons.
    *   Added `.season() -> Result<Season, DateError>` methods to `ParsiDate` and `ParsiDateTime` to determine the season a given date falls into.
    *   The `Season` enum provides `name_persian()` and `name_english()` methods to retrieve season names.
    *   The `Season` enum now implements `fmt::Display` using the Persian name (e.g., `println!("{}", Season::Bahar);` prints "بهار").
    *   Added `.start_of_season() -> Result<ParsiDate/Time, DateError>` and `.end_of_season() -> Result<ParsiDate/Time, DateError>` methods to `ParsiDate` and `ParsiDateTime` to get the first and last date/datetime of the season containing the instance.
    *   Added a new format specifier `%K` to `ParsiDate::format()`/`format_strftime()` and `ParsiDateTime::format()` for displaying the full Persian season name (e.g., "تابستان").
    *   Added comprehensive tests for all season-related functionality.

Sign: parsidate-20250412-5b5da84ef2a0-e257858a7eca95f93b008ec2a96edf6d



## [1.4.0]

### Added

 **DateTime support with ParsiDateTime**,
Key features introduced:
- **`ParsiDateTime` Struct:** Located in `src/datetime.rs`, stores a `ParsiDate` and H:M:S components.
- **Conversions:** Added `from_gregorian(NaiveDateTime)` and `to_gregorian() -> NaiveDateTime`.
- **Current Time:** Implemented `ParsiDateTime::now()` to get the current system date and time.
- **Formatting:** Extended `format()` method with time specifiers (%H, %M, %S, %T).
- **Parsing:** Implemented `ParsiDateTime::parse()` supporting date and time specifiers.
- **Arithmetic:** Enabled addition/subtraction with `chrono::Duration` and specific `add/sub_days/months/years` methods preserving time. Overloaded operators for `Duration`.
- **Validation:** `is_valid()` checks both date and time components.
- **Error Handling:** Introduced `DateError::InvalidTime` and `ParseErrorKind::InvalidTimeValue`.
- **Serde:** Extended optional `serde` support to `ParsiDateTime`.
- **Internal Visibility:** Adjusted visibility of `ParsiDate` helper methods (`to_gregorian_internal`, `weekday_internal`, etc.) to `pub(crate)` to allow sharing logic with `ParsiDateTime`.
- **Testing:** Added extensive unit tests covering all new `ParsiDateTime` functionality.
- **Documentation:** Updated crate-level documentation, examples, and added doc comments for `ParsiDateTime`.


## [1.3.3]

### Changed
**Change Project Structure:**
The libs.rs file has now been split into the following files for better debugging and faster development:
- src/lib.rs
- src/constants.rs
- src/error.rs
- src/tests.rs
- src/date.rs

> The codes, functions, logical structure, and algorithms of the program have not changed and are the same as the previous version. If you are currently unable to update, there is no need to worry.


## [1.3.2]

### Added
- **Date Arithmetic**:
  - Implemented `add_days`, `sub_days`, `add_months`, `sub_months`, `add_years`, `sub_years`
  - Proper handling of month/year rollovers and day clamping (e.g., 31-day to 30-day month transitions, leap day adjustments)
  
- **String Parsing**:
  - Added `ParsiDate::parse` function with strftime-like format specifiers (`%Y`, `%m`, `%d`, `%B`)
  - Support for common separators
  - Detailed error reporting via `ParseErrorKind`

- **Advanced Formatting**:
  - Enhanced `ParsiDate::format` with strftime-like format strings (`%Y`, `%m`, `%d`, `%B`, `%A`, `%j`, `%w`, `%%`)
  - Maintained support for predefined styles ("short", "long", "iso")

- **Serde Support**:
  - Added optional serialization/deserialization via `serde` feature flag
  - Compatibility with common formats like JSON, TOML, etc.

- **New Functions**:
  - `ParsiDate::today()` - Get current system date
  - `ordinal()` - Get day number within year (1-366)
  - `from_ordinal()` - Construct date from year and ordinal day

- **Helper Methods**:
  - Constructor helpers: `with_year`, `with_month`, `with_day`
  - Boundary finders: `first_day_of_month`, `last_day_of_month`, `first_day_of_year`, `last_day_of_year`

- **Error Handling**:
  - Expanded `DateError` enum with new variants:
    - Parsing errors (`ParseError`, `ParseErrorKind`)
    - Arithmetic errors (`ArithmeticOverflow`, `InvalidOrdinal`)

- **Constants**:
  - Defined `MIN_PARSI_DATE` and `MAX_PARSI_DATE` for clear date range boundaries

- **Performance**:
  - Added `unsafe fn new_unchecked` for cases where date validity is guaranteed

### Improved
- **Documentation**:
  - Significantly updated and expanded rustdoc for all public APIs
  - Added comprehensive examples and usage guidelines

- **Testing**:
  - Added extensive unit tests covering new functionalities
  - Added edge case testing for arithmetic operations and parsing

### Fixed
- (No breaking changes or fixes listed in the provided notes)
