use rel_core::*;

use super::systems::*;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(self, app: &mut App) {
        app.add_systems(Schedule::Startup, spawn)
            .add_systems(Schedule::Cleanup, despawn)
            .add_systems(Schedule::Update, editor);
    }
}
