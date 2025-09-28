use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FontError {
    #[error("Font '{0}' not found")]
    FontNotFound(String),
    #[error("Character '{0}' not supported in font '{1}'")]
    CharacterNotSupported(char, String),
}

#[derive(Clone, Debug)]
pub struct FontCharacter {
    pub width: u32,
    pub height: u32,
    pub data: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Font {
    pub name: String,
    pub description: String,
    pub height: u32,
    pub characters: HashMap<char, FontCharacter>,
}

impl Font {
    pub fn get_character(&self, ch: char) -> Result<&FontCharacter, FontError> {
        // Convert to uppercase for consistency
        let ch = ch.to_ascii_uppercase();
        
        self.characters
            .get(&ch)
            .or_else(|| self.characters.get(&'?')) // Fallback to '?' for unknown characters
            .ok_or_else(|| FontError::CharacterNotSupported(ch, self.name.clone()))
    }
}

/// Unicode block characters used for rendering
pub const FULL_BLOCK: char = '█';
pub const UPPER_HALF_BLOCK: char = '▀';
pub const LOWER_HALF_BLOCK: char = '▄';
pub const LEFT_HALF_BLOCK: char = '▌';
pub const RIGHT_HALF_BLOCK: char = '▐';
pub const LIGHT_SHADE: char = '░';
pub const MEDIUM_SHADE: char = '▒';
pub const DARK_SHADE: char = '▓';

/// Create the standard font with Unicode block characters
fn create_standard_font() -> Font {
    let mut characters = HashMap::new();
    
    // Space character
    characters.insert(' ', FontCharacter {
        width: 3,
        height: 5,
        data: vec![
            "   ".to_string(),
            "   ".to_string(),
            "   ".to_string(),
            "   ".to_string(),
            "   ".to_string(),
        ],
    });
    
    // Letter A
    characters.insert('A', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            " ███ ".to_string(),
            "█   █".to_string(),
            "█████".to_string(),
            "█   █".to_string(),
            "█   █".to_string(),
        ],
    });
    
    // Letter B
    characters.insert('B', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "████ ".to_string(),
            "█   █".to_string(),
            "████ ".to_string(),
            "█   █".to_string(),
            "████ ".to_string(),
        ],
    });
    
    // Letter C
    characters.insert('C', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            " ███ ".to_string(),
            "█    ".to_string(),
            "█    ".to_string(),
            "█    ".to_string(),
            " ███ ".to_string(),
        ],
    });
    
    // Letter D
    characters.insert('D', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "████ ".to_string(),
            "█   █".to_string(),
            "█   █".to_string(),
            "█   █".to_string(),
            "████ ".to_string(),
        ],
    });
    
    // Letter E
    characters.insert('E', FontCharacter {
        width: 4,
        height: 5,
        data: vec![
            "████".to_string(),
            "█   ".to_string(),
            "███ ".to_string(),
            "█   ".to_string(),
            "████".to_string(),
        ],
    });
    
    // Letter F
    characters.insert('F', FontCharacter {
        width: 4,
        height: 5,
        data: vec![
            "████".to_string(),
            "█   ".to_string(),
            "███ ".to_string(),
            "█   ".to_string(),
            "█   ".to_string(),
        ],
    });
    
    // Letter G
    characters.insert('G', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            " ███ ".to_string(),
            "█    ".to_string(),
            "█ ██ ".to_string(),
            "█  █ ".to_string(),
            " ███ ".to_string(),
        ],
    });
    
    // Letter H
    characters.insert('H', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "█   █".to_string(),
            "█   █".to_string(),
            "█████".to_string(),
            "█   █".to_string(),
            "█   █".to_string(),
        ],
    });
    
    // Letter I
    characters.insert('I', FontCharacter {
        width: 3,
        height: 5,
        data: vec![
            "███".to_string(),
            " █ ".to_string(),
            " █ ".to_string(),
            " █ ".to_string(),
            "███".to_string(),
        ],
    });
    
    // Letter J
    characters.insert('J', FontCharacter {
        width: 4,
        height: 5,
        data: vec![
            "████".to_string(),
            "   █".to_string(),
            "   █".to_string(),
            "█  █".to_string(),
            " ██ ".to_string(),
        ],
    });
    
    // Letter K
    characters.insert('K', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "█  █ ".to_string(),
            "█ █  ".to_string(),
            "██   ".to_string(),
            "█ █  ".to_string(),
            "█  █ ".to_string(),
        ],
    });
    
    // Letter L
    characters.insert('L', FontCharacter {
        width: 4,
        height: 5,
        data: vec![
            "█   ".to_string(),
            "█   ".to_string(),
            "█   ".to_string(),
            "█   ".to_string(),
            "████".to_string(),
        ],
    });
    
    // Letter M
    characters.insert('M', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            "█     █".to_string(),
            "██   ██".to_string(),
            "█ █ █ █".to_string(),
            "█  █  █".to_string(),
            "█     █".to_string(),
        ],
    });
    
    // Letter N
    characters.insert('N', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "█   █".to_string(),
            "██  █".to_string(),
            "█ █ █".to_string(),
            "█  ██".to_string(),
            "█   █".to_string(),
        ],
    });
    
    // Letter O
    characters.insert('O', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            " ███ ".to_string(),
            "█   █".to_string(),
            "█   █".to_string(),
            "█   █".to_string(),
            " ███ ".to_string(),
        ],
    });
    
    // Letter P
    characters.insert('P', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "████ ".to_string(),
            "█   █".to_string(),
            "████ ".to_string(),
            "█    ".to_string(),
            "█    ".to_string(),
        ],
    });
    
    // Letter Q
    characters.insert('Q', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            " ███ ".to_string(),
            "█   █".to_string(),
            "█ █ █".to_string(),
            "█  █ ".to_string(),
            " ████".to_string(),
        ],
    });
    
    // Letter R
    characters.insert('R', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "████ ".to_string(),
            "█   █".to_string(),
            "████ ".to_string(),
            "█ █  ".to_string(),
            "█  █ ".to_string(),
        ],
    });
    
    // Letter S
    characters.insert('S', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            " ████".to_string(),
            "█    ".to_string(),
            " ███ ".to_string(),
            "    █".to_string(),
            "████ ".to_string(),
        ],
    });
    
    // Letter T
    characters.insert('T', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "█████".to_string(),
            "  █  ".to_string(),
            "  █  ".to_string(),
            "  █  ".to_string(),
            "  █  ".to_string(),
        ],
    });
    
    // Letter U
    characters.insert('U', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "█   █".to_string(),
            "█   █".to_string(),
            "█   █".to_string(),
            "█   █".to_string(),
            " ███ ".to_string(),
        ],
    });
    
    // Letter V
    characters.insert('V', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "█   █".to_string(),
            "█   █".to_string(),
            "█   █".to_string(),
            " █ █ ".to_string(),
            "  █  ".to_string(),
        ],
    });
    
    // Letter W
    characters.insert('W', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            "█     █".to_string(),
            "█     █".to_string(),
            "█  █  █".to_string(),
            "█ █ █ █".to_string(),
            " █   █ ".to_string(),
        ],
    });
    
    // Letter X
    characters.insert('X', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "█   █".to_string(),
            " █ █ ".to_string(),
            "  █  ".to_string(),
            " █ █ ".to_string(),
            "█   █".to_string(),
        ],
    });
    
    // Letter Y
    characters.insert('Y', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "█   █".to_string(),
            " █ █ ".to_string(),
            "  █  ".to_string(),
            "  █  ".to_string(),
            "  █  ".to_string(),
        ],
    });
    
    // Letter Z
    characters.insert('Z', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "█████".to_string(),
            "   █ ".to_string(),
            "  █  ".to_string(),
            " █   ".to_string(),
            "█████".to_string(),
        ],
    });
    
    // Numbers 0-9
    characters.insert('0', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            " ███ ".to_string(),
            "█  ██".to_string(),
            "█ █ █".to_string(),
            "██  █".to_string(),
            " ███ ".to_string(),
        ],
    });
    
    characters.insert('1', FontCharacter {
        width: 3,
        height: 5,
        data: vec![
            " █ ".to_string(),
            "██ ".to_string(),
            " █ ".to_string(),
            " █ ".to_string(),
            "███".to_string(),
        ],
    });
    
    characters.insert('2', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            " ███ ".to_string(),
            "█   █".to_string(),
            "   █ ".to_string(),
            "  █  ".to_string(),
            "█████".to_string(),
        ],
    });
    
    characters.insert('3', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            " ███ ".to_string(),
            "    █".to_string(),
            " ███ ".to_string(),
            "    █".to_string(),
            " ███ ".to_string(),
        ],
    });
    
    characters.insert('4', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "█   █".to_string(),
            "█   █".to_string(),
            "█████".to_string(),
            "    █".to_string(),
            "    █".to_string(),
        ],
    });
    
    characters.insert('5', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "█████".to_string(),
            "█    ".to_string(),
            "████ ".to_string(),
            "    █".to_string(),
            "████ ".to_string(),
        ],
    });
    
    characters.insert('6', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            " ███ ".to_string(),
            "█    ".to_string(),
            "████ ".to_string(),
            "█   █".to_string(),
            " ███ ".to_string(),
        ],
    });
    
    characters.insert('7', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "█████".to_string(),
            "    █".to_string(),
            "   █ ".to_string(),
            "  █  ".to_string(),
            " █   ".to_string(),
        ],
    });
    
    characters.insert('8', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            " ███ ".to_string(),
            "█   █".to_string(),
            " ███ ".to_string(),
            "█   █".to_string(),
            " ███ ".to_string(),
        ],
    });
    
    characters.insert('9', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            " ███ ".to_string(),
            "█   █".to_string(),
            " ████".to_string(),
            "    █".to_string(),
            " ███ ".to_string(),
        ],
    });
    
    // Some basic punctuation
    characters.insert('!', FontCharacter {
        width: 1,
        height: 5,
        data: vec![
            "█".to_string(),
            "█".to_string(),
            "█".to_string(),
            " ".to_string(),
            "█".to_string(),
        ],
    });
    
    characters.insert('?', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            " ███ ".to_string(),
            "    █".to_string(),
            "  ██ ".to_string(),
            "     ".to_string(),
            "  █  ".to_string(),
        ],
    });
    
    characters.insert('.', FontCharacter {
        width: 1,
        height: 5,
        data: vec![
            " ".to_string(),
            " ".to_string(),
            " ".to_string(),
            " ".to_string(),
            "█".to_string(),
        ],
    });
    
    characters.insert(',', FontCharacter {
        width: 2,
        height: 5,
        data: vec![
            "  ".to_string(),
            "  ".to_string(),
            "  ".to_string(),
            " █".to_string(),
            "█ ".to_string(),
        ],
    });
    
    Font {
        name: "standard".to_string(),
        description: "Standard Unicode block font".to_string(),
        height: 5,
        characters,
    }
}

