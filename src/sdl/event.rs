use sdl3_sys::everything::*;
use std::{ffi::CString, ptr};

pub struct SdlEvent {}

impl SdlEvent {
    pub fn new() -> Self {
        Self {}
    }

    pub fn poll(&self) -> Vec<Event> {
        let mut events = Vec::new();

        unsafe {
            let mut sdl_event: SDL_Event = std::mem::zeroed();
            while SDL_PollEvent(&mut sdl_event) {
                if let Some(event) = Event::from_sdl_event(sdl_event) {
                    events.push(event);
                }
            }
        }

        return events;
    }
}

pub enum Event {
    Quit,
    MouseWheel { x: f32, y: f32, wheel: f32 },
    MouseMotion { x: f32, y: f32 },
    MouseButtonDown { x: f32, y: f32, button: MouseButton },
    MouseButtonUp { x: f32, y: f32, button: MouseButton },
    KeyDown { key: Key },
    KeyUp { key: Key },
}

impl Event {
    fn from_sdl_event(event: SDL_Event) -> Option<Self> {
        match unsafe { event.event_type() } {
            SDL_EVENT_QUIT => Some(Event::Quit),
            SDL_EVENT_MOUSE_WHEEL => unsafe {
                Some(Event::MouseWheel {
                    x: event.wheel.mouse_x,
                    y: event.wheel.mouse_y,
                    wheel: event.wheel.y,
                })
            },
            SDL_EVENT_MOUSE_MOTION => unsafe {
                Some(Event::MouseMotion {
                    x: event.motion.x,
                    y: event.motion.y,
                })
            },
            SDL_EVENT_MOUSE_BUTTON_DOWN => unsafe {
                Some(Event::MouseButtonDown {
                    x: event.button.x,
                    y: event.button.y,
                    button: event.button.button.into(),
                })
            },
            SDL_EVENT_MOUSE_BUTTON_UP => unsafe {
                Some(Event::MouseButtonUp {
                    x: event.button.x,
                    y: event.button.y,
                    button: event.button.button.into(),
                })
            },
            SDL_EVENT_KEY_DOWN => unsafe {
                Some(Event::KeyDown {
                    key: event.key.key.into(),
                })
            },
            SDL_EVENT_KEY_UP => unsafe {
                Some(Event::KeyUp {
                    key: event.key.key.into(),
                })
            },
            _ => None,
        }
    }
}

#[derive(PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Unknown(u8),
}

impl From<u8> for MouseButton {
    fn from(button: u8) -> Self {
        match button as i32 {
            SDL_BUTTON_LEFT => MouseButton::Left,
            SDL_BUTTON_MIDDLE => MouseButton::Middle,
            SDL_BUTTON_RIGHT => MouseButton::Right,
            _ => MouseButton::Unknown(button),
        }
    }
}

#[derive(PartialEq)]
pub enum Key {
    Space,
    Escape,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Up,
    Down,
    Left,
    Right,
    Unknown(SDL_Keycode),
}

impl From<SDL_Keycode> for Key {
    fn from(keycode: SDL_Keycode) -> Self {
        match keycode {
            SDLK_SPACE => Key::Space,
            SDLK_ESCAPE => Key::Escape,

            SDLK_A => Key::A,
            SDLK_B => Key::B,
            SDLK_C => Key::C,
            SDLK_D => Key::D,
            SDLK_E => Key::E,
            SDLK_F => Key::F,
            SDLK_G => Key::G,
            SDLK_H => Key::H,
            SDLK_I => Key::I,
            SDLK_J => Key::J,
            SDLK_K => Key::K,
            SDLK_L => Key::L,
            SDLK_M => Key::M,
            SDLK_N => Key::N,
            SDLK_O => Key::O,
            SDLK_P => Key::P,
            SDLK_Q => Key::Q,
            SDLK_R => Key::R,
            SDLK_S => Key::S,
            SDLK_T => Key::T,
            SDLK_U => Key::U,
            SDLK_V => Key::V,
            SDLK_W => Key::W,
            SDLK_X => Key::X,
            SDLK_Y => Key::Y,
            SDLK_Z => Key::Z,

            SDLK_0 => Key::Num0,
            SDLK_1 => Key::Num1,
            SDLK_2 => Key::Num2,
            SDLK_3 => Key::Num3,
            SDLK_4 => Key::Num4,
            SDLK_5 => Key::Num5,
            SDLK_6 => Key::Num6,
            SDLK_7 => Key::Num7,
            SDLK_8 => Key::Num8,
            SDLK_9 => Key::Num9,

            // Arrows
            SDLK_UP => Key::Up,
            SDLK_DOWN => Key::Down,
            SDLK_LEFT => Key::Left,
            SDLK_RIGHT => Key::Right,

            k => Key::Unknown(k),
        }
    }
}
