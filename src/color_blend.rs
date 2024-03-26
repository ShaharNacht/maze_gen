use std::array;

use sdl2::pixels::Color;

pub trait ColorBlend<O> {
    fn blend(self, other: O, factor: f64) -> Color;
}

impl<S, O> ColorBlend<O> for S
where
    S: Into<Color>,
    O: Into<Color>,
{
    fn blend(self, other: O, mut factor: f64) -> Color {
        fn to_f64_array(color: impl Into<Color>) -> [f64; 4] {
            let u8_tuple = color.into().rgba();
            let u8_arr: [u8; 4] = u8_tuple.into();

            u8_arr.map(|value| value as f64)
        }

        fn lerp(value1: f64, value2: f64, factor: f64) -> f64 {
            value1 * (1.0 - factor) + value2 * factor
        }

        factor = factor.clamp(0.0, 1.0);

        let color1 = to_f64_array(self);
        let color2 = to_f64_array(other);

        let u8_tuple: (u8, u8, u8, u8) =
            array::from_fn(|i| lerp(color1[i], color2[i], factor) as u8).into();

        u8_tuple.into()
    }
}
