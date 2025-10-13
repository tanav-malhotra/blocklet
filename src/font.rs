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

/// Create the shadow font with Unicode box drawing characters and built-in shadows  
fn create_standard_shadow_font() -> Font {
    let mut characters = HashMap::new();
    
    // Space character
    characters.insert(' ', FontCharacter {
        width: 3,
        height: 7,
        data: vec![
            "   ".to_string(),
            "   ".to_string(),
            "   ".to_string(),
            "   ".to_string(),
            "   ".to_string(),
            "   ".to_string(),
            "   ".to_string(),
        ],
    });
    
    // Letter A
    characters.insert('A', FontCharacter {
        width: 7,
        height: 7,
        data: vec![
            " █████╗ ".to_string(),
            "██╔══██╗".to_string(),
            "███████║".to_string(),
            "██╔══██║".to_string(),
            "██║  ██║".to_string(),
            "╚═╝  ╚═╝".to_string(),
            "        ".to_string(),
        ],
    });
    
    // Letter B  
    characters.insert('B', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            "██████╗ ".to_string(),
            "██╔══██╗".to_string(),
            "██████╔╝".to_string(),
            "██╔══██╗".to_string(),
            "██████╔╝".to_string(),
            "╚═════╝ ".to_string(),
            "        ".to_string(),
        ],
    });
    
    // Letter C
    characters.insert('C', FontCharacter {
        width: 9,
        height: 7,
        data: vec![
            " ██████╗ ".to_string(),
            "██╔════╝ ".to_string(),
            "██║      ".to_string(),
            "██║      ".to_string(),
            "╚██████╗ ".to_string(),
            " ╚═════╝ ".to_string(),
            "         ".to_string(),
        ],
    });
    
    // Letter D
    characters.insert('D', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            "██████╗ ".to_string(),
            "██╔══██╗".to_string(),
            "██║  ██║".to_string(),
            "██║  ██║".to_string(),
            "██████╔╝".to_string(),
            "╚═════╝ ".to_string(),
            "        ".to_string(),
        ],
    });
    
    // Letter E
    characters.insert('E', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            "███████╗".to_string(),
            "██╔════╝".to_string(),
            "█████╗  ".to_string(),
            "██╔══╝  ".to_string(),
            "███████╗".to_string(),
            "╚══════╝".to_string(),
            "        ".to_string(),
        ],
    });
    
    // Letter F
    characters.insert('F', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            "███████╗".to_string(),
            "██╔════╝".to_string(),
            "█████╗  ".to_string(),
            "██╔══╝  ".to_string(),
            "██║     ".to_string(),
            "╚═╝     ".to_string(),
            "        ".to_string(),
        ],
    });
    
    // Letter G
    characters.insert('G', FontCharacter {
        width: 9,
        height: 7,
        data: vec![
            " ██████╗ ".to_string(),
            "██╔════╝ ".to_string(),
            "██║  ███╗".to_string(),
            "██║   ██║".to_string(),
            "╚██████╔╝".to_string(),
            " ╚═════╝ ".to_string(),
            "         ".to_string(),
        ],
    });
    
    // Letter H
    characters.insert('H', FontCharacter {
        width: 9,
        height: 7,
        data: vec![
            "██╗   ██╗".to_string(),
            "██║   ██║".to_string(),
            "████████║".to_string(),
            "██╔═══██║".to_string(),
            "██║   ██║".to_string(),
            "╚═╝   ╚═╝".to_string(),
            "         ".to_string(),
        ],
    });
    
    // Letter I
    characters.insert('I', FontCharacter {
        width: 3,
        height: 7,
        data: vec![
            "██╗".to_string(),
            "██║".to_string(),
            "██║".to_string(),
            "██║".to_string(),
            "██║".to_string(),
            "╚═╝".to_string(),
            "   ".to_string(),
        ],
    });
    
    // Letter J
    characters.insert('J', FontCharacter {
        width: 9,
        height: 7,
        data: vec![
            "     ██╗ ".to_string(),
            "     ██║ ".to_string(),
            "     ██║ ".to_string(),
            "██   ██║ ".to_string(),
            "╚██████║ ".to_string(),
            " ╚═════╝ ".to_string(),
            "         ".to_string(),
        ],
    });
    
    // Letter K
    characters.insert('K', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            "██╗  ██╗".to_string(),
            "██║ ██╔╝".to_string(),
            "█████╔╝ ".to_string(),
            "██╔═██╗ ".to_string(),
            "██║  ██╗".to_string(),
            "╚═╝  ╚═╝".to_string(),
            "        ".to_string(),
        ],
    });
    
    // Letter L
    characters.insert('L', FontCharacter {
        width: 7,
        height: 7,
        data: vec![
            "██╗     ".to_string(),
            "██║     ".to_string(),
            "██║     ".to_string(),
            "██║     ".to_string(),
            "███████╗".to_string(),
            "╚══════╝".to_string(),
            "        ".to_string(),
        ],
    });
    
    // Letter M
    characters.insert('M', FontCharacter {
        width: 11,
        height: 7,
        data: vec![
            "███╗   ███╗".to_string(),
            "████╗ ████║".to_string(),
            "██╔████╔██║".to_string(),
            "██║╚██╔╝██║".to_string(),
            "██║ ╚═╝ ██║".to_string(),
            "╚═╝     ╚═╝".to_string(),
            "           ".to_string(),
        ],
    });
    
    // Letter N
    characters.insert('N', FontCharacter {
        width: 9,
        height: 7,
        data: vec![
            "███╗   ██╗".to_string(),
            "████╗  ██║".to_string(),
            "██╔██╗ ██║".to_string(),
            "██║╚██╗██║".to_string(),
            "██║ ╚████║".to_string(),
            "╚═╝  ╚═══╝".to_string(),
            "          ".to_string(),
        ],
    });
    
    // Letter O
    characters.insert('O', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            " ██████╗ ".to_string(),
            "██╔═══██╗".to_string(),
            "██║   ██║".to_string(),
            "██║   ██║".to_string(),
            "╚██████╔╝".to_string(),
            " ╚═════╝ ".to_string(),
            "         ".to_string(),
        ],
    });
    
    // Letter P
    characters.insert('P', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            "██████╗ ".to_string(),
            "██╔══██╗".to_string(),
            "██████╔╝".to_string(),
            "██╔═══╝ ".to_string(),
            "██║     ".to_string(),
            "╚═╝     ".to_string(),
            "        ".to_string(),
        ],
    });
    
    // Letter Q
    characters.insert('Q', FontCharacter {
        width: 9,
        height: 7,
        data: vec![
            " ██████╗ ".to_string(),
            "██╔═══██╗".to_string(),
            "██║   ██║".to_string(),
            "██║   ██║".to_string(),
            "╚██████╔╝".to_string(),
            " ╚═══██╔╝".to_string(),
            "     ╚═╝ ".to_string(),
        ],
    });
    
    // Letter R
    characters.insert('R', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            "██████╗ ".to_string(),
            "██╔══██╗".to_string(),
            "██████╔╝".to_string(),
            "██╔══██╗".to_string(),
            "██║  ██║".to_string(),
            "╚═╝  ╚═╝".to_string(),
            "        ".to_string(),
        ],
    });
    
    // Letter S
    characters.insert('S', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            "███████╗".to_string(),
            "██╔════╝".to_string(),
            "███████╗".to_string(),
            "╚════██║".to_string(),
            "███████║".to_string(),
            "╚══════╝".to_string(),
            "        ".to_string(),
        ],
    });
    
    // Letter T
    characters.insert('T', FontCharacter {
        width: 9,
        height: 7,
        data: vec![
            "████████╗".to_string(),
            "╚══██╔══╝".to_string(),
            "   ██║   ".to_string(),
            "   ██║   ".to_string(),
            "   ██║   ".to_string(),
            "   ╚═╝   ".to_string(),
            "         ".to_string(),
        ],
    });
    
    // Letter U
    characters.insert('U', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            "██╗   ██╗".to_string(),
            "██║   ██║".to_string(),
            "██║   ██║".to_string(),
            "██║   ██║".to_string(),
            "╚██████╔╝".to_string(),
            " ╚═════╝ ".to_string(),
            "         ".to_string(),
        ],
    });
    
    // Letter V
    characters.insert('V', FontCharacter {
        width: 9,
        height: 7,
        data: vec![
            "██╗   ██╗".to_string(),
            "██║   ██║".to_string(),
            "╚██╗ ██╔╝".to_string(),
            " ╚██╗██╔╝".to_string(),
            "  ╚███╔╝ ".to_string(),
            "   ╚═╝   ".to_string(),
            "         ".to_string(),
        ],
    });
    
    // Letter W
    characters.insert('W', FontCharacter {
        width: 11,
        height: 7,
        data: vec![
            "██╗    ██╗ ".to_string(),
            "██║    ██║ ".to_string(),
            "██║ █╗ ██║ ".to_string(),
            "██║███╗██║ ".to_string(),
            "╚███╔███╔╝ ".to_string(),
            " ╚══╝╚══╝  ".to_string(),
            "            ".to_string(),
        ],
    });
    
    // Letter X
    characters.insert('X', FontCharacter {
        width: 9,
        height: 7,
        data: vec![
            "██╗  ██╗ ".to_string(),
            "╚██╗██╔╝ ".to_string(),
            " ╚███╔╝  ".to_string(),
            " ██╔██╗  ".to_string(),
            "██╔╝ ██╗ ".to_string(),
            "╚═╝  ╚═╝ ".to_string(),
            "         ".to_string(),
        ],
    });
    
    // Letter Y
    characters.insert('Y', FontCharacter {
        width: 9,
        height: 7,
        data: vec![
            "██╗   ██╗".to_string(),
            "╚██╗ ██╔╝".to_string(),
            " ╚████╔╝ ".to_string(),
            "  ╚██╔╝  ".to_string(),
            "   ██║   ".to_string(),
            "   ╚═╝   ".to_string(),
            "         ".to_string(),
        ],
    });
    
    // Letter Z
    characters.insert('Z', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            "███████╗".to_string(),
            "╚════██║".to_string(),
            "   ██╔╝ ".to_string(),
            "  ██╔╝  ".to_string(),
            "███████╗".to_string(),
            "╚══════╝".to_string(),
            "        ".to_string(),
        ],
    });
    
    // Numbers 0-9
    characters.insert('0', FontCharacter {
        width: 9,
        height: 7,
        data: vec![
            " ██████╗ ".to_string(),
            "██╔═══██╗".to_string(),
            "██║   ██║".to_string(),
            "██║   ██║".to_string(),
            "╚██████╔╝".to_string(),
            " ╚═════╝ ".to_string(),
            "         ".to_string(),
        ],
    });
    
    characters.insert('1', FontCharacter {
        width: 5,
        height: 7,
        data: vec![
            " ██╗ ".to_string(),
            "███║ ".to_string(),
            "╚██║ ".to_string(),
            " ██║ ".to_string(),
            " ██║ ".to_string(),
            " ╚═╝ ".to_string(),
            "     ".to_string(),
        ],
    });
    
    characters.insert('2', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            "██████╗ ".to_string(),
            "╚════██╗".to_string(),
            " █████╔╝".to_string(),
            "██╔═══╝ ".to_string(),
            "███████╗".to_string(),
            "╚══════╝".to_string(),
            "        ".to_string(),
        ],
    });
    
    characters.insert('3', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            "██████╗ ".to_string(),
            "╚════██╗".to_string(),
            " █████╔╝".to_string(),
            " ╚═══██╗".to_string(),
            "██████╔╝".to_string(),
            "╚═════╝ ".to_string(),
            "        ".to_string(),
        ],
    });
    
    characters.insert('4', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            "██╗  ██╗".to_string(),
            "██║  ██║".to_string(),
            "███████║".to_string(),
            "╚════██║".to_string(),
            "     ██║".to_string(),
            "     ╚═╝".to_string(),
            "        ".to_string(),
        ],
    });
    
    characters.insert('5', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            "███████╗".to_string(),
            "██╔════╝".to_string(),
            "███████╗".to_string(),
            "╚════██║".to_string(),
            "██████╔╝".to_string(),
            "╚═════╝ ".to_string(),
            "        ".to_string(),
        ],
    });
    
    characters.insert('6', FontCharacter {
        width: 9,
        height: 7,
        data: vec![
            " ██████╗ ".to_string(),
            "██╔════╝ ".to_string(),
            "██████╗  ".to_string(),
            "██╔══██╗ ".to_string(),
            "╚██████╔╝".to_string(),
            " ╚═════╝ ".to_string(),
            "         ".to_string(),
        ],
    });
    
    characters.insert('7', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            "███████╗".to_string(),
            "╚════██║".to_string(),
            "    ██╔╝".to_string(),
            "   ██╔╝ ".to_string(),
            "  ██╔╝  ".to_string(),
            "  ╚═╝   ".to_string(),
            "        ".to_string(),
        ],
    });
    
    characters.insert('8', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            " █████╗ ".to_string(),
            "██╔══██╗".to_string(),
            "╚█████╔╝".to_string(),
            "██╔══██╗".to_string(),
            "╚█████╔╝".to_string(),
            " ╚════╝ ".to_string(),
            "        ".to_string(),
        ],
    });
    
    characters.insert('9', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            " █████╗ ".to_string(),
            "██╔══██╗".to_string(),
            "╚██████║".to_string(),
            " ╚═══██║".to_string(),
            " █████╔╝".to_string(),
            " ╚════╝ ".to_string(),
            "        ".to_string(),
        ],
    });
    
    // Basic punctuation with 7-line shadow format
    characters.insert('!', FontCharacter {
        width: 3,
        height: 7,
        data: vec![
            "██╗".to_string(),
            "██║".to_string(),
            "██║".to_string(),
            "██║".to_string(),
            "╚═╝".to_string(),
            "██╗".to_string(),
            "╚═╝".to_string(),
        ],
    });
    
    characters.insert('?', FontCharacter {
        width: 8,
        height: 7,
        data: vec![
            " █████╗ ".to_string(),
            "██╔══██╗".to_string(),
            "╚═══██╔╝".to_string(),
            "   ██╔╝ ".to_string(),
            "   ╚═╝  ".to_string(),
            "   ██╗  ".to_string(),
            "   ╚═╝  ".to_string(),
        ],
    });
    
    characters.insert('.', FontCharacter {
        width: 3,
        height: 7,
        data: vec![
            "   ".to_string(),
            "   ".to_string(),
            "   ".to_string(),
            "   ".to_string(),
            "██╗".to_string(),
            "╚═╝".to_string(),
            "   ".to_string(),
        ],
    });
    
    characters.insert(',', FontCharacter {
        width: 4,
        height: 7,
        data: vec![
            "    ".to_string(),
            "    ".to_string(),
            "    ".to_string(),
            "    ".to_string(),
            " ██╗".to_string(),
            "██╔╝".to_string(),
            "╚═╝ ".to_string(),
        ],
    });
    
    Font {
        name: "standard_shadow".to_string(),
        description: "Standard Unicode box drawing font with built-in shadows and descenders".to_string(),
        height: 7,
        characters,
    }
}

