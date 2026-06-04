use super::{Annotation, AnnotationElement};

#[derive(Debug, Clone)]
pub enum ImageData {
    PNG(Vec<u8>)
}

pub struct Image {
    data: ImageData,

    left: f32, 
    top: f32, 
    right: f32, 
    bottom: f32
}
impl Image {
    pub fn from_png(png_data: &[u8], left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            data: ImageData::PNG(png_data.into()),

            left, top, right, bottom
        }
    }
}

impl Annotation for Image {
    fn to_buffer(&self) -> Vec<AnnotationElement> {
        return vec![AnnotationElement::Image { 
            image: self.data.clone(), 
            left: self.left, 
            top: self.top, 
            right: self.right, 
            bottom: self.bottom 
        }]
    }
}