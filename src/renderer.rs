use crate::font::{get_font, FontError};
use anyhow::{Context, Result};
use std::cmp;

#[derive(Debug, Clone)]
pub struct RenderOptions {
    pub font_name: String,
    pub max_width: u32,
    pub height: u32,
    pub spacing: u32,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            font_name: "standard".to_string(),
            max_width: 0, // No limit
            height: 5,
            spacing: 1,
        }
    }
}

/// Render text using Unicode block characters
pub fn render_text(text: &str, font_name: &str, max_width: u32, height: u32) -> Result<String> {
    let options = RenderOptions {
        font_name: font_name.to_string(),
        max_width,
        height,
        spacing: 1,
    };
    
    render_text_with_options_internal(text, &options)
}

/// Render text with optional drop-shadow
pub fn render_text_with_shadow(text: &str, _font_name: &str, max_width: u32, height: u32, enable_shadow: bool) -> Result<String> {
    let font_name = if enable_shadow {
        "standard_shadow"
    } else {
        "standard_solid"
    };
    
    let options = RenderOptions {
        font_name: font_name.to_string(),
        max_width,
        height: if enable_shadow { 7 } else { height },
        spacing: 1,
    };
    
    render_text_with_options_internal(text, &options)
}

/// Render text with full options (deprecated - use render_text or render_text_with_shadow instead)
pub fn render_text_with_options(text: &str, _font_name: &str, max_width: u32, height: u32, enable_shadow: bool, _lowercase: bool) -> Result<String> {
    let font_name = if enable_shadow {
        "standard_shadow"
    } else {
        "standard_solid"
    };
    
    let options = RenderOptions {
        font_name: font_name.to_string(),
        max_width,
        height: if enable_shadow { 7 } else { 5 },
        spacing: 1,
    };
    
    render_text_with_options_internal(text, &options)
}

/// Render text with full options (internal)
fn render_text_with_options_internal(text: &str, options: &RenderOptions) -> Result<String> {
    let font = get_font(&options.font_name)
        .context(format!("Failed to load font '{}'", options.font_name))?;
    
    // Handle empty text
    if text.is_empty() {
        return Ok(String::new());
    }
    
    // Split text into words for potential word wrapping
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        return Ok(String::new());
    }
    
    let mut result_lines: Vec<Vec<String>> = Vec::new();
    let mut current_line_chars: Vec<char> = Vec::new();
    let mut current_line_width = 0;
    
    for word in words {
        let word_chars: Vec<char> = word.chars().collect();
        let word_width = calculate_word_width(&word_chars, font)?;
        
        // Check if we need to wrap to a new line
        if options.max_width > 0 
            && !current_line_chars.is_empty() 
            && current_line_width + options.spacing + word_width > options.max_width {
            // Render current line and start a new one
            let line_output = render_character_line(&current_line_chars, font, options.height)?;
            result_lines.push(line_output);
            current_line_chars.clear();
            current_line_width = 0;
        }
        
        // Add space before word (except for first word in line)
        if !current_line_chars.is_empty() {
            current_line_chars.push(' ');
            current_line_width += font.get_character(' ')?.width;
        }
        
        // Add word characters
        for ch in word_chars {
            current_line_chars.push(ch);
            current_line_width += font.get_character(ch).unwrap_or_else(|_| font.get_character('?').unwrap()).width;
        }
    }
    
    // Render the last line
    if !current_line_chars.is_empty() {
        let line_output = render_character_line(&current_line_chars, font, options.height)?;
        result_lines.push(line_output);
    }
    
    // Combine all lines
    let mut final_output = String::new();
    for (i, line_rows) in result_lines.iter().enumerate() {
        if i > 0 {
            final_output.push('\n'); // Add blank line between text lines
        }
        for row in line_rows {
            final_output.push_str(row);
            final_output.push('\n');
        }
    }
    
    // Remove trailing newline
    if final_output.ends_with('\n') {
        final_output.pop();
    }
    
    Ok(final_output)
}

