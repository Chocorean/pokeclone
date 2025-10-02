use bevy::prelude::*;
use webbrowser;

const HYPERLINK_COLOR: Color = Color::srgb_u8(51, 153, 255);

#[derive(Component)]
pub struct Hyperlink(String);

pub fn hyperlink(label: &str, link: &str, font: Handle<Font>) -> impl Bundle {
    (
        Button,
        Node {
            border: UiRect::bottom(Val::Px(2.)),
            ..default()
        },
        children![(
            Text::new(label.to_string()),
            TextFont {
                font: font.clone(),
                ..default()
            },
            TextColor(HYPERLINK_COLOR),
        )],
        BorderColor(Color::NONE),
        Hyperlink(link.to_string()),
    )
}

/// System showing underline below hovered hyperlinks and opening links
pub fn handle_hyperlinks(
    interactions: Query<
        (&Interaction, &mut BorderColor, &Hyperlink),
        (Changed<Interaction>, With<Hyperlink>),
    >,
) {
    for (interaction, mut bc, hlink) in interactions {
        match *interaction {
            Interaction::Hovered => {
                bc.0 = HYPERLINK_COLOR;
            }
            Interaction::Pressed => {
                webbrowser::open(hlink.0.as_str()).expect("failed to open browser");
            }
            Interaction::None => {
                bc.0 = Color::NONE;
            }
        }
    }
}
