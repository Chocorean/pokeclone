use bevy::{
    prelude::*,
    render::{
        camera::{ImageRenderTarget, RenderTarget},
        render_resource::{Extent3d, TextureDescriptor, TextureDimension},
        view::RenderLayers,
    },
};
use bevy_ecs_ldtk::{
    LdtkProjectHandle, LevelEvent, LevelIid,
    assets::{LdtkProject, LevelMetadataAccessor},
};

use crate::{AppState, player::Player, ui::setup_game_ui};

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
                setup_world_camera.before(setup_game_ui),
            )
            .add_systems(
                Update,
                camera_follow_player.run_if(in_state(AppState::InGame)),
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
pub fn setup_world_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
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
    player_tf: Single<&Transform, (With<Player>, Changed<Transform>)>,
    mut camera_tf: Single<&mut Transform, (With<WorldCamera>, Without<Player>)>,
    level: Query<&LevelIid>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    // find level and get its width/height
    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single().unwrap())
        .unwrap();
    let Some(level) = level
        .iter()
        .find_map(|level_iid| ldtk_project.get_raw_level_by_iid(&level_iid.to_string()))
    else {
        error!("Failed to find level, camera_follow_player may be broken.");
        return;
    };

    let view_w = CAMERA_WIDTH * CAMERA_SCALE;
    let view_h = CAMERA_HEIGHT * CAMERA_SCALE;
    let half_w = view_w * 0.5;
    let half_h = view_h * 0.5;

    let level_w = level.px_wid as f32;
    let level_h = level.px_hei as f32;

    // Start centered on player
    let mut target_x = player_tf.translation.x;
    let mut target_y = player_tf.translation.y;

    // If level wider than view, clamp X; else leave centered on player
    if level_w > view_w {
        if target_x - half_w < 0.0 {
            target_x = half_w;
        } else if target_x + half_w > level_w {
            target_x = level_w - half_w;
        }
    }

    // If level taller than view, clamp Y; else leave centered on player
    if level_h > view_h {
        if target_y - half_h < 0.0 {
            target_y = half_h;
        } else if target_y + half_h > level_h {
            target_y = level_h - half_h;
        }
    }

    camera_tf.translation.x = target_x;
    camera_tf.translation.y = target_y;
}
