**Parsidate** is a Rust library for converting between Gregorian and Persian (Jalali) dates. It provides utilities for date conversion, validation, formatting, and calculating differences between dates.

## Features

- Convert Gregorian dates to Persian (Jalali) dates and vice versa.
- Validate Persian dates.
- Format Persian dates in different styles (short, long, ISO).
- Calculate the number of days between two Persian dates.
- Get the weekday name in Persian.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
parsidate = "0.1.0"
```

## Usage

Here are some examples of how to use the library:

### Convert Gregorian to Persian

```rust
use chrono::NaiveDate;
use parsidate::ParsiDate;

let gregorian = NaiveDate::from_ymd_opt(2025, 3, 27).unwrap();
let persian = ParsiDate::from_gregorian(gregorian).unwrap();
assert_eq!(persian.to_string(), "1404/01/07");
```

### Convert Persian to Gregorian

```rust
use chrono::NaiveDate;
use parsidate::ParsiDate;

let persian = ParsiDate { year: 1404, month: 1, day: 7 };
let gregorian = persian.to_gregorian().unwrap();
assert_eq!(gregorian, NaiveDate::from_ymd_opt(2025, 3, 27).unwrap());
```

### Validate Persian Dates

```rust
use parsidate::ParsiDate;

let valid_date = ParsiDate { year: 1404, month: 1, day: 7 };
assert!(valid_date.is_valid());

let invalid_date = ParsiDate { year: 1404, month: 1, day: 32 };
assert!(!invalid_date.is_valid());
```

### Format Persian Dates

```rust
use parsidate::ParsiDate;

let date = ParsiDate { year: 1404, month: 1, day: 7 };
assert_eq!(date.format("short"), "1404/01/07");
assert_eq!(date.format("long"), "7 Farvardin 1404");
assert_eq!(date.format("iso"), "1404-01-07");
```

### Calculate Days Between Two Dates

```rust
use parsidate::ParsiDate;

let date1 = ParsiDate { year: 1404, month: 1, day: 7 };
let date2 = ParsiDate { year: 1404, month: 1, day: 10 };
assert_eq!(date1.days_between(&date2), 3);
```

### Get Weekday Name

```rust
use parsidate::ParsiDate;

let date = ParsiDate { year: 1404, month: 1, day: 7 };
assert_eq!(date.weekday(), "Panjshanbe");
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Author

Developed by **Sina Jalalvandi**. For inquiries, contact: [Jalalvandi.Sina@gmail.com](mailto:Jalalvandi.Sina@gmail.com).
```