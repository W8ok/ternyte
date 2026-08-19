use rel_core::*;

use super::gates::GatesPlugin;
use super::systems;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(self, app: &mut App) {
        app.add_plugins(GatesPlugin);
    }
}
