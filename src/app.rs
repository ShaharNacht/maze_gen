use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;

use crate::graphics::Graphics;
use crate::maze::Maze;
use crate::stable_loop::StableLoop;
use crate::str_err::Result;
use crate::ui::Ui;
use crate::Context;
use crate::TARGET_FPS;

pub struct App<'ttf> {
    maze: Maze,
    ui: Ui,
    graphics: Graphics<'ttf>,
}

impl<'ttf> App<'ttf> {
    pub fn new(
        maze_width: i64,
        maze_height: i64,
        ttf_ctx: &'ttf Sdl2TtfContext,
        canvas: &WindowCanvas,
    ) -> Result<Self> {
        let maze = Maze::new(maze_width, maze_height);
        let ui = Ui::new();
        let graphics = Graphics::new(ttf_ctx, canvas)?;

        Ok(Self { maze, ui, graphics })
    }

    fn handle_events(&mut self, events: impl Iterator<Item = Event>) -> bool {
        for event in events {
            match event {
                Event::Quit { .. }
                | Event::KeyUp {
                    scancode: Some(Scancode::Escape),
                    ..
                } => {
                    return false;
                }

                _ => {}
            }
        }

        true
    }
}

impl<'ttf> StableLoop for App<'ttf> {
    type Ctx = Context;

    fn target_fps(&self) -> f64 {
        TARGET_FPS
    }

    fn update(&mut self, ctx: &mut Self::Ctx, _current_fps: usize) -> bool {
        self.handle_events(ctx.event_pump().poll_iter())
    }

    fn draw(&mut self, ctx: &mut Self::Ctx, current_fps: usize) {}
}
