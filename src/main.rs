use bevy::{
	prelude::*,
	window::{PrimaryWindow, WindowResolution},
};
use gizmos::GizmoPlugin;
use gui::GuiPlugin;
use input::{Action, InputPlugin};
use leafwing_input_manager::prelude::ActionState;
use movement::MovementPlugin;
use particle::ParticlePlugin;

use crate::assets::{TextureMap, load_assets};

mod assets;
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

pub const TIMESTEP: f32 = 1.0 / 60.0;

fn main() {
	let window_size = Vec2::new(1600.0, 900.0);
	App::new()
		.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
		.insert_resource(WindowDimensions(window_size))
		.insert_resource(Time::<Fixed>::from_seconds(TIMESTEP as f64))
		.init_resource::<WrappingForce>()
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
		.add_systems(Startup, (spawn_camera, load_assets))
		.init_resource::<TextureMap>()
		.add_systems(Update, (update_window_dimensions, toggle_wrap))
		.run();
}

fn spawn_camera(mut commands: Commands) {
	commands.spawn((
		Camera2d,
		Projection::Orthographic(OrthographicProjection {
			viewport_origin: Vec2::ZERO,
			..OrthographicProjection::default_2d()
		}),
	));
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
	let primary = unwrap_or_return!(windows.single().ok());
	let actual_dimensions = Vec2::new(primary.width(), primary.height());
	if actual_dimensions.x == 0.0 || actual_dimensions.y == 0.0 {
		return;
	}

	dimensions.0 = actual_dimensions;
}

#[derive(Resource)]
pub struct WrappingForce(bool);

impl Default for WrappingForce {
	fn default() -> Self {
		Self(true)
	}
}

fn toggle_wrap(action_state: Query<&ActionState<Action>>, mut wrapping: ResMut<WrappingForce>) {
	if action_state
		.single()
		.unwrap()
		.just_pressed(&Action::ToggleWrap)
	{
		wrapping.0 = !wrapping.0;
	}
}
