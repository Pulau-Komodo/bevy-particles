use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
	common::{despawn_gizmo, spawn_gizmo},
	draw_properties,
	input::Action,
	particle::spawn_particle_at_location,
};

pub struct ParticleEmitterPlugin;

impl Plugin for ParticleEmitterPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(spawn_positive_particle_emitter)
			.add_system(spawn_negative_particle_emitter)
			.add_system(despawn_positive_particle_emitter)
			.add_system(despawn_negative_particle_emitter)
			.add_system(activate_particle_emitters);
	}
}

#[derive(Component)]
struct ParticleEmitter {
	interval: f32,
	time_since_emitting: f32,
	positive: bool,
}

fn spawn_positive_particle_emitter(
	commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
) {
	spawn_gizmo(
		commands,
		windows,
		action_state,
		Action::SpawnPositiveEmitter,
		draw_properties::POSITIVE_EMITTER,
		ParticleEmitter {
			interval: 0.1,
			time_since_emitting: 0.0,
			positive: true,
		},
	);
}

fn spawn_negative_particle_emitter(
	commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
) {
	spawn_gizmo(
		commands,
		windows,
		action_state,
		Action::SpawnNegativeEmitter,
		draw_properties::NEGATIVE_EMITTER,
		ParticleEmitter {
			interval: 0.1,
			time_since_emitting: 0.0,
			positive: false,
		},
	);
}

fn despawn_positive_particle_emitter(
	commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
	emitters: Query<(&ParticleEmitter, Entity, &Transform)>,
) {
	despawn_gizmo(
		commands,
		windows,
		action_state,
		Action::DespawnPositiveEmitter,
		Action::DespawnAllPositiveEmitters,
		emitters.iter().filter_map(|(emitter, entity, transform)| {
			emitter.positive.then_some((entity, transform))
		}),
	);
}

fn despawn_negative_particle_emitter(
	commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
	emitters: Query<(&ParticleEmitter, Entity, &Transform)>,
) {
	despawn_gizmo(
		commands,
		windows,
		action_state,
		Action::DespawnNegativeEmitter,
		Action::DespawnAllNegativeEmitters,
		emitters.iter().filter_map(|(emitter, entity, transform)| {
			(!emitter.positive).then_some((entity, transform))
		}),
	);
}

fn activate_particle_emitters(
	mut commands: Commands,
	time: Res<Time>,
	mut emitters: Query<(&mut ParticleEmitter, &Transform)>,
) {
	for (mut emitter, transform) in &mut emitters {
		let location = transform.translation.truncate();
		if emitter.time_since_emitting > emitter.interval {
			spawn_particle_at_location(&mut commands, location, emitter.positive);
			emitter.time_since_emitting -= emitter.interval;
		} else {
			emitter.time_since_emitting += time.delta_seconds();
		}
	}
}
