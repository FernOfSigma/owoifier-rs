use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref TABLE: Vec<(Regex, String)> = {
        let mut v = vec![]; 
        let patterns = [
            ["n([aeiou])", "ny$1"],
            ["(?:r|l)", "w"],
            ["ove", "uv"],
            ["th", "d"],
        ];

        for [x, y] in patterns {
            // Matches uppercase.
            let re_upper = Regex::new(&x.to_uppercase()).unwrap();
            // Matches other cases that have not been matched yet.
            let re_other = RegexBuilder::new(x)
                .case_insensitive(true)
                .build()
                .unwrap();

            v.push((re_upper, y.to_uppercase()));
            v.push((re_other, y.to_string()));
        }

        v
    };
}

pub trait OwOifier {
    fn owoify(&self) -> String;
}

impl OwOifier for str {
    /// Translates English text to OwO.
    ///
    /// # Examples
    /// ```
    /// use owoifier::OwOifier;
    /// assert_eq!("Hello world!".owoify(), "Hewwo wowwd!");
    /// ```
    fn owoify(&self) -> String {
        let mut owo = self.to_string();
        for (re, dst) in TABLE.iter().cloned() {
            owo = re.replace_all(&owo, dst).to_string();
        }
        owo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_owoify() {
        assert_eq!("na ne ni no nu".owoify(), "nya nye nyi nyo nyu");
        assert_eq!("NA NE NI NO NU".owoify(), "NYA NYE NYI NYO NYU");
        assert_eq!("r l".owoify(), "w w");
        assert_eq!("R L".owoify(), "W W");
        assert_eq!("ove".owoify(), "uv");
        assert_eq!("OVE".owoify(), "UV");
        assert_eq!("th".owoify(), "d");
        assert_eq!("TH".owoify(), "D");
    }
}