/// Create a smaller font for more compact output
fn create_small_font() -> Font {
    let mut characters = HashMap::new();
    
    // Space character
    characters.insert(' ', FontCharacter {
        width: 2,
        height: 3,
        data: vec![
            "  ".to_string(),
            "  ".to_string(),
            "  ".to_string(),
        ],
    });
    
    // Letter A
    characters.insert('A', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            "███".to_string(),
            "█ █".to_string(),
        ],
    });
    
    // Complete the alphabet for small font
    characters.insert('B', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            "██ ".to_string(),
            "██ ".to_string(),
        ],
    });
    
    characters.insert('C', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            "█  ".to_string(),
            "██ ".to_string(),
        ],
    });
    
    characters.insert('D', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            "█ █".to_string(),
            "██ ".to_string(),
        ],
    });
    
    characters.insert('E', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "███".to_string(),
            "██ ".to_string(),
            "███".to_string(),
        ],
    });
    
    characters.insert('F', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "███".to_string(),
            "██ ".to_string(),
            "█  ".to_string(),
        ],
    });
    
    characters.insert('G', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            "█ █".to_string(),
            "██ ".to_string(),
        ],
    });
    
    characters.insert('H', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "█ █".to_string(),
            "███".to_string(),
            "█ █".to_string(),
        ],
    });
    
    characters.insert('I', FontCharacter {
        width: 1,
        height: 3,
        data: vec![
            "█".to_string(),
            "█".to_string(),
            "█".to_string(),
        ],
    });
    
    characters.insert('J', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            " ██".to_string(),
            "  █".to_string(),
            "██ ".to_string(),
        ],
    });
    
    characters.insert('K', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "█ █".to_string(),
            "██ ".to_string(),
            "█ █".to_string(),
        ],
    });
    
    characters.insert('L', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "█  ".to_string(),
            "█  ".to_string(),
            "███".to_string(),
        ],
    });
    
    characters.insert('M', FontCharacter {
        width: 5,
        height: 3,
        data: vec![
            "█   █".to_string(),
            "█ █ █".to_string(),
            "█   █".to_string(),
        ],
    });
    
    characters.insert('N', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "█ █".to_string(),
            "███".to_string(),
            "█ █".to_string(),
        ],
    });
    
    characters.insert('O', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            "█ █".to_string(),
            "██ ".to_string(),
        ],
    });
    
    characters.insert('P', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            "██ ".to_string(),
            "█  ".to_string(),
        ],
    });
    
    characters.insert('Q', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            "█ █".to_string(),
            "███".to_string(),
        ],
    });
    
    characters.insert('R', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            "██ ".to_string(),
            "█ █".to_string(),
        ],
    });
    
    characters.insert('S', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            " ██".to_string(),
            "██ ".to_string(),
        ],
    });
    
    characters.insert('T', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "███".to_string(),
            " █ ".to_string(),
            " █ ".to_string(),
        ],
    });
    
    characters.insert('U', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "█ █".to_string(),
            "█ █".to_string(),
            "██ ".to_string(),
        ],
    });
    
    characters.insert('V', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "█ █".to_string(),
            "█ █".to_string(),
            " █ ".to_string(),
        ],
    });
    
    characters.insert('W', FontCharacter {
        width: 5,
        height: 3,
        data: vec![
            "█   █".to_string(),
            "█ █ █".to_string(),
            " █ █ ".to_string(),
        ],
    });
    
    characters.insert('X', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "█ █".to_string(),
            " █ ".to_string(),
            "█ █".to_string(),
        ],
    });
    
    characters.insert('Y', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "█ █".to_string(),
            " █ ".to_string(),
            " █ ".to_string(),
        ],
    });
    
    characters.insert('Z', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "███".to_string(),
            " █ ".to_string(),
            "███".to_string(),
        ],
    });
    
    // Numbers for small font
    characters.insert('0', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            "█ █".to_string(),
            "██ ".to_string(),
        ],
    });
    
    characters.insert('1', FontCharacter {
        width: 2,
        height: 3,
        data: vec![
            "█ ".to_string(),
            "█ ".to_string(),
            "██".to_string(),
        ],
    });
    
    characters.insert('2', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            " █ ".to_string(),
            "███".to_string(),
        ],
    });
    
    characters.insert('3', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            " ██".to_string(),
            "██ ".to_string(),
        ],
    });
    
    characters.insert('4', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "█ █".to_string(),
            "███".to_string(),
            "  █".to_string(),
        ],
    });
    
    characters.insert('5', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "███".to_string(),
            "██ ".to_string(),
            "██ ".to_string(),
        ],
    });
    
    characters.insert('6', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            "██ ".to_string(),
            "██ ".to_string(),
        ],
    });
    
    characters.insert('7', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "███".to_string(),
            "  █".to_string(),
            " █ ".to_string(),
        ],
    });
    
    characters.insert('8', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            "██ ".to_string(),
            "██ ".to_string(),
        ],
    });
    
    characters.insert('9', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            "██ ".to_string(),
            "██ ".to_string(),
        ],
    });
    
    // Fallback character
    characters.insert('?', FontCharacter {
        width: 3,
        height: 3,
        data: vec![
            "██ ".to_string(),
            " █ ".to_string(),
            " █ ".to_string(),
        ],
    });
    
    Font {
        name: "small".to_string(),
        description: "Small Unicode block font".to_string(),
        height: 3,
        characters,
    }
}

lazy_static::lazy_static! {
    static ref FONTS: HashMap<String, Font> = {
        let mut fonts = HashMap::new();
        fonts.insert("standard".to_string(), create_standard_font());
        fonts.insert("small".to_string(), create_small_font());
        fonts
    };
}

pub fn get_font(name: &str) -> Result<&Font, FontError> {
    FONTS.get(name).ok_or_else(|| FontError::FontNotFound(name.to_string()))
}

pub fn list_fonts() {
    println!("Available fonts:");
    for (name, font) in FONTS.iter() {
        println!("  {:<10} - {} (height: {})", name, font.description, font.height);
    }
}

pub fn get_available_fonts() -> Vec<String> {
    FONTS.keys().cloned().collect()
}
