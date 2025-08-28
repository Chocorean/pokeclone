use bevy::prelude::*;

use crate::character_manual::{
    animation::{AnimationConfig, trigger_animation},
    player::PlayerBundle,
};

pub mod animation;
pub mod player;

#[derive(Component)]
pub enum CharacterSprite {
    Left,
    Right,
    Down,
    Up,
}

pub fn setup_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the sprite sheet using the `AssetServer`
    let texture = asset_server.load("textures/player.png");

    // The sprite sheet has 9 sprites arranged in 3 rows and 3 cols, and they are all 14px x 21px
    let layout = TextureAtlasLayout::from_grid(UVec2::new(14, 21), 3, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // The all sprites run at 10 FPS for now
    let animation_config_down = AnimationConfig::new(0, 2, 10);
    let animation_config_up = AnimationConfig::new(3, 5, 10);
    let animation_config_side = AnimationConfig::new(6, 8, 10);

    // Spawn the sprite facing down
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config_down.first_sprite_index,
            }),
            ..default()
        },
        Transform::from_xyz(8.0, 11.0, 10.0),
        CharacterSprite::Down,
        // PlayerBundle {},
        animation_config_down,
    ));
    // Spawn the other sprites, hidden by default
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config_up.first_sprite_index,
            }),
            ..default()
        },
        Visibility::Hidden,
        Transform::from_xyz(8.0, 11.0, 10.0),
        CharacterSprite::Up,
        // PlayerBundle {},
        animation_config_up,
    ));
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config_side.first_sprite_index,
            }),
            ..default()
        },
        Visibility::Hidden,
        Transform::from_xyz(8.0, 11.0, 10.0),
        CharacterSprite::Left,
        // PlayerBundle {},
        animation_config_side.clone(),
    ));
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config_side.first_sprite_index,
            }),
            flip_x: true,
            ..default()
        },
        Visibility::Hidden,
        Transform::from_xyz(8.0, 11.0, 10.0),
        CharacterSprite::Right,
        // PlayerBundle {},
        animation_config_side,
    ));
}

pub fn handle_character_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (&CharacterSprite, &mut Visibility, &mut AnimationConfig),
        With<CharacterSprite>,
    >,
) {
    for (sprite, mut visibility, mut config) in &mut query {
        if keyboard_input.just_pressed(KeyCode::KeyW) {
            match sprite {
                CharacterSprite::Up => {
                    // Switch to the UpSprite
                    *visibility = Visibility::Visible;
                    trigger_animation(&mut config);
                }
                _ => {
                    // Hide if not UpSprite
                    *visibility = Visibility::Hidden;
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::KeyS) {
            match sprite {
                CharacterSprite::Down => {
                    // Switch to the UpSprite
                    *visibility = Visibility::Visible;
                    trigger_animation(&mut config);
                }
                _ => {
                    // Hide if not UpSprite
                    *visibility = Visibility::Hidden;
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::KeyA) {
            match sprite {
                CharacterSprite::Left => {
                    // Switch to the UpSprite
                    *visibility = Visibility::Visible;
                    trigger_animation(&mut config);
                }
                _ => {
                    // Hide if not UpSprite
                    *visibility = Visibility::Hidden;
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::KeyD) {
            match sprite {
                CharacterSprite::Right => {
                    // Switch to the UpSprite
                    *visibility = Visibility::Visible;
                    trigger_animation(&mut config);
                }
                _ => {
                    // Hide if not UpSprite
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}
