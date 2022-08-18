use bevy::prelude::*;
use gui::GuiPlugin;
use input::InputPlugin;
use particle::ParticlePlugin;
use particle_attractor::ParticleAttractorPlugin;
use particle_deleter::ParticleDeleterPlugin;
use particle_emitter::ParticleEmitterPlugin;

mod common;
mod draw_properties;
mod gui;
mod input;
mod macros;
mod particle;
mod particle_attractor;
mod particle_deleter;
mod particle_emitter;

pub const CLICK_RADIUS: f32 = 15.0;
pub const CLICK_RADIUS_SQUARED: f32 = CLICK_RADIUS * CLICK_RADIUS;

fn main() {
	App::new()
		.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
		.insert_resource(WindowDescriptor {
			width: 1600.0,
			height: 900.0,
			title: String::from("Particle simulator"),
			..default()
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(InputPlugin)
		.add_plugin(ParticlePlugin)
		.add_plugin(ParticleEmitterPlugin)
		.add_plugin(ParticleDeleterPlugin)
		.add_plugin(ParticleAttractorPlugin)
		.add_plugin(GuiPlugin)
		.add_startup_system(spawn_camera)
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
