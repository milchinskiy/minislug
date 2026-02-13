use minislug::{slugify, slugify_with, SlugOptions};

#[test]
fn basic_ascii() {
    assert_eq!(slugify("Hello, world!"), "hello-world");
    assert_eq!(slugify("  spaced   out "), "spaced-out");
    assert_eq!(slugify("a/b\\c"), "a-b-c");
}

#[test]
fn collapse_and_trim() {
    assert_eq!(slugify("a---b   c"), "a-b-c");
    assert_eq!(slugify("--a--"), "a");
    assert_eq!(slugify("a..b"), "a-b");
    assert_eq!(slugify("a..."), "a");
}

#[test]
fn forbidden_chars_boundary() {
    assert_eq!(slugify("a***b???c"), "a-b-c");
    assert_eq!(slugify("a:<b>|c\"d*e?f"), "a-b-c-d-e-f");
}

#[test]
fn control_and_nul() {
    assert_eq!(slugify("a\0b"), "a-b");
    assert_eq!(slugify("a\u{0008}b"), "a-b"); // backspace control
    assert_eq!(slugify("\n\t\r"), "file");
}

#[test]
fn underscore_default_kept() {
    assert_eq!(slugify("hello_world"), "hello_world");
    assert_eq!(slugify("__init__"), "__init__");
}

#[test]
fn underscore_as_separator_when_disabled() {
    let opt = SlugOptions {
        keep_underscore: false,
        ..Default::default()
    };
    assert_eq!(slugify_with("hello_world", opt), "hello-world");
    assert_eq!(slugify_with("__a__b__", opt), "a-b");
}

#[test]
fn custom_separator() {
    let opt = SlugOptions {
        separator: '_',
        keep_underscore: false, // otherwise '_' would be kept and separator won't appear much
        ..Default::default()
    };
    assert_eq!(slugify_with("a b c", opt), "a_b_c");
}

#[test]
fn empty_and_dot_cases() {
    let opt = SlugOptions::default();
    assert_eq!(slugify(""), opt.fallback);
    assert_eq!(slugify("..."), opt.fallback);
    assert_eq!(slugify("."), opt.fallback);
    assert_eq!(slugify(".."), opt.fallback);
}

#[test]
fn windows_trailing_dot_space_trim() {
    assert_eq!(slugify("hello."), "hello");
    assert_eq!(slugify("hello "), "hello");
    assert_eq!(slugify("hello.. "), "hello");
    assert_eq!(slugify("...hello..."), "hello");
}

#[test]
fn windows_reserved_names() {
    assert_eq!(slugify("CON"), "_con");
    assert_eq!(slugify("con.com"), "con-com");
    assert_eq!(slugify("nul"), "_nul");
    assert_eq!(slugify("com1"), "_com1");
    assert_eq!(slugify("com0"), "com0");
    assert_eq!(slugify("CON.com1"), "con-com1");
    assert_eq!(slugify("lpt9"), "_lpt9");
    assert_eq!(slugify("lpt10"), "lpt10");
}

#[test]
fn max_len_bytes_truncation_is_safe_utf8() {
    let opt = SlugOptions {
        max_len_bytes: 5,
        ..Default::default()
    };
    assert_eq!(slugify_with("abcdef", opt), "abcde"); // ASCII
}

#[cfg(feature = "unicode")]
#[test]
fn unicode_kept_when_enabled_and_allowed() {
    let opt = SlugOptions {
        allow_unicode: true,
        ..Default::default()
    };
    assert_eq!(slugify_with("Привіт світ", opt), "привіт-світ");
    assert_eq!(slugify_with("Тюлений Олень", opt), "тюлений-олень");
    assert_eq!(slugify_with("Вещати умеют мнози, а разумети не вси", opt), "вещати-умеют-мнози-а-разумети-не-вси");
}

#[cfg(not(feature = "unicode"))]
#[cfg(not(feature = "transliterate"))]
#[test]
fn unicode_ignored_when_feature_disabled() {
    let opt = SlugOptions {
        allow_unicode: false,
        ..Default::default()
    };
    assert_eq!(slugify_with("Привіт світ", opt), opt.fallback);
    assert_eq!(slugify_with("Вещати умеют мнози, а разумети не вси", opt), opt.fallback);
}

#[cfg(feature = "transliterate")]
#[test]
fn transliteration_basic_latin() {
    assert_eq!(slugify("Crème brûlée"), "creme-brulee");
    assert_eq!(slugify("Ångström"), "angstrom");
    assert_eq!(slugify("straße"), "strasse");
    assert_eq!(slugify("FLŰGGÅƏNK∂€ČHIŒβØL∫en"), "fluggaenkoechioebolsen");
}

#[cfg(feature = "transliterate")]
#[test]
fn transliteration_cyrillic() {
    assert_eq!(slugify("Прeвед мЕдВеД"), "preved-medved");
    assert_eq!(slugify("Киев"), "kiev");
}

#[cfg(feature = "transliterate")]
#[test]
fn transliteration_respects_keep_underscore() {
    let opt = SlugOptions {
        keep_underscore: true,
        ..Default::default()
    };
    assert_eq!(slugify_with("Харьков_Ужгород", opt), "harkov_uzhgorod");
}
