extern crate regex;
extern crate clap;

use std::io;
use std::io::prelude::*;
use regex::{Captures,Regex};
use clap::{Arg, ArgMatches, App};


fn main() {

    let args = App::new("color")
        .version("0.1.0")
        .author("christheblog")
        .about("Highlight a pattern on the screen")
        .arg(Arg::with_name("pattern")
                 .help("Sets the input file to use")
                 .required(true)
                 .index(1))
        .arg(Arg::with_name("regex")
                 .short("r")
                 .long("regex")
                 .takes_value(false)
                 .help("Interpret pattern as a regular expression"))
        .arg(Arg::with_name("red").long("red").takes_value(false).help("Color pattern in red"))
        .arg(Arg::with_name("green").long("green").takes_value(false).help("Color pattern in green"))
        .arg(Arg::with_name("blue").long("blue").takes_value(false).help("Color pattern in blue"))
        .arg(Arg::with_name("yellow").long("yellow").takes_value(false).help("Color pattern in yellow"))
        .arg(Arg::with_name("magenta").long("magenta").takes_value(false).help("Color pattern in magenta"))
        .arg(Arg::with_name("cyan").long("cyan").takes_value(false).help("Color pattern in cyan"))
        .arg(Arg::with_name("black").long("black").takes_value(false).help("Color pattern in black"))
        .arg(Arg::with_name("white").long("white").takes_value(false).help("Color pattern in white"))
        .get_matches();

    // Reading arguments
    let target = args.value_of("pattern").unwrap();
    let col = get_color(&args, Color::Green);

    if args.is_present("regex") {
        let stdin = io::stdin();
        let compiled: Regex = Regex::new(&target).unwrap();
        highlight_regex_stream(compiled, &col, stdin.lock()).unwrap();
    } else {
        let stdin = io::stdin();
        highlight_stream(&target, &col, stdin.lock()).unwrap();
    }
}

// Highlight a string in a stream of lines
fn highlight_stream<R>(target: &str, color: &Color, reader: R) -> io::Result<()>
where R: BufRead {
    for line_result in reader.lines() {
        let line = line_result?;
        println!("{}", highlight(target, color, &line));
    }
    Ok(())
}

fn highlight(target: &str, color: &Color, line: &str) -> String {
    let replacement = &format!["{}{}{}", color.code(), target, Color::Sane.code()];
    format!("{}", line.replace(target, replacement))
}

// Highlight a rex exp in a stream of lines
fn highlight_regex_stream<R>(re: Regex, color: &Color, reader: R) -> io::Result<()>
where R: BufRead {
    for line_result in reader.lines() {
        let line = line_result?;
        println!("{}", highlight_regex(&re, color, &line));
    }
    Ok(())
}

fn highlight_regex(re: &Regex, color: &Color, line: &str) -> String {
    let replaced = re.replace_all(line,
        |cap: &Captures| {
            format!["{}{}{}", color.code(), cap.get(0).map(|s| s.as_str()).unwrap(), Color::Sane.code()]
        });
    format!("{}", replaced)
}


// Read color from the arguments. If not found, returns the default value
fn get_color(args: &ArgMatches, default_color: Color) -> Color {
    if args.is_present("red") { Color::Red }
    else if args.is_present("green") { Color::Green }
    else if args.is_present("yellow") { Color::Yellow }
    else if args.is_present("blue") { Color::Blue }
    else if args.is_present("cyan") { Color::Cyan }
    else if args.is_present("magenta") { Color::Magenta }
    else if args.is_present("white") { Color::White }
    else if args.is_present("black") { Color::Black }
    else { default_color }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    Sane,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White
}

impl Color {
    fn code(&self) -> &'static str {
        match self {
            Color::Sane => "\u{001B}[0m",
            Color::Black => "\u{001B}[30m",
            Color::Red => "\u{001B}[31m",
            Color::Green => "\u{001B}[32m",
            Color::Yellow => "\u{001B}[33m",
            Color::Blue => "\u{001B}[34m",
            Color::Magenta => "\u{001B}[35m",
            Color::Cyan => "\u{001B}[36m",
            Color::White => "\u{001B}[37m",
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_all_colors() {
        for color in vec![
            Color::Sane, Color::Black, Color::Red,
            Color::Green, Color::Yellow, Color::Blue,
            Color::Magenta, Color::Cyan, Color::White] {
            assert_eq!(highlight("Hello", &color, "Hello world"),
                       format!["{}Hello{} world", color.code(), Color::Sane.code()]);
        }
    }

    // String search

    #[test]
    fn test_highlight_begininig() {
        assert_eq!(highlight("Hello", &Color::Red, "Hello world"),
                   format!["{}Hello{} world", Color::Red.code(), Color::Sane.code()]);
    }

    #[test]
    fn test_highlight_middle() {
        assert_eq!(highlight("wor", &Color::Red, "Hello world"),
                   format!["Hello {}wor{}ld", Color::Red.code(), Color::Sane.code()]);
    }

    #[test]
    fn test_highlight_end() {
        assert_eq!(highlight("world", &Color::Red, "Hello world"),
                   format!["Hello {}world{}", Color::Red.code(), Color::Sane.code()]);
    }

    #[test]
    fn test_highlight_not_found() {
        assert_eq!(highlight("Boom", &Color::Red, "Hello world"), "Hello world");
    }


    // Regexp search

    #[test]
    fn test_highlight_regex_begininig() {
        let re: Regex = Regex::new("^[Hh][eE]llo").unwrap();
        assert_eq!(highlight_regex(&re, &Color::Red, "Hello world"),
                   format!["{}Hello{} world", Color::Red.code(), Color::Sane.code()]);
    }

    #[test]
    fn test_highlight_regex_middle() {
        let re: Regex = Regex::new("w[oO]r").unwrap();
        assert_eq!(highlight_regex(&re, &Color::Red, "Hello world"),
                   format!["Hello {}wor{}ld", Color::Red.code(), Color::Sane.code()]);
    }

    #[test]
    fn test_highlight_regex_end() {
        let re: Regex = Regex::new("world$").unwrap();
        assert_eq!(highlight_regex(&re, &Color::Red, "Hello world"),
                   format!["Hello {}world{}", Color::Red.code(), Color::Sane.code()]);
    }

    #[test]
    fn test_highlight_regex_not_found() {
        let re: Regex = Regex::new("Boom").unwrap();
        assert_eq!(highlight_regex(&re, &Color::Red, "Hello world"), "Hello world");
    }
}
