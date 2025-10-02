mod button;
mod h_sep;
mod hyperlink;
mod slider;
mod team_member;

pub use button::*;
pub use h_sep::*;
pub use hyperlink::*;
pub use slider::*;
pub use team_member::*;

use bevy::color::{Color, LinearRgba};

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let LinearRgba {
        red: ar,
        green: ag,
        blue: ab,
        alpha: aa,
    } = a.to_linear();
    let LinearRgba {
        red: br,
        green: bg,
        blue: bb,
        alpha: ba,
    } = b.to_linear();
    Color::linear_rgba(
        ar + (br - ar) * t,
        ag + (bg - ag) * t,
        ab + (bb - ab) * t,
        aa + (ba - aa) * t,
    )
}
