# minislug

A tiny, dependency-free **slugifier** that turns any `&str` / `String`
into a **safe cross-platform filename**.

This crate produces a *single* safe **path component** (a filename).
It is **not** intended to sanitize full paths.

## Quick start

```rust
use minislug::slugify;

assert_eq!(slugify("Hello, world!"), "hello-world");
assert_eq!(slugify("a/b\\c"), "a-b-c");
```

## API

- `slugify<S: AsRef<str>>(input: S) -> String`
- `slugify_with(input: &str, opt: SlugOptions) -> String`

`slugify()` is just `slugify_with(..., SlugOptions::default())`.

### `SlugOptions`

```rust
pub struct SlugOptions {
    pub separator: char,
    pub lowercase: bool,
    pub max_len_bytes: usize,
    pub allow_unicode: bool,
    pub keep_underscore: bool,
    pub avoid_leading_dot: bool,
    pub fallback: &'static str,
}
```

Defaults (`SlugOptions::default()`):

- `separator = '-'`
- `lowercase = true`
- `max_len_bytes = 255`
- `allow_unicode = false`
- `keep_underscore = true`
- `avoid_leading_dot = true`
- `fallback = "file"`

Example:

```rust
use minislug::{slugify_with, SlugOptions};

let opt = SlugOptions {
    keep_underscore: false,
    ..Default::default()
};

assert_eq!(slugify_with("hello_world", opt), "hello-world");
```

## Behavior (what "safe" means here)

### Forbidden characters

The following are treated as **hard forbidden** filename characters and become
word boundaries:

- Windows-forbidden: `< > : " / \\ | ? *`
- NUL (`\0`) and all Unicode **control characters**

These are replaced by the configured separator (with collapsing; see below).

### Separator collapsing

Runs of separators / whitespace are collapsed into a single separator, and
leading/trailing separators are trimmed:

```rust
use minislug::slugify;

assert_eq!(slugify("  spaced   out "), "spaced-out");
assert_eq!(slugify("--a--"), "a");
```

### Trailing dot/space trimming (Windows quirk)

Windows does not allow filenames ending with a dot or space. `minislug` trims
trailing dots/spaces (and trailing separators):

```rust
use minislug::slugify;

assert_eq!(slugify("hello."), "hello");
assert_eq!(slugify("hello  "), "hello");
```

### Reserved device names (Windows quirk)

Windows reserves these device names (case-insensitive):

- `CON`, `PRN`, `AUX`, `NUL`
- `COM1`..`COM9`
- `LPT1`..`LPT9`

If the resulting slug matches one of these, `minislug` prefixes it with `_`:

```rust
use minislug::slugify;

assert_eq!(slugify("CON"), "_con");
assert_eq!(slugify("com1"), "_com1");
```

### Hidden files (leading dot)

If `avoid_leading_dot = true` and the result would start with `.`,
`minislug` prefixes `_`.

### Length limit

`max_len_bytes` caps the output length in **UTF-8 bytes**. Truncation is done
by popping whole `char`s, so the output remains valid UTF-8.

### Separator policy

`SlugOptions.separator` is **clamped** to a small allow-list for safety:

- `-`, `_`, `+`, `~` are accepted
- anything else falls back to `-`

This keeps the output conservative and avoids common filesystem / shell pitfalls.

### Underscore policy

If `keep_underscore = true`, `_` is preserved exactly. If `false`, `_` is treated
as a word boundary like whitespace.

## Optional features

### `unicode`

Enables keeping Unicode letters/digits **as-is** (only when `allow_unicode = true`).

```toml
minislug = { version = "x.x", features = ["unicode"] }
```

```rust
use minislug::{slugify_with, SlugOptions};

let opt = SlugOptions { allow_unicode: true, ..Default::default() };
assert_eq!(slugify_with("Тюлений Олень", opt), "тюлений-олень");
```

If the feature is **disabled**, `allow_unicode` is ignored and non-ASCII letters
will not be kept.

### `transliterate`

Adds lightweight, dependency-free transliteration for selected Unicode characters
into ASCII.

```toml
minislug = { version = "x.x", features = ["transliterate"] }
```

```rust
use minislug::slugify;

assert_eq!(slugify("Crème brûlée"), "creme-brulee");
```

Notes:

- Transliteration is **best-effort**: unmapped characters behave like word boundaries.
- If you enable both `unicode` and `transliterate` and set `allow_unicode = true`,
  Unicode alphanumerics are preserved and transliteration will not run for those
  characters. If you want transliteration instead, set `allow_unicode = false`.

## `no_std`

Default features include `std`. To use `no_std` + `alloc`:

```toml
minislug = { version = "x.x", default-features = false }
```

Your target must provide the `alloc` crate.

## MSRV

Minimum supported Rust version: **1.56** (edition 2021).

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.
