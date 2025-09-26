use bevy::prelude::*;

mod goals;
mod herbs;
mod npcs;
mod signs;
mod walls;

use bevy_ecs_ldtk::GridCoords;
pub(crate) use herbs::LevelHerbs;
pub(crate) use npcs::{LevelNPCs, NPCKind};
pub(crate) use walls::LevelWalls;

use crate::{
    AppState,
    player::Player,
    save::Save,
    team::Team,
    world::{
        goals::GoalsPlugin, herbs::HerbsPlugin, npcs::NPCsPlugin, signs::SignsPlugin,
        walls::WallsPlugin,
    },
};

#[derive(Resource)]
pub struct GridSize(pub i32);
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GridSize(16))
            .add_plugins(GoalsPlugin)
            .add_plugins(HerbsPlugin)
            .add_plugins(NPCsPlugin)
            .add_plugins(SignsPlugin)
            .add_plugins(WallsPlugin)
            .add_systems(OnEnter(AppState::ResumeGame), load_game)
            .add_systems(
                OnTransition {
                    // `init_team` loads an empty team, so it shall only be called when starting a new game.
                    exited: AppState::MainMenu,
                    entered: AppState::InGame,
                },
                init_team,
            )
            .add_systems(Update, (apply_save).run_if(in_state(AppState::InGame)))
            // Some systems still run in `AppState::InFight` state
            .add_systems(
                Update,
                translate_grid_coords_entities
                    .run_if(in_state(AppState::InFight).or(in_state(AppState::InGame))),
            )
            .add_systems(
                // When we leave the game
                // It's not an OnExit because we might leave this state when entering in combat or something
                OnTransition {
                    exited: AppState::InGame,
                    entered: AppState::MainMenu,
                },
                clean_up_world,
            );
    }
}

/// Load the save as a Resource.
fn load_game(mut commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
    let save = Save::load().unwrap();
    commands.insert_resource(save.team.clone());
    commands.insert_resource(save);
    next_state.set(AppState::InGame);
}

/// Apply the content of the save to the world.
/// It runs only once because it removes the Save from the resources after loading it,
/// and does not run if it cannot find a Save resource.
fn apply_save(
    mut commands: Commands,
    mut player_q: Query<&mut GridCoords, With<Player>>,
    save_res: Option<Res<Save>>,
) {
    if let Some(save) = save_res
        && let Ok(mut player_coords) = player_q.single_mut()
    {
        *player_coords = GridCoords::new(save.coords.0, save.coords.1);
        commands.remove_resource::<Save>();
    }
}

fn init_team(mut commands: Commands) {
    commands.insert_resource(Team::new());
}

/// Despawn the world and its camera.
fn clean_up_world(
    mut commands: Commands,
    world_q: Single<Entity, With<crate::camera::WorldBundle>>,
    cam_q: Single<Entity, With<crate::camera::WorldCamera>>,
) {
    commands.entity(*world_q).despawn();
    commands.entity(*cam_q).despawn();
}

/// Move everything accordingly to the player's movement.
fn translate_grid_coords_entities(
    grid_size: Res<GridSize>,
    mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation = bevy_ecs_ldtk::utils::grid_coords_to_translation(
            *grid_coords,
            IVec2::splat(grid_size.0),
        )
        .extend(transform.translation.z);
    }
}
