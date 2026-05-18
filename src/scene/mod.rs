use crate::input;
use crate::sdl::{
    Sdl,
    event::{Key, MouseButton},
    types::*,
};
use hecs::World;

pub mod editor;
pub mod menu;

#[derive(Clone, Copy)]
pub enum SceneSelect {
    MainMenu,
    Editor,
}

pub fn manager(sdl: &mut Sdl, world: &mut World) -> bool {
    let scene = world.query::<&SceneSelect>().iter().next().cloned();

    match scene {
        Some(SceneSelect::Editor) => {
            if !editor::interact(sdl, world) {
                return false;
            }

            editor::hover(world);
            editor::click(world);
            editor::render(sdl, world);
        }
        Some(SceneSelect::MainMenu) => {
            if !menu::interact(world) {
                return false;
            }

            menu::hover(world);
            menu::click(world);
            menu::render(sdl, world);
        }
        None => {}
    }

    return true;
}
