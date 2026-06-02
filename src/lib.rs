use tiny_skia;
use krilla;

mod figure;
mod paint;
mod text;
mod pdf;

use figure::Figure;
use paint::{Color, Dash, Stroke};


pub mod prelude {
    pub use crate::figure::{Figure, annotation::TextBox};
    pub use crate::paint::{Color, Dash};
    pub use crate::pdf::Document;
    pub use crate::{plot, Config};
}

#[derive(Debug, Clone)]
pub enum Config<T> {
    Pending,
    On(T),
    Off,
}
impl<T> Config<T> {
    pub fn unwrap(self) -> T {
        return match self {
            Config::Pending => panic!("Attmepted to unwrap Config::Pending variant!"),
            Config::Off => panic!("Attmepted to unwrap Config::Off variant!"),
            Config::On(inner) => inner,
        }
    }

    pub fn unwrap_clone(&self) -> T where T: Clone {
        return match self {
            Config::Pending => panic!("Attmepted to unwrap Config::Pending variant!"),
            Config::Off => panic!("Attmepted to unwrap Config::Off variant!"),
            Config::On(inner) => inner.clone(),
        }
    }
}

//
// Trait to declare numeric types as castable to f32
pub trait ToF32 { fn to_f32(&self) -> f32; }
// Macro for declaring all numerics as f32 in less lines of code
macro_rules! impl_to_f32 {
    ($($t:ty),*) => {
        $(
            impl ToF32 for $t { 
                #[inline]
                fn to_f32(&self) -> f32 { *self as f32 }
            }
        )*
    };
}
// Declare ToF32 for all types (except f32)
impl_to_f32!(f64, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
// Declare ToF32 for f32 (no casting)
impl ToF32 for f32 { 
    #[inline]
    fn to_f32(&self) -> f32 { *self }
}