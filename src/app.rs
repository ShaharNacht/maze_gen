use rand::rngs::ThreadRng;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;

use crate::context::Context;
use crate::graphics::{FontLoadError, Graphics};
use crate::layout::{Layout, LayoutConfig};
use crate::maze::Maze;
use crate::point::WindowPoint;
use crate::stable_loop::StableLoop;
use crate::ui::{ButtonId, Ui};
use crate::{TARGET_FPS, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct App<'ttf> {
    graphics: Graphics<'ttf>,
    layout: Layout,
    rng: ThreadRng,

    maze: Maze,
    ui: Ui,
}

impl<'ttf> App<'ttf> {
    pub fn new(
        maze_width: i64,
        maze_height: i64,
        layout_config: LayoutConfig,
        ttf_ctx: &'ttf Sdl2TtfContext,
        canvas: &WindowCanvas,
    ) -> Result<Self, FontLoadError> {
        let graphics = Graphics::new(ttf_ctx, canvas)?;
        let layout = Layout::new(layout_config);
        let rng = rand::thread_rng();

        let maze = Maze::new(maze_width, maze_height);
        let ui = Ui::new();

        Ok(Self {
            graphics,
            layout,
            rng,
            maze,
            ui,
        })
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

                Event::MouseMotion { x, y, .. } => {
                    self.ui.on_mouse_move(WindowPoint::new(x as _, y as _));
                }

                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    self.ui.on_mouse_press(WindowPoint::new(x as _, y as _));
                }

                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    if let Some(clicked_button) =
                        self.ui.on_mouse_release(WindowPoint::new(x as _, y as _))
                    {
                        match clicked_button {
                            ButtonId::Step => {
                                self.maze.step(&mut self.rng);
                            }

                            _ => {}
                        }
                    }
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

        self.ui.update();

        println!(
            "{:?}",
            self.layout.apply(
                WINDOW_WIDTH as _,
                WINDOW_HEIGHT as _,
                self.maze.width() as _,
                self.maze.height() as _
            )
        );

        true
    }

    fn draw(&mut self, ctx: &mut Self::Ctx, _current_fps: usize) {
        let canvas = ctx.canvas();

        if let Err(e) = self.graphics.draw(canvas, &self.maze, &self.ui) {
            eprintln!("Failed to draw app: {}", e);
        }

        canvas.present();
    }
}
