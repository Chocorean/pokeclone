use bevy::prelude::*;
use webbrowser;

const HYPERLINK_COLOR: Color = Color::srgb_u8(51, 153, 255);

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
        // BorderColor(Color::linear_rgb(0.2, 0.2, 0.2)),
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
