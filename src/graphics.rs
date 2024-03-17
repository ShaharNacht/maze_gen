use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget, TextureCreator, TextureQuery, WindowCanvas};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::WindowContext;

use crate::color_blend::ColorBlend;
use crate::maze::Maze;
use crate::point::{Convert, MazePoint, WindowPoint};
use crate::str_err::{Result, StrErr};
use crate::ui::Ui;
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
    pub fn new(ttf_ctx: &'ttf Sdl2TtfContext, canvas: &WindowCanvas) -> Result<Self> {
        let font = ttf_ctx.load_font(FONT, FONT_SIZE)?;

        let texture_creator = canvas.texture_creator();

        Ok(Self {
            font,
            texture_creator,
        })
    }

    pub fn draw<T: RenderTarget>(
        &self,
        canvas: &mut Canvas<T>,
        maze: &Maze,
        ui: &Ui,
    ) -> Result<()> {
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();

        self.draw_maze(canvas, maze)?;
        self.draw_ui(canvas, ui)?;

        Ok(())
    }

    fn draw_maze<T: RenderTarget>(&self, canvas: &mut Canvas<T>, maze: &Maze) -> Result<()> {
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
                canvas.fill_rect(rect)?;
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

            canvas.draw_line(p1, p2)?;
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
            canvas.fill_rect(rect)?;
        }

        Ok(())
    }

    fn draw_ui<T: RenderTarget>(&self, canvas: &mut Canvas<T>, ui: &Ui) -> Result<()> {
        let rect = Rect::new(
            GFX_UI_X as _,
            GFX_UI_Y as _,
            GFX_UI_WIDTH as _,
            GFX_UI_HEIGHT as _,
        );
        canvas.set_draw_color(UI_COLOR);
        canvas.fill_rect(rect)?;

        for button in ui.buttons() {
            let rect = Rect::new(
                button.position.x as _,
                button.position.y as _,
                button.width as _,
                button.height as _,
            );

            if button.is_pressed {
                canvas.set_draw_color(UI_BUTTON_CLICKED_COLOR);
            } else {
                canvas.set_draw_color(
                    UI_BUTTON_COLOR.blend(UI_BUTTON_HIGHLIGHT_COLOR, button.highlight),
                );
            }

            canvas.fill_rect(rect)?;

            let text = button.text();

            let text_surface = self
                .font
                .render(text.0)
                .blended(UI_BUTTON_TEXT_COLOR)
                .str_err()?;
            let text_texture = self
                .texture_creator
                .create_texture_from_surface(text_surface)
                .str_err()?;
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

            canvas.copy(&text_texture, None, text_rect)?;
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
}
