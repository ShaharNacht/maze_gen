use std::thread;
use std::time::{Duration, Instant};

pub trait StableLoop {
    fn update(&mut self, current_fps: usize) -> bool;

    fn draw(&mut self, current_fps: usize);

    fn target_fps(&self) -> f64 {
        60.0
    }

    fn max_updates_per_frame(&self) -> u32 {
        8
    }

    fn wait(&self, duration: Duration) {
        #[cfg(not(target_os = "emscripten"))]
        thread::sleep(duration);
    }

    #[cfg(not(target_os = "emscripten"))]
    /// Don't override this function
    fn main_loop(&mut self) {
        let mut state = StableLoopState::new();

        while state.single_iteration(self) {}
    }

    #[cfg(target_os = "emscripten")]
    /// Don't override this function
    fn main_loop(mut self)
    where
        Self: 'static + Sized,
    {
        let dpr = emscripten_functions::emscripten::get_device_pixel_ratio();

        emscripten_functions::emscripten::run_script(format!(
            r##"
					let canvas = document.getElementById("canvas");
					canvas.style.width = canvas.width / {0} + "px";
					canvas.style.height = canvas.height / {0} + "px";
				"##,
            dpr
        ));

        let mut state = StableLoopState::new();

        emscripten_functions::emscripten::set_main_loop(
            move || {
                state.single_iteration(&mut self);
            },
            0,
            true,
        );
    }
}

struct StableLoopState {
    time: Time,

    prev_time: Duration,
    prev_second: Duration,
    time_accumulator: Duration,

    current_fps: usize,
    fps_counter: usize,
}

impl StableLoopState {
    fn new() -> Self {
        let time = Time::new();

        let prev_time = time.time_since_start();
        let prev_second = prev_time;
        let time_accumulator = Duration::ZERO;

        let current_fps = 0;
        let fps_counter = 0;

        Self {
            time,
            prev_time,
            prev_second,
            time_accumulator,
            current_fps,
            fps_counter,
        }
    }

    fn single_iteration<T: StableLoop + ?Sized>(&mut self, app: &mut T) -> bool {
        let now = self.time.time_since_start();
        self.time_accumulator += now - self.prev_time;

        let target_frame_time: Duration = Duration::from_secs(1).div_f64(app.target_fps());
        let mut keep_going = true;
        let mut needs_draw = false;

        while self.time_accumulator >= target_frame_time {
            if app.update(self.current_fps) {
                needs_draw = true;

                if self.time_accumulator > target_frame_time * app.max_updates_per_frame() {
                    self.time_accumulator = Duration::ZERO;
                } else {
                    self.time_accumulator -= target_frame_time;
                }
            } else {
                keep_going = false;
                break;
            }
        }

        if now >= self.prev_second + Duration::from_secs(1) {
            self.current_fps = self.fps_counter;
            self.fps_counter = 0;

            while now >= self.prev_second + Duration::from_secs(1) {
                self.prev_second += Duration::from_secs(1);
            }
        }

        if keep_going && needs_draw {
            app.draw(self.current_fps);

            self.fps_counter += 1;
        }

        self.prev_time = now;

        if keep_going {
            app.wait(target_frame_time - self.time_accumulator);
        }

        keep_going
    }
}

struct Time {
    #[cfg(not(target_os = "emscripten"))]
    start_time: Instant,

    #[cfg(target_os = "emscripten")]
    start_time: f64,
}

impl Time {
    fn new() -> Self {
        #[cfg(not(target_os = "emscripten"))]
        let start_time = Instant::now();

        #[cfg(target_os = "emscripten")]
        let start_time = emscripten_functions::emscripten::get_now();

        Self { start_time }
    }

    fn time_since_start(&self) -> Duration {
        #[cfg(not(target_os = "emscripten"))]
        return self.start_time.elapsed();

        #[cfg(target_os = "emscripten")]
        return Duration::from_secs_f64(
            (emscripten_functions::emscripten::get_now() - self.start_time) / 1000.0,
        );
    }
}
