use crate::input::*;
use crate::sdl::{Sdl, event::*, types::*};

pub fn handle(sdl: &mut Sdl) -> bool {
    let events = sdl.event.poll();
    for event in events {
        match event {
            Event::Quit => return false,
            Event::MouseMotion { x, y } => {
                MOUSE_X.with(|mx| *mx.borrow_mut() = x);
                MOUSE_Y.with(|my| *my.borrow_mut() = y);
            }
            Event::MouseButtonDown { x, y, button } => {
                MOUSE_BUTTONS.with(|b| {
                    if !b.borrow().contains(&button) {
                        b.borrow_mut().push(button);
                    }
                });
            }
            Event::MouseButtonUp { x, y, button } => {
                MOUSE_BUTTONS.with(|b| {
                    b.borrow_mut().retain(|b| *b != button);
                });
            }
            Event::MouseWheel { x, y, wheel } => {
                const ZOOM_SPEED: f32 = 0.1;
                sdl.camera.zoom_towards(x, y, wheel * ZOOM_SPEED);
            }
            Event::KeyDown { key } => {
                KEYS.with(|k| {
                    if !k.borrow().contains(&key) {
                        k.borrow_mut().push(key);
                    }
                });
            }
            Event::KeyUp { key } => {
                KEYS.with(|k| {
                    k.borrow_mut().retain(|k| *k != key);
                });
            }
            _ => {}
        }
    }
    return true;
}
