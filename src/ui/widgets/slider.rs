use bevy::{
    color::palettes::{css::GRAY, tailwind::GRAY_700},
    prelude::*,
};

use crate::ui::widgets::lerp_color;

const LOW_COLOR: Color = Color::linear_rgb(1., 0., 0.);
const FULL_COLOR: Color = Color::linear_rgb(0.196, 0.804, 0.196);

#[derive(Component)]
pub struct Slider {
    min: u8,
    max: u8,
    value: u8,
}

#[derive(Component)]
pub struct SliderIndex(usize);

#[derive(Component)]
pub struct SliderFill;

pub fn slider(min: u8, max: u8, value: u8, index: usize, font: Handle<Font>) -> impl Bundle {
    let range = (max.max(min) - min) as f32;
    let p = if range > 0.0 {
        ((value as f32 - min as f32) / range).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let fill_color = lerp_color(FULL_COLOR, LOW_COLOR, 1.0 - p);
    (
        Visibility::Visible,
        Node {
            width: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            padding: UiRect::horizontal(Val::Px(5.)),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Text::new(format!("{value} ❤ ({max})")),
                TextFont {
                    font,
                    font_size: 8.,
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                // rounded hp bar
                Node {
                    width: Val::Percent(100.),
                    height: Val::Px(10.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Stretch,
                    // Keep the rounded ends and clip the fill
                    border: UiRect::all(Val::Px(1.)),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BorderRadius::all(Val::Px(5.)),
                BackgroundColor(GRAY_700.into()),
                BorderColor(GRAY.into()),
                children![
                    (
                        Node {
                            width: Val::Percent(p * 100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(fill_color),
                        BorderRadius::all(Val::Px(5.)),
                        SliderFill,
                    ),
                    // Spacer (right side) takes remaining space
                    (Node {
                        flex_grow: 1.0,
                        ..default()
                    },),
                ],
                // data
                Slider { min, max, value },
                SliderIndex(index),
            )
        ],
    )
}

// Update the bar when the Slider value changes
pub fn sync_slider_visuals(
    mut tracks: Query<(&Slider, &Children), Changed<Slider>>,
    mut node_q: Query<&mut Node>,
    mut fill_bg_q: Query<&mut BackgroundColor, With<SliderFill>>,
) {
    for (slider, children) in &mut tracks {
        // find the fill child
        for child in children.iter() {
            if let Ok(mut fill_node) = node_q.get_mut(child) {
                if fill_bg_q.get_mut(child).is_ok() {
                    let range = (slider.max.max(slider.min) - slider.min) as f32;
                    let p = if range > 0.0 {
                        ((slider.value as f32 - slider.min as f32) / range).clamp(0.0, 1.0)
                    } else {
                        0.0
                    };
                    fill_node.width = Val::Percent(p * 100.0);

                    if let Ok(mut bg) = fill_bg_q.get_mut(child) {
                        bg.0 = lerp_color(FULL_COLOR, LOW_COLOR, 1.0 - p);
                    }
                }
            }
        }
    }
}
