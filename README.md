# ParsiDate: Comprehensive Persian Calendar Date & Time for Rust

[![crates.io](https://img.shields.io/crates/v/parsidate.svg)](https://crates.io/crates/parsidate)
[![docs.rs (with version)](https://img.shields.io/docsrs/parsidate/1.6.0)](https://docs.rs/parsidate/latest/parsidate/)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/parsidate)](https://crates.io/crates/parsidate)
[![license](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](./LICENSE)
[![Tests](https://github.com/jalalvandi/ParsiDate/actions/workflows/Tests.yml/badge.svg)](https://github.com/jalalvandi/ParsiDate/actions/workflows/Tests.yml)
[![Lint](https://github.com/jalalvandi/ParsiDate/actions/workflows/lint.yml/badge.svg)](https://github.com/jalalvandi/ParsiDate/actions/workflows/lint.yml)
![Maintenance](https://img.shields.io/badge/maintained-actively-green)


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
*   **Date/Time Information:** Get the Persian weekday name (شنبه-جمعه), weekday number (0-6), ordinal day of the year (1-366), Persian season (`Season` enum), and access individual date/time components. 
*   **Helpers:** Get the first/last day of the month/year/season, or create modified dates/datetimes easily (`with_year`, `with_month`, `with_day`, `with_hour`, `with_minute`, `with_second`, `with_time`). 
*   **Current Date/Time:** Get the current system date (`ParsiDate::today()`) or date-time (`ParsiDateTime::now()`) as Persian objects.
*   **Week of Year:** Calculate the week number within the Persian year (Saturday start).
*   **Season:** Display and work with Persian seasons (Spring, Summer, Autumn, Winter), get the Persian name of a season, determine the season for a given date, and access the start and end dates of each season.
*   **Serde Support:** Optional serialization/deserialization for `ParsiDate`, `ParsiDateTime`, and `Season` via the `serde` feature flag.
*   **Range:** Supports Persian years from 1 to 9999.

### ⚙️ Installation

Add `parsidate` and its required dependency `chrono` to your `Cargo.toml`:

```toml
[dependencies]
parsidate = "1.6.1"
chrono = "0.4"
```

For serialization support, you can enable the features you need:

```toml
[dependencies]
parsidate = { version = "1.6.1", features = ["serde"] }
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }  # Required for derive macros

# OR for full serialization support including JSON:
parsidate = { version = "1.6.0", features = ["full"] }
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }
```

Available features:
- `serde`: Enables basic serialization/deserialization support
- `json`: Enables JSON serialization/deserialization (includes `serde`)
- `full`: Enables all available features

### 🚀 Usage Examples

```rust
use chrono::{NaiveDate, NaiveDateTime, Duration};
// Import Season along with other types
use parsidate::{ParsiDate, ParsiDateTime, DateError, Season};

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
let parsed_long = ParsiDate::parse("02 مرداد 1403", "%d %B %Y").unwrap();
assert_eq!(parsed_long, pd);

// Date Arithmetic
let next_day_date = pd.add_days(1).unwrap();
assert_eq!(next_day_date, ParsiDate::new(1403, 5, 3).unwrap());
let prev_month_date = pd.sub_months(1).unwrap();
assert_eq!(prev_month_date, ParsiDate::new(1403, 4, 2).unwrap()); // Tir 2nd

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
let parsed_dt_b = ParsiDateTime::parse("02 مرداد 1403 - 15:30:45", "%d %B %Y - %T").unwrap();
assert_eq!(parsed_dt_b, pdt);

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

// --- Season Support Examples ---
let winter_date = ParsiDate::new(1403, 11, 10).unwrap(); // Bahman 10th (Winter)
let season = winter_date.season().unwrap();
assert_eq!(season, Season::Zemestan);
assert_eq!(season.name_persian(), "زمستان");
assert_eq!(winter_date.format("%d %B is in %K"), "10 بهمن is in زمستان");

// Get season boundaries
let start_winter = winter_date.start_of_season().unwrap();
let end_winter = winter_date.end_of_season().unwrap(); // 1403 is leap
assert_eq!(start_winter, ParsiDate::new(1403, 10, 1).unwrap()); // Dey 1st
assert_eq!(end_winter, ParsiDate::new(1403, 12, 30).unwrap()); // Esfand 30th

// Season support works with ParsiDateTime too
let dt_spring = ParsiDateTime::new(1404, 2, 20, 10, 0, 0).unwrap(); // Ordibehesht 20th (Spring)
assert_eq!(dt_spring.season().unwrap(), Season::Bahar);
assert_eq!(dt_spring.format("%Y/%m/%d (%K) %T"), "1404/02/20 (بهار) 10:00:00");
let spring_end_dt = dt_spring.end_of_season().unwrap();
assert_eq!(spring_end_dt.date(), ParsiDate::new(1404, 3, 31).unwrap()); // Khordad 31st
assert_eq!(spring_end_dt.time(), (10, 0, 0)); // Time preserved
```

### Serialization/Deserialization Support (`serde` feature)

```rust
// --- Serde (Requires 'serde' feature) ---
#[cfg(feature = "serde")]
{
    // Make sure serde_json is a dev-dependency or added normally
    // use serde_json;
    use parsidate::{ParsiDate, ParsiDateTime, Season}; // Need Season here too if used

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

    // --- Season Enum ---
    let season = Season::Paeez;
    let json_season = serde_json::to_string(&season).unwrap();
    println!("Serialized Season: {}", json_season); // Output: "Paeez" (enum variant name)

    let deserialized_season: Season = serde_json::from_str(&json_season).unwrap();
    assert_eq!(deserialized_season, Season::Paeez);
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
| `%A`      | Full Persian weekday name           | `سه‌شنبه`                           |               |
| `%w`      | Weekday as number (Saturday=0)      | `3`                                |               |
| `%j`      | Day of year as zero-padded number   | `126`                              |               |
| `%K`      | Full Persian season name            | `تابستان`                          |               |
| `%H`      | Hour (24-hour clock), zero-padded   | `15`                               | DateTime only |
| `%M`      | Minute, zero-padded                 | `30`                               | DateTime only |
| `%S`      | Second, zero-padded                 | `45`                               | DateTime only |
| `%T`      | Equivalent to `%H:%M:%S`            | `15:30:45`                         | DateTime only |
| `%W`      | Week number of the year             | `19`                               |               |
| `%%`      | A literal `%` character             | `%`                                |               |

#### Parsing (`ParsiDate::parse`, `ParsiDateTime::parse`)

| Specifier | Description                         | Notes                                                     |
| :-------- | :---------------------------------- | :-------------------------------------------------------- |
| `%Y`      | Parses a 4-digit year               | Requires exactly 4 digits.                                |
| `%m`      | Parses a 2-digit month (01-12)      | Requires exactly 2 digits.                                |
| `%d`      | Parses a 2-digit day (01-31)        | Requires exactly 2 digits.                                |
| `%B`      | Parses a full Persian month name    | Case-sensitive, matches names like "فروردین", "مرداد".    |
| `%H`      | Parses a 2-digit hour (00-23)       | Requires exactly 2 digits. DateTime only.                 |
| `%M`      | Parses a 2-digit minute (00-59)     | Requires exactly 2 digits. DateTime only.                 |
| `%S`      | Parses a 2-digit second (00-59)     | Requires exactly 2 digits. DateTime only.                 |
| `%T`      | Parses time in `HH:MM:SS` format    | Requires correct separators and 2 digits per component. DateTime only. |
| `%%`      | Matches a literal `%` character     |                                                           |

**Note:** Parsing requires the input string to **exactly** match the format string, including separators and the number of digits specified (e.g., `%d` requires `02`, not `2`). `%A`, `%w`, `%j`, `%K` are **not** supported for parsing. The final parsed date/time is validated logically (e.g., day exists in month, time components are in range). <!-- Added %K here -->

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
Version:1.6.1
Sign: parsidate-20250604-e62e50090da3-d83a3ca6effcd0c0090c02213ae867cb
```