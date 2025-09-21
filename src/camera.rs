use bevy::{
    prelude::*,
    render::{
        camera::{ImageRenderTarget, RenderTarget},
        render_resource::{Extent3d, TextureDescriptor, TextureDimension},
        view::RenderLayers,
    },
};
use bevy_ecs_ldtk::{GridCoords, LdtkWorldBundle, LevelSelection};
use bevy_egui::EguiUserTextures;

use crate::{AppState, player::Player, save::Save, world::GridSize};

pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_main_camera)
            .add_systems(
                OnTransition {
                    exited: AppState::MainMenu,
                    entered: AppState::InGame,
                },
                setup_world_camera,
            )
            .add_systems(
                OnTransition {
                    exited: AppState::ResumeGame,
                    entered: AppState::InGame,
                },
                setup_world_camera,
            )
            .add_systems(
                Update,
                camera_follow_player
                    .run_if(in_state(AppState::InFight).or(in_state(AppState::InGame))),
            );
    }
}

#[derive(Component)]
pub struct MainCamera;

/// Camera used to render the whole window.
/// Different from the camera used to render the game.
pub fn setup_main_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::WindowSize,
            ..OrthographicProjection::default_2d()
        }),
        MainCamera,
        RenderLayers::layer(1), // prevent it from seeing the ldtk workd
    ));
}

#[derive(Component)]
pub struct WorldBundle;

#[derive(Deref, Resource)]
pub struct WorldTexture(pub Handle<Image>);

#[derive(Component)]
pub struct WorldCamera;

/// Initialize the world, and if a save is found, it is loaded.
/// It spawns an additional camera that renders to a texture,
/// which is then used in the UI.
/// todo: need to refactor and seperate world and world ui
pub fn setup_world_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    save_res: Option<Res<Save>>,
    mut images: ResMut<Assets<Image>>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
) {
    commands.spawn((
        WorldBundle,
        LdtkWorldBundle {
            // ldtk_handle: asset_server.load("ldtk/mymap.ldtk").into(),
            ldtk_handle: asset_server.load("ldtk/map_small.ldtk").into(),
            ..Default::default()
        },
        // AudioPlayer::new(asset_server.load("sfx/town.flac")),
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
    egui_user_textures.add_image(image_handle.clone());

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

/// Move the camera accordingly when the player's coordinates have changed.
pub fn camera_follow_player(
    time: Res<Time>,
    grid_size: Res<GridSize>,
    player_q: Query<&GridCoords, With<Player>>,
    mut camera_q: Query<&mut Transform, With<WorldCamera>>,
) {
    if let Ok(player_coords) = player_q.single()
        && let Ok(mut cam_transform) = camera_q.single_mut()
    {
        // target position
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
