use bevy::prelude::*;

mod game;
mod index;
mod main_menu;
mod widgets;

use bevy_egui::{
    EguiContexts, EguiPlugin, EguiPrimaryContextPass,
    egui::{FontData, FontDefinitions, FontFamily},
};
use game::*;
use main_menu::*;

use crate::AppState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default());
        app.add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin);
        app.add_systems(
            EguiPrimaryContextPass,
            (
                setup_main_menu_ui.run_if(in_state(AppState::MainMenu)),
                handle_game_ui_input.run_if(in_state(AppState::InGame)),
                setup_game_ui.run_if(in_state(AppState::InGame).or(in_state(AppState::InFight))),
            ),
        );
        app.add_systems(
            EguiPrimaryContextPass,
            add_custom_fonts.before(setup_game_ui),
        );
        // app.add_systems(Update, load_custom_fonts);
    }
}

fn add_custom_fonts(mut contexts: EguiContexts, mut has_run: Local<bool>) {
    if *has_run {
        return;
    }

    let ctx = contexts.ctx_mut().unwrap();

    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "mmc".to_owned(),
        FontData::from_static(include_bytes!("../../assets/fonts/mmc.otf")).into(),
    );

    // This bit is useful to load the font from another place, let's keep it for now.
    // let mut newfam = std::collections::BTreeMap::new();
    // newfam.insert(FontFamily::Name("mmc".into()), vec!["mmc".to_owned()]);
    // // fonts.families.append(&mut newfam);

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "mmc".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("mmc".to_owned());

    ctx.set_fonts(fonts);
    *has_run = true;
}
