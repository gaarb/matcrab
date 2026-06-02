use crate::{Color, Dash, Stroke, text::{string_to_lines, text_width}};


pub enum HorizontalAlignment {
    Left,
    Center,
    Right
}

pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom,
}


pub struct TextBox {
    // The text to be displayed
    pub text: String,

    // Font formatting
    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,
    pub padding: f32,
    pub line_spacing: f32,
    pub font_color: Color,
    pub font_size: f32,

    // Box formatting
    pub box_stroke: Stroke,
    pub box_fill: Color,

    // Location/size (left, top, right, bottom)
    pub ltrb: (f32, f32, f32, f32),
}
impl Default for TextBox {
    fn default() -> Self {
        Self {
            // The text to be displayed (empty for default)
            text: String::new(),

            // Font formatting
            horizontal_alignment: HorizontalAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,
            padding: 5.,
            line_spacing: 1.2,
            font_color: Color::BLACK,
            font_size: 9.,

            // Box formatting
            box_stroke: Stroke::default(),
            box_fill: Color::WHITE,

            // Position (origin)
            ltrb: (0., 100., 0., 100.)
        }
    }
}