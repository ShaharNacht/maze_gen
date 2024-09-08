use std::error::Error;
use std::fmt::{self, Display};
use std::num::TryFromIntError;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::{Point as SdlPoint, Rect};
use sdl2::render::{
    Canvas, RenderTarget, Texture, TextureCreator, TextureQuery, TextureValueError, WindowCanvas,
};
use sdl2::ttf::{Font, FontError, Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};

use crate::color_blend::ColorBlend;
use crate::layout::{WindowLayout, WindowMazeLayout};
use crate::maze::Maze;
use crate::point::{Convert, MazePoint, WindowPoint};
use crate::point_new::Point;
use crate::ui::{ButtonState, Ui};
use crate::{
    BACKGROUND_COLOR, CURSOR_COLOR, FONT, FONT_SIZE, GFX_UI_HEIGHT, GFX_UI_WIDTH, GFX_UI_X,
    GFX_UI_Y, UI_BUTTON_CLICKED_COLOR, UI_BUTTON_COLOR, UI_BUTTON_HIGHLIGHT_COLOR,
    UI_BUTTON_TEXT_COLOR, UI_COLOR, VISITED_CELL_COLOR, WALL_COLOR,
};

pub struct Graphics<'ttf> {
    font: Font<'ttf, 'static>,
    texture_creator: TextureCreator<WindowContext>,
}

impl<'ttf> Graphics<'ttf> {
    pub fn new(
        ttf_ctx: &'ttf Sdl2TtfContext,
        canvas: &WindowCanvas,
    ) -> Result<Self, FontLoadError> {
        let font = ttf_ctx.load_font(FONT, FONT_SIZE)?;

        let texture_creator = canvas.texture_creator();

        Ok(Self {
            font,
            texture_creator,
        })
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas<impl RenderTarget>,
        layout: &WindowLayout,
        maze: &Maze,
        ui: &Ui,
    ) -> Result<(), DrawError> {
        canvas.set_draw_color(UI_COLOR);
        canvas.clear();

        self.draw_maze(canvas, &layout.maze, maze)?;
        // self.draw_ui(canvas, ui)?;

        Ok(())
    }

    fn draw_maze(
        &self,
        canvas: &mut Canvas<impl RenderTarget>,
        layout: &WindowMazeLayout,
        maze: &Maze,
    ) -> Result<(), DrawError> {
        self.draw_maze_background(canvas, layout, maze)?;

        self.draw_maze_visited_cells(canvas, layout, maze)?;

        self.draw_maze_walls(canvas, layout, maze)?;

        self.draw_maze_cursor(canvas, layout, maze)?;

        Ok(())
    }

    fn draw_maze_background(
        &self,
        canvas: &mut Canvas<impl RenderTarget>,
        layout: &WindowMazeLayout,
        maze: &Maze,
    ) -> Result<(), DrawError> {
        canvas.set_draw_color(BACKGROUND_COLOR);

        Self::fill_rect(
            canvas,
            Rect::new(
                layout.position.x,
                layout.position.y,
                layout.width,
                layout.height,
            ),
        )
    }

    fn draw_maze_visited_cells(
        &self,
        canvas: &mut Canvas<impl RenderTarget>,
        layout: &WindowMazeLayout,
        maze: &Maze,
    ) -> Result<(), DrawError> {
        canvas.set_draw_color(VISITED_CELL_COLOR);

        for cell in maze.all_cells().filter(|&cell| maze.is_visited(cell)) {
            let cell_x = layout.cell_x_positions[cell.x as usize];
            let cell_y = layout.cell_y_positions[cell.y as usize];
            let (cell_width, cell_height) =
                layout.cell_size(Point::new(cell.x as usize, cell.y as usize));

            Self::fill_rect(canvas, Rect::new(cell_x, cell_y, cell_width, cell_height))?;
        }

        Ok(())
    }

