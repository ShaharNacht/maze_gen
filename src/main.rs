mod str_err;
mod point;
mod maze;
mod graphics;
mod ui;

use std::thread::sleep;
use std::time::Duration;

use point::WindowPoint;
use sdl2::pixels::Color;

use crate::str_err::{ Result, StrErr };
use crate::maze::Maze;
use crate::graphics::Graphics;
use crate::ui::Ui;

const FPS: u32 = 60;

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

const BACKGROUND_COLOR: Color = Color::RGB( 0x28, 0x24, 0x2E );
const WALL_COLOR: Color = Color::RGB( 0xFF, 0xFB, 0xDE );
const CURSOR_COLOR: Color = Color::RGB( 0xBD, 0x51, 0x6D );
const UI_COLOR: Color = Color::RGB( 0x34, 0x4B, 0x68 );
const UI_BUTTON_COLOR: Color = Color::RGB( 0x53, 0x78, 0xA7 );
const UI_BUTTON_HIGHLIGHT_COLOR: Color = Color::RGB( 0x81, 0xC0, 0xC6 );
const UI_BUTTON_CLICKED_COLOR: Color = Color::RGB( 0x43, 0x61, 0x87 );
const UI_BUTTON_TEXT_COLOR: Color = BACKGROUND_COLOR;

fn main() -> Result<()>
{
	let mut manager = Manager::new( "Maze Generator", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32 )?;
	
	let maze = Maze::new( MAZE_WIDTH, MAZE_HEIGHT );
	let mut ui = Ui::new();
	
	let ttf_ctx = sdl2::ttf::init().str_err()?;
	let graphics = Graphics::new( &ttf_ctx, &manager.canvas )?;
	
	manager.main_loop( | event_pump, canvas |
	{
		for event in event_pump.poll_iter()
		{
			use sdl2::event::Event::*;
			use sdl2::keyboard::Scancode;
			
			match event
			{
				Quit {..} |
				KeyUp { scancode: Some(Scancode::Escape), .. } =>
				{
					return Ok(false);
				}
				
				_ => {}
			}
		}
		
		let mouse_state = event_pump.mouse_state();
		let mouse = WindowPoint::new( mouse_state.x() as i64, mouse_state.y() as i64 );
		let mouse_pressed = mouse_state.is_mouse_button_pressed(sdl2::mouse::MouseButton::Left);
		ui.update( mouse, mouse_pressed );
		
		graphics.draw( canvas, &maze, &ui )?;
		canvas.present();
		
		sleep( Duration::from_secs(1) / FPS );
		
		Ok(true)
	} )?;
	
	Ok(())
}

pub struct Manager
{
	sdl_ctx: sdl2::Sdl,
	video_subsystem: sdl2::VideoSubsystem,
	canvas: sdl2::render::WindowCanvas
}

impl Manager
{
	pub fn new( window_title: &str, window_width: u32, window_height: u32 ) -> Result<Self>
	{
		sdl2::hint::set( "SDL_WINDOWS_DPI_AWARENESS", "permonitorv2" );
		
		let sdl_ctx = sdl2::init()?;
		
		let video_subsystem = sdl_ctx.video()?;
		
		let window = video_subsystem.window( window_title, window_width, window_height )
			.position_centered()
			.build().str_err()?;
		
		let canvas = window.into_canvas()
			.accelerated()
			.build().str_err()?;
		
		Ok( Self { sdl_ctx, video_subsystem, canvas } )
	}
	
	pub fn window(&self) -> &sdl2::video::Window
	{
		self.canvas.window()
	}
	
	pub fn window_mut(&mut self) -> &mut sdl2::video::Window
	{
		self.canvas.window_mut()
	}
	
	pub fn main_loop<F>( &mut self, mut loop_func: F ) -> Result<()>
		where F: FnMut( &mut sdl2::EventPump, &mut sdl2::render::WindowCanvas ) -> Result<bool>
	{
		let mut event_pump = self.sdl_ctx.event_pump()?;
		let mut keep_running = true;
		
		while keep_running
		{
			match loop_func( &mut event_pump, &mut self.canvas )
			{
				Ok(result) => keep_running = result,
				Err(err) => return Err(err)
			}
		}
		
		Ok(())
	}
}