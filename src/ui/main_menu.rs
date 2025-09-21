use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Color32},
};

use crate::{AppState, save::Save};

/// Build the "main menu" window, with a few buttons: Continue (if save exists), New Game, and Options.
pub fn setup_main_menu_ui(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<AppState>>,
) -> Result {
    // buttons states
    let mut resume = false;
    let mut new = false;
    let mut options = false;

    let ctx = contexts.ctx_mut()?;
    egui_extras::install_image_loaders(ctx);

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(24.);
            ui.heading(
                egui::RichText::new("PokeClone game")
                    .color(Color32::WHITE)
                    .size(40.0),
            );
            ui.add_space(24.);
            ui.hyperlink("https://github.com/chocorean/pokeclone");
            ui.add(egui::github_link_file_line!(
                "https://github.com/chocorean/pokeclone/blob/main/",
                "Direct link to source code."
            ));
        });

        ui.separator();

        let mut buttons = vec![("New Game", &mut new), ("Options", &mut options)];
        if Save::exists() {
            buttons.insert(0, ("Resume", &mut resume));
        }

        ui.vertical_centered(|ui| {
            for (str, state) in buttons {
                ui.add_space(12.);
                *state = ui.button(egui::RichText::new(str).size(24.)).clicked();
            }
        });
    });

    if resume {
        next_state.set(AppState::ResumeGame);
    }

    if new {
        next_state.set(AppState::InGame);
    }

    if options {
        next_state.set(AppState::OptionsMenu);
    }

    Ok(())
}
