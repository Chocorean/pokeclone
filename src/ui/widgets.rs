use bevy_egui::egui::{self, Color32, CornerRadius, Response, Stroke, Ui, Vec2, Widget};

/// A custom styled button for your UI.
pub struct MenuButton<'a> {
    pub text: &'a str,
    pub width: f32,
    pub height: f32,
    pub color: Color32,
    pub text_color: Color32,
    pub border: Stroke,
}

#[allow(dead_code)]
impl<'a> MenuButton<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            width: 60.0,
            height: 20.0,
            color: Color32::from_rgb(139, 69, 19),
            text_color: Color32::from_rgb(210, 180, 140),
            border: Stroke::new(4.0, Color32::from_rgb(92, 51, 23)),
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    pub fn text_color(mut self, color: Color32) -> Self {
        self.text_color = color;
        self
    }

    pub fn border(mut self, border: Stroke) -> Self {
        self.border = border;
        self
    }
}

impl<'a> Widget for MenuButton<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(self.width, self.height), egui::Sense::click());

        // Button background
        let bg_color = if response.hovered() {
            self.color.gamma_multiply(1.2)
        } else if response.clicked() {
            self.color.gamma_multiply(0.8)
        } else {
            self.color
        };

        ui.painter().rect(
            rect,
            CornerRadius::same(2),
            bg_color,
            self.border,
            egui::StrokeKind::Outside,
        );

        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            self.text,
            egui::FontId::default(),
            self.text_color,
        );

        response
    }
}
