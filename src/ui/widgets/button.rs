use bevy::prelude::*;

/// Button was already taken.
#[derive(Component)]
pub struct CButton;

pub fn button(label: &str, font: Handle<Font>) -> impl Bundle {
    (
        Button,
        CButton,
        Node {
            border: UiRect::all(Val::Px(1.)),
            padding: UiRect::all(Val::Px(5.)),
            ..default()
        },
        children![(
            Text::new(label.to_string()),
            TextFont {
                font: font.clone(),
                ..default()
            },
        )],
        BackgroundColor(Color::linear_rgb(0.1, 0.1, 0.1)),
        BorderRadius::all(Val::Px(5.)),
    )
}

pub fn handle_buttons(
    buttons: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<CButton>),
    >,
) {
    for (interaction, mut ba_c, mut bo_c) in buttons {
        match *interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => {
                ba_c.0 = Color::linear_rgb(0.2, 0.2, 0.2);
                bo_c.0 = Color::linear_rgb(0.8, 0.8, 0.8);
            }
            Interaction::None => {
                ba_c.0 = Color::linear_rgb(0.1, 0.1, 0.1);
                bo_c.0 = Color::NONE;
            }
        }
    }
}
