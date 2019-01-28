use std::env;
use std::io;
use std::io::prelude::*;



fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() !=2 {
        println!("Usage: color <pattern> --<red|green|blue|yellow|cyan|magenta|black|white>");
    } else {
        let stdin = io::stdin();
        let target = &args[0];
        let col = Color::parse(&args[1][2..]).unwrap();
        color(&target, &col, stdin.lock()).unwrap();
    }
}


fn color<R>(target: &str, color: &Color, reader: R) -> io::Result<()>
where R: BufRead {
    let replacement = &format!["{}{}{}", color.code(), target, Color::Sane.code()];
    for line_result in reader.lines() {
        let line = line_result?;
        println!("{}", line.replace(target, replacement));
    }
    Ok(())
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

    fn parse(str : &str) -> Result<Color,String> {
        match str {
            "red" => Ok(Color::Red),
            "black" => Ok(Color::Black),
            "green" => Ok(Color::Green),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            "magenta" => Ok(Color::Magenta),
            "cyan" => Ok(Color::Cyan),
            "white" => Ok(Color::White),
            unparsed => Err(format!("Unrecogised color: {}",unparsed))
        }
    }

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
