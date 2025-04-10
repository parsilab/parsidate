# ParsiDate: Comprehensive Persian Calendar Date & Time for Rust

[![crates.io](https://img.shields.io/crates/v/parsidate.svg)](https://crates.io/crates/parsidate)
![docs.rs (with version)](https://img.shields.io/docsrs/parsidate/1.4.0)
![Crates.io Total Downloads](https://img.shields.io/crates/d/parsidate)
[![license](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](./LICENSE)


`parsidate` provides comprehensive functionality for working with the Persian (Jalali/Shamsi) calendar system in Rust. It allows for seamless representation, conversion, validation, formatting, parsing, and arithmetic for both **dates (`ParsiDate`)** and **date-times (`ParsiDateTime`)**. It leverages the `chrono` crate for Gregorian representations, current time, and duration calculations.

### ✨ Features

*   **Date and DateTime Handling:** Represents both dates (`ParsiDate`) and date-times (`ParsiDateTime` with hour, minute, second).
*   **Conversion:** Easily convert between `chrono::NaiveDate`/`NaiveDateTime` (Gregorian) and `ParsiDate`/`ParsiDateTime`.
*   **Validation:** Check if combinations form valid Persian dates or date-times.
*   **Formatting:** Display dates and times in various formats using custom `strftime`-like patterns with Persian names and time components.
*   **Parsing:** Parse strings into `ParsiDate` or `ParsiDateTime` objects based on specified formats, including Persian month names and time components.
*   **Arithmetic:**
    *   Add or subtract days, months, and years to `ParsiDate` and `ParsiDateTime` (preserving time for the latter), correctly handling month lengths and leap years (including day clamping).
    *   Add or subtract `chrono::Duration` to/from `ParsiDateTime` for precise time calculations.
*   **Leap Year Calculation:** Determine if a Persian year is leap (using a 33-year cycle approximation) or if a Gregorian year is leap.
*   **Date/Time Information:** Get the Persian weekday name (شنبه-جمعه), weekday number (0-6), ordinal day of the year (1-366), and access individual date/time components.
*   **Helpers:** Get the first/last day of the month/year, or create modified dates/datetimes easily (`with_year`, `with_month`, `with_day`, `with_hour`, `with_minute`, `with_second`, `with_time`).
*   **Current Date/Time:** Get the current system date (`ParsiDate::today()`) or date-time (`ParsiDateTime::now()`) as Persian objects.
*   **Serde Support:** Optional serialization/deserialization for both `ParsiDate` and `ParsiDateTime` via the `serde` feature flag.
*   **Range:** Supports Persian years from 1 to 9999.

### ⚙️ Installation

Add `parsidate` to your `Cargo.toml`:

```toml
[dependencies]
parsidate = "1.4.0"
chrono = "0.4"
```

If you need serialization/deserialization support, enable the `serde` feature:

```toml
[dependencies]
parsidate = { version = "1.4.0", features = ["serde"] }
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] } # Required for derive
```

### 🚀 Usage Examples

```rust
use chrono::{NaiveDate, NaiveDateTime, Duration};
use parsidate::{ParsiDate, ParsiDateTime, DateError}; // Import both

// --- ParsiDate Usage (Date only) ---
// Create a ParsiDate (validates on creation)
let pd = ParsiDate::new(1403, 5, 2).unwrap(); // 2 Mordad 1403
assert_eq!(pd.year(), 1403);
assert_eq!(pd.month(), 5); // 5 = Mordad
assert_eq!(pd.day(), 2);

// Check validity
assert!(pd.is_valid());
let invalid_date_res = ParsiDate::new(1404, 12, 30); // 1404 is not leap
assert_eq!(invalid_date_res, Err(DateError::InvalidDate));

// Gregorian to Persian Date
let g_date = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
let pd_from_g = ParsiDate::from_gregorian(g_date).unwrap();
assert_eq!(pd_from_g, pd);

// Persian Date to Gregorian
let g_date_conv = pd.to_gregorian().unwrap();
assert_eq!(g_date_conv, g_date);

// Formatting Date
assert_eq!(pd.format("%Y-%m-%d is a %A"), "1403-05-02 is a سه‌شنبه");
assert_eq!(pd.format("%d %B %Y"), "02 مرداد 1403");
assert_eq!(pd.to_string(), "1403/05/02"); // Default Display

// Parsing Date
let parsed_short = ParsiDate::parse("1403/05/02", "%Y/%m/%d").unwrap();
assert_eq!(parsed_short, pd);

// Date Arithmetic
let next_day_date = pd.add_days(1).unwrap();
assert_eq!(next_day_date, ParsiDate::new(1403, 5, 3).unwrap());

// Get Today's Date
match ParsiDate::today() {
    Ok(today) => println!("Today's Persian date: {}", today.format("long")),
    Err(e) => eprintln!("Error getting today's date: {}", e),
}


// --- ParsiDateTime Usage (Date and Time) ---
// Create a ParsiDateTime (validates date and time)
let pdt = ParsiDateTime::new(1403, 5, 2, 15, 30, 45).unwrap();
assert_eq!(pdt.year(), 1403);
assert_eq!(pdt.hour(), 15);
assert_eq!(pdt.minute(), 30);
assert_eq!(pdt.second(), 45);
assert_eq!(pdt.date(), pd); // Access the ParsiDate part

// Invalid time creation
let invalid_time_res = ParsiDateTime::new(1403, 5, 2, 24, 0, 0);
assert_eq!(invalid_time_res, Err(DateError::InvalidTime));

// Gregorian DateTime to Persian DateTime
let g_dt = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap().and_hms_opt(15, 30, 45).unwrap();
let pdt_from_g = ParsiDateTime::from_gregorian(g_dt).unwrap();
assert_eq!(pdt_from_g, pdt);

// Persian DateTime to Gregorian DateTime
let g_dt_conv = pdt.to_gregorian().unwrap();
assert_eq!(g_dt_conv, g_dt);

// Formatting DateTime
assert_eq!(pdt.format("%Y/%m/%d %H:%M:%S"), "1403/05/02 15:30:45");
assert_eq!(pdt.format("%A %d %B ساعت %T"), "سه‌شنبه 02 مرداد ساعت 15:30:45");
assert_eq!(pdt.to_string(), "1403/05/02 15:30:45"); // Default Display

// Parsing DateTime
let parsed_dt = ParsiDateTime::parse("1403/05/02 15:30:45", "%Y/%m/%d %H:%M:%S").unwrap();
assert_eq!(parsed_dt, pdt);
let parsed_dt_t = ParsiDateTime::parse("1403-05-02T15:30:45", "%Y-%m-%dT%T").unwrap();
assert_eq!(parsed_dt_t, pdt);

// DateTime Arithmetic with Duration
let next_hour = pdt.add_duration(Duration::hours(1)).unwrap();
assert_eq!(next_hour, ParsiDateTime::new(1403, 5, 2, 16, 30, 45).unwrap());
let prev_minute_rollover = pdt.sub_duration(Duration::minutes(31)).unwrap();
assert_eq!(prev_minute_rollover, ParsiDateTime::new(1403, 5, 2, 14, 59, 45).unwrap());
// Using operators
assert_eq!(pdt + Duration::seconds(15), Ok(ParsiDateTime::new(1403, 5, 2, 15, 31, 0).unwrap()));

// DateTime Arithmetic with days/months/years (preserves time)
let next_day_dt = pdt.add_days(1).unwrap();
assert_eq!(next_day_dt, ParsiDateTime::new(1403, 5, 3, 15, 30, 45).unwrap());
let next_month_dt = pdt.add_months(1).unwrap();
assert_eq!(next_month_dt, ParsiDateTime::new(1403, 6, 2, 15, 30, 45).unwrap());

// Modifying time components
let pdt_morning = pdt.with_hour(9).unwrap();
assert_eq!(pdt_morning.hour(), 9);
let pdt_start_of_minute = pdt.with_second(0).unwrap();
assert_eq!(pdt_start_of_minute.second(), 0);

// Get Current DateTime
match ParsiDateTime::now() {
    Ok(now) => println!("Current Persian DateTime: {}", now),
    Err(e) => eprintln!("Error getting current DateTime: {}", e),
}

```

### Serialization/Deserialization Support (`serde` feature)

```rust
// --- Serde (Requires 'serde' feature) ---
#[cfg(feature = "serde")]
{
    // Make sure serde_json is a dev-dependency or added normally
    // use serde_json;

    // --- ParsiDate ---
    let pd_serde = ParsiDate::new(1403, 5, 2).unwrap();
    let json_pd = serde_json::to_string(&pd_serde).unwrap();
    println!("Serialized ParsiDate: {}", json_pd); // Output: {"year":1403,"month":5,"day":2}

    let deserialized_pd: ParsiDate = serde_json::from_str(&json_pd).unwrap();
    assert_eq!(deserialized_pd, pd_serde);
    assert!(deserialized_pd.is_valid());

    // Note: Default deserialization doesn't validate logical correctness for ParsiDate.
    let json_invalid_pd = r#"{"year":1404,"month":12,"day":30}"#; // Logically invalid
    let deserialized_invalid_pd: ParsiDate = serde_json::from_str(json_invalid_pd).unwrap();
    assert!(!deserialized_invalid_pd.is_valid());

    // --- ParsiDateTime ---
    let pdt_serde = ParsiDateTime::new(1403, 5, 2, 10, 20, 30).unwrap();
    let json_pdt = serde_json::to_string(&pdt_serde).unwrap();
    // Note the nested structure
    println!("Serialized ParsiDateTime: {}", json_pdt); // Output: {"date":{"year":1403,"month":5,"day":2},"hour":10,"minute":20,"second":30}

    let deserialized_pdt: ParsiDateTime = serde_json::from_str(&json_pdt).unwrap();
    assert_eq!(deserialized_pdt, pdt_serde);
    assert!(deserialized_pdt.is_valid());

    // Deserialization doesn't validate logical correctness for ParsiDateTime either.
    let json_invalid_pdt = r#"{"date":{"year":1403,"month":5,"day":2},"hour":25,"minute":0,"second":0}"#; // Invalid hour
    let deserialized_invalid_pdt: ParsiDateTime = serde_json::from_str(json_invalid_pdt).unwrap();
    assert!(!deserialized_invalid_pdt.is_valid()); // is_valid() check is needed
    assert_eq!(deserialized_invalid_pdt.hour(), 25); // Field gets populated
}
```

### Formatting and Parsing Specifiers

#### Formatting (`ParsiDate::format`, `ParsiDateTime::format`)

| Specifier | Description                         | Example (`1403-05-02`, `15:30:45`) | Notes         |
| :-------- | :---------------------------------- | :--------------------------------- | :------------ |
| `%Y`      | Year with century                   | `1403`                             |               |
| `%m`      | Month as zero-padded number         | `05`                               |               |
| `%d`      | Day of month as zero-padded number  | `02`                               |               |
| `%B`      | Full Persian month name             | `مرداد`                            |               |
| `%A`      | Full Persian weekday name           | `سه‌شنبه`                          |               |
| `%w`      | Weekday as number (Saturday=0)      | `3`                                |               |
| `%j`      | Day of year as zero-padded number   | `126`                              |               |
| `%H`      | Hour (24-hour clock), zero-padded   | `15`                               | DateTime only |
| `%M`      | Minute, zero-padded                 | `30`                               | DateTime only |
| `%S`      | Second, zero-padded                 | `45`                               | DateTime only |
| `%T`      | Equivalent to `%H:%M:%S`            | `15:30:45`                         | DateTime only |
| `%%`      | A literal `%` character             | `%`                                |               |

#### Parsing (`ParsiDate::parse`, `ParsiDateTime::parse`)

| Specifier | Description                         | Notes                                                     |
| :-------- | :---------------------------------- | :-------------------------------------------------------- |
| `%Y`      | Parses a 4-digit year               | Requires exactly 4 digits.                                |
| `%m`      | Parses a 2-digit month (01-12)      | Requires exactly 2 digits.                                |
| `%d`      | Parses a 2-digit day (01-31)        | Requires exactly 2 digits.                                |
| `%B`      | Parses a full Persian month name    | Case-sensitive, matches names like "فروردین", "مرداد".      |
| `%H`      | Parses a 2-digit hour (00-23)       | Requires exactly 2 digits. DateTime only.                 |
| `%M`      | Parses a 2-digit minute (00-59)     | Requires exactly 2 digits. DateTime only.                 |
| `%S`      | Parses a 2-digit second (00-59)     | Requires exactly 2 digits. DateTime only.                 |
| `%T`      | Parses time in `HH:MM:SS` format    | Requires correct separators and 2 digits per component. DateTime only. |
| `%%`      | Matches a literal `%` character     |                                                           |

**Note:** Parsing requires the input string to **exactly** match the format string, including separators and the number of digits specified (e.g., `%d` requires `02`, not `2`). `%A`, `%w`, `%j` are **not** supported for parsing. The final parsed date/time is validated logically (e.g., day exists in month, time components are in range).

### ⚠️ Error Handling

Most methods that can fail return a `Result<T, DateError>`. The `DateError` enum indicates the type of error:

*   `InvalidDate`: Date components do not form a valid Persian date.
*   `InvalidTime`: Time components (hour, minute, second) are out of range (e.g., hour 24). Specific to `ParsiDateTime`.
*   `GregorianConversionError`: Error during Gregorian <=> Persian conversion (e.g., date out of supported range).
*   `ParseError(ParseErrorKind)`: Input string failed to parse. `ParseErrorKind` gives specific details like:
    *   `FormatMismatch`: Input doesn't match format structure/literals.
    *   `InvalidNumber`: Failed to parse numeric component or wrong digit count.
    *   `InvalidMonthName`: Failed to parse `%B`.
    *   `UnsupportedSpecifier`: Used a format specifier not supported for parsing.
    *   `InvalidDateValue`: Parsed date components are logically invalid.
    *   `InvalidTimeValue`: Parsed time components are logically invalid.
*   `ArithmeticOverflow`: Date/time arithmetic resulted in a value outside the supported range (Year 1-9999) or internal overflow (e.g., adding large `Duration`).
*   `InvalidOrdinal`: Invalid day-of-year number provided (e.g., 0 or > 366).

### Contributing

Contributions (bug reports, feature requests, pull requests) are welcome! Please open an issue to discuss significant changes first.

### 📄 License

Licensed under the [Apache License, Version 2.0](./LICENSE).

```
Version:1.4.1
Sign: parsidate-20250410-f747d2246203-e40c0c12e3ffd6632e4a2ccd1b2b7e7d
```
