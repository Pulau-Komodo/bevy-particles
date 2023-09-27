use bevy::{
	prelude::*,
	window::{PrimaryWindow, WindowResolution},
};
use gizmos::GizmoPlugin;
use gui::GuiPlugin;
use input::InputPlugin;
use movement::MovementPlugin;
use particle::ParticlePlugin;

mod common;
mod draw_properties;
mod gizmos;
mod gui;
mod input;
mod macros;
mod movement;
mod particle;

pub const CLICK_RADIUS: f32 = 15.0;
pub const CLICK_RADIUS_SQUARED: f32 = CLICK_RADIUS * CLICK_RADIUS;

fn main() {
	let window_size = Vec2::new(1600.0, 900.0);
	App::new()
		.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
		.insert_resource(WindowDimensions(window_size))
		.add_plugins((
			DefaultPlugins.set(WindowPlugin {
				primary_window: Some(Window {
					resolution: WindowResolution::from(window_size), //::new(window_size.x, window_size.y),
					title: String::from("Particle simulator"),
					..default()
				}),
				..default()
			}),
			InputPlugin,
			MovementPlugin,
			ParticlePlugin,
			GizmoPlugin,
			GuiPlugin,
		))
		.add_systems(Startup, spawn_camera)
		.add_systems(Update, update_window_dimensions)
		.run();
}

fn spawn_camera(mut commands: Commands) {
	commands.spawn(Camera2dBundle {
		projection: OrthographicProjection {
			viewport_origin: Vec2::ZERO,
			near: -1000.0,
			..default()
		},
		..default()
	});
}

#[derive(Resource)]
pub struct WindowDimensions(Vec2);

impl WindowDimensions {
	pub fn get(&self) -> Vec2 {
		self.0
	}
}

/// A layer between the actual window size and the size the simulation will act like it has. Minimizing sets the window dimensions to 0, and that messes things up. This system will just not update anything when the dimensions are 0.
fn update_window_dimensions(
	windows: Query<&Window, With<PrimaryWindow>>,
	mut dimensions: ResMut<WindowDimensions>,
) {
	let primary = unwrap_or_return!(windows.get_single().ok());
	let actual_dimensions = Vec2::new(primary.width(), primary.height());
	if actual_dimensions.x == 0.0 || actual_dimensions.y == 0.0 {
		return;
	}

	dimensions.0 = actual_dimensions;
}
