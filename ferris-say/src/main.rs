use ansi_term::{self, Colour}; // 0.12.1
use clap::{ArgEnum, Parser};

fn draw(quote: &str, color: &Colour) {
    // Ferris drawing
    const FERRIS: &'static str = r"
    .
     .
      .
       █ █           █ █
        ▀█  ▄█████▄  █▀
         ▀▄███▀█▀███▄▀ 
         ▄▀███▀▀▀███▀▄ 
         █ ▄▀▀▀▀▀▀▀▄ █
    ";
    println!("{}", format!("\"{}\"{}", quote, color.paint(FERRIS)));
}
fn input()-> (String, Colour) {
    #[derive(Parser, Debug)]
    #[clap(author, version, about = "cowsay rusty version")]
    struct Args {
        /// Quote that ferris will say
        #[clap(short, long)]
        quote: String,

        /// Colors to choose
        #[clap(arg_enum)]
        color: Colors,
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
    enum Colors {
        // Not black because it's the our background color
        Red,
        Green,
        Yellow,
        Blue,
        Purple,
        Cyan,
        White,
    }

    let args = Args::parse();
    // validate colors argument:
    let color_matched = match args.color {
        Colors::Red => Colour::Red,
        Colors::Green => Colour::Green,
        Colors::Yellow => Colour::Yellow,
        Colors::Blue => Colour::Blue,
        Colors::Purple => Colour::Purple,
        Colors::Cyan => Colour::Cyan,
        Colors::White => Colour::White,
    };
    (args.quote, color_matched)
}

fn main() {
    let (q, c) = input();
    draw(&q, &c)
}
