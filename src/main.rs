#![allow(unused)]
use rel_core::*;

mod components;
mod input;
mod render;

mod raylib;
use raylib::RaylibPlugin;
mod ui;
use ui::UiPlugin;
mod editor;
use editor::EditorPlugin;

fn main() {
    App::new()
        .add_plugins((RaylibPlugin, UiPlugin, EditorPlugin))
        .run();
}
