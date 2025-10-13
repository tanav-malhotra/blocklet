use clap::{Arg, Command};
use std::process;

pub mod font;
pub mod renderer;

fn main() {
    let matches = Command::new("blocklet")
        .version("0.1.2")
        .author("Tanav Malhotra <tanavm2009@gmail.com>")
        .about("A cross-platform CLI tool that generates ASCII art using Unicode block characters")
        .arg(
            Arg::new("text")
                .help("The text to convert to ASCII art (multiple arguments = multiple lines)")
                .required(false)
                .num_args(1..)
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
            Arg::new("font")
                .short('f')
                .long("font")
                .value_name("FONT")
                .help("Font to use (standard, standard_shadow, standard_solid)")
                .default_value("standard_shadow")
        )
        .arg(
            Arg::new("no-shadow")
                .short('n')
                .long("no-shadow")
                .help("Use solid font without shadow (same as --font standard_solid)")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let texts: Vec<String> = matches
        .get_many::<String>("text")
        .map(|vals| vals.map(|s| s.to_string()).collect())
        .unwrap_or_default();
    
    if texts.is_empty() {
        eprintln!("Error: Please provide text to render");
        process::exit(1);
    }
    
    let width = *matches.get_one::<u32>("width").unwrap();
    let no_shadow = matches.get_flag("no-shadow");
    
    // Determine font based on flags
    let font_name = if no_shadow {
        "standard_solid"
    } else {
        matches.get_one::<String>("font").unwrap().as_str()
    };
    
    // Determine height based on font
    let height = match font_name {
        "standard_shadow" | "standard" | "standard_solid" => 7,
        _ => 7, // default
    };

    // Render each text argument as a separate line
    let mut first = true;
    for text in texts {
        if !first {
            println!(); // Add blank line between outputs
        }
        first = false;
        
        match renderer::render_text(&text, font_name, width, height) {
            Ok(output) => print!("{}", output),
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }
    
    println!(); // Final newline
}
