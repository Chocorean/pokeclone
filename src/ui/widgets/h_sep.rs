use bevy::prelude::*;

pub fn h_sep() -> impl Bundle {
    (
        Node {
            width: Val::Percent(80.0), // full width
            height: Val::Px(2.0),      // line thickness
            margin: UiRect::vertical(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.7, 0.7, 0.7)),
    )
}
