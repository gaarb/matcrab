use krilla;
use rustybuzz::{self, ttf_parser};

// Compile raw data for default font directly in the crate
const DEFAULT_FONT: &'static [u8] = include_bytes!("../assets/OpenSans-VariableFont_wdth,wght.ttf");

// Get the default font as a krilla font object
pub fn default_font() -> krilla::text::Font {
    krilla::text::Font::new(DEFAULT_FONT.to_vec().into(), 0).unwrap()
}

// Get the width of the input text in the default font using rustybuzz parser
pub fn text_width(text: &str, font_size: f32) -> f32 {
    // Load the font face
    let face = rustybuzz::Face::from_slice(DEFAULT_FONT, 0).unwrap();
    
    // Create and populate the text buffer
    let mut buffer = rustybuzz::UnicodeBuffer::new();
    buffer.push_str(text);
    // Automatically determine direction and script (usually Left-to-Right / Latin)
    buffer.guess_segment_properties(); 

    // Shape the text
    let glyph_buffer = rustybuzz::shape(&face, &[], buffer);

    // Sum up the X-advances of all glyphs
    let mut total_advance_font_units = 0;
    for pos in glyph_buffer.glyph_positions() {
        total_advance_font_units += pos.x_advance;
    }

    // rustybuzz returns values in "font design units". 
    // We need to scale this to your actual font size (pixels/points).
    let units_per_em = face.units_per_em() as f32;
    let scale = font_size / units_per_em;

    return (total_advance_font_units as f32) * scale
}

// Get the height of the default font using rustybuzz parser
pub fn text_height(font_size: f32) -> f32 {
    let face = ttf_parser::Face::parse(DEFAULT_FONT, 0).unwrap();

    let units_per_em = face.units_per_em() as f32;
    let scale = font_size / units_per_em;

    return face.capital_height().map(|h| h as f32 * scale).unwrap();
}


// Get the em size for default font at given size using rustybuzz parser
pub fn em_size(font_size: f32) -> f32 {
    let face = ttf_parser::Face::parse(DEFAULT_FONT, 0).unwrap();

    let units_per_em = face.units_per_em() as f32;
    let scale = font_size / units_per_em;

    return scale;
}


// Take an input string and split it into lines given a max allowable line width
pub fn string_to_lines<T: AsRef<str>>(text: T, font_size: f32, max_width: f32) -> Vec<String> {
    // Make sure the string is not empty
    if text.as_ref().is_empty() {return Vec::new()}

    // Start the output vector with empty string
    let mut output: Vec<String> = Vec::new();

    // Width of a space character - will use a lot later
    let space_width: f32 = text_width(" ", font_size);
    
    // Loop through the input text
    // First split at the line breaks
    for line in text.as_ref().lines() {
        // Width of the line
        let line_width: f32 = text_width(line, font_size);
        // Check if the width of the line will fit in the max width
        if line_width <= max_width {
            // Add the line to the output and move on
            output.push(String::from(line));
            continue;
        }
        // Does not fit and need to chop line up
        else {
            // Start the next line in output
            output.push(String::new());
            // width of the current line
            let mut line_width: f32 = 0.;
            // Iterate through the words
            for word in line.trim_end().split(' ') {
                // Length of the individual word
                let word_width = text_width(word, font_size);
                // Check if we can add the word without exceeding allowable length
                if line_width + word_width <= max_width {
                    // Add the word and a space
                    let index = output.len() - 1;
                    output[index].push_str(word);
                    output[index].push(' ');
                    line_width += word_width + space_width;
                }
                // Word won't fit
                else {
                    // Check edge case of whether this is first word in the line
                    let index = output.len() - 1;
                    if output[index].is_empty() {
                        // Is first word and still doesn't fit, add it to the line anyway
                        output[index].push_str(word);
                        line_width += word_width;
                    }
                    // Line already has words in it
                    else {
                        // Start a new line
                        output.push(String::new());
                        line_width = 0.;
                        // Add the word
                        let index = output.len() - 1;
                        output[index].push_str(word);
                        output[index].push(' ');
                        line_width += word_width + space_width;
                    }
                }
            }
        }
    }
    // Return the output vector
    return output
}