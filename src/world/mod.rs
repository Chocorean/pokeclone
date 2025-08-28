use bevy::{
    platform::collections::HashSet,
    prelude::*,
    render::{
        camera::{ImageRenderTarget, RenderTarget},
        render_resource::{Extent3d, TextureDescriptor, TextureDimension},
    },
};
use bevy_ecs_ldtk::prelude::*;

use crate::{
    appstate::AppState,
    character::{Direction, Player, PlayerBundle},
    save::Save,
    team::Team,
};

// npc trainer uuid
// ca7c1690-5e50-11f0-85ca-e96bd84a6222

#[derive(Resource)]
pub struct GridSize(pub i32);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelWalls>()
            .init_resource::<LevelHerbs>()
            .insert_resource(GridSize(16))
            .register_ldtk_int_cell_for_layer::<WallBundle>("Walls", 1)
            .register_ldtk_int_cell_for_layer::<HerbBundle>("Walls", 2)
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<GoalBundle>("Goal")
            .register_ldtk_entity::<NPCsBundle>("NPCs")
            .add_systems(OnEnter(AppState::LoadGame), load_game)
            .add_systems(
                OnEnter(AppState::InGame),
                init_team.run_if(not(resource_exists::<Team>)),
            )
            .add_systems(
                Update,
                (
                    cache_herb_locations,
                    update_world,
                    cache_wall_locations,
                    translate_grid_coords_entities,
                    handle_player_interaction,
                    camera_follow_player,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Component)]
pub struct WorldBundle;

#[derive(Resource)]
pub struct WorldTexture(pub Handle<Image>);

#[derive(Component)]
pub struct WorldCamera;

/// Load the default world, or the one the player has saved in.
/// It spawns an additional camera that renders to a texture,
/// which is then used in the UI.
/// todo: need to refactor and seperate world and world ui
pub fn setup_world_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    save_res: Option<Res<Save>>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn((
        WorldBundle,
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("ldtk/map_small.ldtk").into(),
            ..Default::default()
        },
        AudioPlayer::new(asset_server.load("sfx/town.flac")),
    ));
    let index = if let Some(save) = save_res {
        save.level as usize
    } else {
        0
    };
    commands.insert_resource(LevelSelection::index(index));

    // --- create render texture ---
    let size = Extent3d {
        width: 800,
        height: 600,
        ..default()
    };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("world"),
            size,
            dimension: TextureDimension::D2,
            format: bevy::render::render_resource::TextureFormat::Bgra8UnormSrgb,
            usage: bevy::render::render_resource::TextureUsages::TEXTURE_BINDING
                | bevy::render::render_resource::TextureUsages::COPY_DST
                | bevy::render::render_resource::TextureUsages::RENDER_ATTACHMENT,
            mip_level_count: 1,
            sample_count: 1,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);

    let image_handle = images.add(image);

    commands.spawn((
        Camera2d,
        Camera {
            order: 0,
            target: RenderTarget::Image(ImageRenderTarget::from(image_handle.clone())),
            ..default()
        },
        // zoom x2
        Transform::from_scale(Vec3::splat(0.5)),
        WorldCamera,
    ));

    // Store texture handle as resource so UI can use it
    commands.insert_resource(WorldTexture(image_handle));
}

pub fn camera_follow_player(
    time: Res<Time>,
    grid_size: Res<GridSize>,
    player_q: Query<&GridCoords, With<Player>>,
    mut camera_q: Query<&mut Transform, With<WorldCamera>>,
) {
    if let Ok(player_coords) = player_q.single()
        && let Ok(mut cam_transform) = camera_q.single_mut()
    {
        // let center = bevy_ecs_ldtk::utils::grid_coords_to_translation(
        //     *player_coords,
        //     IVec2::splat(grid_size.0),
        // );
        // cam_transform.translation = center.extend(cam_transform.translation.z);

        // target position (preserve camera z)
        let target_xy = bevy_ecs_ldtk::utils::grid_coords_to_translation(
            *player_coords,
            IVec2::splat(grid_size.0),
        );
        let target = target_xy.extend(cam_transform.translation.z);

        // hardcoded smoothing (higher = snappier)
        let smoothing: f32 = 4.0;
        let alpha = 1.0 - (-smoothing * time.delta_secs()).exp();

        cam_transform.translation = cam_transform.translation.lerp(target, alpha);
    }
}

/// A better name would be load_save, since it just loads the save as a Resource.
pub fn load_game(mut commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
    let save = Save::load().unwrap();
    commands.insert_resource(save.team.clone());
    commands.insert_resource(save);
    next_state.set(AppState::InGame);
}

pub fn init_team(mut commands: Commands) {
    commands.insert_resource(Team::new());
}

/// Load the player's saved position, if any.
/// Might bug when we change level ?
pub fn update_world(
    mut commands: Commands,
    mut player_q: Query<&mut GridCoords, With<Player>>,
    save_res: Option<Res<Save>>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }

    if let Some(save) = save_res
        && let Ok(mut player_coords) = player_q.single_mut()
    {
        *player_coords = GridCoords::new(save.coords.0, save.coords.1);
        commands.remove_resource::<Save>();

        *has_run = true;
    }
}

/// todo: only run once per level load? or update after an NPC move?
pub fn cache_wall_locations(
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

fn cache_herb_locations(
    mut level_walls: ResMut<LevelHerbs>,
    herbs: Query<&GridCoords, With<Herb>>,
) {
    for herb_coords in herbs.iter() {
        level_walls.herb_locations.insert(*herb_coords);
    }
}

pub fn translate_grid_coords_entities(
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

#[derive(Default, Resource)]
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

#[derive(Default, Resource)]
pub struct LevelHerbs {
    pub herb_locations: HashSet<GridCoords>,
}

#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Default, Component)]
pub struct Herb;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct HerbBundle {
    herb: Herb,
}

#[derive(Default, Bundle, LdtkEntity)]
struct GoalBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

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

pub fn handle_player_interaction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_q: Query<(&GridCoords, &Direction), With<Player>>,
    npc_q: Query<(&GridCoords, &EntityInstance), With<NPC>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        let (player_grid_coords, direction) = player_q.single().unwrap();
        let facing_coords = direction.next_coords(*player_grid_coords);
        // Here you would check if there's an NPC or object at facing_coords
        // and trigger a dialogue or interaction event.
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
