use bevy::{prelude::*, render::pipelined_rendering::PipelinedRenderingPlugin};

mod debug;

fn main() {
	let mut app = App::new();
	let window = Window {
		title: "Anarchy Chess v-17,3.0".to_string(),
		..default()
	};

	app.add_plugins(DefaultPlugins
		.set(WindowPlugin {
			primary_window: Some(window),
			..default()
		})
		// This is to fix a current crash on linux with 550 NVidia drivers TODO: Remove when this gets fixed
		.disable::<PipelinedRenderingPlugin>()
	);

	// Debug plugin is added only in debug builds
	#[cfg(debug_assertions)]
	{app.add_plugins(debug::DebugPlugin)};
	app.run();
}
