use std::error::Error;

fn complex_error_to_string_error< T: Error >( err: T ) -> String
{
	return format!( "{}", err );
}

enum ResultStr<T>
{
	Ok(T),
	Err(String)
}

impl< T, E: Error > From< Result< T, E > > for ResultStr<T>
{
	fn from( value: Result< T, E > ) -> Self
	{
		match value
		{
			Ok(x) => ResultStr::Ok(x),
			Err(err) => ResultStr::Err( format!( "{}", err ) )
		}
	}
}

impl<T> Into< Result< T, String > > for ResultStr<T>
{
	fn into(self) -> Result< T, String >
	{
		match self
		{
			ResultStr::Ok(x) => Ok(x),
			ResultStr::Err(err) => Err(err)
		}
	}
}