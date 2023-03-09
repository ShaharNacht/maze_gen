use std::ops::{ Add, Sub, Mul, Div };

use crate::maze::Maze;
use crate::{ GFX_MAZE_X, GFX_MAZE_Y, GFX_MAZE_WIDTH, GFX_MAZE_HEIGHT };

#[derive( Clone, Copy, PartialEq, Eq )]
pub struct MazePoint
{
	pub x: i64,
	pub y: i64
}

impl MazePoint
{
	pub fn new( x: i64, y: i64 ) -> Self
	{
		Self { x, y }
	}
}

impl From<(i64,i64)> for MazePoint
{
	fn from( value: (i64,i64) ) -> Self
	{
		Self::new( value.0, value.1 )
	}
}

impl From<MazePoint> for (i64,i64)
{
	fn from( value: MazePoint ) -> Self
	{
		( value.x, value.y )
	}
}

impl<T> Add<T> for MazePoint
	where T: Into<MazePoint>
{
	type Output = MazePoint;
	
	fn add( self, rhs: T ) -> Self::Output
	{
		let rhs = rhs.into();
		
		Self::Output::new( self.x + rhs.x, self.y + rhs.y )
	}
}

impl<T> Sub<T> for MazePoint
	where T: Into<MazePoint>
{
	type Output = MazePoint;
	
	fn sub( self, rhs: T ) -> Self::Output
	{
		let rhs = rhs.into();
		
		Self::Output::new( self.x - rhs.x, self.y - rhs.y )
	}
}

impl<T> Mul<T> for MazePoint
	where T: Into<MazePoint>
{
	type Output = MazePoint;
	
	fn mul( self, rhs: T ) -> Self::Output
	{
		let rhs = rhs.into();
		
		Self::Output::new( self.x * rhs.x, self.y * rhs.y )
	}
}

impl Mul<i64> for MazePoint
{
	type Output = MazePoint;
	
	fn mul( self, rhs: i64 ) -> Self::Output
	{
		Self::Output::new( self.x * rhs, self.y * rhs )
	}
}

impl<T> Div<T> for MazePoint
	where T: Into<MazePoint>
{
	type Output = MazePoint;
	
	fn div( self, rhs: T ) -> Self::Output
	{
		let rhs = rhs.into();
		
		Self::Output::new( self.x / rhs.x, self.y / rhs.y )
	}
}

impl Div<i64> for MazePoint
{
	type Output = MazePoint;
	
	fn div( self, rhs: i64 ) -> Self::Output
	{
		Self::Output::new( self.x / rhs, self.y / rhs )
	}
}

#[derive( Clone, Copy, PartialEq, Eq )]
pub struct WindowPoint
{
	pub x: i64,
	pub y: i64
}

impl WindowPoint
{
	pub fn new( x: i64, y: i64 ) -> Self
	{
		Self { x, y }
	}
}

impl From<(i64,i64)> for WindowPoint
{
	fn from( value: (i64,i64) ) -> Self
	{
		Self::new( value.0, value.1 )
	}
}

impl From<WindowPoint> for (i64,i64)
{
	fn from( value: WindowPoint ) -> Self
	{
		( value.x, value.y )
	}
}

impl Into<sdl2::rect::Point> for WindowPoint
{
	fn into(self) -> sdl2::rect::Point
	{
		sdl2::rect::Point::new( self.x as i32, self.y as i32 )
	}
}

impl<T> Add<T> for WindowPoint
	where T: Into<WindowPoint>
{
	type Output = WindowPoint;
	
	fn add( self, rhs: T ) -> Self::Output
	{
		let rhs = rhs.into();
		
		Self::Output::new( self.x + rhs.x, self.y + rhs.y )
	}
}

impl<T> Sub<T> for WindowPoint
	where T: Into<WindowPoint>
{
	type Output = WindowPoint;
	
	fn sub( self, rhs: T ) -> Self::Output
	{
		let rhs = rhs.into();
		
		Self::Output::new( self.x - rhs.x, self.y - rhs.y )
	}
}

impl<T> Mul<T> for WindowPoint
	where T: Into<WindowPoint>
{
	type Output = WindowPoint;
	
	fn mul( self, rhs: T ) -> Self::Output
	{
		let rhs = rhs.into();
		
		Self::Output::new( self.x * rhs.x, self.y * rhs.y )
	}
}

impl Mul<i64> for WindowPoint
{
	type Output = WindowPoint;
	
	fn mul( self, rhs: i64 ) -> Self::Output
	{
		Self::Output::new( self.x * rhs, self.y * rhs )
	}
}

impl<T> Div<T> for WindowPoint
	where T: Into<WindowPoint>
{
	type Output = WindowPoint;
	
	fn div( self, rhs: T ) -> Self::Output
	{
		let rhs = rhs.into();
		
		Self::Output::new( self.x / rhs.x, self.y / rhs.y )
	}
}

impl Div<i64> for WindowPoint
{
	type Output = WindowPoint;
	
	fn div( self, rhs: i64 ) -> Self::Output
	{
		Self::Output::new( self.x / rhs, self.y / rhs )
	}
}

pub trait Convert<T>
{
	fn convert( self, maze: &Maze ) -> T;
}

impl Convert<WindowPoint> for MazePoint
{
	fn convert( self, maze: &Maze ) -> WindowPoint
	{
		let cell_width = GFX_MAZE_WIDTH / maze.width();
		let cell_height = GFX_MAZE_HEIGHT / maze.height();
		
		WindowPoint::new( self.x * cell_width + GFX_MAZE_X, self.y * cell_height + GFX_MAZE_Y )
	}
}

impl Convert<MazePoint> for WindowPoint
{
	fn convert( self, maze: &Maze ) -> MazePoint
	{
		let cell_width = GFX_MAZE_WIDTH / maze.width();
		let cell_height = GFX_MAZE_HEIGHT / maze.height();
		
		let mut x = ( self.x - GFX_MAZE_X ) / cell_width;
		let mut y = ( self.y - GFX_MAZE_Y ) / cell_height;
		
		x = x.clamp( 0, maze.width() - 1 );
		y = y.clamp( 0, maze.height() - 1 );
		
		MazePoint::new( x, y )
	}
}