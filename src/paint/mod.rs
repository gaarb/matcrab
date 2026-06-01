

// Default color theme
pub const MATLAB_GEM: [Color; 7] = [
    Color {r:   0, g: 114, b: 189, a: 255},
    Color {r: 217, g:  83, b:  25, a: 255},
    Color {r: 237, g: 177, b:  32, a: 255},
    Color {r: 126, g:  47, b: 142, a: 255},
    Color {r: 119, g: 172, b:  48, a: 255},
    Color {r:  77, g: 190, b: 238, a: 255},
    Color {r: 162, g:  20, b:  47, a: 255},
];
// List of dash types
const DASH_VARIANTS: [Dash; 4] = [
    Dash::Solid,
    Dash::Dashed,
    Dash::DashDot,
    Dash::Dotted
];

// An rgba color
#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self { return Self {r, g, b, a}; }
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self { return Self {r, g, b, a: 255}; }

    // Constant colors
    pub const WHITE:        Self = Self {r: 255, g: 255, b: 255, a: 255};
    pub const BLACK:        Self = Self {r:   0, g:   0, b:   0, a: 255};
    pub const RED:          Self = Self {r: 255, g:   0, b:   0, a: 255};
    pub const GREEN:        Self = Self {r:   0, g: 255, b:   0, a: 255};
    pub const BLUE:         Self = Self {r:   0, g:   0, b: 255, a: 255};
    pub const TRANSPARENT:  Self = Self {r:   0, g:   0, b: 0, a: 0};

    // Default colors for the grid lines
    pub const DEFAULT_MAJOR: Self = Self {r: 140, g: 140, b: 140, a: 255};
    pub const DEFAULT_MINOR: Self = Self {r: 220, g: 220, b: 220, a: 255};
}
impl Into<tiny_skia::Color> for Color {
    fn into(self) -> tiny_skia::Color {
        return tiny_skia::Color::from_rgba8(self.r, self.g, self.b, self.a);
    }
}
impl Into<Option<krilla::paint::Fill>> for Color {
    fn into(self) -> Option<krilla::paint::Fill> {
        if self.a > 0 {
            return Some(krilla::paint::Fill {
                paint: krilla::color::rgb::Color::new(self.r, self.g, self.b).into(),
                opacity: krilla::num::NormalizedF32::new(self.a as f32 / 255.).unwrap(),
                ..Default::default()
            });
        } else { return None; }
    }
}



// Dash type for a line stroke
#[derive(Debug, Clone)]
pub enum Dash {
    Solid,
    Dashed,
    DashDot,
    Dotted,
}
impl Into<Option<krilla::paint::StrokeDash>> for Dash {
    fn into(self) -> Option<krilla::paint::StrokeDash> {
        match self {
            Self::Solid => return None,
            Self::Dashed => return Some(krilla::paint::StrokeDash { array: vec![10., 5.], offset: 0. }),
            Self::DashDot => return Some(krilla::paint::StrokeDash { array: vec![10., 5., 1., 5.], offset: 0. }),
            Self::Dotted => return Some(krilla::paint::StrokeDash { array: vec![1., 5.], offset: 0. }),
        }
    }
}
impl Into<Option<tiny_skia::StrokeDash>> for Dash {
    fn into(self) -> Option<tiny_skia::StrokeDash> {
        match self {
            Self::Solid => return None,
            Self::Dashed => return tiny_skia::StrokeDash::new(vec![10., 5.], 0.),
            Self::DashDot => return tiny_skia::StrokeDash::new(vec![10., 5., 1., 5.], 0.),
            Self::Dotted => return tiny_skia::StrokeDash::new(vec![1., 5.], 0.),
        }
    }
}


// A line stroke
#[derive(Debug, Clone)]
pub struct Stroke {
    pub color: Color,
    pub dash: Dash,
    pub width: f32
}
//
impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            dash: Dash::Solid,
            width: 1.
        }
    }
}
// Convert to a krilla stroke
impl Into<Option<krilla::paint::Stroke>> for Stroke {
    fn into(self) -> Option<krilla::paint::Stroke> {
        if self.color.a > 0 { return Some(krilla::paint::Stroke {
            paint: krilla::color::rgb::Color::new(self.color.r, self.color.g, self.color.b).into(),
            width: self.width,
            miter_limit: 1.,
            line_cap: krilla::paint::LineCap::Round,
            line_join: krilla::paint::LineJoin::Round,
            opacity: krilla::num::NormalizedF32::new(self.color.a as f32/255.).unwrap(),
            dash: self.dash.into()
        }); }
        else { return None; }
    }
}

impl Into<tiny_skia::Stroke> for Stroke {
    fn into(self) -> tiny_skia::Stroke {
        tiny_skia::Stroke {
            width: self.width,
            miter_limit: 1.,
            line_cap: tiny_skia::LineCap::Round,
            line_join: tiny_skia::LineJoin::Round,
            dash: self.dash.clone().into()
        }
    }
}


//
pub(crate) struct StrokePalette {
    colors: Vec<Color>,
    dashes: Vec<Dash>,
    current_color: usize,
    current_dash: usize,
}
//
impl StrokePalette {
    pub(crate) fn next(&mut self) -> (Color, Dash) {
        // Variables to be returned
        let color: Color = self.colors[self.current_color].clone();
        let dash: Dash = self.dashes[self.current_dash].clone();
        // Increment color
        // Safe to increment by 1
        if self.current_color < (self.colors.len() - 1) {
            self.current_color += 1;
        }
        // Increment will go past last index
        else {
            // Set to 0 and increment dash
            self.current_color = 0;
            // Increment dash by 1 if won't exceed last index, otherwise set to 0
            if self.current_dash < (self.dashes.len() - 1) { self.current_dash += 1; } else { self.current_dash = 0; }
        }
        // Return the color and dash
        return (color, dash);
    }
}
//
impl Default for StrokePalette {
    fn default() -> Self {
        Self {
            colors: MATLAB_GEM.into(),
            dashes: DASH_VARIANTS.into(),
            current_color: 0,
            current_dash: 0
        }
    }
}