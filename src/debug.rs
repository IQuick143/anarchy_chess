use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;

/// Plugin for housing all dev-tools
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(EditorPlugin::default());
	}
}
