use std::borrow::Cow;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_TABLE: [(Regex, &'static str); 6] = {
        let patterns = [
            ["(?:r|l)", "w"],
            ["(?:R|L)", "W"],
            ["n([aeiou])", "ny$1"],
            ["n([AEIOU])", "ny$1"],
            ["N([aeiou])", "Ny$1"],
            ["N([AEIOU])", "NY$1"],
        ];
        patterns.map(|[expr, rep]| (Regex::new(expr).unwrap(), rep))
    };
}

/// Translates English text to OwO.
///
/// # Examples
/// ```
/// use owoifier::owoify;
///
/// let output = owoify("Hello world!");
/// assert_eq!(output, "Hewwo wowwd!");
/// ```
pub fn owoify(text: &str) -> Cow<str> {
    let mut output = Cow::from(text);
    for (re, rep) in REGEX_TABLE.iter().cloned() {
        output = Cow::Owned(re.replace_all(&output, rep).to_string());
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_owoify() {
        assert_eq!(owoify("r l"), "w w");
        assert_eq!(owoify("R L"), "W W");
        assert_eq!(owoify("na ne ni no nu"), "nya nye nyi nyo nyu");
        assert_eq!(owoify("nA nE nI nO nU"), "nyA nyE nyI nyO nyU");
        assert_eq!(owoify("Na Ne Ni No Nu"), "Nya Nye Nyi Nyo Nyu");
        assert_eq!(owoify("NA NE NI NO NU"), "NYA NYE NYI NYO NYU");
    }
}
