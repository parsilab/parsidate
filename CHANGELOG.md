# Changelog

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
