use raylib_sys::*;
use rel_core::*;
use std::ffi::CString;

use crate::components::*;

pub fn setup(app: &mut App) {
    unsafe { SetTraceLogLevel(TraceLogLevel::LOG_WARNING as i32) };
    unsafe {
        SetConfigFlags(
            ConfigFlags::FLAG_WINDOW_RESIZABLE as u32 | ConfigFlags::FLAG_VSYNC_HINT as u32,
        )
    };
    unsafe { InitWindow(1280, 720, CString::new("Ternyte").unwrap().as_ptr()) };
}

pub fn quit(app: &mut App) {
    if (unsafe { WindowShouldClose() }) {
        app.quit();
    }
}

pub fn cleanup(app: &mut App) {
    unsafe { CloseWindow() };
}
