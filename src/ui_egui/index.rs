use bevy_egui::egui::{self, Grid, RichText};

use crate::dex::{Creature, Dex, Species};

/// Basic brick for the Dex. It shows basic information about a creature.
// todo: hover/click for more data.
fn creature_entry_ui(ui: &mut egui::Ui, creature: &Creature) -> egui::Response {
    ui.horizontal_top(|ui| {
        // Draw image, name, etc.
        ui.add(
            egui::Image::new(format!("file://{}", creature.texture_path()))
                .fit_to_exact_size(egui::Vec2::new(64., 64.)),
        );
        ui.label(format!("{} ({})", creature.name, creature.element));
        Grid::new(format!("stats_{}", creature.name)).show(ui, |ui| {
            ui.label(RichText::new("Base Stats").heading());
            ui.label(RichText::new("Value").heading());
            ui.end_row();
            for (name, stat) in creature.clone().stats {
                ui.label(name);
                ui.label(stat.to_string());
                ui.end_row();
            }
        });
        // ... more fields ...
    })
    .response
}

fn species_dropdown_ui(ui: &mut egui::Ui, species: &Species) -> egui::Response {
    ui.vertical(|ui| {
        egui::CollapsingHeader::new(&species.name).show(ui, |ui| {
            for creature in &species.individuals {
                creature_entry_ui(ui, &creature.0);
                ui.separator();
            }
        })
    })
    .response
}

// Todo: filters
pub fn dex_list_ui(ui: &mut egui::Ui, dex: &Dex) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        for species in &dex.species {
            species_dropdown_ui(ui, species);
        }
    });
}