/// Calculate the width of a word in characters
fn calculate_word_width(chars: &[char], font: &crate::font::Font) -> Result<u32, FontError> {
    let mut width = 0;
    for &ch in chars {
        let char_info = font.get_character(ch)?;
        width += char_info.width; // No spacing between characters
    }
    Ok(width)
}

/// Render a line of characters
fn render_character_line(chars: &[char], font: &crate::font::Font, target_height: u32) -> Result<Vec<String>, FontError> {
    if chars.is_empty() {
        return Ok(vec![String::new(); target_height as usize]);
    }
    
    // Get font characters for all input characters
    let font_chars: Result<Vec<_>, _> = chars
        .iter()
        .map(|&ch| font.get_character(ch))
        .collect();
    let font_chars = font_chars?;
    
    // Use the font's native height or the requested height
    let render_height = cmp::min(target_height, font.height) as usize;
    
    // Create output lines
    let mut output_lines = vec![String::new(); render_height];
    
    // Process each character - no spacing between characters for shadow font
    for font_char in font_chars.iter() {
        // Add character data to each line (no spacing between characters)
        for (line_idx, output_line) in output_lines.iter_mut().enumerate() {
            if line_idx < font_char.data.len() {
                output_line.push_str(&font_char.data[line_idx]);
            } else {
                // Pad with spaces if character is shorter than render height
                output_line.push_str(&" ".repeat(font_char.width as usize));
            }
        }
    }
    
    Ok(output_lines)
}


/// Render a single character (useful for testing)
pub fn render_character(ch: char, font_name: &str) -> Result<String> {
    let font = get_font(font_name)
        .context(format!("Failed to load font '{}'", font_name))?;
    
    let font_char = font.get_character(ch)?;
    
    Ok(font_char.data.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_render_single_character() {
        let result = render_character('A', "standard");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("█"));
        println!("Character 'A':\n{}", output);
    }
    
    #[test]
    fn test_render_simple_word() {
        let result = render_text("HI", "standard", 0, 5);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("█"));
        println!("Word 'HI':\n{}", output);
    }
    
    #[test]
    fn test_render_with_numbers() {
        let result = render_text("TEST123", "standard", 0, 5);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("█"));
        println!("Text 'TEST123':\n{}", output);
    }
    
    #[test]
    fn test_word_wrapping() {
        let result = render_text("HELLO WORLD", "standard", 20, 5);
        assert!(result.is_ok());
        let output = result.unwrap();
        println!("Text with wrapping:\n{}", output);
        
        // Should contain multiple sections separated by blank lines if wrapped
        let lines: Vec<&str> = output.lines().collect();
        assert!(lines.len() > 5); // More than just one word's height
    }
    
    #[test]
    fn test_empty_text() {
        let result = render_text("", "standard", 0, 5);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }
    
    #[test]
    fn test_invalid_font() {
        let result = render_text("TEST", "nonexistent", 0, 5);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_no_small_font() {
        let result = render_text("AB", "small", 0, 3);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_render_with_shadow() {
        let result = render_text_with_shadow("HI", "standard", 0, 7, true);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("█")); // Should contain main blocks
        assert!(output.contains("═")); // Should contain shadow blocks (box drawing)
        println!("Text with shadow 'HI':\n{}", output);
        
        // Should have 7 lines for shadow
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 7);
    }
    
    #[test]
    fn test_render_without_shadow() {
        let result = render_text_with_shadow("HI", "standard", 0, 5, false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("█")); // Should contain main blocks
        assert!(!output.contains("═")); // Should not contain shadow blocks
        println!("Text without shadow 'HI':\n{}", output);
        
        // Should have 5 lines without shadow
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 5);
    }
    
    #[test]
    fn test_shadow_offset() {
        let result = render_text_with_shadow("A", "standard", 0, 7, true);
        assert!(result.is_ok());
        let output = result.unwrap();
        let lines: Vec<&str> = output.lines().collect();
        
        // The shadow line (6th line) should have box drawing characters
        assert!(lines.len() == 7);
        assert!(lines[5].contains('═') || lines[5].contains('╝'));
    }
}
