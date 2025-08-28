use bevy::prelude::*;

use crate::ui::game::WorldUI;

pub fn fight_ui(mut commands: Commands, ui_q: Query<Entity, With<WorldUI>>) {
    if let Ok(ui) = ui_q.single() {
        commands.entity(ui).with_children(|parent| {
            // parent.spawn((Node {}));
        });
    }
}
