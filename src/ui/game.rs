use bevy::prelude::*;
use bevy_ecs_ldtk::{GridCoords, LevelSelection};
use bevy_egui::{
    EguiContexts,
    egui::{self, Color32, Frame, RichText},
};

use crate::{appstate::AppState, camera::WorldTexture, character::Player, save::Save, team::Team};

#[derive(Component)]
pub struct GameUI;

#[derive(Component)]
pub struct TeamUI;

#[derive(Component)]
pub struct WorldUI;

#[derive(Component)]
pub struct GameNode;

#[derive(Component)]
pub struct SaveButton;

pub fn setup_game_ui(
    mut contexts: EguiContexts,
    // mut egui_user_textures: ResMut<EguiUserTextures>,
    world_tex: Res<WorldTexture>,
    level_res: Res<LevelSelection>,
    player_q: Query<&GridCoords, With<Player>>,
    team: Res<Team>,
) -> Result {
    // textures
    let world_texture_id = contexts.image_id(&world_tex).unwrap();

    // buttons
    let mut save = false;

    let ctx = contexts.ctx_mut()?;

    egui::SidePanel::right("team_panel")
        .resizable(false)
        .min_width(200.0)
        .show(ctx, |ui| {
            // ui.vertical(|ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                for member in team.0.iter() {
                    Frame::new()
                        .stroke(egui::Stroke::new(1., Color32::GRAY))
                        .corner_radius(2)
                        .inner_margin(egui::Margin::same(4))
                        .outer_margin(0)
                        .fill(Color32::GRAY)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::Image::new(format!("file://assets/{}", member.sprite()))
                                        .fit_to_exact_size(egui::Vec2::new(64., 64.)),
                                );
                                ui.vertical(|ui: &mut egui::Ui| {
                                    ui.label(RichText::new(member.name()).color(Color32::WHITE));
                                    let hp_bar = egui::ProgressBar::new(
                                        member.hp as f32 / member.max_hp() as f32,
                                    )
                                    .desired_height(8.)
                                    .fill(
                                        if member.hp == member.max_hp() {
                                            Color32::GREEN
                                        } else {
                                            Color32::ORANGE
                                        },
                                    );
                                    ui.add(hp_bar);
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "{}/{}",
                                            member.hp,
                                            member.max_hp()
                                        ))
                                        .color(Color32::WHITE),
                                    )
                                })
                            });
                        });
                }
            });
        });
    egui::TopBottomPanel::top("actions_panel").show(ctx, |ui| {
        save = ui.button("Save").clicked();
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.centered_and_justified(|ui| {
            let max_rect = ui.max_rect();
            ui.image(egui::load::SizedTexture::new(
                world_texture_id,
                egui::vec2(max_rect.width(), max_rect.height()),
            ));
        });
    });

    // buttons actions
    if save {
        new_save(level_res, player_q, team);
    }

    Ok(())
}

/// Gather what matters and save it all.
/// Might be a better way to do so, is that Events ? <-- TODO investigate
fn new_save(
    level_res: Res<LevelSelection>,
    player_q: Query<&GridCoords, With<Player>>,
    team: Res<Team>,
) {
    let level_id = match *level_res {
        LevelSelection::Indices(x) => x.level,
        _ => todo!("not supported"),
    };
    let coords = player_q.single().unwrap();
    Save::new(level_id as i32, *coords, team.clone());
}

/// Game UI specific input handling
/// Does not cover in-game actions like moving the player
pub fn handle_game_ui_input(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_q: Query<&GridCoords, With<Player>>,
    level_res: Res<LevelSelection>,
    team: Res<Team>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    } else if keyboard_input.just_pressed(KeyCode::F1) {
        new_save(level_res, player_q, team);
    }
}
