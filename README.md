# personnel-locale
Simple wrapper for fluent-rs.

# Examples
```rust
use personnel::Locale;

let locale = Locale::new(include_str!("../test.ftl")).unwrap();

println!(locale.get_message("hello-world", None::<String>, &[]).unwrap());
```
