use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
	common::{despawn_gizmo, spawn_gizmo, wrapping_offset_2d},
	draw_properties,
	input::Action,
	movement::Movement,
	particle::Particle,
	unwrap_or_return,
};

pub struct ParticleAttractorPlugin;

impl Plugin for ParticleAttractorPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(spawn_particle_attractor)
			.add_system(despawn_particle_attractor)
			.add_system(activate_particle_attractors);
	}
}

#[derive(Component)]
pub struct ParticleAttractor {
	force: f32,
}

fn spawn_particle_attractor(
	mut commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
) {
	spawn_gizmo(
		&mut commands,
		windows,
		action_state,
		Action::SpawnAttractor,
		draw_properties::ATTRACTOR,
		(ParticleAttractor { force: 10000.0 },),
	);
}

fn despawn_particle_attractor(
	commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
	attractors: Query<(Entity, &Transform), With<ParticleAttractor>>,
) {
	despawn_gizmo(
		commands,
		windows,
		action_state,
		Action::DespawnAttractor,
		Action::DespawnAllAttractors,
		&attractors,
	);
}

pub fn activate_particle_attractors(
	time: Res<Time>,
	windows: Res<Windows>,
	attractors: Query<(&ParticleAttractor, &Transform)>,
	mut particles: Query<(&mut Movement, &Transform), With<Particle>>,
) {
	let window = unwrap_or_return!(windows.get_primary());

	for (attractor, attractor_transform) in &attractors {
		let attractor_position = attractor_transform.translation.truncate();
		for (mut movement, particle_transform) in &mut particles {
			let particle_position = particle_transform.translation.truncate();
			if attractor_position == particle_position {
				continue;
			}

			let offset = wrapping_offset_2d(
				attractor_position,
				particle_position,
				Vec2::new(window.width(), window.height()),
			);
			let force = attractor.force
				* offset.length_recip().min(0.1).powf(1.05)
				* offset.normalize()
				* time.delta_seconds();

			movement.add(force);
		}
	}
}
