use super::GRID_SIZE;
use crate::{
    components::{base::*, gate::*, ui::*},
    input,
    sdl::{
        Sdl,
        camera::Camera,
        event::{Key, MouseButton},
        types::*,
    },
};
use hecs::*;

#[inline]
pub fn snap_to_grid(pos: Position) -> Position {
    Position {
        x: (pos.x / GRID_SIZE).floor() * GRID_SIZE,
        y: (pos.y / GRID_SIZE).floor() * GRID_SIZE,
    }
}

#[inline]
pub fn selected(world: &World) -> Vec<Entity> {
    world.query::<Entity>().with::<&Selected>().iter().collect()
}

#[inline]
pub fn change_tool(world: &mut World, new_tool: Tool) {
    if let Some(tool) = world
        .query_mut::<&mut Tool>()
        .with::<&Resource>()
        .into_iter()
        .next()
    {
        *tool = new_tool;
    }
}
