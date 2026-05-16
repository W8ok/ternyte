use crate::{components::*, input::Input};
use sdl3::event::Event;

pub struct SDLEvent {
    event_pump: sdl3::EventPump,
}

impl SDLEvent {
    pub fn new(sdl: &sdl3::Sdl) -> Self {
        Self {
            event_pump: sdl.event_pump().unwrap(),
        }
    }

    #[inline]
    pub fn handle(&mut self, input: &mut Input, window: &mut Size<u32>) -> bool {
        for sdl_event in self.event_pump.poll_iter() {
            match sdl_event {
                Event::Quit { .. } => {
                    return false;
                }

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    input.press_key(keycode);
                }

                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    input.release_key(keycode);
                }

                Event::MouseMotion { x, y, .. } => {
                    input.set_mouse_pos(Position::new(x, y));
                }

                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    input.click_mouse(mouse_btn);
                    input.set_mouse_pos(Position::new(x, y));
                }

                Event::MouseButtonUp {
                    mouse_btn, x, y, ..
                } => {
                    input.release_mouse(mouse_btn);
                    input.set_mouse_pos(Position::new(x, y));
                }

                Event::Window {
                    win_event: sdl3::event::WindowEvent::Resized(w, h),
                    ..
                } => *window = Size::new(w as u32, h as u32),

                _ => {}
            }
        }

        return true;
    }
}
