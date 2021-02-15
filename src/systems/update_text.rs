use crate::components::FPSText;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::{Query, Res, Text, With};

pub fn update_text(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FPSText>>) {
	for mut text in query.iter_mut() {
		if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
			if let Some(average) = fps.average() {
				text.value = format!("FPS: {:.2}", average);
			}
		}
	}
}
