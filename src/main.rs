mod app;
mod color_blend;
mod graphics;
mod maze;
mod point;
mod stable_loop;
mod str_err;
mod ui;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::{EventPump, Sdl, VideoSubsystem};
use stable_loop::StableLoop;

use crate::app::App;
use crate::str_err::{Result, StrErr};

const TARGET_FPS: f64 = 60.0;

const MAZE_WIDTH: i64 = 16;
const MAZE_HEIGHT: i64 = 16;

const GFX_MAZE_X: i64 = 0;
const GFX_MAZE_Y: i64 = 0;
const GFX_MAZE_WIDTH: i64 = 768;
const GFX_MAZE_HEIGHT: i64 = 768;

const GFX_UI_X: i64 = 0;
const GFX_UI_Y: i64 = GFX_MAZE_HEIGHT;
const GFX_UI_WIDTH: i64 = GFX_MAZE_WIDTH;
const GFX_UI_HEIGHT: i64 = 96;
const GFX_UI_PADDING: i64 = 8;

const WINDOW_WIDTH: i64 = GFX_MAZE_WIDTH;
const WINDOW_HEIGHT: i64 = GFX_MAZE_HEIGHT + GFX_UI_HEIGHT;

const FONT: &str = "font/Cabin-Bold.ttf";
const FONT_SIZE: u16 = 32;

const BACKGROUND_COLOR: Color = Color::RGB(0x28, 0x24, 0x2E);
const WALL_COLOR: Color = Color::RGB(0xFF, 0xFB, 0xDE);
const CURSOR_COLOR: Color = Color::RGB(0xBD, 0x51, 0x6D);
const VISITED_CELL_COLOR: Color = Color::RGB(0x6F, 0x9D, 0x81);
const UI_COLOR: Color = Color::RGB(0x34, 0x4B, 0x68);
const UI_BUTTON_COLOR: Color = Color::RGB(0x53, 0x78, 0xA7);
const UI_BUTTON_HIGHLIGHT_COLOR: Color = Color::RGB(0x81, 0xC0, 0xC6);
const UI_BUTTON_CLICKED_COLOR: Color = Color::RGB(0x43, 0x61, 0x87);
const UI_BUTTON_TEXT_COLOR: Color = BACKGROUND_COLOR;

fn main() -> Result<()> {
    let (mut ctx, ttf_ctx) =
        Context::new("Maze Generator", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)?;

    let mut app = App::new(MAZE_WIDTH, MAZE_HEIGHT, &ttf_ctx, &ctx.canvas)?;

    app.main_loop(&mut ctx);

    Ok(())
}

pub struct Context {
    _sdl_ctx: Sdl,
    _video_subsystem: VideoSubsystem,
    canvas: WindowCanvas,
    event_pump: EventPump,
}

impl Context {
    pub fn new(
        window_title: &str,
        window_width: u32,
        window_height: u32,
    ) -> Result<(Self, Sdl2TtfContext)> {
        sdl2::hint::set("SDL_WINDOWS_DPI_AWARENESS", "permonitorv2");

        let sdl_ctx = sdl2::init()?;

        let ttf_ctx = sdl2::ttf::init().str_err()?;

        let video_subsystem = sdl_ctx.video()?;

        let window = video_subsystem
            .window(window_title, window_width, window_height)
            .position_centered()
            .build()
            .str_err()?;

        let canvas = window.into_canvas().accelerated().build().str_err()?;

        let event_pump = sdl_ctx.event_pump()?;

        Ok((
            Self {
                _sdl_ctx: sdl_ctx,
                _video_subsystem: video_subsystem,
                canvas,
                event_pump,
            },
            ttf_ctx,
        ))
    }

    pub fn event_pump(&mut self) -> &mut EventPump {
        &mut self.event_pump
    }

    pub fn canvas(&mut self) -> &mut WindowCanvas {
        &mut self.canvas
    }
}
