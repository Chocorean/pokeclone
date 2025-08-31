use std::time::Duration;

use bevy::prelude::*;

use crate::appstate::AppState;

pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (execute_animations).run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Component, Clone, Debug)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
    playing: bool,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        let fps = 10;
        Self {
            first_sprite_index: 0,
            last_sprite_index: 0,
            fps,
            frame_timer: Self::timer_from_fps(fps),
            playing: false,
        }
    }
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
            playing: false,
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
pub fn execute_animations(
    time: Res<Time>,
    player_q: Single<(&mut Sprite, &mut AnimationConfig), With<crate::player::Player>>,
) {
    let (mut sprite, mut config) = player_q.into_inner();
    if !config.playing {
        return;
    }

    // We track how long the current sprite has been displayed for
    config.frame_timer.tick(time.delta());

    // If it has been displayed for the user-defined amount of time (fps)...
    if config.frame_timer.just_finished()
        && let Some(atlas) = &mut sprite.texture_atlas
    {
        if atlas.index == config.last_sprite_index {
            // ...and it IS the last frame, then we move back to the first frame and stop.
            atlas.index = config.first_sprite_index;
            config.playing = false;
            dbg!(config.clone(), atlas.index);
        } else {
            // ...and it is NOT the last frame, then we move to the next frame...
            atlas.index += 1;
            // ...and reset the frame timer to start counting all over again
            config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
        }
    }
    // }
}

pub fn trigger_animation(animation: &mut Mut<AnimationConfig>) {
    // We create a new timer when the animation is triggered
    animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
    animation.playing = true;
}
