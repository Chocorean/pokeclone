use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Color32, Frame, Pos2, Rect, RichText},
};

use crate::{
    AppState,
    camera::WorldTexture,
    event::NewSaveEvent,
    fight::FightState,
    index::{Creature, Dex},
    team::Team,
    ui::index::dex_list_ui,
};

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
    mut event_writer: EventWriter<NewSaveEvent>,
    wild_creature: Option<Res<Creature>>,
    state: Res<State<AppState>>,
    team: Res<Team>,
    world_tex: Res<WorldTexture>,
    mut next_state: ResMut<NextState<AppState>>,
    mut next_fight_state: ResMut<NextState<FightState>>,
    fight_state: Res<State<FightState>>,
    dex: Res<Dex>,
    mut enable_index: Local<bool>,
) -> Result {
    // textures
    let world_texture_id = contexts.image_id(&world_tex).unwrap();

    // buttons
    let mut save = false;
    // dex
    // map
    // etc

    let ctx = contexts.ctx_mut()?;

    egui::SidePanel::right("team_panel")
        .resizable(false)
        .min_width(200.0)
        .show(ctx, |ui| {
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
                                // Crash bc in camera.rs, egui_textures is used as well. Need to refactor
                                // let handle: Handle<Image> = asset_server.load(member.sprite());
                                // let texture_id = egui_textures.image_id(&handle).unwrap();
                                // // let sized = SizedTexture::from(texture_id);
                                // let image = egui::Image::from_texture((
                                //     texture_id,
                                //     egui::Vec2::from([64., 64.]),
                                // ));
                                // ui.add(image);
                                ui.add(
                                    egui::Image::new(format!(
                                        "file://assets/{}",
                                        member.texture_path(&dex)
                                    ))
                                    .fit_to_exact_size(egui::Vec2::new(64., 64.)),
                                );
                                ui.vertical(|ui: &mut egui::Ui| {
                                    ui.label(
                                        RichText::new(member.name(&dex)).color(Color32::WHITE),
                                    );
                                    let hp_bar = egui::ProgressBar::new(
                                        member.hp as f32 / member.max_hp(&dex) as f32,
                                    )
                                    .desired_height(8.)
                                    .fill(
                                        if member.hp
                                            >= (member.max_hp(&dex) as f32 * 0.8).round() as u8
                                        {
                                            Color32::GREEN
                                        } else if member.hp
                                            >= (member.max_hp(&dex) as f32 * 0.2).round() as u8
                                        {
                                            Color32::ORANGE
                                        } else {
                                            Color32::RED
                                        },
                                    );
                                    ui.add(hp_bar);
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "{}/{}",
                                            member.hp,
                                            member.max_hp(&dex)
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
                ui.checkbox(&mut enable_index, "Index");
                save = ui.button("Save").clicked();
            } else {
                ui.disable();
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
            // Show spinning prism on top of player
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

            // Fight floating window!
            let (title, foes) = if let Some(creature) = wild_creature {
                (
                    format!("A wild {} wants to fight!", creature.name),
                    vec![creature.clone()],
                )
            } else {
                todo!("need to implment fight versus trainer");
                (format!("A trainer wants to fight!"), vec![])
            };
            egui::Window::new(title)
                .resizable(false)
                .max_height(max_rect.height() * 0.5)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::Image::new(format!(
                                    "file://assets/{}",
                                    team.0[0].texture_path(&dex)
                                ))
                                .uv(Rect::from_min_max(Pos2::new(1., 0.), Pos2::new(0., 1.)))
                                .fit_to_exact_size(egui::Vec2::new(128., 128.)),
                            );
                            ui.add_space(max_rect.width() - 256. - 32.);
                            ui.add(
                                egui::Image::new(format!(
                                    "file://assets/{}",
                                    foes[0].texture_path()
                                ))
                                .fit_to_exact_size(egui::Vec2::new(128., 128.)),
                            );
                        });
                        match fight_state.get() {
                            FightState::MainAction => {
                                ui.horizontal_centered(|ui| {
                                    if ui.button("Attack").clicked() {
                                        next_fight_state.set(FightState::AttackChoice);
                                    }
                                    if ui.button("Tame").clicked() {
                                        // try to tame
                                    }
                                });
                                ui.horizontal_centered(|ui| {
                                    if ui.button("Items").clicked() {}
                                    if ui.button("Flee").clicked() {
                                        next_state.set(AppState::InGame);
                                    }
                                });
                            }
                            FightState::AttackChoice => {
                                ui.horizontal_top(|ui| {
                                    egui::CollapsingHeader::new("Select an attack").show(
                                        ui,
                                        |ui| {
                                            // one elemental attack, plus all attacks defined by physical caracteristics.
                                            let fighter = team.0.first().unwrap();
                                            for attack in
                                                dex.filter_attacks_for_team_member(fighter.clone())
                                            {
                                                ui.button(attack.name());
                                            }
                                        },
                                    );
                                });
                            }
                            _ => {}
                        };
                    });
                });
        }
    });

    // buttons actions
    egui::Window::new("Index")
        .open(&mut enable_index)
        .show(ctx, |ui| {
            dex_list_ui(ui, &dex);
        });
    if save {
        event_writer.write(NewSaveEvent {});
    }
    // other buttons

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
