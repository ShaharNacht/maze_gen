mod app;
mod color_blend;
mod color_hex;
mod context;
mod graphics;
mod layout;
mod maze;
mod point;
mod point_new;
mod stable_loop;
mod ui;

use sdl2::pixels::Color;

use crate::app::App;
use crate::color_hex::hex_rgb;
use crate::context::Context;
use crate::layout::LayoutConfig;
use crate::stable_loop::StableLoop;

const TARGET_FPS: f64 = 60.0;

const MAZE_WIDTH: i64 = 16;
const MAZE_HEIGHT: i64 = 16;

const LAYOUT_CONFIG: LayoutConfig = LayoutConfig {
    padding: 0.05,
    maze_width: 1.0,
    maze_height: 1.0,
    maze_wall_thickness: 0.0,
};

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

const BACKGROUND_COLOR: Color = hex_rgb(0x28242E);
const WALL_COLOR: Color = hex_rgb(0xFFFBDE);
const CURSOR_COLOR: Color = hex_rgb(0xBD516D);
const VISITED_CELL_COLOR: Color = hex_rgb(0x6F9D81);
const UI_COLOR: Color = hex_rgb(0x344B68);
const UI_BUTTON_COLOR: Color = hex_rgb(0x5378A7);
const UI_BUTTON_HIGHLIGHT_COLOR: Color = hex_rgb(0x81C0C6);
const UI_BUTTON_CLICKED_COLOR: Color = hex_rgb(0x436187);
const UI_BUTTON_TEXT_COLOR: Color = BACKGROUND_COLOR;

fn main() {
    let (mut ctx, ttf_ctx) =
        Context::new("Maze Generator", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).unwrap();

    let ctx_ref;
    let ttf_ctx_ref;

    #[cfg(not(target_os = "emscripten"))]
    {
        ctx_ref = &mut ctx;
        ttf_ctx_ref = &ttf_ctx;
    }

    #[cfg(target_os = "emscripten")]
    {
        // emscripten's main loop requires everything to be 'static
        ctx_ref = Box::leak(Box::new(ctx));
        ttf_ctx_ref = Box::leak(Box::new(ttf_ctx));
    }

    let mut app = App::new(
        MAZE_WIDTH,
        MAZE_HEIGHT,
        LAYOUT_CONFIG,
        ttf_ctx_ref,
        ctx_ref.canvas(),
    )
    .unwrap();

    app.main_loop(ctx_ref);
}
