use bevy::prelude::*;
use bevy_easy_gif::GifNode;

use crate::{dex::Dex, team::TeamMember, ui::widgets::slider};

#[derive(Component)]
#[allow(dead_code)]
pub struct TeamMemberWidget(TeamMember);

const BACKGROUND_COLOR: Color = Color::linear_rgb(0.32156863, 0.36078432, 0.51568628);

pub fn team_member_widget(
    mbr: TeamMember,
    index: usize,
    font: Handle<Font>,
    dex: &Dex,
) -> impl Bundle {
    let handle = mbr.handle(&dex);
    (
        TeamMemberWidget(mbr.clone()),
        Node {
            width: Val::Percent(100.),
            height: Val::Px(64.),
            display: Display::Flex,
            margin: UiRect::new(Val::Px(10.), Val::Px(0.), Val::Px(5.), Val::Px(5.)),
            border: UiRect::all(Val::Px(1.)),
            padding: UiRect::right(Val::Px(15.)),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            ..default()
        },
        BackgroundColor(BACKGROUND_COLOR),
        BorderColor(BACKGROUND_COLOR),
        BorderRadius::left(Val::Px(10.)),
        children![
            (
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    // row_gap: Val::Px(12.0),
                    ..default()
                },
                children![
                    (
                        Text::new(mbr.name(&dex)),
                        TextFont {
                            font: font.clone(),
                            font_size: 12.,
                            ..default()
                        }
                    ),
                    slider(0, mbr.max_hp(&dex), mbr.hp, index, font)
                ],
            ),
            GifNode { handle }
        ],
    )
}
