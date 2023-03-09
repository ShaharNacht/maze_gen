use crate::point::MazePoint;

pub struct Maze
{
	width: i64,
	height: i64
}

impl Maze
{
	pub fn new( width: i64, height: i64 ) -> Self
	{
		Self { width, height }
	}
	
	pub fn width(&self) -> i64
	{
		self.width
	}
	
	pub fn height(&self) -> i64
	{
		self.height
	}
	
	pub fn all_points(&self) -> impl Iterator< Item = MazePoint > + '_
	{
		( 0 .. self.height ).flat_map( |y| ( 0 .. self.width ).map( move |x| MazePoint::new( x, y ) ) )
	}
}