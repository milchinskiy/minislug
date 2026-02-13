#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(feature = "std")]
use std::string::String;

mod options;
pub use options::SlugOptions;

#[cfg(feature = "transliterate")]
mod translit;

/// Convert any string-like input into a safe filename slug
/// with default options.
///
/// # Examples
/// ```rust
/// use minislug::slugify;
///
/// assert_eq!(slugify("Hello, world!"), "hello-world");
/// assert_eq!(slugify("  spaced   out "), "spaced-out");
/// assert_eq!(slugify("a/b\\c"), "a-b-c");
/// ```
#[must_use]
pub fn slugify<S: AsRef<str>>(input: S) -> String {
    slugify_with(input.as_ref(), SlugOptions::default())
}

/// Slugify with custom options.
///
/// # Examples
/// ```rust
/// use minislug::{SlugOptions, slugify_with};
///
/// let opt = SlugOptions {
///     lowercase: true,
///     keep_underscore: true,
///     ..Default::default()
/// };
/// assert_eq!(slugify_with("Hello, world!", opt), "hello-world");
/// assert_eq!(slugify_with("  spaced   out ", opt), "spaced-out");
/// assert_eq!(slugify_with("a/b\\c", opt), "a-b-c");
/// ```
#[must_use]
pub fn slugify_with(input: &str, opt: SlugOptions) -> String {
    let sep = sanitize_separator(opt.separator);

    let mut out = String::with_capacity(core::cmp::min(input.len(), opt.max_len_bytes));
    let mut last_was_sep = true; // leading seps will be trimmed

    for ch in input.chars() {
        // Hard forbidden filename chars -> separator boundary
        if is_forbidden_filename_char(ch) {
            push_sep(&mut out, sep, &mut last_was_sep);
            continue;
        }

        // ASCII fast path
        if ch.is_ascii_alphanumeric() {
            push_ascii_alnum(&mut out, ch, opt.lowercase);
            last_was_sep = false;
            continue;
        }

        // underscore policy
        if ch == '_' && opt.keep_underscore {
            out.push('_');
            last_was_sep = false;
            continue;
        }

        // Unicode keep-as-is
        #[cfg(feature = "unicode")]
        {
            if opt.allow_unicode && ch.is_alphanumeric() {
                if opt.lowercase {
                    for lc in ch.to_lowercase() {
                        out.push(lc);
                    }
                } else {
                    out.push(ch);
                }
                last_was_sep = false;
                continue;
            }
        }

        // Transliteration into ASCII
        #[cfg(feature = "transliterate")]
        {
            if let Some(s) = translit::transliterate(ch, opt.lowercase) {
                if s.is_empty() {
                    continue;
                }
                let mut pushed_any = false;
                for t in s.chars() {
                    if t.is_ascii_alphanumeric() {
                        push_ascii_alnum(&mut out, t, opt.lowercase);
                        last_was_sep = false;
                        pushed_any = true;
                    } else if t == '_' && opt.keep_underscore {
                        out.push('_');
                        last_was_sep = false;
                        pushed_any = true;
                    } else if is_separatorish(t) {
                        push_sep(&mut out, sep, &mut last_was_sep);
                    } else {
                        // anything weird from transliteration => separator
                        push_sep(&mut out, sep, &mut last_was_sep);
                    }
                }
                if pushed_any {
                    continue;
                }
            }
        }

        // Common separators & whitespace -> separator
        if is_separatorish(ch) {
            push_sep(&mut out, sep, &mut last_was_sep);
            continue;
        }

        // Everything else -> separator
        push_sep(&mut out, sep, &mut last_was_sep);
    }

    // Windows quirks: trailing dot/space invalid; also trim trailing separators
    trim_end_seps_dots_spaces(&mut out, sep);
    trim_start_seps(&mut out, sep);

    // Avoid empty / "." / ".."
    if out.is_empty() || out == "." || out == ".." {
        out.clear();
        out.push_str(opt.fallback);
    }

    // Avoid hidden files (leading dot) if requested
    if opt.avoid_leading_dot && out.starts_with('.') {
        out.insert(0, '_');
    }

    // Avoid Windows reserved device names (case-insensitive)
    if is_windows_reserved_name(&out) {
        out.insert(0, '_');
    }

    // Enforce max length in bytes (UTF-8). Pop chars until <= max
    while out.len() > opt.max_len_bytes {
        out.pop();
    }
    trim_end_seps_dots_spaces(&mut out, sep);

    if out.is_empty() || out == "." || out == ".." {
        out.clear();
        out.push_str(opt.fallback);
    }

    out
}

#[inline]
const fn sanitize_separator(sep: char) -> char {
    match sep {
        '-' | '_' | '+' | '~' => sep,
        _ => '-',
    }
}

#[inline]
fn push_sep(out: &mut String, sep: char, last_was_sep: &mut bool) {
    if !*last_was_sep && !out.is_empty() {
        out.push(sep);
        *last_was_sep = true;
    }
}

#[inline]
fn push_ascii_alnum(out: &mut String, ch: char, lowercase: bool) {
    if lowercase {
        out.push(ch.to_ascii_lowercase());
    } else {
        out.push(ch);
    }
}

#[inline]
fn is_separatorish(ch: char) -> bool {
    // NOTE: '.' treated as separatorish because Windows forbids trailing '.' and it is often a boundary in filenames
    ch.is_whitespace() || matches!(ch, '-' | '.' | ',' | ';' | ':' | '+' | '=')
}

#[inline]
fn is_forbidden_filename_char(ch: char) -> bool {
    // Forbidden in Windows: < > : " / \ | ? * plus NUL and control chars.
    matches!(ch, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*') || ch == '\0' || ch.is_control()
}

fn trim_end_seps_dots_spaces(s: &mut String, sep: char) {
    while let Some(last) = s.chars().last() {
        if last == sep || last == '.' || last == ' ' {
            s.pop();
        } else {
            break;
        }
    }
}

fn trim_start_seps(s: &mut String, sep: char) {
    while s.starts_with(sep) {
        let mut it = s.chars();
        it.next();
        *s = it.collect();
    }
}

fn is_windows_reserved_name(name: &str) -> bool {
    let upper = ascii_upper(name);
    matches!(upper.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || (upper.starts_with("COM") && is_1_to_9_suffix(&upper[3..]))
        || (upper.starts_with("LPT") && is_1_to_9_suffix(&upper[3..]))
}

fn ascii_upper(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        out.push(ch.to_ascii_uppercase());
    }
    out
}

fn is_1_to_9_suffix(s: &str) -> bool {
    s.len() == 1 && matches!(s.as_bytes()[0], b'1'..=b'9')
}
