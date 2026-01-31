use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum FontFace {
    Bold = 1,
    Dark = 2,
    Italic = 3,
    Underline = 4,
    Blink = 5,
    Reverse = 7,
    Concealed = 8,
    Crossed = 9,
    
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Grey = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,

    BGrey = 90,
    BRed = 91,
    BGreen = 92,
    BYellow = 93,
    BBlue = 94,
    BMagenta = 95,
    BCyan = 96,
    BWhite = 97,


    OnGrey = 40,
    OnRed = 41,
    OnGreen = 42,
    OnYellow = 43,
    OnBlue = 44,
    OnMagenta = 45,
    OnCyan = 46,
    OnWhite = 47,


    OnBGrey = 100,
    OnBRed = 101,
    OnBGreen = 102,
    OnBYellow = 103,
    OnBBlue = 104,
    OnBMagenta = 105,
    OnBCyan = 106,
    OnBWhite = 107,
}

pub trait Print {
    fn print_line(self, face: FontFace, color: Color);
    fn b(self, color: Color);
    fn i(self, color: Color);
    fn u(self, color: Color);
    fn bl(self, color: Color);
    fn r(self, color: Color);
    fn con(self, color: Color);
    fn cr(self, color: Color);
}

impl Print for fmt::Arguments<'_> {
    fn print_line(self, face: FontFace, color: Color) {
        println!("\x1b[{};{}m{}\x1b[0m",face as u8, color as u8, self);
    }

    fn b(self, color: Color) {
        self.print_line(FontFace::Bold, color);
    }

    fn i(self, color: Color) {
        self.print_line(FontFace::Italic, color);
    }

    fn u(self, color: Color) {
        self.print_line(FontFace::Underline, color);
    }

    fn bl(self, color: Color) {
        self.print_line(FontFace::Blink, color);
    }

    fn r(self, color: Color) {
        self.print_line(FontFace::Reverse, color);
    }

    fn con(self, color: Color) {
        self.print_line(FontFace::Concealed, color);
    }

    fn cr(self, color: Color) {
        self.print_line(FontFace::Crossed, color);
    }
}

fn color() {
    format_args!("Hello world").b(Color::Red);
    format_args!("Hello world").i(Color::Green);
    format_args!("Hello world").bl(Color::Cyan);
    format_args!("Hello world").u(Color::Blue);
    format_args!("Hello world").bl(Color::Magenta);
    format_args!("Hello world").r(Color::BCyan);
    format_args!("Hello world").con(Color::OnBlue);
    format_args!("Hello world").cr(Color::OnBYellow);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        color();
    }
}
