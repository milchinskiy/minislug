#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(feature = "std")]
use std::string::String;

/// Transliterate a single char into an ASCII-ish string.
/// - Returns `Some(&'static str)` (or an owned string in a couple cases) for known mappings.
/// - Returns `None` if no mapping exists.
pub fn transliterate(ch: char, lowercase: bool) -> Option<String> {
    // fast path: ASCII handled by caller
    if ch.is_ascii() {
        return None;
    }

    // Most mappings are lowercase; we optionally “title-case” the first letter for uppercase inputs
    // when lowercase == false.
    let is_upper = ch.is_uppercase() && !lowercase;

    let base: &'static str = match ch {
        // Latin-1-ish
        'Ç' | 'Ć' | 'Ĉ' | 'Ċ' | 'Č' | 'ç' | 'ć' | 'ĉ' | 'ċ' | 'č' => "c",
        'Ð' | 'Ď' | 'Đ' | 'ð' | 'ď' | 'đ' | 'Д' | 'д' => "d",
        'Ə' | 'ə' | '€' | 'È' | 'É' | 'Ê' | 'Ë' | 'Ē' | 'Ĕ' | 'Ė' | 'Ę' | 'Ě' | 'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ĕ'
        | 'ė' | 'ę' | 'ě' | 'Е' | 'е' | 'Ё' | 'ё' | 'Э' | 'э' => "e",
        'Ì' | 'Í' | 'Î' | 'Ï' | 'Ĩ' | 'Ī' | 'Ĭ' | 'Į' | 'İ' | 'ì' | 'í' | 'î' | 'ï' | 'ĩ' | 'ī' | 'ĭ' | 'į' | 'ı'
        | 'И' | 'и' | 'І' | 'і' => "i",
        'Ñ' | 'Ń' | 'Ņ' | 'Ň' | 'ñ' | 'ń' | 'ņ' | 'ň' | 'Н' | 'н' => "n",
        '∂' | 'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' | 'Ō' | 'Ŏ' | 'Ő' | 'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' | 'ŏ'
        | 'ő' | 'О' | 'о' => "o",
        'Ù' | 'Ú' | 'Û' | 'Ü' | 'Ũ' | 'Ū' | 'Ŭ' | 'Ů' | 'Ű' | 'Ų' | 'ù' | 'ú' | 'û' | 'ü' | 'ũ' | 'ū' | 'ŭ' | 'ů'
        | 'ű' | 'ų' | 'У' | 'у' => "u",
        'Ý' | 'Ÿ' | 'ý' | 'ÿ' | 'Й' | 'й' | 'Ы' | 'ы' => "y",
        'Ł' | 'ł' | 'Л' | 'л' => "l",
        'Ž' | 'ž' | 'Ź' | 'ź' | 'Ż' | 'ż' | 'З' | 'з' => "z",
        '∫' | 'Š' | 'š' | 'Ś' | 'ś' | 'С' | 'с' => "s",
        'Þ' | 'þ' => "th",
        // Multi-letter specials (handled below because we want owned String)
        'Æ' | 'æ' => return Some(case_adjust("ae", is_upper)),
        'Œ' | 'œ' => return Some(case_adjust("oe", is_upper)),
        'ß' => return Some("ss".into()),

        // Cyrillic (rough, practical)
        'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' | 'Ā' | 'Ă' | 'Ą' | 'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą'
        | 'А' | 'а' => "a",
        'β' | 'Б' | 'б' => "b",
        'В' | 'в' => "v",
        'Г' | 'г' | 'Ґ' | 'ґ' => "g",
        'Ж' | 'ж' => "zh",
        'К' | 'к' => "k",
        'М' | 'м' => "m",
        'П' | 'п' => "p",
        'Р' | 'р' => "r",
        'Т' | 'т' => "t",
        'Ф' | 'ф' => "f",
        'Х' | 'х' => "h",
        'Ц' | 'ц' => "ts",
        'Ч' | 'ч' => "ch",
        'Ш' | 'ш' => "sh",
        'Щ' | 'щ' => "shch",
        'Ю' | 'ю' => "yu",
        'Я' | 'я' => "ya",
        'Є' | 'є' => "ye",
        'Ї' | 'ї' => "yi",

        // Soft/hard sign => drop (treat as boundary by returning empty)
        'Ъ' | 'ъ' | 'Ь' | 'ь' => return Some(String::new()),

        _ => return None,
    };

    Some(case_adjust(base, is_upper))
}

fn case_adjust(s: &str, title_case: bool) -> String {
    if !title_case || s.is_empty() {
        return s.into();
    }
    // Title-case: upper first ASCII letter, rest as-is.
    let mut out = String::with_capacity(s.len());
    let mut it = s.chars();
    if let Some(first) = it.next() {
        out.push(first.to_ascii_uppercase());
    }
    for c in it {
        out.push(c);
    }
    out
}
