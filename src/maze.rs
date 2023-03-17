use rand::thread_rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use crate::point::MazePoint;

pub struct Maze
{
	width: i64,
	height: i64,
	path: Vec<MazePoint>,
	
	rng: ThreadRng
}

impl Maze
{
	pub fn new( width: i64, height: i64 ) -> Self
	{
		let rng = thread_rng();
		
		let path = vec![ MazePoint::new( 0, 0 ) ];
		
		Self { width, height, path, rng }
	}
	
	pub fn width(&self) -> i64
	{
		self.width
	}
	
	pub fn height(&self) -> i64
	{
		self.height
	}
	
	pub fn step(&mut self)
	{
		if let Some(cursor) = self.cursor()
		{
			const DIRECTIONS: [ ( i64, i64 ); 4 ] = [
				( -1,  0 ),
				(  1,  0 ),
				(  0, -1 ),
				(  0,  1 )
			];
			
			let direction = *DIRECTIONS.choose(&mut self.rng).unwrap();
			
			self.path.push( cursor + direction );
		}
	}
	
	pub fn all_points(&self) -> impl Iterator< Item = MazePoint > + '_
	{
		( 0 .. self.height ).flat_map( |y| ( 0 .. self.width ).map( move |x| MazePoint::new( x, y ) ) )
	}
	
	pub fn cursor(&self) -> Option<MazePoint>
	{
		self.path.last().copied()
	}
}