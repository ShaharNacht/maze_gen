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
			
			let possible_steps: Vec<MazePoint> = DIRECTIONS.into_iter().map( |dir| cursor + dir )
				.filter( |&step| self.is_point_inside(step) )
				.collect();
			
			let step = possible_steps.choose(&mut self.rng);
			
			if let Some(&step) = step
			{
				self.path.push(step);
			}
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
	
	fn is_point_inside( &self, point: MazePoint ) -> bool
	{
		point.x >= 0 &&
			point.x < self.width &&
			point.y >= 0 &&
			point.y < self.height
	}
}