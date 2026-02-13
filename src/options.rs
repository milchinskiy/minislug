#[derive(Clone, Copy, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct SlugOptions {
    /// Separator used between "words".
    /// Default: `-`
    pub separator: char,
    /// Lowercase ASCII output.
    /// NOTE: Unicode lowercasing only matters when `unicode` feature + `allow_unicode`
    /// Default: true
    pub lowercase: bool,
    /// Max output length in bytes (UTF-8). Common FS limit is 255 bytes.
    /// Default: 255
    pub max_len_bytes: usize,
    /// If true, keep Unicode letters/digits (requires feature `unicode`).
    /// Default: false
    pub allow_unicode: bool,
    /// If true, keep '_' as-is. If false, '_' is treated as a separator boundary.
    /// Default: true
    pub keep_underscore: bool,
    /// If true, avoid names starting with '.' (hidden files on Unix).
    /// Default: true
    pub avoid_leading_dot: bool,
    /// Fallback name for empty / "." / ".." results.
    /// Default: "file"
    pub fallback: &'static str,
}

impl Default for SlugOptions {
    fn default() -> Self {
        Self {
            separator: '-',
            lowercase: true,
            max_len_bytes: 255,
            allow_unicode: false,
            keep_underscore: true, // practical default for filenames
            avoid_leading_dot: true,
            fallback: "file",
        }
    }
}
