use hecs::*;

use crate::components::base::*;
use crate::input;
use crate::input::*;
use crate::scene;
use crate::sdl::{Sdl, event::*, types::*};

pub fn handle(sdl: &mut Sdl, world: &mut World) -> bool {
    let events = sdl.event.poll();
    for event in events {
        match event {
            Event::Quit => return false,
            Event::WindowResize { width, height } => {
                scene::builder(sdl, world);
            }
            Event::MouseMotion { x, y } => {
                MOUSE_X.with(|mx| *mx.borrow_mut() = x);
                MOUSE_Y.with(|my| *my.borrow_mut() = y);

                let (x, y) = sdl.camera.screen_to_world(x, y);
                MOUSE_X_CAM.with(|mx| *mx.borrow_mut() = x);
                MOUSE_Y_CAM.with(|my| *my.borrow_mut() = y);
                input::update_mouse_delta();
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
