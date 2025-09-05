use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Color32, Frame, Pos2, Rect, RichText},
};

use crate::{AppState, camera::WorldTexture, event::NewSaveEvent, team::Team};

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

/// Build the whole game UI
/// What it shows depends on the current `AppState`
pub fn setup_game_ui(
    mut contexts: EguiContexts,
    world_tex: Res<WorldTexture>,
    team: Res<Team>,
    mut event_writer: EventWriter<NewSaveEvent>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) -> Result {
    // textures
    let world_texture_id = contexts.image_id(&world_tex).unwrap();

    // buttons
    let mut save = false;
    let mut attack = false;
    let mut attack_choice: Option<u8> = None;
    let mut tame = false;
    let mut flee = false;

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
        ui.horizontal_centered(|ui| {
            if *state == AppState::InGame {
                save = ui.button("Save").clicked();
            } else if *state == AppState::InFight {
                attack = ui.button("Attack").clicked();
                // todo build proper attack ui
                tame = ui.button("Tame").clicked();
                flee = ui.button("Flee").clicked();
            }
        });
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        let max_rect = ui.max_rect();
        ui.centered_and_justified(|ui| {
            ui.image(egui::load::SizedTexture::new(
                world_texture_id,
                egui::vec2(max_rect.width(), max_rect.height()),
            ));
        });
        if *state == AppState::InFight {
            println!("{}", max_rect);
            let rect = egui::Rect::from_min_size(
                Pos2::new(
                    max_rect.width() / 2. - 4.,
                    max_rect.height() / 2. - 16.,
                    // 400., 400.,
                ),
                bevy_egui::egui::Vec2::new(24., 24.),
            );
            ui.put(
                rect,
                egui::Image::new("file://assets/textures/animations/blue_prism.gif"),
            );
        }
    });

    // buttons actions
    if save {
        event_writer.write(NewSaveEvent {});
    }

    // other fight buttons...
    if flee {
        next_state.set(AppState::InGame);
    }

    Ok(())
}

/// Game UI specific input handling
/// Does not cover in-game actions like moving the player
pub fn handle_game_ui_input(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<NewSaveEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    } else if keyboard_input.just_pressed(KeyCode::F1) {
        event_writer.write(NewSaveEvent {});
    }
}
