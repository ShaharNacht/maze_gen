use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;

use crate::graphics::Graphics;
use crate::maze::Maze;
use crate::point::WindowPoint;
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
        let event_pump = ctx.event_pump();

        let keep_going = self.handle_events(event_pump.poll_iter());

        if !keep_going {
            return false;
        }

        let mouse_state = event_pump.mouse_state();
        let mouse = WindowPoint::new(mouse_state.x() as i64, mouse_state.y() as i64);
        let mouse_pressed = mouse_state.is_mouse_button_pressed(MouseButton::Left);
        self.ui.update(mouse, mouse_pressed, &mut self.maze);

        true
    }

    fn draw(&mut self, ctx: &mut Self::Ctx, _current_fps: usize) {
        let canvas = ctx.canvas();

        self.graphics.draw(canvas, &self.maze, &self.ui).unwrap();
        canvas.present();
    }
}
