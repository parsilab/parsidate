
# ParsiDate: Comprehensive Persian Calendar Date & Time for Rust 

[![crates.io](https://img.shields.io/crates/v/parsidate.svg)](https://crates.io/crates/parsidate)
[![docs.rs (with version)](https://img.shields.io/docsrs/parsidate/latest)](https://docs.rs/parsidate/latest/parsidate/)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/parsidate)](https://crates.io/crates/parsidate)
[![license](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](./LICENSE)
[![Tests](https://github.com/parsicore/ParsiDate/actions/workflows/Tests.yml/badge.svg)](https://github.com/parsicore/ParsiDate/actions/workflows/Tests.yml)
[![Lint](https://github.com/parsicore/ParsiDate/actions/workflows/lint.yml/badge.svg)](https://github.com/parsicore/ParsiDate/actions/workflows/lint.yml)
![Maintenance](https://img.shields.io/badge/maintained-actively-green)

`parsidate` provides comprehensive functionality for working with the Persian (Jalali/Shamsi) calendar system in Rust. It allows for seamless representation, conversion, validation, formatting, parsing, and arithmetic for **naive dates (`ParsiDate`)**, **naive date-times (`ParsiDateTime`)**, and **timezone-aware date-times (`ZonedParsiDateTime`)**. It leverages the `chrono` crate for Gregorian representations and duration calculations.

### ✨ Features

*   **Three Core Types:**
    *   `ParsiDate`: A naive date (year, month, day).
    *   `ParsiDateTime`: A naive date and time (hour, minute, second).
    *   `ZonedParsiDateTime`: A timezone-aware date and time, handling DST and offsets correctly (requires the `timezone` feature).
*   **Conversion:** Easily convert between `chrono` types (`NaiveDate`, `NaiveDateTime`) and `parsidate` types.
*   **Validation:** Robust validation to ensure all created dates and times are logically correct.
*   **Formatting:** Display dates and times using `strftime`-like patterns with Persian names (`%B`, `%A`), seasons (`%K`), and time components (`%H`, `%M`, `%S`, `%T`).
*   **Parsing:** Parse strings into `ParsiDate` or `ParsiDateTime` from various formats.
*   **Arithmetic:**
    *   Add or subtract days, months, and years, with correct handling of month lengths and leap years.
    *   Add or subtract `chrono::Duration` for precise time calculations.
*   **Date/Time Information:** Get the Persian weekday, ordinal day, season, week number, and access individual date/time components.
*   **Helpers:** Easily get the first/last day of the month, year, or season, or create modified dates/datetimes (`with_year`, `with_hour`, etc.).
*   **Current Time:** Get the current system date (`ParsiDate::today()`), naive datetime (`ParsiDateTime::now()`), or zoned datetime (`ZonedParsiDateTime::now(tz)`).
*   **Serde Support:** Optional serialization/deserialization for all date/time types via the `serde` feature.
*   **Range:** Supports Persian years from 1 to 9999.

### ⚙️ Installation & Features

Add `parsidate` to your `Cargo.toml`. You can enable features based on your needs.

```toml
[dependencies]
parsidate = "1.7.0"
# Add other dependencies as needed
chrono = "0.4"
```

Available features:

-   **`serde`** (default): Enables serialization and deserialization support via the `serde` crate.
-   **`timezone`**: Enables the `ZonedParsiDateTime` struct and timezone functionality. Requires the `chrono-tz` crate.

To enable specific features:

```toml
[dependencies]
# Example: Enable both serde and timezone support
parsidate = { version = "1.7.0", features = ["serde", "timezone"] }

# For timezone support, you also need chrono-tz
chrono-tz = "0.8"
```

The `full` feature enables all available features: `parsidate = { version = "1.7.0", features = ["full"] }`.

### 🚀 Usage Examples

#### Naive Dates (`ParsiDate`)

```rust
use parsidate::{ParsiDate, DateError};
use chrono::NaiveDate;

// Creation and Validation
let pd = ParsiDate::new(1403, 5, 2).unwrap(); // 2 Mordad 1403
assert!(ParsiDate::new(1404, 12, 30).is_err()); // Invalid leap day

// Conversion
let g_date = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
assert_eq!(ParsiDate::from_gregorian(g_date).unwrap(), pd);
assert_eq!(pd.to_gregorian().unwrap(), g_date);

// Formatting & Parsing
assert_eq!(pd.format("%d %B %Y"), "02 مرداد 1403");
assert_eq!(ParsiDate::parse("1403/05/02", "%Y/%m/%d").unwrap(), pd);

// Arithmetic
let next_day = pd.add_days(1).unwrap();
assert_eq!(next_day, ParsiDate::new(1403, 5, 3).unwrap());
```

#### Naive DateTimes (`ParsiDateTime`)

```rust
use parsidate::{ParsiDateTime, DateError};
use chrono::Duration;

// Creation
let pdt = ParsiDateTime::new(1403, 5, 2, 15, 30, 45).unwrap();
assert_eq!(pdt.hour(), 15);

// Formatting & Parsing
assert_eq!(pdt.format("%Y/%m/%d %H:%M:%S"), "1403/05/02 15:30:45");
assert_eq!(ParsiDateTime::parse("1403-05-02T15:30:45", "%Y-%m-%dT%T").unwrap(), pdt);

// Arithmetic
let next_hour = pdt.add_duration(Duration::hours(1)).unwrap();
assert_eq!(next_hour.hour(), 16);
let next_day_dt = pdt.add_days(1).unwrap(); // Preserves time
assert_eq!(next_day_dt.day(), 3);
```

#### Timezone-Aware DateTimes (`ZonedParsiDateTime`)

This functionality requires the `timezone` feature.

```rust
// This example needs the `timezone` feature enabled for `parsidate`
// and the `chrono-tz` crate added to Cargo.toml.
#[cfg(feature = "timezone")]
{
    use parsidate::ZonedParsiDateTime;
    use chrono_tz::Asia::Tehran;
    use chrono_tz::Europe::London;

    // Get the current time in a specific timezone
    let tehran_now = ZonedParsiDateTime::now(Tehran);
    println!("The current time in Tehran is: {}", tehran_now);

    // Create a specific zoned time
    let dt = ZonedParsiDateTime::new(1403, 10, 10, 12, 0, 0, Tehran).unwrap();
    assert_eq!(dt.hour(), 12);
    // The default format includes the UTC offset
    assert_eq!(dt.to_string(), "1403/10/10 12:00:00 +0330");

    // Convert to another timezone
    let london_dt = dt.with_timezone(&London);
    // 12:00 in Tehran (UTC+3:30) is 8:30 in London (UTC+0)
    assert_eq!(london_dt.hour(), 8);
    assert_eq!(london_dt.minute(), 30);
    println!("{} in Tehran is {} in London.", dt, london_dt);
}
```

#### Seasons and Other Helpers

```rust
use parsidate::{ParsiDate, Season};

let winter_date = ParsiDate::new(1403, 11, 10).unwrap(); // Bahman 10th
assert_eq!(winter_date.season().unwrap(), Season::Zemestan);
assert_eq!(winter_date.format("%d %B is in %K"), "10 بهمن is in زمستان");

// Get season boundaries
let end_of_winter = winter_date.end_of_season().unwrap(); // 1403 is a leap year
assert_eq!(end_of_winter, ParsiDate::new(1403, 12, 30).unwrap());
```

### Formatting and Parsing Specifiers

The library supports `strftime`-like specifiers for formatting and parsing.

| Specifier | Description                         | Example (`1403-05-02`, `15:30:45`) | Notes         |
| :-------- | :---------------------------------- | :--------------------------------- | :------------ |
| `%Y`      | Year with century                   | `1403`                             |               |
| `%m`      | Month as zero-padded number         | `05`                               |               |
| `%d`      | Day of month as zero-padded number  | `02`                               |               |
| `%B`      | Full Persian month name             | `مرداد`                            |               |
| `%A`      | Full Persian weekday name           | `سه‌شنبه`                           |               |
| `%w`      | Weekday as number (Saturday=0)      | `3`                                |               |
| `%j`      | Day of year as zero-padded number   | `126`                              |               |
| `%K`      | Full Persian season name            | `تابستان`                          |               |
| `%H`      | Hour (24-hour clock), zero-padded   | `15`                               | `ParsiDateTime` |
| `%M`      | Minute, zero-padded                 | `30`                               | `ParsiDateTime` |
| `%S`      | Second, zero-padded                 | `45`                               | `ParsiDateTime` |
| `%T`      | Equivalent to `%H:%M:%S`            | `15:30:45`                         | `ParsiDateTime` |
| `%W`      | Week number of the year             | `19`                               |               |
| `%%`      | A literal `%` character             | `%`                                |               |

**Note:** Parsing requires an exact match to the format string. Specifiers like `%A`, `%w`, `%j`, `%K`, and `%W` are not supported for parsing.

### ⚠️ Error Handling

Most methods that can fail return a `Result<T, DateError>`. The `DateError` enum provides detailed information about the cause of failure, including `InvalidDate`, `InvalidTime`, `ParseError(ParseErrorKind)`, `GregorianConversionError`, and `ArithmeticOverflow`.

### Contributing

Contributions (bug reports, feature requests, pull requests) are welcome! Please open an issue to discuss significant changes first.

### 📄 License

Licensed under the [Apache License, Version 2.0](./LICENSE).
```
Version:1.7.0
Sign: parsidate-20250607-fea13e856dcd-459c6e73c83e49e10162ee28b26ac7cd
```