use std::io::{stdin, BufRead};

use clap::{arg, command, Values};

use owoifier::owoify;

fn show(text: &str) {
    println!("{}", owoify(text));
}

fn show_values(values: Values) {
    show(&values.collect::<Vec<_>>().join(" "));
}

fn read_from_stdin() {
    for ln in stdin().lock().lines() {
        show(&ln.unwrap());
    }
}

fn main() {
    let matches = command!()
        .arg(arg!([TEXT] ... "Text that needs to be translated"))
        .get_matches();

    match matches.values_of("TEXT") {
        Some(values) => show_values(values),
        None => read_from_stdin(),
    }
}
