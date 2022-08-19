use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
	common::{despawn_gizmo, spawn_gizmo, wrapping_offset_2d},
	draw_properties,
	input::Action,
	particle::Particle,
	unwrap_or_return,
};

pub struct ParticleDeleterPlugin;

impl Plugin for ParticleDeleterPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(spawn_particle_deleter)
			.add_system(despawn_particle_deleter)
			.add_system(activate_particle_deleters);
	}
}

#[derive(Component)]
struct ParticleDeleter {
	radius_squared: f32,
}

impl ParticleDeleter {
	fn new(radius: f32) -> Self {
		Self {
			radius_squared: radius.powi(2),
		}
	}
}

fn spawn_particle_deleter(
	commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
) {
	spawn_gizmo(
		commands,
		windows,
		action_state,
		Action::SpawnDeleter,
		draw_properties::DELETER,
		ParticleDeleter::new(100.0),
	);
}

fn despawn_particle_deleter(
	commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
	deleters: Query<(Entity, &Transform), With<ParticleDeleter>>,
) {
	despawn_gizmo(
		commands,
		windows,
		action_state,
		Action::DespawnDeleter,
		Action::DespawnAllDeleters,
		&deleters,
	);
}

fn activate_particle_deleters(
	mut commands: Commands,
	windows: Res<Windows>,
	deleters: Query<(&ParticleDeleter, &Transform)>,
	particles: Query<(Entity, &Transform), With<Particle>>,
) {
	let window = unwrap_or_return!(windows.get_primary());

	'particle: for (particle, particle_transform) in &particles {
		let particle_position = particle_transform.translation.truncate();
		for (deleter, deleter_transform) in &deleters {
			let deleter_position = deleter_transform.translation.truncate();
			let distance_squared = wrapping_offset_2d(
				particle_position,
				deleter_position,
				Vec2::new(window.width(), window.height()),
			)
			.length_squared();

			if distance_squared < deleter.radius_squared {
				commands.entity(particle).despawn();
				continue 'particle;
			}
		}
	}
}
