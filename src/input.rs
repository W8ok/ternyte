use crate::components::*;
use glam::Vec2;
use raylib_sys::*;

// ========================================================
// KEYBOARD
// ========================================================
pub use raylib_sys::KeyboardKey as Key;

pub fn is_key_pressed(key: Key) -> bool {
    unsafe { IsKeyPressed(key as i32) }
}

pub fn is_key_released(key: Key) -> bool {
    unsafe { IsKeyReleased(key as i32) }
}

pub fn is_key_down(key: Key) -> bool {
    unsafe { IsKeyDown(key as i32) }
}

pub fn is_key_up(key: Key) -> bool {
    unsafe { IsKeyUp(key as i32) }
}

// ========================================================
// MOUSE
// ========================================================
pub use raylib_sys::MouseButton as Mouse;

pub fn get_mouse_coord() -> Coordinate {
    unsafe { Coordinate::new(GetMouseX(), GetMouseY()) }
}

pub fn get_mouse_pos() -> Vec2 {
    let pos = unsafe { GetMousePosition() };
    Vec2 { x: pos.x, y: pos.y }
}

pub fn is_mouse_pressed(button: Mouse) -> bool {
    unsafe { IsMouseButtonPressed(button as i32) }
}

pub fn is_mouse_released(button: Mouse) -> bool {
    unsafe { IsMouseButtonReleased(button as i32) }
}

pub fn is_mouse_down(button: Mouse) -> bool {
    unsafe { IsMouseButtonDown(button as i32) }
}

pub fn is_mouse_up(button: Mouse) -> bool {
    unsafe { IsMouseButtonUp(button as i32) }
}

pub fn get_mouse_wheel_move() -> f32 {
    unsafe { GetMouseWheelMove() }
}

pub fn get_mouse_delta() -> Vec2 {
    let delta = unsafe { GetMouseDelta() };
    Vec2 {
        x: delta.x,
        y: delta.y,
    }
}

// ========================================================
// WINDOW
// ========================================================
pub fn get_screen_size() -> Coordinate {
    Coordinate {
        x: unsafe { GetScreenWidth() },
        y: unsafe { GetScreenHeight() },
    }
}

pub fn is_window_resized() -> bool {
    unsafe { IsWindowResized() }
}

pub fn screen_to_world(coord: Coordinate, camera: &crate::render::Camera) -> Coordinate {
    let cam_coord = Coordinate::new(camera.target.x as i32, camera.target.y as i32);
    coord + cam_coord
}
