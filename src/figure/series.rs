use crate::Config;
use crate::paint::*;
use crate::ToF32;

pub(crate) struct Series {
    pub(crate) x: Vec<f32>,
    pub(crate) y: Vec<f32>,
    pub(crate) stroke: Stroke,
    pub(crate) label: Option<String>
}