/// Create the solid font without box drawing lines (no shadow version)
fn create_standard_solid_font() -> Font {
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
    
    // Letter A - solid blocks
    characters.insert('A', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            " █████ ".to_string(),
            "██   ██".to_string(),
            "███████".to_string(),
            "██   ██".to_string(),
            "██   ██".to_string(),
        ],
    });
    
    // Letter B - solid blocks
    characters.insert('B', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            "██████ ".to_string(),
            "██   ██".to_string(),
            "██████ ".to_string(),
            "██   ██".to_string(),
            "██████ ".to_string(),
        ],
    });
    
    // Letter C - solid blocks
    characters.insert('C', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            " ██████".to_string(),
            "██     ".to_string(),
            "██     ".to_string(),
            "██     ".to_string(),
            " ██████".to_string(),
        ],
    });
    
    // Letter D - solid blocks
    characters.insert('D', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            "██████ ".to_string(),
            "██   ██".to_string(),
            "██   ██".to_string(),
            "██   ██".to_string(),
            "██████ ".to_string(),
        ],
    });
    
    // Letter E - solid blocks
    characters.insert('E', FontCharacter {
        width: 6,
        height: 5,
        data: vec![
            "██████".to_string(),
            "██    ".to_string(),
            "██████".to_string(),
            "██    ".to_string(),
            "██████".to_string(),
        ],
    });
    
    // Letter F - solid blocks
    characters.insert('F', FontCharacter {
        width: 6,
        height: 5,
        data: vec![
            "██████".to_string(),
            "██    ".to_string(),
            "██████".to_string(),
            "██    ".to_string(),
            "██    ".to_string(),
        ],
    });
    
    // Letter G - solid blocks
    characters.insert('G', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            " ██████".to_string(),
            "██     ".to_string(),
            "██  ███".to_string(),
            "██   ██".to_string(),
            " ██████".to_string(),
        ],
    });
    
    // Letter H - solid blocks
    characters.insert('H', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            "██   ██".to_string(),
            "██   ██".to_string(),
            "███████".to_string(),
            "██   ██".to_string(),
            "██   ██".to_string(),
        ],
    });
    
    // Letter I - solid blocks  
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
    
    // Letter J - solid blocks
    characters.insert('J', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "█████".to_string(),
            "   ██".to_string(),
            "   ██".to_string(),
            "█  ██".to_string(),
            " ███ ".to_string(),
        ],
    });
    
    // Letter K - solid blocks
    characters.insert('K', FontCharacter {
        width: 6,
        height: 5,
        data: vec![
            "██  ██".to_string(),
            "██ ██ ".to_string(),
            "████  ".to_string(),
            "██ ██ ".to_string(),
            "██  ██".to_string(),
        ],
    });
    
    // Letter L - solid blocks
    characters.insert('L', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            "██   ".to_string(),
            "██   ".to_string(),
            "██   ".to_string(),
            "██   ".to_string(),
            "█████".to_string(),
        ],
    });
    
    // Letter M - solid blocks
    characters.insert('M', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            "██   ██".to_string(),
            "███ ███".to_string(),
            "██ █ ██".to_string(),
            "██   ██".to_string(),
            "██   ██".to_string(),
        ],
    });
    
    // Letter N - solid blocks
    characters.insert('N', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            "██   ██".to_string(),
            "███  ██".to_string(),
            "██ █ ██".to_string(),
            "██  ███".to_string(),
            "██   ██".to_string(),
        ],
    });
    
    // Letter O - solid blocks
    characters.insert('O', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            " █████ ".to_string(),
            "██   ██".to_string(),
            "██   ██".to_string(),
            "██   ██".to_string(),
            " █████ ".to_string(),
        ],
    });
    
    // Letter P - solid blocks
    characters.insert('P', FontCharacter {
        width: 6,
        height: 5,
        data: vec![
            "██████".to_string(),
            "██  ██".to_string(),
            "██████".to_string(),
            "██    ".to_string(),
            "██    ".to_string(),
        ],
    });
    
    // Letter Q - solid blocks
    characters.insert('Q', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            " █████ ".to_string(),
            "██   ██".to_string(),
            "██ █ ██".to_string(),
            "██  ██ ".to_string(),
            " ██████".to_string(),
        ],
    });
    
    // Letter R - solid blocks
    characters.insert('R', FontCharacter {
        width: 6,
        height: 5,
        data: vec![
            "██████".to_string(),
            "██  ██".to_string(),
            "██████".to_string(),
            "██ ██ ".to_string(),
            "██  ██".to_string(),
        ],
    });
    
    // Letter S - solid blocks
    characters.insert('S', FontCharacter {
        width: 6,
        height: 5,
        data: vec![
            " █████".to_string(),
            "██    ".to_string(),
            " ████ ".to_string(),
            "    ██".to_string(),
            "█████ ".to_string(),
        ],
    });
    
    // Letter T - solid blocks
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
    
    // Letter U - solid blocks
    characters.insert('U', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            "██   ██".to_string(),
            "██   ██".to_string(),
            "██   ██".to_string(),
            "██   ██".to_string(),
            " █████ ".to_string(),
        ],
    });
    
    // Letter V - solid blocks
    characters.insert('V', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            "██   ██".to_string(),
            "██   ██".to_string(),
            " ██ ██ ".to_string(),
            "  ███  ".to_string(),
            "   █   ".to_string(),
        ],
    });
    
    // Letter W - solid blocks
    characters.insert('W', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            "██   ██".to_string(),
            "██   ██".to_string(),
            "██ █ ██".to_string(),
            "███ ███".to_string(),
            "██   ██".to_string(),
        ],
    });
    
    // Letter X - solid blocks
    characters.insert('X', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            "██   ██".to_string(),
            " ██ ██ ".to_string(),
            "  ███  ".to_string(),
            " ██ ██ ".to_string(),
            "██   ██".to_string(),
        ],
    });
    
    // Letter Y - solid blocks
    characters.insert('Y', FontCharacter {
        width: 7,
        height: 5,
        data: vec![
            "██   ██".to_string(),
            " ██ ██ ".to_string(),
            "  ███  ".to_string(),
            "   █   ".to_string(),
            "   █   ".to_string(),
        ],
    });
    
    // Letter Z - solid blocks
    characters.insert('Z', FontCharacter {
        width: 6,
        height: 5,
        data: vec![
            "██████".to_string(),
            "   ██ ".to_string(),
            "  ██  ".to_string(),
            " ██   ".to_string(),
            "██████".to_string(),
        ],
    });
    
    // Numbers 0-9 with solid blocks
    characters.insert('0', FontCharacter {
        width: 5,
        height: 5,
        data: vec![
            " ███ ".to_string(),
            "██ ██".to_string(),
            "█ █ █".to_string(),
            "██ ██".to_string(),
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
    
    // Basic punctuation
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
        name: "standard_solid".to_string(),
        description: "Standard solid block font without shadows".to_string(),
        height: 5,
        characters,
    }
}

/// Create the standard font (defaults to shadow version)
fn create_standard_font() -> Font {
    create_standard_shadow_font()
}


lazy_static::lazy_static! {
    static ref FONTS: HashMap<String, Font> = {
        let mut fonts = HashMap::new();
        fonts.insert("standard".to_string(), create_standard_font());
        fonts.insert("standard_shadow".to_string(), create_standard_shadow_font());
        fonts.insert("standard_solid".to_string(), create_standard_solid_font());
        fonts
    };
}

pub fn get_font(name: &str) -> Result<&Font, FontError> {
    FONTS.get(name).ok_or_else(|| FontError::FontNotFound(name.to_string()))
}

