use crate::components::{base::*, ui::*};
use crate::input;
use crate::sdl::{
    Sdl,
    event::{Key, MouseButton},
    types::*,
};
use hecs::*;

pub mod editor;
pub mod menu;

#[derive(Clone, Copy)]
pub enum SceneSelect {
    Menu,
    Editor,
}

pub fn manager(sdl: &mut Sdl, world: &mut World) -> bool {
    let scene = world.query::<&SceneSelect>().iter().next().cloned();

    match scene {
        Some(SceneSelect::Editor) => {
            if !editor::interact(sdl, world) {
                return false;
            }

            editor::placement(world);
            editor::hover(world);
            editor::click(world);
            editor::render(sdl, world);
        }
        Some(SceneSelect::Menu) => {
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

pub fn builder(sdl: &mut Sdl, world: &mut World) {
    let to_despawn: Vec<Entity> = world.query::<(Entity)>().with::<&Ui>().iter().collect();

    for entity in to_despawn {
        world.despawn(entity);
    }

    editor::new(sdl, world);
    menu::new(sdl, world);
}
