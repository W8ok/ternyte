use rel_core::*;

use super::components::*;
use super::systems::*;

pub struct GatesPlugin;

impl Plugin for GatesPlugin {
    fn build(self, app: &mut App) {
        app.event::<GateKind>().add_systems(spawn_gate);
        app.state(crate::ui::UiState::Editor)
            .add_systems(Schedule::PreUpdate, place_gates);
    }
}
