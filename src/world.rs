use bevy::{platform::collections::HashSet, prelude::*};
use bevy_ecs_ldtk::prelude::*;

use crate::{
    appstate::AppState,
    player::{Direction, Player, PlayerBundle, move_player_from_input},
    save::Save,
    team::Team,
};

#[derive(Resource)]
pub struct GridSize(pub i32);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelWalls>()
            .init_resource::<LevelHerbs>()
            .init_resource::<LevelGoals>()
            .insert_resource(GridSize(16))
            .register_ldtk_int_cell_for_layer::<WallBundle>("Walls", 1)
            .register_ldtk_int_cell_for_layer::<HerbBundle>("Walls", 2)
            .register_ldtk_entity::<GoalBundle>("Goal")
            .register_ldtk_entity::<EntryBundle>("Entry")
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<NPCsBundle>("NPCs")
            .add_systems(OnEnter(AppState::ResumeGame), load_game)
            .add_systems(
                OnTransition {
                    // `init_team` loads an empty team, so it shall only be called when starting a new game.
                    exited: AppState::MainMenu,
                    entered: AppState::InGame,
                },
                init_team,
            )
            .add_systems(
                Update,
                (
                    apply_save,
                    cache_herb_locations,
                    cache_goal_locations,
                    cache_wall_locations,
                    translate_grid_coords_entities,
                    handle_player_interaction,
                    handle_through_goal,
                    move_player_from_input,
                )
                    .run_if(in_state(AppState::InGame)),
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
/// TODO: include the level number
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

// WALLS

#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Default, Resource)]
/// Store walls and NPCs locations for collision checking.
pub struct LevelWalls {
    wall_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

impl LevelWalls {
    pub fn in_wall(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.wall_locations.contains(grid_coords)
    }
}

/// todo: seperate wall and npcs
/// only run once per level load
/// for NPCs, update schedule instead
fn cache_wall_locations(
    mut level_walls: ResMut<LevelWalls>,
    mut level_events: EventReader<LevelEvent>,
    grid_size: Res<GridSize>,
    obstacles: Query<&GridCoords, With<Wall>>,
    npcs_q: Query<&GridCoords, With<NPC>>,
    ldtk_project_entities: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for level_event in level_events.read() {
        if let LevelEvent::SpawnTriggered(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities.single().unwrap())
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            let mut wall_locations: HashSet<GridCoords> = obstacles.iter().copied().collect();

            for npc_coords in npcs_q.iter() {
                wall_locations.insert(*npc_coords);
            }

            let new_level_walls = LevelWalls {
                wall_locations,
                level_width: level.px_wid / grid_size.0,
                level_height: level.px_hei / grid_size.0,
            };

            *level_walls = new_level_walls;
        }
    }
}

// HERBS

#[derive(Default, Component)]
pub struct Herb;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct HerbBundle {
    herb: Herb,
}

#[derive(Default, Resource)]
/// Store herbs locations for event trigger.
pub struct LevelHerbs {
    pub herb_locations: HashSet<GridCoords>,
}

/// Store herbs locations for easier retrieval.
fn cache_herb_locations(
    mut level_walls: ResMut<LevelHerbs>,
    herbs: Query<&GridCoords, With<Herb>>,
) {
    for herb_coords in herbs.iter() {
        level_walls.herb_locations.insert(*herb_coords);
    }
}

// NPCs

// npc trainer uuid
// ca7c1690-5e50-11f0-85ca-e96bd84a6222

#[derive(Default, Component, Debug)]
pub struct NPC;

#[derive(Default, Bundle, LdtkEntity)]
struct NPCsBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
    npc: NPC,
    #[grid_coords]
    grid_coords: GridCoords,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

/// Handle for players interacting with in-world entities, such as NPCs, signs, objects...
/// Might need some refactoring around reading the json values
pub fn handle_player_interaction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_q: Query<(&GridCoords, &Direction), With<Player>>,
    npc_q: Query<(&GridCoords, &EntityInstance), With<NPC>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        let (player_grid_coords, direction) = player_q.single().unwrap();
        let facing_coords = direction.next_coords(*player_grid_coords);
        for (npc_coords, npc) in npc_q.iter() {
            if npc_coords == &facing_coords {
                // Access custom fields by name
                let chat = npc
                    .field_instances
                    .iter()
                    .find(|f| f.identifier == "chat")
                    .unwrap()
                    .value
                    .clone();
                let chat = match chat {
                    FieldValue::String(s) => s.unwrap_or(String::from("...")),
                    _ => String::from("..."),
                };
                println!("NPC says: {}", chat);
            }
        }
    }
}

/// GOALS

// level 0 Goal
// f634c0c0-5e50-11f0-a81f-7d5d71ee8bd5
// level 1 goal
// 99a501a0-8560-11f0-ab4e-67d06badcd69

#[derive(Default, Component)]
struct Goal;

#[derive(Default, Bundle, LdtkEntity)]
struct GoalBundle {
    goal: Goal,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Default, Resource)]
struct LevelGoals {
    pub goal_locations: HashSet<GridCoords>,
}

fn cache_goal_locations(
    mut level_goals: ResMut<LevelGoals>,
    goals: Query<&GridCoords, With<Goal>>,
) {
    for goal_coords in goals.iter() {
        level_goals.goal_locations.insert(*goal_coords);
    }
}

fn handle_through_goal(
    mut commands: Commands,
    level_goals: ResMut<LevelGoals>,
    mut player_q: Single<&mut GridCoords, With<Player>>,
    goal_q: Query<(&EntityInstance, &GridCoords), (With<Goal>, Without<Player>)>,
    entry_q: Query<(&EntityInstance, &GridCoords), (With<Goal>, Without<Player>)>,
) {
    let coords = player_q.clone();
    if level_goals.goal_locations.contains(&coords) {
        // only triggerred if the player walks on top of a goal
        for (entity, g_coords) in goal_q {
            if *g_coords == **player_q {
                // only run if player is walking on top of the goal
                let destination_value = entity
                    .get_field_instance("destination")
                    .unwrap()
                    .value
                    .clone();
                let entry_entity_ref = match destination_value {
                    FieldValue::EntityRef(Some(x)) => x,
                    _ => panic!("Something aint right in {} metadata", entity.iid),
                };
                let world_dest = entry_entity_ref.level_iid.clone();
                // Does not work because the neighbors level are not loaded.
                let coords_dest = entry_q
                    .iter()
                    .find(|e| e.0.iid == entry_entity_ref.entity_iid)
                    .unwrap()
                    .1;
                commands.insert_resource(LevelSelection::Iid(LevelIid::new(world_dest)));
                **player_q = *coords_dest;
            }
        }
    }
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

// ENTRIES
#[derive(Default, Component)]
struct Entry;

#[derive(Default, Bundle, LdtkEntity)]
struct EntryBundle {
    entry: Entry,
    #[grid_coords]
    grid_coords: GridCoords,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}
