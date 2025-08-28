use bevy::prelude::*;

pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

#[derive(Component)]
pub struct MyCameraMarker;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            scaling_mode: bevy::render::camera::ScalingMode::WindowSize,
            ..OrthographicProjection::default_2d()
        }),
        // Transform::from_xyz(1280.0 / 4.0, 720.0 / 4.0, 0.0),
        Transform::from_xyz(1280.0 / 4.0, 720.0 / 4.0, 1.0),
        MyCameraMarker,
    ));
}