    fn draw_maze_walls(
        &self,
        canvas: &mut Canvas<impl RenderTarget>,
        layout: &WindowMazeLayout,
        maze: &Maze,
    ) -> Result<(), DrawError> {
        let top_left = Point::new(layout.cell_x_positions[0], layout.cell_y_positions[0]);
        let top_right = Point::new(
            layout.cell_x_positions[maze.width() as usize],
            layout.cell_y_positions[0],
        );
        let bottom_left = Point::new(
            layout.cell_x_positions[0],
            layout.cell_y_positions[maze.height() as usize],
        );
        let bottom_right = Point::new(
            layout.cell_x_positions[maze.width() as usize],
            layout.cell_y_positions[maze.height() as usize],
        );

        for (start, end) in [
            (top_left, top_right),
            (bottom_left, bottom_right),
            (top_left, bottom_left),
            (top_right, bottom_right),
        ] {
            Self::draw_line(canvas, start, end, layout.wall_thickness, WALL_COLOR)?;
        }

        for wall in maze.walls() {
            let is_horizontal = wall.first_cell().x != wall.second_cell().x;

            let p1_maze =
                Point::<Maze>::new(wall.second_cell().x as usize, wall.second_cell().y as usize);
            let p2_maze = p1_maze + if is_horizontal { (0, 1) } else { (1, 0) };

            let p1_window = Point::new(
                layout.cell_x_positions[p1_maze.x],
                layout.cell_y_positions[p1_maze.y],
            );
            let p2_window = Point::new(
                layout.cell_x_positions[p2_maze.x],
                layout.cell_y_positions[p2_maze.y],
            );

            Self::draw_line(
                canvas,
                p1_window,
                p2_window,
                layout.wall_thickness,
                WALL_COLOR,
            )?;
        }

        Ok(())
    }

    fn draw_maze_cursor(
        &self,
        canvas: &mut Canvas<impl RenderTarget>,
        layout: &WindowMazeLayout,
        maze: &Maze,
    ) -> Result<(), DrawError> {
        if let Some(cursor) = maze.cursor() {
            let cell_x = layout.cell_x_positions[cursor.x as usize];
            let cell_y = layout.cell_y_positions[cursor.y as usize];
            let (cell_width, cell_height) =
                layout.cell_size(Point::new(cursor.x as usize, cursor.y as usize));

            let cursor_rect = Rect::new(
                cell_x + (cell_width as i32 / 4),
                cell_y + (cell_height as i32 / 4),
                cell_width / 2,
                cell_height / 2,
            );

            canvas.set_draw_color(CURSOR_COLOR);
            Self::fill_rect(canvas, cursor_rect)?;
        }

        Ok(())
    }

    // fn draw_ui(&self, canvas: &mut Canvas<impl RenderTarget>, ui: &Ui) -> Result<(), DrawError> {
    //     let rect = Rect::new(
    //         GFX_UI_X as _,
    //         GFX_UI_Y as _,
    //         GFX_UI_WIDTH as _,
    //         GFX_UI_HEIGHT as _,
    //     );
    //     canvas.set_draw_color(UI_COLOR);
    //     Self::fill_rect(canvas, rect)?;

    //     for button in ui.buttons() {
    //         let rect = Rect::new(
    //             button.position.x as _,
    //             button.position.y as _,
    //             button.width as _,
    //             button.height as _,
    //         );

    //         if let ButtonState::Normal { highlight, .. } = button.state {
    //             canvas.set_draw_color(UI_BUTTON_COLOR.blend(UI_BUTTON_HIGHLIGHT_COLOR, highlight));
    //         } else if let ButtonState::Pressed = button.state {
    //             canvas.set_draw_color(UI_BUTTON_CLICKED_COLOR);
    //         }

    //         Self::fill_rect(canvas, rect)?;

    //         let text = button.text();

    //         let text_texture = Self::font_texture(
    //             text.0,
    //             UI_BUTTON_TEXT_COLOR,
    //             &self.font,
    //             &self.texture_creator,
    //         )?;

    //         let TextureQuery {
    //             width: text_width,
    //             height: text_height,
    //             ..
    //         } = text_texture.query();

