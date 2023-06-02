use sdl2::pixels::Color;

pub trait ColorBlend
{
	fn blend( self, other: Self, factor: f64 ) -> Color;
}

impl<T> ColorBlend for T
	where T: Into<Color>
{
	fn blend( self, other: Self, factor: f64 ) -> Color
	{
		fn to_f64_tuple( color: impl Into<Color> ) -> ( f64, f64, f64, f64 )
		{
			let color = color.into().rgba();
			( color.0 as f64, color.1 as f64, color.2 as f64, color.3 as f64 )
		}
		
		fn lerp( value1: f64, value2: f64, factor: f64 ) -> f64
		{
			value1 * ( 1.0 - factor ) + value2 * factor
		}
		
		let factor = factor.clamp( 0.0, 1.0 );
		
		let color1 = to_f64_tuple(self);
		let color2 = to_f64_tuple(other);
		
		let result = (
			lerp( color1.0, color2.0, factor ) as u8,
			lerp( color1.1, color2.1, factor ) as u8,
			lerp( color1.2, color2.2, factor ) as u8,
			lerp( color1.3, color2.3, factor ) as u8
		);
		
		result.into()
	}
}