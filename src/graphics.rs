use std::error::Error;
use std::fmt::{self, Display};

use sdl2::pixels::Color;
use sdl2::rect::{Point as SdlPoint, Rect};
use sdl2::render::{
    Canvas, RenderTarget, Texture, TextureCreator, TextureQuery, TextureValueError, WindowCanvas,
};
use sdl2::ttf::{Font, FontError, Sdl2TtfContext};
use sdl2::video::WindowContext;

use crate::color_blend::ColorBlend;
use crate::maze::Maze;
use crate::point::{Convert, MazePoint, WindowPoint};
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
        maze: &Maze,
        ui: &Ui,
    ) -> Result<(), DrawError> {
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();

        self.draw_maze(canvas, maze)?;
        self.draw_ui(canvas, ui)?;

        Ok(())
    }

    fn draw_maze(
        &self,
        canvas: &mut Canvas<impl RenderTarget>,
        maze: &Maze,
    ) -> Result<(), DrawError> {
        canvas.set_draw_color(VISITED_CELL_COLOR);
        for cell in maze.all_cells() {
            if maze.is_visited(cell) {
                let window_point: WindowPoint = cell.convert(maze);
                let cell_width = self.cell_width(maze);
                let cell_height = self.cell_height(maze);

                let rect = Rect::new(
                    window_point.x as _,
                    window_point.y as _,
                    cell_width as _,
                    cell_height as _,
                );
                Self::fill_rect(canvas, rect)?;
            }
        }

        canvas.set_draw_color(WALL_COLOR);
        for wall in maze.walls() {
            let horizontal = wall.0.x != wall.1.x;
            let p1: WindowPoint;
            let p2: WindowPoint;

            if horizontal {
                p1 = wall.1.convert(maze);
                p2 = (wall.1 + (0, 1)).convert(maze);
            } else {
                p1 = wall.1.convert(maze);
                p2 = (wall.1 + (1, 0)).convert(maze);
            }

            Self::draw_line(canvas, p1, p2)?;
        }

        if let Some(cursor) = maze.cursor() {
            let cursor_window: WindowPoint = cursor.convert(maze);
            let cell_width = self.cell_width(maze);
            let cell_height = self.cell_height(maze);

            let rect = Rect::new(
                (cursor_window.x + cell_width / 4) as _,
                (cursor_window.y + cell_height / 4) as _,
                (cell_width / 2) as _,
                (cell_height / 2) as _,
            );
            canvas.set_draw_color(CURSOR_COLOR);
            Self::fill_rect(canvas, rect)?;
        }

        Ok(())
    }

    fn draw_ui(&self, canvas: &mut Canvas<impl RenderTarget>, ui: &Ui) -> Result<(), DrawError> {
        let rect = Rect::new(
            GFX_UI_X as _,
            GFX_UI_Y as _,
            GFX_UI_WIDTH as _,
            GFX_UI_HEIGHT as _,
        );
        canvas.set_draw_color(UI_COLOR);
        Self::fill_rect(canvas, rect)?;

        for button in ui.buttons() {
            let rect = Rect::new(
                button.position.x as _,
                button.position.y as _,
                button.width as _,
                button.height as _,
            );

            if let ButtonState::Normal { highlight, .. } = button.state {
                canvas.set_draw_color(UI_BUTTON_COLOR.blend(UI_BUTTON_HIGHLIGHT_COLOR, highlight));
            } else if let ButtonState::Pressed = button.state {
                canvas.set_draw_color(UI_BUTTON_CLICKED_COLOR);
            }

            Self::fill_rect(canvas, rect)?;

            let text = button.text();

            let text_texture = Self::font_texture(
                text.0,
                UI_BUTTON_TEXT_COLOR,
                &self.font,
                &self.texture_creator,
            )?;

            let TextureQuery {
                width: text_width,
                height: text_height,
                ..
            } = text_texture.query();

            let text_center = (
                (button.position.x + button.width / 2) as _,
                (button.position.y + button.height / 2) as _,
            );
            let text_rect = Rect::from_center(text_center, text_width, text_height);

            Self::draw_texture(canvas, &text_texture, None, text_rect)?;
        }

        Ok(())
    }

    fn cell_width(&self, maze: &Maze) -> i64 {
        let p1 = MazePoint::new(0, 0);
        let p2 = p1 + (1, 0);

        let p1: WindowPoint = p1.convert(maze);
        let p2: WindowPoint = p2.convert(maze);

        (p2 - p1).x
    }

    fn cell_height(&self, maze: &Maze) -> i64 {
        let p1 = MazePoint::new(0, 0);
        let p2 = p1 + (0, 1);

        let p1: WindowPoint = p1.convert(maze);
        let p2: WindowPoint = p2.convert(maze);

        (p2 - p1).y
    }

    fn fill_rect(
        canvas: &mut Canvas<impl RenderTarget>,
        rect: impl Into<Option<Rect>>,
    ) -> Result<(), DrawError> {
        canvas.fill_rect(rect).map_err(DrawError::FillRect)
    }

    fn draw_line(
        canvas: &mut Canvas<impl RenderTarget>,
        start: impl Into<SdlPoint>,
        end: impl Into<SdlPoint>,
    ) -> Result<(), DrawError> {
        canvas.draw_line(start, end).map_err(DrawError::DrawLine)
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
    DrawTexture(String),
    FontRendering(FontError),
    SurfaceToTexture(TextureValueError),
}

impl Error for DrawError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
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
            Self::DrawTexture(e) => write!(f, "Failed to draw texture: {}", e),
            Self::FontRendering(e) => write!(f, "Failed to render font: {}", e),
            Self::SurfaceToTexture(e) => write!(f, "Failed to convert surface to texture: {}", e),
        }
    }
}
