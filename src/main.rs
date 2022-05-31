use clap::{arg, command};

mod owoifier {
    use std::borrow::Cow;
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref MAPPING: Vec<(Regex, &'static str)> = {
            let patterns = [
                [r"(?:r|l)", "w"],
                [r"(?:R|L)", "W"],
                [r"n([aeiou])", "ny$1"],
                [r"n([AEIOU])", "ny$1"],
                [r"N([aeiou])", "Ny$1"],
                [r"N([AEIOU])", "NY$1"],
            ];

            patterns
                .into_iter()
                .map(|[pat, rep]| (Regex::new(pat).unwrap(), rep))
                .collect()
        };
    }

    pub fn owoify(text: &str) -> Cow<str> {
        let mut temp = Cow::from(text);
        for (re, rep) in MAPPING.to_owned().into_iter() {
            temp = Cow::Owned(re.replace_all(&temp, rep).to_string());
        }
        temp
    }
}

mod cli {
    use super::owoifier;
    use std::io::{self, BufRead};
    use clap::Values;

    fn output(text: &str) {
        println!("{}", owoifier::owoify(&text));
    }

    pub fn handle_values(values: Values) {
        output(&values.collect::<Vec<_>>().join(" "));
    }

    pub fn handle_stdin() {
        for ln in io::stdin().lock().lines() {
            output(&ln.unwrap());
        }
    }
}

fn main() {
    let matches = command!()
        .arg(arg!([TEXT] ... "text that needs to be translated"))
        .get_matches();

    match matches.values_of("TEXT") {
        Some(values) => cli::handle_values(values),
        None => cli::handle_stdin(),
    }
}
