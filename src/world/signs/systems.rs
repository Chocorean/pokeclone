use bevy::prelude::*;
use bevy_easy_gif::prelude::{Gif, GifAsset};
use bevy_ecs_ldtk::{EntityInstance, GridCoords};

use crate::{
    player::Player,
    utils::{Direction, read_str_from_ldtk_entity},
    world::signs::components::Sign,
};

pub fn handle_player_interaction_with_sign(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_q: Query<(&GridCoords, &Direction), With<Player>>,
    sign_q: Query<(&GridCoords, &EntityInstance), With<Sign>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        let (player_grid_coords, direction) = player_q.single().unwrap();
        let facing_coords = direction.next_coords(*player_grid_coords);
        for (sign_coords, sign) in sign_q.iter() {
            if sign_coords == &facing_coords {
                // Access custom fields by name
                let msg = read_str_from_ldtk_entity("chat", sign);
                println!("{msg}");
            }
        }
    }
}

pub fn add_shiny_to_sign(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    sign_q: Query<Entity, Added<Sign>>,
) {
    let handle: Handle<GifAsset> = asset_server.load("textures/animations/shiny.gif");
    for entity in sign_q {
        let gif = commands
            .spawn(Gif {
                handle: handle.clone(),
            })
            .id();
        commands.entity(entity).add_child(gif);
    }
}
