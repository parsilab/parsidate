---

## title: ParsiDate layout: default

# ParsiDate: Persian Date & Time Utilities for Rust

Welcome to **ParsiDate**, the comprehensive Rust library for working with the Persian (Jalali/Shamsi) calendar. Whether you're building a terminal app, a time-aware API, or just handling Persian date/time conversions, ParsiDate has you covered.

&#x20;    &#x20;

---

## 🚀 Features at a Glance

* 📅 `ParsiDate`: Naive Persian date (year, month, day)
* 🕰️ `ParsiDateTime`: Naive date with time (hour, minute, second)
* 🌍 `ZonedParsiDateTime`: Timezone-aware date & time *(with **`timezone`** feature)*
* 🔁 Bi-directional conversion with `chrono`
* 🧠 Validation to prevent invalid dates
* 🎨 Custom Persian formatting and parsing
* ➕ Date/time arithmetic
* 📌 Date metadata (season, weekday, week number, etc.)
* ✅ Serde support (via `serde` feature)
* 🧰 Helpers for start/end of month/season/year
* 📅 Year range: 1 to 9999

---

## 📦 Installation

Add it to your `Cargo.toml`:

```toml
[dependencies]
parsidate = "1.7.0"
chrono = "0.4"
```

Enable optional features:

```toml
parsidate = { version = "1.7.0", features = ["serde", "timezone"] }
chrono-tz = "0.8"
```

Full feature:

```toml
parsidate = { version = "1.7.0", features = ["full"] }
```

---

## 📚 Documentation

Full docs are available on [**docs.rs**](https://docs.rs/parsidate). It includes detailed API reference, formatting/parsing specs, supported ranges, feature flags, and error types.

Also, check out our [GitHub Wiki](https://github.com/jalalvandi/ParsiDate/wiki) for in-depth guides, examples, and best practices.

---

## 🧪 Quick Example

```rust
use parsidate::ParsiDate;
use chrono::NaiveDate;

let pd = ParsiDate::new(1403, 5, 2).unwrap();
let g = pd.to_gregorian().unwrap();
assert_eq!(ParsiDate::from_gregorian(g).unwrap(), pd);
```

For more, see [usage examples](https://docs.rs/parsidate/latest/parsidate/#--usage-examples).

---

## 📈 Formatting & Parsing

ParsiDate supports `strftime`-like format specifiers with Persian locale. Examples:

```rust
pd.format("%A, %d %B %Y") // سه‌شنبه، 02 مرداد 1403
ParsiDate::parse("1403/05/02", "%Y/%m/%d")
```

See the full [formatting spec table](https://docs.rs/parsidate/latest/parsidate/#formatting-and-parsing-specifiers).

---

## 📬 Contribution

We welcome contributions! If you’ve got bug reports, feature requests, or pull requests—bring them on. Check the [Contributing Guide](https://github.com/jalalvandi/ParsiDate/blob/main/CONTRIBUTING.md) to get started.

---

## 📄 License

Licensed under the [Apache License 2.0](./LICENSE).

---

**Version:** 1.7.0
**Sign:** parsidate-20250607-fea13e856dcd-459c6e73c83e49e10162ee28b26ac7cd
