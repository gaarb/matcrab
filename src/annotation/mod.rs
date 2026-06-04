use crate::{Color, Dash, Stroke, text::{string_to_lines, text_height, text_width}};


pub trait Annotation {
    fn to_buffer(&self) -> Vec<AnnotationElement>;
}


pub enum AnnotationElement {
    Rectangle {
        // Position/size
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
        // Style
        stroke: Stroke,
        fill: Color
    },

    //
    Text {
        // Start position
        x: f32,
        y: f32,

        // Text
        text: String,

        // Text formatting
        font_size: f32,
        font_color: Color,
    },
}


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


pub struct Rectangle {
    // Box formatting
    pub box_stroke: Stroke,
    pub box_fill: Color,

    // Location/size (left, top, right, bottom)
    pub ltrb: (f32, f32, f32, f32),
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            // Box formatting
            box_stroke: Stroke::default(),
            box_fill: Color::WHITE,

            // Position (origin)
            ltrb: (0., 100., 0., 100.)
        }
    }
}

impl Annotation for Rectangle {
    fn to_buffer(&self) -> Vec<AnnotationElement> {
        let mut buffer: Vec<AnnotationElement> = Vec::new();

        let (left, top, right, bottom) = self.ltrb;

        // Box element is first and is a Rectangle element
        let rectangle = AnnotationElement::Rectangle {
            left,
            right,
            top,
            bottom,
            stroke: self.box_stroke.clone(),
            fill: self.box_fill.clone()
        };
        buffer.push(rectangle);

        return buffer;
    }
}

// A single-line piece of text
pub struct Text {
    // The text to be displayed
    pub text: String,

    // Font formatting
    pub font_color: Color,
    pub font_size: f32,

    // Location of bottom left corner of text (before padding)
    pub x: f32,
    pub y: f32,
}
impl Default for Text {
    fn default() -> Self {
        Self {
            // The text to be displayed (empty for default)
            text: String::new(),

            // Font formatting
            font_color: Color::BLACK,
            font_size: 9.,

            // Position (origin)
            x: 0.,
            y: 9.,
        }
    }
}

impl Annotation for Text {
    fn to_buffer(&self) -> Vec<AnnotationElement> {
        // Buffer for annotation elements
        let mut buffer: Vec<AnnotationElement> = Vec::new();
        //
        buffer.push(AnnotationElement::Text { 
            x: self.x, 
            y: self.y, 
            text: self.text.clone(), 
            font_size: self.font_size, 
            font_color: self.font_color.clone() 
        });
        // Return the buffer
        return buffer;
    }
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
            padding: 3.,
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

impl Annotation for TextBox {
    fn to_buffer(&self) -> Vec<AnnotationElement> {
        let mut buffer: Vec<AnnotationElement> = Vec::new();

        let (left, top, right, bottom) = self.ltrb;

        // Box element is first and is a Rectangle element
        let rectangle = AnnotationElement::Rectangle {
            left,
            right,
            top,
            bottom,
            stroke: self.box_stroke.clone(),
            fill: self.box_fill.clone()
        };
        buffer.push(rectangle);

        //
        let (width, height): (f32, f32) = (right - left, bottom - top);
        let line_height = self.line_spacing * self.font_size;
        let text = string_to_lines(&self.text, self.font_size, width - 2.*self.padding).unwrap();

        // Get starting y coordinate based on text alignment
        let mut y: f32 = match self.vertical_alignment {
            VerticalAlignment::Top => top + self.font_size + self.padding,
            VerticalAlignment::Middle => {
                let total_line_height: f32 = self.font_size + line_height * (text.len() - 1) as f32;
                self.font_size + top + 0.5 * (height - total_line_height)
            },
            VerticalAlignment::Bottom => {
                let total_line_height: f32 = self.font_size + line_height * (text.len() - 1) as f32;
                self.font_size + bottom - self.padding - total_line_height
            }
        };

        for line in text {
            // x coordinate to start drawing the text at
            let x: f32 = match self.horizontal_alignment {
                // Aligned on left edge of textbox
                HorizontalAlignment::Left => left + self.padding,
                //
                HorizontalAlignment::Center => left + 0.5*width - 0.5*text_width(&line, self.font_size),
                //
                HorizontalAlignment::Right => right - self.padding - text_width(&line, self.font_size),
            };
            // Add the text element to the buffer
            buffer.push(AnnotationElement::Text { x, y, text: line, font_size: self.font_size, font_color: self.font_color.clone() });
            // Increment y coordinate
            y += line_height;
        }
        // Return the buffer
        return buffer;
    }
}


