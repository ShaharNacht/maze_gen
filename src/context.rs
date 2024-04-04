use std::error::Error;
use std::fmt::{self, Display};

use sdl2::render::WindowCanvas;
use sdl2::ttf::{self, Sdl2TtfContext};
use sdl2::video::WindowBuildError;
use sdl2::{EventPump, IntegerOrSdlError, Sdl, VideoSubsystem};

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
    ) -> Result<(Self, Sdl2TtfContext), InitError> {
        sdl2::hint::set("SDL_WINDOWS_DPI_AWARENESS", "permonitorv2");

        let sdl_ctx = sdl2::init().sdl_init_error()?;

        let ttf_ctx = sdl2::ttf::init()?;

        let video_subsystem = sdl_ctx.video().video_subsystem_error()?;

        let window = video_subsystem
            .window(window_title, window_width, window_height)
            .position_centered()
            .build()?;

        let canvas = window.into_canvas().accelerated().build().canvas_error()?;

        let event_pump = sdl_ctx.event_pump().event_pump_error()?;

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

#[derive(Debug)]
pub enum InitError {
    SdlInit(String),
    TtfInit(ttf::InitError),
    VideoSubsystem(String),
    Window(WindowBuildError),
    Canvas(IntegerOrSdlError),
    EventPump(String),
}

impl Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SdlInit(e) => write!(f, "SDL init error: {}", e),
            Self::TtfInit(e) => write!(f, "SDL TTF init error: {}", e),
            Self::VideoSubsystem(e) => write!(f, "Video subsystem creation error: {}", e),
            Self::Window(e) => write!(f, "Window creation error: {}", e),
            Self::Canvas(e) => write!(f, "Canvas creation error: {}", e),
            Self::EventPump(e) => write!(f, "Event pump creation error: {}", e),
        }
    }
}

impl Error for InitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::TtfInit(e) => Some(e),
            Self::Window(e) => Some(e),
            Self::Canvas(e) => Some(e),

            Self::SdlInit(_) | Self::VideoSubsystem(_) | Self::EventPump(_) => None,
        }
    }
}

impl From<ttf::InitError> for InitError {
    fn from(value: ttf::InitError) -> Self {
        Self::TtfInit(value)
    }
}

impl From<WindowBuildError> for InitError {
    fn from(value: WindowBuildError) -> Self {
        Self::Window(value)
    }
}

trait StringErrorToInitError<T> {
    fn sdl_init_error(self) -> Result<T, InitError>;

    fn video_subsystem_error(self) -> Result<T, InitError>;

    fn event_pump_error(self) -> Result<T, InitError>;
}

impl<T> StringErrorToInitError<T> for Result<T, String> {
    fn sdl_init_error(self) -> Result<T, InitError> {
        self.map_err(|e| InitError::SdlInit(e))
    }

    fn video_subsystem_error(self) -> Result<T, InitError> {
        self.map_err(|e| InitError::VideoSubsystem(e))
    }

    fn event_pump_error(self) -> Result<T, InitError> {
        self.map_err(|e| InitError::EventPump(e))
    }
}

trait CanvasErrorToInitError<T> {
    fn canvas_error(self) -> Result<T, InitError>;
}

impl<T> CanvasErrorToInitError<T> for Result<T, IntegerOrSdlError> {
    fn canvas_error(self) -> Result<T, InitError> {
        self.map_err(|e| InitError::Canvas(e))
    }
}
