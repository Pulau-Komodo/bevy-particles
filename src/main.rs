use bevy::prelude::*;
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
	let window_dimensions = Vec2::new(1600.0, 900.0);

	App::new()
		.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
		.insert_resource(WindowDescriptor {
			width: window_dimensions.x,
			height: window_dimensions.y,
			title: String::from("Particle simulator"),
			..default()
		})
		.insert_resource(WindowDimensions(window_dimensions))
		.add_plugins(DefaultPlugins)
		.add_plugin(InputPlugin)
		.add_plugin(MovementPlugin)
		.add_plugin(ParticlePlugin)
		.add_plugin(GizmoPlugin)
		.add_plugin(GuiPlugin)
		.add_startup_system(spawn_camera)
		.add_system(update_window_dimensions)
		.run();
}

fn spawn_camera(mut commands: Commands) {
	commands.spawn_bundle(Camera2dBundle {
		projection: OrthographicProjection {
			left: 0.0,
			right: 1.0,
			bottom: 0.0,
			top: 1.0,
			window_origin: bevy::render::camera::WindowOrigin::BottomLeft,
			..default()
		},
		..default()
	});
}

pub struct WindowDimensions(Vec2);

impl WindowDimensions {
	pub fn get(&self) -> Vec2 {
		self.0
	}
}

/// A layer between the actual window size and the size the simulation will act like it has. Minimizing sets the window dimensions to 0, and that messes things up. This system will just not update anything when the dimensions are 0.
fn update_window_dimensions(windows: Res<Windows>, mut dimensions: ResMut<WindowDimensions>) {
	let primary = unwrap_or_return!(windows.get_primary());
	let actual_dimensions = Vec2::new(primary.width(), primary.height());
	if actual_dimensions.x == 0.0 || actual_dimensions.y == 0.0 {
		return;
	}

	dimensions.0 = actual_dimensions;
}
