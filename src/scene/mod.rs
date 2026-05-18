use crate::sdl::{Sdl, types::*};
use crate::ui::{button::ButtonAction, *};
mod editor;
mod main_menu;

pub enum SceneSelect {
    MainMenu,
    Editor,
}

pub struct Scene {
    current: SceneSelect,
    scenes: Vec<SceneSelect>,
}

impl Scene {
    pub fn new(current: SceneSelect) -> Self {
        Self {
            current,
            scenes: Vec::new(),
        }
    }

    pub fn interract(&self, sdl: &mut Sdl, ui: &mut Ui) -> bool {
        return match self.current {
            SceneSelect::Editor => editor::interract(sdl, ui),
            SceneSelect::MainMenu => main_menu::interract(ui),
        };
    }

    pub fn render(&self, sdl: &mut Sdl, ui: &mut Ui) {
        match self.current {
            SceneSelect::Editor => editor::render(sdl),
            SceneSelect::MainMenu => main_menu::render(sdl),
        }
        ui.render(sdl);
    }

    pub fn create_new_scene(scene: SceneSelect) {}
}
