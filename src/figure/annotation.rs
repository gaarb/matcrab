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
    //
    pub(crate) text: String,
    pub(crate) horizontal_alignment: HorizontalAlignment,
    pub(crate) vertical_alignment: VerticalAlignment,
    pub(crate) horizontal_padding: f32,
    //
    pub(crate) font_color: Color,
    pub(crate) font_size: f32,
    //
    pub(crate) box_stroke: Stroke,
    pub(crate) box_fill: Color,
    //
    pub(crate) ltrb: (f32, f32, f32, f32),
}
impl TextBox {
    //
    pub fn new<T: Into<String>>(text: T, l: f32, t: f32, r: f32, b: f32) -> Self {
        Self {
            //
            text: text.into(),
            horizontal_alignment: HorizontalAlignment::Left,
            vertical_alignment: VerticalAlignment::Bottom,
            horizontal_padding: 5.,
            //
            font_color: Color::BLACK,
            font_size: 9.,
            //
            box_stroke: Stroke { color: Color::BLACK, dash: Dash::Solid, width: 1. },
            box_fill: Color::TRANSPARENT,
            //
            ltrb: (l, t, r, b),
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