use std::collections::HashSet;

use rand::seq::SliceRandom;
use rand::Rng;

use crate::point::MazePoint;

pub struct Maze {
    width: i64,
    height: i64,

    walls: HashSet<Wall>,
    path: Vec<MazePoint>,
    visited: HashSet<MazePoint>,
}

impl Maze {
    pub fn new(width: i64, height: i64) -> Self {
        let start_cell = MazePoint::new(0, 0);

        let mut walls = HashSet::new();
        Self::fill_all_walls(&mut walls, width, height);

        let path = vec![start_cell];

        let mut visited = HashSet::new();
        visited.insert(start_cell);

        Self {
            width,
            height,
            walls,
            path,
            visited,
        }
    }

    pub fn width(&self) -> i64 {
        self.width
    }

    pub fn height(&self) -> i64 {
        self.height
    }

    pub fn step(&mut self, rng: &mut impl Rng) {
        if let Some(cursor) = self.cursor() {
            const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            let possible_steps: Vec<MazePoint> = DIRECTIONS
                .into_iter()
                .map(|dir| cursor + dir)
                .filter(|&step| self.is_point_inside(step))
                .collect();

            if !possible_steps.is_empty() {
                let step = *possible_steps.choose(rng).unwrap();

                self.walls.remove(&Wall::new(cursor, step).unwrap());

                self.path.push(step);
                self.visited.insert(step);
            }
        }
    }

    pub fn all_cells(&self) -> impl Iterator<Item = MazePoint> + '_ {
        (0..self.height).flat_map(|y| (0..self.width).map(move |x| MazePoint::new(x, y)))
    }

    pub fn cursor(&self) -> Option<MazePoint> {
        self.path.last().copied()
    }

    pub fn is_visited(&self, cell: MazePoint) -> bool {
        self.visited.contains(&cell)
    }

    pub fn walls(&self) -> impl Iterator<Item = &Wall> {
        self.walls.iter()
    }

    fn is_point_inside(&self, point: MazePoint) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }

    fn fill_all_walls(walls: &mut HashSet<Wall>, width: i64, height: i64) {
        for y in 0..height {
            for x in 0..width {
                let current = MazePoint::new(x, y);
                let right = current + (1, 0);
                let down = current + (0, 1);

                if right.x < width {
                    walls.insert(Wall::new(current, right).unwrap());
                }

                if down.y < height {
                    walls.insert(Wall::new(current, down).unwrap());
                }
            }
        }
    }
}

/// A wall between two cells in a Maze
///
/// Enforces an invariat to make sure the two cells are adjacent,
/// and that the internal order of the cells doesn't depend on the provided order,
/// so that equality checks and hashes don't depend the order.
#[derive(PartialEq, Eq, Hash)]
pub struct Wall(MazePoint, MazePoint);

impl Wall {
    fn new(cell1: MazePoint, cell2: MazePoint) -> Result<Self, String> {
        let x_diff = (cell2.x - cell1.x).abs();
        let y_diff = (cell2.y - cell1.y).abs();

        if x_diff + y_diff != 1 {
            return Err(
                "A wall must be between two adjacent cells (no diagonals allowed)".to_string(),
            );
        }

        use std::cmp::Ordering::*;

        match (cell1.x.cmp(&cell2.x), cell1.y.cmp(&cell2.y)) {
            (_, Less) | (Less, Equal) => Ok(Self(cell1, cell2)),

            (_, Greater) | (Greater, Equal) => Ok(Self(cell2, cell1)),

            _ => unreachable!(),
        }
    }

    pub fn first_cell(&self) -> MazePoint {
        self.0
    }

    pub fn second_cell(&self) -> MazePoint {
        self.1
    }
}
