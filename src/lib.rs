/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Basic crate to return the name for a codepoint as defined by the Unicode standard.
//!
//! Names are scrapped from the
//! [UnicodeData.txt](https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt) file.

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// Returns the Unicode name for a codepoint if it exists, `"UNKNOWN CHARACTER"` otherwise. Returns
/// the same data as [get_name_checked] except it will always return some string.
pub fn get_name(c: u32) -> &'static str {
    get_name_checked(c).unwrap_or("UNKNOWN CHARACTER")
}

/// Returns the Unicode name in an Option for a codepoint if it exists, None otherwise. Codepoints
/// inside ranges that don't have a specific name per codepont will return the name of the range.
pub fn get_name_checked(c: u32) -> Option<&'static str> {
    Some(match UNICODE.get(&c) {
        Some(s) => s,

        None => match c {
            0x3400..=0x4DBF => "CJK Ideograph Extension A",
            0x4E00..=0x9FFF => "CJK Ideograph",
            0xAC00..=0xD7A3 => "Hangul Syllable",
            0xD800..=0xDB7F => "Non Private Use High Surrogate",
            0xDB80..=0xDBFF => "Private Use High Surrogate",
            0xDC00..=0xDFFF => "Low Surrogate",
            0xE000..=0xF8FF => "Private Use",
            0x17000..=0x187F7 => "Tangut Ideograph",
            0x18D00..=0x18D08 => "Tangut Ideograph Supplement",
            0x20000..=0x2A6DF => "CJK Ideograph Extension B",
            0x2A700..=0x2B739 => "CJK Ideograph Extension C",
            0x2B740..=0x2B81D => "CJK Ideograph Extension D",
            0x2B820..=0x2CEA1 => "CJK Ideograph Extension E",
            0x2CEB0..=0x2EBE0 => "CJK Ideograph Extension F",
            0x2EBF0..=0x2EE5D => "CJK Ideograph Extension I",
            0x30000..=0x3134A => "CJK Ideograph Extension G",
            0x31350..=0x323AF => "CJK Ideograph Extension H",
            0xF0000..=0xFFFFD => "Plane 15 Private Use",
            0x100000..=0x10FFFD => "Plane 16 Private Use",

            _ => return None,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::get_name;

    #[test]
    fn normal() {
        assert_eq!(get_name(0), "NULL");
    }

    #[test]
    fn range() {
        assert_eq!(get_name(0x3401), "CJK Ideograph Extension A");
    }
}
