use sdl2::video::Window;

use crate::point_new::Point;

#[derive(Clone, Copy)]
pub struct LayoutConfig {
    pub padding: f64,

    pub maze_width: f64,
    pub maze_height: f64,
    pub maze_wall_thickness: f64,
}

pub struct Layout {
    total_width: f64,
    total_height: f64,

    padding: f64,

    maze: MazeLayout,
}

impl Layout {
    pub fn new(config: LayoutConfig) -> Self {
        let LayoutConfig {
            padding,
            maze_width,
            maze_height,
            maze_wall_thickness,
        } = config;

        let total_width = maze_width + padding * 2.0;
        let total_height = maze_height + padding * 2.0; // + padding * 11.0

        let maze = MazeLayout::new(
            Point::new(padding, padding),
            maze_width,
            maze_height,
            maze_wall_thickness,
        );

        Self {
            total_width,
            total_height,
            padding,
            maze,
        }
    }

    pub fn apply(
        &self,
        window_width: u32,
        window_height: u32,
        maze_cols: usize,
        maze_rows: usize,
    ) -> WindowLayout {
        WindowLayout::new(self, window_width, window_height, maze_cols, maze_rows)
    }
}

#[derive(Debug)]
pub struct WindowLayout {
    pub maze: WindowMazeLayout,
}

impl WindowLayout {
    fn new(
        layout: &Layout,
        window_width: u32,
        window_height: u32,
        maze_cols: usize,
        maze_rows: usize,
    ) -> Self {
        let layout_aspect_ratio = layout.total_width / layout.total_height;
        let window_aspect_ratio = window_width as f64 / window_height as f64;

        let scale_factor;
        let layout_window_x;
        let layout_window_y;
        let layout_window_width;
        let layout_window_height;
        if window_aspect_ratio >= layout_aspect_ratio {
            layout_window_height = window_height as f64;
            layout_window_width = layout_window_height * layout_aspect_ratio;

            scale_factor = layout_window_height / layout.total_height;

            layout_window_x = window_width as f64 / 2.0 - layout_window_width / 2.0;
            layout_window_y = 0.0;
        } else {
            layout_window_width = window_width as f64;
            layout_window_height = layout_window_width / layout_aspect_ratio;

            scale_factor = layout_window_width / layout.total_width;

            layout_window_x = 0.0;
            layout_window_y = window_height as f64 / 2.0 - layout_window_height / 2.0;
        }

        let maze = WindowMazeLayout::new(
            layout,
            scale_factor,
            layout_window_x,
            layout_window_y,
            layout_window_width,
            layout_window_height,
            maze_cols,
            maze_rows,
        );

        Self { maze }
    }
}

#[derive(Debug)]
pub struct WindowMazeLayout {
    pub position: Point<Window>,
    pub width: u32,
    pub height: u32,
    pub wall_thickness: u32,

    pub cell_x_positions: Vec<i32>,
    pub cell_y_positions: Vec<i32>,
}

impl WindowMazeLayout {
    fn new(
        layout: &Layout,
        scale_factor: f64,
        layout_window_x: f64,
        layout_window_y: f64,
        layout_window_width: f64,
        layout_window_height: f64,
        cols: usize,
        rows: usize,
    ) -> Self {
        let x =
            layout_window_x + (layout.maze.position.x / layout.total_width * layout_window_width);
        let y =
            layout_window_y + (layout.maze.position.y / layout.total_height * layout_window_height);

        let width = layout.maze.width / layout.total_width * layout_window_width;
        let height = layout.maze.height / layout.total_height * layout_window_height;

        let wall_thickness = layout.maze.wall_thickness * scale_factor;

        let mut cell_x_positions = vec![];
        for col in 0..=cols {
            cell_x_positions.push((x + (col as f64 / cols as f64 * width)) as i32);
        }

        let mut cell_y_positions = vec![];
        for row in 0..=rows {
            cell_y_positions.push((y + (row as f64 / rows as f64 * height)) as i32);
        }

        Self {
            position: Point::new(x as _, y as _),
            width: width as _,
            height: height as _,
            wall_thickness: wall_thickness as _,
            cell_x_positions,
            cell_y_positions,
        }
    }
}

struct MazeLayout {
    position: Point<Layout>,
    width: f64,
    height: f64,
    wall_thickness: f64,
}

impl MazeLayout {
    fn new(position: Point<Layout>, width: f64, height: f64, wall_thickness: f64) -> Self {
        Self {
            position,
            width,
            height,
            wall_thickness,
        }
    }
}
