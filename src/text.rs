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
pub fn string_to_lines<T: AsRef<str>>(text: T, font_size_and_max_width: Option<(f32, f32)>) -> Vec<String> {
    // Get owned string of input text
    let text: &str = text.as_ref();
    // Check if a max_width specified
    match font_size_and_max_width {
        // If no max width given we just have to split by lines
        None => return text.lines().map(|line| line.into()).collect(),

        // If a max width given we will still split where there are \n's, but check
        // each individual line for width and split into multiple lines as necessary
        Some((font_size, max_width)) => {
            // Vector to hold the output
            let mut output: Vec<String> = Vec::new();
            // Iterate by lines (splits at any \n)
            for line in text.lines() {
                // If the line width ok then push it directly to the output vector and continue to the next line
                if text_width(line, font_size) <= max_width {
                    output.push(line.to_owned());
                    continue;
                }
                // If the line is too long we will have to figure out where to split it
                else {
                    let mut accumulator: String = String::new();
                    let mut accumulator_width: f32 = 0.;
                    let space_width = text_width(" ", font_size);
                    // Iterate over all the words, excluding the trailing whitespace
                    for word in line.trim_end().split(' ') {
                        // Check that the current word will not push us over the size limit
                        let word_width: f32 = text_width(word, font_size);
                        if word_width + accumulator_width <= max_width {
                            
                            // Add the word if there is room
                            accumulator.push_str(word);
                            accumulator_width += word_width;
                            // Check if we can also fit a space
                            if space_width + accumulator_width <= max_width {
                                accumulator.push(' ');
                                accumulator_width += space_width;
                            }
                            // Can't fit a space, push the accumulator to start a new line
                            // Starts with the current word
                            else {
                                output.push(accumulator.clone());
                                accumulator.clear();
                                accumulator.push_str(word);
                                accumulator.push(' ');
                                accumulator_width = word_width + space_width;
                            }
                            // Go to next word
                            continue;
                        }
                        // If we can't fit word on current line, push the accumulator to start a new line
                        else {
                            // Add current line to output and clear
                            output.push(accumulator.clone());
                            accumulator.clear();
                            // Add the current word to the line (will overflow if one word is larger than the max allowable width)
                            accumulator.push_str(word);
                            accumulator.push(' ');
                            accumulator_width = word_width + space_width;
                            // Go to next word
                            continue;
                        }
                    }
                    // Push anything left in the accumulator to the line
                    output.push(accumulator);
                }
                
            }
            // Return the lines
            return output;
        }
    }
}