use bevy::{
    prelude::*,
    render::{
        camera::{ImageRenderTarget, RenderTarget},
        render_resource::{Extent3d, TextureDescriptor, TextureDimension},
        view::RenderLayers,
    },
};
use bevy_ecs_ldtk::{
    LdtkProjectHandle, LevelIid, LevelSelection,
    assets::{LdtkProject, LevelIndices, LevelMetadataAccessor},
};
use bevy_egui::EguiUserTextures;

use crate::{AppState, player::Player};

const CAMERA_WIDTH: f32 = 800.;
const CAMERA_HEIGHT: f32 = 600.;
const CAMERA_SCALE: f32 = 0.5;

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
                Update,
                camera_follow_player
                    // .run_if(in_state(AppState::InGame)), TODO Check si on peut remplacer ca
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

/// Initialize the world camera, which displays the actual game.
pub fn setup_world_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
) {
    // --- create render texture ---
    let size = Extent3d {
        width: CAMERA_WIDTH as u32,
        height: CAMERA_HEIGHT as u32,
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
        Transform::from_scale(Vec3::splat(CAMERA_SCALE)),
        WorldCamera,
    ));

    // Store texture handle as resource so UI can use it
    commands.insert_resource(WorldTexture(image_handle));
}

/// Move the camera accordingly when the player's coordinates have changed, with clamping.
///
/// Ripped from [invertedEcho's project](https://github.com/invertedEcho/platformer-bevy-ldtk/blob/master/src/camera/systems.rs#L19)
pub fn camera_follow_player(
    player_coords: Single<&Transform, (With<Player>, Changed<Transform>)>,
    mut cam_transform: Single<&mut Transform, (With<WorldCamera>, Without<Player>)>,
    level_selection: Res<LevelSelection>,
    level_query: Query<&LevelIid, (Without<Projection>, Without<Player>)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    // find level and get its width/height
    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single().unwrap())
        .unwrap();
    let Some(current_level) = level_query.iter().find_map(|level_iid| {
        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .unwrap();

        // TODO: why levelindices? we dont use indices
        level_selection
            .is_match(&LevelIndices::default(), level)
            .then_some(level)
    }) else {
        error!("Failed to find level, camera_follow_player may be broken.");
        return;
    };

    let current_level_width = current_level.px_wid as f32;
    let current_level_height = current_level.px_hei as f32;

    // if map is smaller than camera view (i.e inside building),
    // just center on player
    if current_level_height < CAMERA_HEIGHT || current_level_width < CAMERA_WIDTH {
        cam_transform.translation = player_coords.translation;
        return;
    }

    let half_window_width = CAMERA_WIDTH / 2.0;

    // left edge of camera should not go beyond level width
    let new_camera_translation_x =
        (half_window_width * CAMERA_SCALE).max(player_coords.translation.x);

    // right edge of camera should not go beyond level width
    if new_camera_translation_x + half_window_width * CAMERA_SCALE < current_level_width {
        cam_transform.translation.x += new_camera_translation_x - cam_transform.translation.x;
    }

    // bottom of camera should not go below level height
    let half_window_height = CAMERA_HEIGHT / 2.0;
    let new_camera_translation_y =
        (half_window_height * CAMERA_SCALE).max(player_coords.translation.y);

    let top_of_player = player_coords.translation.y + half_window_height * CAMERA_SCALE;

    // top of camera should not go above level height
    if top_of_player > current_level_height {
        return;
    }

    cam_transform.translation.y += new_camera_translation_y - cam_transform.translation.y;
}
