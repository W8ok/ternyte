use rel_core::*;

use super::systems::*;

pub struct RaylibPlugin;

impl Plugin for RaylibPlugin {
    fn build(self, app: &mut App) {
        app.add_systems(Schedule::Startup, setup);
        app.add_systems(Schedule::Update, quit);
        app.add_systems(Schedule::Cleanup, cleanup);
    }
}
