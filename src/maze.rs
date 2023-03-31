use std::collections::HashSet;

use rand::thread_rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use crate::str_err::Result;
use crate::point::MazePoint;

pub struct Maze
{
	width: i64,
	height: i64,
	
	walls: HashSet<Wall>,
	path: Vec<MazePoint>,
	visited: HashSet<MazePoint>,
	
	rng: ThreadRng
}

impl Maze
{
	pub fn new( width: i64, height: i64 ) -> Self
	{
		let start_point = MazePoint::new( 0, 0 );
		
		let mut walls = HashSet::new();
		Self::fill_all_walls( &mut walls, width, height );
		
		let path = vec![start_point];
		
		let mut visited = HashSet::new();
		visited.insert(start_point);
		
		let rng = thread_rng();
		
		Self { width, height, walls, path, visited, rng }
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
				self.visited.insert(step);
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
	
	pub fn is_visited( &self, point: MazePoint ) -> bool
	{
		self.visited.contains(&point)
	}
	
	fn is_point_inside( &self, point: MazePoint ) -> bool
	{
		point.x >= 0 &&
			point.x < self.width &&
			point.y >= 0 &&
			point.y < self.height
	}
	
	fn fill_all_walls( walls: &mut HashSet<Wall>, width: i64, height: i64 )
	{
		for y in 0 .. height - 1
		{
			for x in 0 .. width - 1
			{
				let current = MazePoint::new( x, y );
				let right = current + ( 1, 0 );
				let down = current + ( 0, 1 );
				
				walls.insert( Wall::new( current, right ).unwrap() );
				walls.insert( Wall::new( current, down ).unwrap() );
			}
		}
	}
}

/// A wall between two cells in a Maze
/// 
/// Enforces an invariat to make sure the two cells are adjacent,
/// and that the internal order of the cells doesn't depend on the provided order,
/// so that equality checks and hashes don't depend the order.
#[derive( PartialEq, Eq, Hash )]
struct Wall( MazePoint, MazePoint );

impl Wall
{
	fn new( cell1: MazePoint, cell2: MazePoint ) -> Result<Self>
	{
		let x_diff = ( cell2.x - cell1.x ).abs();
		let y_diff = ( cell2.y - cell1.y ).abs();
		
		if x_diff + y_diff != 1
		{
			return Err( "A wall must be between two adjacent cells (no diagonals allowed)".to_string() );
		}
		
		let first;
		let second;
		
		use std::cmp::Ordering::*;
		
		match cell1.y.cmp(&cell2.y)
		{
			Less =>
			{
				first = cell1;
				second = cell2;
			}
			
			Greater =>
			{
				first = cell2;
				second = cell1;
			}
			
			Equal => match cell1.x.cmp(&cell2.x)
			{
				Less =>
				{
					first = cell1;
					second = cell2;
				}
				
				Greater =>
				{
					first = cell2;
					second = cell1;
				}
				
				Equal => unreachable!()
			}
		}
		
		Ok( Self( first, second ) )
	}
}