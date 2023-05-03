# personnel-locale
Simple wrapper for fluent-rs. The goal of this project is to encapsulate a fluent crate boilerplate, so it will be easy to use in smaller projects.

It's initially written for my Personnel Manager project, which you can check on my GitHub page, but you are free to use it any way you like it.


# What is Fluent?

To understand anything below ou need to know what is Fluent in the first place.
You can check it here: https://projectfluent.org/

# Examples

## Message only
```rust
use personnel::Locale;

let locale = Locale::new(include_str!("../test.ftl")).unwrap();

println!(locale.get_message("hello-world", None::<String>, &[]).unwrap());
```

In the second line, we create our Locale struct. This will encapsulate all Fluent boilerplate for you. 

In the third line, you can see a ```get_message``` function. You will refer to it every time you want
to get a localized string from your Locale. It takes Fluent ```message```, ```attribute``` and
```args```. If you don't understand what is it, you should go back to Fluent documentation and
finish it before reading further. Also, as you can see ```attribute``` and ```args``` are optional
if you don't need or have them, you can pass ```None``` for the ```attribute``` and an empty array like this
```&[]``` for the ```args```.

If you are still confused about ```args``` and ```attribute``` examples are below.


## Attribute

```rust
let locale = Locale::new(include_str!("../test.ftl")).unwrap();

locale
    .get_message("hello-world", Some("special"), &[])
    .unwrap()
```
    
## Args

```rust
let locale = Locale::new(include_str!("../test.ftl")).unwrap();

locale
    .get_message("hello-world", Some("arg"), &[("world-type", "Custom")])
    .unwrap()
```

NOTE: When you use Fluent args an actual message will be ```"Hello, \u{2068}Custom\u{2069} World!"```.
That is one of FLuent's features. Not a bug.

## .flt

Here is fluent file used in all examples:

```fluent
hello-world = Hello, World!
    .special = Hello, Special World!
    .arg = Hello, {$world-type} World!
```
