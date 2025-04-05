# ParsiDate: Comprehensive Persian Calendar for Rust

[![crates.io](https://img.shields.io/crates/v/parsidate.svg)](https://crates.io/crates/parsidate)
[![docs.rs](https://docs.rs/parsidate/badge.svg)](https://docs.rs/parsidate)
[![license](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](./LICENSE)


`parsidate` provides comprehensive functionality for working with the Persian (Jalali/Shamsi) calendar system in Rust. It allows for seamless conversion between Gregorian and Persian dates, validation, formatting, parsing, date arithmetic, and more, leveraging the `chrono` crate for some underlying operations.

### ✨ Features

*   **Conversion:** Easily convert dates between `chrono::NaiveDate` (Gregorian) and `ParsiDate`.
*   **Validation:** Check if a year, month, and day combination forms a valid Persian date.
*   **Formatting:** Display Persian dates in various predefined formats (`"short"`, `"long"`, `"iso"`) and using custom `strftime`-like patterns with Persian names.
*   **Parsing:** Parse strings into `ParsiDate` objects based on specified formats, including Persian month names.
*   **Arithmetic:** Add or subtract days, months, and years, correctly handling month lengths and leap years (including day clamping).
*   **Leap Year Calculation:** Determine if a Persian year is leap (using a 33-year cycle approximation) or if a Gregorian year is leap.
*   **Date Information:** Get the Persian weekday name (شنبه-جمعه), weekday number (0-6), and ordinal day of the year (1-366).
*   **Helpers:** Get the first/last day of the month/year, or create modified dates easily (`with_year`, `with_month`, `with_day`).
*   **Current Date:** Get the current system date as a `ParsiDate`.
*   **Serde Support:** Optional serialization/deserialization via the `serde` feature flag.
*   **Range:** Supports Persian years from 1 to 9999.

### ⚙️ Installation

Add `parsidate` to your `Cargo.toml`:

```toml
[dependencies]
parsidate = "1.3.2"
```

If you need serialization/deserialization support, enable the `serde` feature:

```toml
[dependencies]
parsidate = { version = "1.3.2", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] } # Required for derive
```

### 🚀 Usage Examples

```rust
use chrono::NaiveDate;
use parsidate::{ParsiDate, DateError};

// --- Basic Creation & Accessors ---
// Create a ParsiDate (validates on creation)
let pd = ParsiDate::new(1403, 5, 2).unwrap(); // 2 Mordad 1403
assert_eq!(pd.year(), 1403);
assert_eq!(pd.month(), 5); // 5 = Mordad
assert_eq!(pd.day(), 2);

// Check validity
assert!(pd.is_valid());
let invalid_date_res = ParsiDate::new(1404, 12, 30); // 1404 is not leap
assert_eq!(invalid_date_res, Err(DateError::InvalidDate));

// --- Conversion ---
// Gregorian to Persian
let g_date = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
let pd_from_g = ParsiDate::from_gregorian(g_date).unwrap();
assert_eq!(pd_from_g, pd);

// Persian to Gregorian
let g_date_conv = pd.to_gregorian().unwrap();
assert_eq!(g_date_conv, g_date);

// --- Formatting ---
// Predefined formats
assert_eq!(pd.format("short"), "1403/05/02");
assert_eq!(pd.format("long"), "2 مرداد 1403"); // Day not padded in long format
assert_eq!(pd.format("iso"), "1403-05-02");
// Default Display uses "short"
assert_eq!(pd.to_string(), "1403/05/02");

// Custom strftime-like format
assert_eq!(pd.format("%Y-%m-%d is a %A"), "1403-05-02 is a سه‌شنبه");
assert_eq!(pd.format("%d %B %Y"), "02 مرداد 1403"); // Custom format (%d) pads day
assert_eq!(pd.format("Day %j of year %Y"), "Day 126 of year 1403");

// --- Parsing ---
// Requires exact match including padding and separators
let parsed_short = ParsiDate::parse("1403/05/02", "%Y/%m/%d").unwrap();
assert_eq!(parsed_short, pd);

// Parsing with Persian month name (%B) - requires padded day (%d)
let parsed_long = ParsiDate::parse("02 مرداد 1403", "%d %B %Y").unwrap();
assert_eq!(parsed_long, ParsiDate::new(1403, 5, 2).unwrap());

let parsed_fail = ParsiDate::parse("2 مرداد 1403", "%d %B %Y"); // Fails: %d requires '02'
assert!(parsed_fail.is_err());

// --- Arithmetic ---
// Add/Subtract Days
let next_day = pd.add_days(1).unwrap();
assert_eq!(next_day, ParsiDate::new(1403, 5, 3).unwrap());
let prev_day = pd.sub_days(1).unwrap(); // Equivalent to add_days(-1)
assert_eq!(prev_day, ParsiDate::new(1403, 5, 1).unwrap());
let next_year_day = ParsiDate::new(1403, 12, 30).unwrap().add_days(1).unwrap(); // Cross leap year end
assert_eq!(next_year_day, ParsiDate::new(1404, 1, 1).unwrap());

// Add/Subtract Months (handles clamping)
let end_of_farvardin = ParsiDate::new(1403, 1, 31).unwrap();
let end_of_ordibehesht = end_of_farvardin.add_months(1).unwrap(); // 31 -> 31 days
assert_eq!(end_of_ordibehesht, ParsiDate::new(1403, 2, 31).unwrap());
let end_of_mehr = end_of_farvardin.add_months(6).unwrap(); // 31 -> 30 days (Mehr), clamps day
assert_eq!(end_of_mehr, ParsiDate::new(1403, 7, 30).unwrap());

let start_of_mehr = ParsiDate::new(1403, 7, 1).unwrap();
let start_of_farvardin = start_of_mehr.sub_months(6).unwrap();
assert_eq!(start_of_farvardin, ParsiDate::new(1403, 1, 1).unwrap());

// Add/Subtract Years (handles leap day Esfand 30)
let leap_day = ParsiDate::new(1403, 12, 30).unwrap(); // 1403 is leap
let next_year_clamped = leap_day.add_years(1).unwrap(); // To 1404 (common), clamps day
assert_eq!(next_year_clamped, ParsiDate::new(1404, 12, 29).unwrap());
let prev_year_clamped = leap_day.sub_years(1).unwrap(); // To 1402 (common), clamps day
assert_eq!(prev_year_clamped, ParsiDate::new(1402, 12, 29).unwrap());
let leap_to_leap = leap_day.add_years(4).unwrap(); // To 1407 (leap)
assert_eq!(leap_to_leap, ParsiDate::new(1407, 12, 30).unwrap());

// --- Validation & Leap Year ---
assert!(ParsiDate::is_persian_leap_year(1403));
assert!(!ParsiDate::is_persian_leap_year(1404));
assert_eq!(ParsiDate::days_in_month(1403, 12), 30); // Esfand in leap year
assert_eq!(ParsiDate::days_in_month(1404, 12), 29); // Esfand in common year

// --- Date Information ---
assert_eq!(pd.weekday(), Ok("سه‌شنبه".to_string())); // Tuesday
assert_eq!(ParsiDate::new(1403, 1, 4).unwrap().weekday(), Ok("شنبه".to_string())); // Saturday (Weekday 0)
assert_eq!(pd.ordinal(), Ok(126)); // Day number 126 in the year

// --- Helpers ---
assert_eq!(pd.first_day_of_month(), ParsiDate::new(1403, 5, 1).unwrap());
assert_eq!(pd.last_day_of_month(), ParsiDate::new(1403, 5, 31).unwrap()); // Mordad has 31 days
assert_eq!(ParsiDate::new(1404,12,10).unwrap().last_day_of_month(), ParsiDate::new(1404, 12, 29).unwrap()); // Esfand common year

assert_eq!(pd.first_day_of_year(), ParsiDate::new(1403, 1, 1).unwrap());
assert_eq!(pd.last_day_of_year(), ParsiDate::new(1403, 12, 30).unwrap()); // 1403 is leap

assert_eq!(pd.with_day(10).unwrap(), ParsiDate::new(1403, 5, 10).unwrap());
assert_eq!(pd.with_month(1).unwrap(), ParsiDate::new(1403, 1, 2).unwrap());
assert_eq!(pd.with_year(1400).unwrap(), ParsiDate::new(1400, 5, 2).unwrap());

// --- Today ---
match ParsiDate::today() {
    Ok(today) => println!("Today's Persian date: {}", today.format("long")),
    Err(e) => eprintln!("Error getting today's date: {}", e),
}
```

serialization/deserialization support:
```rust
// --- Serde (Requires 'serde' feature) ---
#[cfg(feature = "serde")]
{
    // Make sure serde_json is a dev-dependency or added normally
    // use serde_json;
    let pd_serde = ParsiDate::new(1403, 5, 2).unwrap();
    let json = serde_json::to_string(&pd_serde).unwrap();
    println!("Serialized: {}", json); // Output: {"year":1403,"month":5,"day":2}

    let deserialized: ParsiDate = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, pd_serde);
    assert!(deserialized.is_valid());

    // Note: Default deserialization doesn't validate logical correctness.
    let json_invalid = r#"{"year":1404,"month":12,"day":30}"#; // Structurally valid, logically invalid
    let deserialized_invalid: ParsiDate = serde_json::from_str(json_invalid).unwrap();
    // The object is created, but is_valid() will return false.
    assert!(!deserialized_invalid.is_valid());
    println!("Deserialized invalid date year: {}", deserialized_invalid.year()); // 1404
}
```

### Formatting and Parsing Specifiers

#### Formatting (`format`, `format_strftime`)

| Specifier | Description                         | Example (for `1403-05-02`) |
| :-------- | :---------------------------------- | :------------------------- |
| `%Y`      | Year with century                   | `1403`                     |
| `%m`      | Month as zero-padded number         | `05`                       |
| `%d`      | Day of month as zero-padded number  | `02`                       |
| `%B`      | Full Persian month name             | `مرداد`                    |
| `%A`      | Full Persian weekday name           | `سه‌شنبه`                   |
| `%w`      | Weekday as number (Saturday=0)      | `3`                        |
| `%j`      | Day of year as zero-padded number   | `126`                      |
| `%%`      | A literal `%` character             | `%`                        |

#### Parsing (`parse`)

| Specifier | Description                         | Notes                                                |
| :-------- | :---------------------------------- | :--------------------------------------------------- |
| `%Y`      | Parses a 4-digit year               | Requires exactly 4 digits.                           |
| `%m`      | Parses a 2-digit month (01-12)      | Requires exactly 2 digits.                           |
| `%d`      | Parses a 2-digit day (01-31)        | Requires exactly 2 digits.                           |
| `%B`      | Parses a full Persian month name    | Case-sensitive, matches names like "فروردین", "مرداد". |
| `%%`      | Matches a literal `%` character     |                                                      |

**Note:** Parsing requires the input string to exactly match the format string, including separators and the number of digits specified (e.g., `%d` requires `02`, not `2`). `%A`, `%w`, `%j` are **not** supported for parsing.

### ⚠️ Error Handling

Most methods that can fail (creation, conversion, parsing, arithmetic) return a `Result<T, DateError>`. The `DateError` enum indicates the type of error:

*   `InvalidDate`: Components do not form a valid Persian date.
*   `GregorianConversionError`: Error during Gregorian <=> Persian conversion (e.g., out of range).
*   `ParseError(ParseErrorKind)`: Input string failed to parse according to the format. `ParseErrorKind` gives specific details.
*   `ArithmeticOverflow`: Date arithmetic resulted in a date outside the supported range (1-9999) or internal overflow.
*   `InvalidOrdinal`: Invalid day-of-year number provided (e.g., 0 or > 366).

### Contributing

Contributions (bug reports, feature requests, pull requests) are welcome! Please open an issue to discuss significant changes first.

### 📄 License

Licensed under either of [Apache License, Version 2.0](LICENSE).