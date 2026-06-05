use krilla;
use rustybuzz::{self, ttf_parser};

// Compile raw data for default font directly in the crate
const DEFAULT_FONT: &'static [u8] = include_bytes!("../assets/OpenSans-VariableFont_wdth,wght.ttf");

#[derive(Debug, Clone)]
pub enum FontWeight {
    Normal,
    Bold,
}
impl FontWeight {
    fn krilla_variation(&self) -> (krilla::text::Tag, f32) {
        return (krilla::text::Tag::new(b"wght"), match self {
            Self::Normal => 400.,
            Self::Bold => 700.,
        });
    }

    fn rustybuzz_varitation(&self) -> rustybuzz::Variation {
        return rustybuzz::Variation {
            tag: ttf_parser::Tag::from_bytes(b"wght"),
            value: match self {
                Self::Normal => 400.,
                Self::Bold => 700.,
            }
        }
    }
}

// Get the default font as a krilla font object
pub fn default_font(weight: &FontWeight) -> krilla::text::Font {
    krilla::text::Font::new_variable(DEFAULT_FONT.to_vec().into(), 0, &[weight.krilla_variation()]).unwrap()
}

// Get the width of the input text in the default font using rustybuzz parser
pub fn text_width(text: &str, font_size: f32, weight: &FontWeight) -> f32 {
    // Load the font face
    let mut face = rustybuzz::Face::from_slice(DEFAULT_FONT, 0).unwrap();
    // Set the variation for normal font weight
    face.set_variations(&[weight.rustybuzz_varitation()]);
    
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
pub fn text_height(font_size: f32, weight: &FontWeight) -> f32 {
    let mut face = rustybuzz::Face::from_slice(DEFAULT_FONT, 0).unwrap();
    // Set the variation for normal font weight
    face.set_variations(&[weight.rustybuzz_varitation()]);

    let units_per_em = face.units_per_em() as f32;
    let scale = font_size / units_per_em;

    return face.capital_height().map(|h| h as f32 * scale).unwrap();
}


// Get the em size for default font at given size using rustybuzz parser
pub fn em_size(font_size: f32, weight: &FontWeight) -> f32 {
    let mut face = rustybuzz::Face::from_slice(DEFAULT_FONT, 0).unwrap();
    // Set the variation for normal font weight
    face.set_variations(&[weight.rustybuzz_varitation()]);

    let units_per_em = face.units_per_em() as f32;
    let scale = font_size / units_per_em;

    return scale;
}


// Take an input string and split it into lines given a max allowable line width
pub fn string_to_lines<T: AsRef<str>>(text: T, font_size: f32, weight: &FontWeight, max_width: f32) -> Option<Vec<String>> {
    // Make sure the string is not empty
    if text.as_ref().is_empty() {return None}

    // Start the output vector with empty string
    let mut output: Vec<String> = Vec::new();

    // Width of a space character - will use a lot later
    let space_width: f32 = text_width(" ", font_size, weight);
    
    // Loop through the input text
    // First split at the line breaks
    for line in text.as_ref().lines() {
        // Width of the line
        let line_width: f32 = text_width(line, font_size, weight);

        // Check if the width of the line will fit in the max width
        if line_width <= max_width {
            // Add the line to the output and move on
            output.push(String::from(line));
            continue;
        }


        // Does not fit and need to chop line up
        else {

            // Iterator over the words in the line
            let mut word_iter = line.trim_end().split(' ');
            // Start the next line in output with the first word from the iterator
            if let Some(word) = word_iter.next() { output.push(String::from(word)); } else { continue; }
            // Starting width of the line
            let mut line_width: f32 = text_width(output.last().unwrap(), font_size, weight);

            // Iterate through the rest of the words
            for word in word_iter {
                // Length of the individual word
                let word_width = text_width(word, font_size, weight);

                // Check if we can add a space then this word without exceeding max allowable length
                if line_width + word_width + space_width <= max_width {
                    // Add a space then the next word
                    output.last_mut().unwrap().push(' ');
                    output.last_mut().unwrap().push_str(word);
                    // Update the line length
                    line_width += word_width + space_width;
                }

                // Word won't fit
                else {
                    // Add a new line and start with this word
                    // Start a new line
                    output.push(String::from(word));
                    line_width = word_width;
                }
            }
        }
    }
    // Return the output vector
    return Some(output)
}