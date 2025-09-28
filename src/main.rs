use clap::{Arg, Command};
use std::process;

pub mod font;
pub mod renderer;

fn main() {
    let matches = Command::new("unicode-figlet")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("A cross-platform CLI tool that generates ASCII art using Unicode block characters")
        .arg(
            Arg::new("text")
                .help("The text to convert to ASCII art")
                .required(false)
                .index(1)
        )
        .arg(
            Arg::new("font")
                .short('f')
                .long("font")
                .value_name("FONT")
                .help("Font style to use (default: standard)")
                .default_value("standard")
        )
        .arg(
            Arg::new("width")
                .short('w')
                .long("width")
                .value_name("WIDTH")
                .help("Maximum width for output (0 = no limit)")
                .value_parser(clap::value_parser!(u32))
                .default_value("0")
        )
        .arg(
            Arg::new("height")
                .short('H')
                .long("height")
                .value_name("HEIGHT")
                .help("Font height in characters")
                .value_parser(clap::value_parser!(u32))
                .default_value("5")
        )
        .arg(
            Arg::new("list-fonts")
                .long("list-fonts")
                .help("List available fonts")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    if matches.get_flag("list-fonts") {
        font::list_fonts();
        return;
    }

    let text = matches.get_one::<String>("text");
    if text.is_none() {
        eprintln!("Error: Please provide text to render or use --list-fonts to see available fonts");
        process::exit(1);
    }
    let text = text.unwrap();
    let font_name = matches.get_one::<String>("font").unwrap();
    let width = *matches.get_one::<u32>("width").unwrap();
    let height = *matches.get_one::<u32>("height").unwrap();

    match renderer::render_text(text, font_name, width, height) {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