    //         let text_center = (
    //             (button.position.x + button.width / 2) as _,
    //             (button.position.y + button.height / 2) as _,
    //         );
    //         let text_rect = Rect::from_center(text_center, text_width, text_height);

    //         Self::draw_texture(canvas, &text_texture, None, text_rect)?;
    //     }

    //     Ok(())
    // }

    fn fill_rect(
        canvas: &mut Canvas<impl RenderTarget>,
        rect: impl Into<Option<Rect>>,
    ) -> Result<(), DrawError> {
        canvas.fill_rect(rect).map_err(DrawError::FillRect)
    }

    fn draw_line(
        canvas: &mut Canvas<impl RenderTarget>,
        start: impl Into<Point<Window>>,
        end: impl Into<Point<Window>>,
        thickness: u32,
        color: impl Into<Color>,
    ) -> Result<(), DrawError> {
        let mut start = start.into();
        let mut end = end.into();

        let angle = (end.y as f64 - start.y as f64).atan2(end.x as f64 - start.x as f64);
        let mut offset = Point::new(
            (angle.cos() * (thickness as f64 / 2.0)) as i32,
            (angle.sin() * (thickness as f64 / 2.0)) as i32,
        );

        start -= offset;

        if offset.x != 0 {
            offset.x -= 1;
        }
        if offset.y != 0 {
            offset.y -= 1;
        }

        end += offset;

        let thickness = thickness
            .try_into()
            .map_err(|e| DrawError::LineTooThick(thickness, e))?;

        let color = color.into();

        canvas
            .thick_line(
                start.x as i16,
                start.y as i16,
                end.x as i16,
                end.y as i16,
                thickness,
                color,
            )
            .map_err(DrawError::DrawLine)
    }

    fn draw_texture(
        canvas: &mut Canvas<impl RenderTarget>,
        texture: &Texture,
        src: impl Into<Option<Rect>>,
        dst: impl Into<Option<Rect>>,
    ) -> Result<(), DrawError> {
        canvas
            .copy(texture, src, dst)
            .map_err(DrawError::DrawTexture)
    }

    fn font_texture<'texture_creator, T>(
        text: impl AsRef<str>,
        color: impl Into<Color>,
        font: &Font,
        texture_creator: &'texture_creator TextureCreator<T>,
    ) -> Result<Texture<'texture_creator>, DrawError> {
        let surface = font
            .render(text.as_ref())
            .blended(color)
            .map_err(DrawError::FontRendering)?;

        texture_creator
            .create_texture_from_surface(surface)
            .map_err(DrawError::SurfaceToTexture)
    }
}

#[derive(Debug)]
pub struct FontLoadError(String);

impl FontLoadError {
    fn new(inner: String) -> Self {
        Self(inner)
    }
}

impl Error for FontLoadError {}

impl Display for FontLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to load font: {}", self.0)
    }
}

impl From<String> for FontLoadError {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Debug)]
pub enum DrawError {
    FillRect(String),
    DrawLine(String),
    LineTooThick(u32, TryFromIntError),
    DrawTexture(String),
    FontRendering(FontError),
    SurfaceToTexture(TextureValueError),
}

impl Error for DrawError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::LineTooThick(_, e) => Some(e),
            Self::FontRendering(e) => Some(e),
            Self::SurfaceToTexture(e) => Some(e),

            Self::FillRect(_) | Self::DrawLine(_) | Self::DrawTexture(_) => None,
        }
    }
}

impl Display for DrawError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FillRect(e) => write!(f, "Failed to fill rect: {}", e),
            Self::DrawLine(e) => write!(f, "Failed to draw line: {}", e),
            Self::LineTooThick(thickness, e) => write!(
                f,
                "Attempted to draw line with thickness of {}, but only up to 255 is supported ({})",
                thickness, e
            ),
            Self::DrawTexture(e) => write!(f, "Failed to draw texture: {}", e),
            Self::FontRendering(e) => write!(f, "Failed to render font: {}", e),
            Self::SurfaceToTexture(e) => write!(f, "Failed to convert surface to texture: {}", e),
        }
    }
}
