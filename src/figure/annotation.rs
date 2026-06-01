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



pub struct Rectangle {
    // 
    pub stroke: Stroke,
    pub fill: Color,
    //
    pub position: (f32, f32),
    pub size: (f32, f32),
}
impl Rectangle {
    // Change stroke variables
    pub fn set_stroke_color(&mut self, color: Color) { self.stroke.color = color; }
    pub fn set_stroke_dash (&mut self, dash:  Dash)  { self.stroke.dash  = dash; }
    pub fn set_stroke_width(&mut self, width: f32  ) { self.stroke.width = width; }
}
impl Default for Rectangle {
    fn default() -> Self {
        Self {
            stroke: Stroke { color: Color::BLACK, dash: Dash::Solid, width: 1. },
            fill: Color::TRANSPARENT,
            position: (0., 0.),
            size: (72., 72.)
        }
    }
}


pub struct TextBox {
    //
    text: String,
    //
    font_color: Color,
    font_size: f32,
    //
    box_stroke: Stroke,
    box_fill: Color,
    //
    position: (f32, f32),
    size: (f32, f32),
}
impl TextBox {
    //
    pub fn new<T: Into<String>>(text: T, x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            //
            text: text.into(),
            //
            font_color: Color::BLACK,
            font_size: 9.,
            //
            box_stroke: Stroke { color: Color::BLACK, dash: Dash::Solid, width: 1. },
            box_fill: Color::TRANSPARENT,
            //
            position: (x, y),
            size: (w, h)
        }
    }
    //
    pub fn set_font_color(&mut self, color: Color) { self.font_color = color; }
    pub fn set_font_size(&mut self, size: f32) { self.font_size = size; }
    //
    pub fn set_box_stroke_color(&mut self, color: Color) { self.box_stroke.color = color; }
    pub fn set_box_stroke_dash(&mut self, dash: Dash) { self.box_stroke.dash = dash; }
    pub fn set_box_stroke_width(&mut self, width: f32) { self.box_stroke.width = width; }
    pub fn set_box_fill(&mut self, color: Color) { self.box_fill = color; }
    //
    //pub(crate) fn get_lines(&mut self) -> Vec<String> { return string_to_lines(&self.text, self.font_size, self.size.0 - 2.*text_width("n", self.font_size)); }
}


pub struct Annotation {
    elements: Vec<AnnotationElement>
}



enum AnnotationElement {
    //
    Rectangle {
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
        stroke: Stroke,
        fill_color: Color,
    },
    //
    Text {
        text: Vec<String>,
        font_size: f32,
        color: Color,
        margin: Option<f32>,
    }
    //
}