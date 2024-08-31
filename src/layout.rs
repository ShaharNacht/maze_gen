use sdl2::video::Window;

use crate::maze::Maze;
use crate::point_new::{ConvertPoint, Point};

pub struct Layout {
    window_width: u32,
    window_height: u32,

    padding: u32,

    maze_layout: MazeLayout,
}

impl Layout {
    pub fn new(
        padding: u32,
        maze_width: u32,
        maze_height: u32,
        maze_cols: usize,
        maze_rows: usize,
    ) -> Self {
        let window_width = maze_width + padding * 2;
        let window_height = maze_height + padding * 2;

        let maze_position = Point::new(padding as _, padding as _);
        let maze_layout =
            MazeLayout::new(maze_position, maze_width, maze_height, maze_cols, maze_rows);

        Self {
            window_width,
            window_height,
            padding,
            maze_layout,
        }
    }

    pub fn is_point_in_maze(&self, point: Point<Window>) -> bool {
        self.maze_layout.is_point_inside(point)
    }
}

pub struct MazeLayout {
    position: Point<Window>,
    width: u32,
    height: u32,

    cols: usize,
    rows: usize,
    cell_width: u32,
    cell_height: u32,
}

impl MazeLayout {
    fn new(position: Point<Window>, width: u32, height: u32, cols: usize, rows: usize) -> Self {
        let cell_width = width / cols as u32;
        let cell_height = height / rows as u32;

        Self {
            position,
            width,
            height,
            cols,
            rows,
            cell_width,
            cell_height,
        }
    }

    fn is_point_inside(&self, point: Point<Window>) -> bool {
        point.x >= self.position.x
            && point.x < (self.position.x + self.width as i32)
            && point.y >= self.position.y
            && point.y < (self.position.y + self.height as i32)
    }
}

impl ConvertPoint<Maze, Window> for Layout {
    fn convert_point(&self, input: Point<Maze>) -> Point<Window> {
        let MazeLayout {
            position: maze_position,
            cell_width,
            cell_height,
            ..
        } = &self.maze_layout;

        let x = maze_position.x + (input.x as i32 * *cell_width as i32);
        let y = maze_position.y + (input.y as i32 * *cell_height as i32);

        Point::new(x, y)
    }
}

impl ConvertPoint<Window, Maze> for Layout {
    fn convert_point(&self, input: Point<Window>) -> Point<Maze> {
        let MazeLayout {
            position: maze_position,
            cols,
            rows,
            cell_width,
            cell_height,
            ..
        } = &self.maze_layout;

        let x = (input.x - maze_position.x) / *cell_width as i32;
        let y = (input.y - maze_position.y) / *cell_height as i32;

        let x = x.clamp(0, *cols as i32 - 1) as usize;
        let y = y.clamp(0, *rows as i32 - 1) as usize;

        Point::new(x, y)
    }
}
