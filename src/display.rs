pub use raylib::prelude::*;

pub struct NesDisplay;

impl NesDisplay {
    pub fn set_options(handle: &mut RaylibHandle, fps: u32, fullscreen: bool) {
        handle.set_target_fps(fps);
        if fullscreen {
            handle.toggle_fullscreen();
        }
    }
    
    pub fn set_font(handle: &mut RaylibHandle, thread: &RaylibThread, path: &str, size: i32) -> Font {
        handle.load_font_ex(thread, path, size, None).expect("Could not load font")
    }
}
