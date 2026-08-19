use rel_core::*;

use super::systems::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(self, app: &mut App) {
        app.add_systems(Schedule::Startup, spawn)
            .add_systems(Schedule::Cleanup, despawn)
            .add_systems(Schedule::Update, menu);
    }
}
