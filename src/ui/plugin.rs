use rel_core::*;

use super::components::*;
use super::editor::EditorPlugin;
use super::menu::MenuPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(self, app: &mut App) {
        app.init_state::<UiState>();
        app.state(UiState::Editor).add_plugins(EditorPlugin);
        app.state(UiState::Menu).add_plugins(MenuPlugin);
    }
}
