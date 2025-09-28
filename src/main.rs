use clap::{Arg, Command};
use std::process;

pub mod font;
pub mod renderer;

fn main() {
    let matches = Command::new("blocklet")
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
                .default_value("6")
        )
        .arg(
            Arg::new("no-shadow")
                .short('n')
                .long("no-shadow")
                .help("Disable drop-shadow effect")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("lowercase")
                .short('l')
                .long("lowercase")
                .help("Enable lowercase letters")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let text = matches.get_one::<String>("text");
    if text.is_none() {
        eprintln!("Error: Please provide text to render");
        process::exit(1);
    }
    let text = text.unwrap();
    let width = *matches.get_one::<u32>("width").unwrap();
    let height = *matches.get_one::<u32>("height").unwrap();
    let no_shadow = matches.get_flag("no-shadow");
    let lowercase = matches.get_flag("lowercase");

    match renderer::render_text_with_options(text, "standard", width, height, !no_shadow, lowercase) {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
