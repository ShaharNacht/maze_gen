use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::{EventPump, Sdl, VideoSubsystem};

use crate::str_err::{Result, StrErr};

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
