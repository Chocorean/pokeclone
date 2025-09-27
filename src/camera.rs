use bevy::{
    prelude::*,
    render::{
        camera::{ImageRenderTarget, RenderTarget},
        render_resource::{Extent3d, TextureDescriptor, TextureDimension},
        view::RenderLayers,
    },
};
use bevy_ecs_ldtk::GridCoords;
use bevy_egui::EguiUserTextures;

use crate::{AppState, player::Player, world::GridSize};

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

/// Initialize the world camera, which displays the actual game.
pub fn setup_world_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
) {
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
    player_coords: Single<&Transform, (With<Player>, Changed<Transform>)>,
    mut cam_transform: Single<&mut Transform, (With<WorldCamera>, Without<Player>)>,
) {
    // not taking the whole transform because it changes the camera config (distance, etc)
    cam_transform.translation = player_coords.translation;
}
