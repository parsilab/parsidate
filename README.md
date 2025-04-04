# ParsiDate: The Must Complete Persian (Jalali) Calendar for Rust

[![crates.io](https://img.shields.io/crates/v/parsidate.svg)](https://crates.io/crates/parsidate)
[![docs.rs](https://docs.rs/parsidate/badge.svg)](https://docs.rs/parsidate)
<!-- [![Build Status](https://your-ci-provider/badge.svg)](https://your-ci-provider/link) -->

(Scroll down for Persian / برای توضیحات فارسی به پایین صفحه مراجعه کنید)

---

## ParsiDate

A most complete Rust crate for handling Persian (also known as Jalali or Shamsi) calendar dates. It provides functionality for converting between Gregorian and Persian dates, validating dates, formatting them, and performing basic date calculations. It leverages the `chrono` crate for Gregorian date representation.

### Features

*   **Gregorian to Persian Conversion:** Convert `chrono::NaiveDate` to `ParsiDate`.
*   **Persian to Gregorian Conversion:** Convert `ParsiDate` back to `chrono::NaiveDate`.
*   **Date Validation:** Check if a year, month, day combination is a valid date in the Persian calendar (`is_valid`).
*   **Leap Year Calculation:** Determine if a Persian year is a leap year (`is_persian_leap_year`).
*   **Date Formatting:** Format `ParsiDate` into various string representations (`format`, `to_string`).
    *   `short`: "YYYY/MM/DD" (e.g., "1403/05/02")
    *   `long`: "D MMMM YYYY" (e.g., "2 مرداد 1403")
    *   `iso`: "YYYY-MM-DD" (e.g., "1403-05-02")
*   **Weekday Calculation:** Get the Persian name of the weekday (`weekday`).
*   **Date Difference:** Calculate the absolute number of days between two `ParsiDate` instances (`days_between`).
*   **Error Handling:** Uses a simple `DateError` enum for invalid operations.

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
parsidate = "1.2.0" # Replace with the latest version
chrono = "0.4"     # ParsiDate depends on chrono
```

### Usage Example

```rust
use chrono::NaiveDate;
use parsidate::{ParsiDate, DateError};

fn main() -> Result<(), DateError> {

    // --- Gregorian to Persian ---
    let gregorian_dt = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
    let persian_dt = ParsiDate::from_gregorian(gregorian_dt)?;
    println!("Gregorian: {} -> Persian: {}", gregorian_dt, persian_dt); // Uses Display impl (short format)
    assert_eq!(persian_dt.year, 1403);
    assert_eq!(persian_dt.month, 5); // Mordad
    assert_eq!(persian_dt.day, 2);


    // --- Persian to Gregorian ---
    let p_date = ParsiDate::new(1403, 1, 1)?; // Farvardin 1st, 1403 (Nowruz)
    let g_date = p_date.to_gregorian()?;
    println!("Persian: {} -> Gregorian: {}", p_date, g_date);
    // 1403 started on March 20, 2024 (Gregorian leap year)
    assert_eq!(g_date, NaiveDate::from_ymd_opt(2024, 3, 20).unwrap());


    // --- Formatting ---
    println!("Short format: {}", persian_dt.format("short")); // 1403/05/02
    println!("Long format: {}", persian_dt.format("long"));   // 2 مرداد 1403
    println!("ISO format: {}", persian_dt.format("iso"));     // 1403-05-02
    println!("Display trait: {}", persian_dt);                // 1403/05/02 (same as short)


    // --- Validation ---
    assert!(ParsiDate::new(1403, 12, 30)?.is_valid()); // 1403 is a Persian leap year
    assert!(!ParsiDate { year: 1404, month: 12, day: 30 }.is_valid()); // 1404 is not leap
    assert!(ParsiDate::new(1404, 13, 1).is_err()); // Invalid month


    // --- Weekday ---
    // 1403/05/02 corresponds to Tuesday, July 23, 2024
    println!("Weekday: {}", persian_dt.weekday()); // سه‌شنبه


    // --- Leap Year ---
    assert!(ParsiDate::is_persian_leap_year(1403));
    assert!(!ParsiDate::is_persian_leap_year(1404));


    // --- Days Between ---
    let date1 = ParsiDate::new(1403, 5, 2)?;
    let date2 = ParsiDate::new(1403, 5, 12)?;
    println!("Days between {} and {}: {}", date1, date2, date1.days_between(&date2)); // 10
    
    Ok(())
}
```

### API Overview

*   **`ParsiDate { year, month, day }`**: Struct representing a Persian date.
*   **`DateError::InvalidDate`**: Enum for errors during conversion or validation.
*   **`ParsiDate::new(y, m, d)`**: Creates a new `ParsiDate`, validating the input.
*   **`ParsiDate::from_gregorian(NaiveDate)`**: Converts Gregorian to Persian.
*   **`parsi_date.to_gregorian()`**: Converts Persian to Gregorian.
*   **`parsi_date.is_valid()`**: Checks if the date is valid.
*   **`ParsiDate::is_persian_leap_year(year)`**: Checks if a Persian year is leap.
*   **`ParsiDate::is_gregorian_leap_year(year)`**: Checks if a Gregorian year is leap.
*   **`parsi_date.format(style)`**: Formats the date to a string.
*   **`parsi_date.weekday()`**: Gets the Persian weekday name.
*   **`parsi_date.days_between(&other)`**: Calculates days between two dates.

### Error Handling

Functions that can fail (like conversion or creation with invalid data) return `Result<ParsiDate, DateError>`. The only current error variant is `DateError::InvalidDate`.

### Testing

Run tests using the standard Rust command:

```bash
cargo test
```

### Contributing

Contributions (bug reports, feature requests, pull requests) are welcome! Please feel free to open an issue or submit a PR.

### License

This project is licensed under the [MIT License](LICENSE-MIT).

---

## فارسی (Persian)

# ParsiDate: تقویم فارسی (جلالی) برای Rust

[![crates.io](https://img.shields.io/crates/v/parsidate.svg)](https://crates.io/crates/parsidate)
[![docs.rs](https://docs.rs/parsidate/badge.svg)](https://docs.rs/parsidate)


کاملترین کتابخانه برای زبان برنامه‌نویسی Rust جهت کار با تاریخ‌های تقویم فارسی (که به نام‌های جلالی یا شمسی نیز شناخته می‌شود). این کتابخانه امکان تبدیل تاریخ بین تقویم میلادی و فارسی، اعتبارسنجی تاریخ‌ها، قالب‌بندی آن‌ها و انجام محاسبات پایه‌ای تاریخ را فراهم می‌کند. این کتابخانه از کتابخانه `chrono` برای نمایش تاریخ میلادی استفاده می‌کند.

### ویژگی‌ها

*   **تبدیل میلادی به فارسی:** تبدیل `chrono::NaiveDate` به `ParsiDate`.
*   **تبدیل فارسی به میلادی:** تبدیل `ParsiDate` به `chrono::NaiveDate`.
*   **اعتبارسنجی تاریخ:** بررسی اینکه آیا ترکیب سال، ماه و روز یک تاریخ معتبر در تقویم فارسی است (`is_valid`).
*   **محاسبه سال کبیسه:** تشخیص اینکه آیا یک سال فارسی کبیسه است (`is_persian_leap_year`).
*   **قالب‌بندی تاریخ:** قالب‌بندی `ParsiDate` به رشته‌های متنی مختلف (`format`, `to_string`).
    *   `short`: "YYYY/MM/DD" (مثال: "1403/05/02")
    *   `long`: "D MMMM YYYY" (مثال: "2 مرداد 1403")
    *   `iso`: "YYYY-MM-DD" (مثال: "1403-05-02")
*   **محاسبه روز هفته:** دریافت نام فارسی روز هفته (`weekday`).
*   **اختلاف تاریخ:** محاسبه تعداد روزهای مطلق بین دو نمونه `ParsiDate` (`days_between`).
*   **مدیریت خطا:** استفاده از یک `enum` به نام `DateError` برای عملیات نامعتبر.

### نصب

این خطوط را به فایل `Cargo.toml` پروژه خود اضافه کنید:

```toml
[dependencies]
parsidate = "1.2.0" # با آخرین نسخه جایگزین کنید
chrono = "0.4"     # ParsiDate به chrono وابسته است
```

### مثال استفاده

```rust
use chrono::NaiveDate;
use parsidate::{ParsiDate, DateError};

fn main() -> Result<(), DateError> {

    // --- تبدیل میلادی به شمسی ---
    let gregorian_dt = NaiveDate::from_ymd_opt(2024, 7, 23).unwrap();
    let persian_dt = ParsiDate::from_gregorian(gregorian_dt)?;
    println!("میلادی: {} -> شمسی: {}", gregorian_dt, persian_dt); // از پیاده‌سازی Display استفاده می‌کند (فرمت کوتاه)
    assert_eq!(persian_dt.year, 1403);
    assert_eq!(persian_dt.month, 5); // مرداد
    assert_eq!(persian_dt.day, 2);


    // --- تبدیل شمسی به میلادی ---
    let p_date = ParsiDate::new(1403, 1, 1)?; // ۱ فروردین ۱۴۰۳ (نوروز)
    let g_date = p_date.to_gregorian()?;
    println!("شمسی: {} -> میلادی: {}", p_date, g_date);
    // سال ۱۴۰۳ در ۲۰ مارس ۲۰۲۴ شروع شد (سال کبیسه میلادی)
    assert_eq!(g_date, NaiveDate::from_ymd_opt(2024, 3, 20).unwrap());


    // --- قالب‌بندی ---
    println!("فرمت کوتاه: {}", persian_dt.format("short")); // 1403/05/02
    println!("فرمت بلند: {}", persian_dt.format("long"));   // 2 مرداد 1403
    println!("فرمت ISO: {}", persian_dt.format("iso"));     // 1403-05-02
    println!("Display trait: {}", persian_dt);              // 1403/05/02 (مشابه فرمت کوتاه)


    // --- اعتبارسنجی ---
    assert!(ParsiDate::new(1403, 12, 30)?.is_valid()); // ۱۴۰۳ سال کبیسه است
    assert!(!ParsiDate { year: 1404, month: 12, day: 30 }.is_valid()); // ۱۴۰۴ کبیسه نیست
    assert!(ParsiDate::new(1404, 13, 1).is_err()); // ماه نامعتبر


    // --- روز هفته ---
    // 1403/05/02 معادل سه‌شنبه ۲۳ جولای ۲۰۲۴ است
    println!("روز هفته: {}", persian_dt.weekday()); // سه‌شنبه


    // --- سال کبیسه ---
    assert!(ParsiDate::is_persian_leap_year(1403));
    assert!(!ParsiDate::is_persian_leap_year(1404));


    // --- اختلاف روزها ---
    let date1 = ParsiDate::new(1403, 5, 2)?;
    let date2 = ParsiDate::new(1403, 5, 12)?;
    println!("تعداد روز بین {} و {}: {}", date1, date2, date1.days_between(&date2)); // 10

    Ok(())
}

```

### مروری بر API

*   **`ParsiDate { year, month, day }`**: ساختاری (struct) برای نمایش تاریخ شمسی.
*   **`DateError::InvalidDate`**: اینام (enum) برای خطاها هنگام تبدیل یا اعتبارسنجی.
*   **`ParsiDate::new(y, m, d)`**: یک `ParsiDate` جدید ایجاد می‌کند و ورودی را اعتبارسنجی می‌کند.
*   **`ParsiDate::from_gregorian(NaiveDate)`**: تاریخ میلادی را به شمسی تبدیل می‌کند.
*   **`parsi_date.to_gregorian()`**: تاریخ شمسی را به میلادی تبدیل می‌کند.
*   **`parsi_date.is_valid()`**: بررسی می‌کند که آیا تاریخ معتبر است.
*   **`ParsiDate::is_persian_leap_year(year)`**: بررسی می‌کند که آیا سال کبیسه است.
*   **`ParsiDate::is_gregorian_leap_year(year)`**: بررسی می‌کند که آیا سال میلادی کبیسه است.
*   **`parsi_date.format(style)`**: تاریخ را به رشته قالب‌بندی می‌کند.
*   **`parsi_date.weekday()`**: نام فارسی روز هفته را برمی‌گرداند.
*   **`parsi_date.days_between(&other)`**: تعداد روزهای بین دو تاریخ را محاسبه می‌کند.

### مدیریت خطا

توابعی که ممکن است با شکست مواجه شوند (مانند تبدیل یا ایجاد با داده نامعتبر) مقدار `Result<ParsiDate, DateError>` را برمی‌گردانند. تنها نوع خطای فعلی `DateError::InvalidDate` است.

### تست

تست‌ها را با استفاده از دستور استاندارد Rust اجرا کنید:

```bash
cargo test
```

### مشارکت در پروژه

از مشارکت شما (گزارش خطا، درخواست ویژگی جدید، پول ریکوئست) استقبال می‌شود! لطفاً یک issue باز کنید یا یک PR ارسال نمایید.

### مجوز انتشار

این پروژه تحت مجوز [MIT](LICENSE-MIT) منتشر شده است.