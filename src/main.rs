mod app;
mod color_blend;
mod context;
mod graphics;
mod maze;
mod point;
mod stable_loop;
mod str_err;
mod ui;

use sdl2::pixels::Color;

use crate::app::App;
use crate::context::Context;
use crate::stable_loop::StableLoop;
use crate::str_err::Result;

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

    let ctx_ref;
    let ttf_ctx_ref;

    #[cfg(not(target_os = "emscripten"))]
    {
        ctx_ref = &mut ctx;
        ttf_ctx_ref = &ttf_ctx;
    }

    #[cfg(target_os = "emscripten")]
    {
        ctx_ref = Box::leak(Box::new(ctx));
        ttf_ctx_ref = Box::leak(Box::new(ttf_ctx));
    }

    let mut app = App::new(MAZE_WIDTH, MAZE_HEIGHT, ttf_ctx_ref, &ctx_ref.canvas())?;

    app.main_loop(ctx_ref);

    Ok(())
}
