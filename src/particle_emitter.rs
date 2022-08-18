use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
	common::{despawn_gizmo, spawn_gizmo},
	draw_order,
	input::Action,
	particle::spawn_particle_at_location,
};

pub struct ParticleEmitterPlugin;

impl Plugin for ParticleEmitterPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(spawn_particle_emitter)
			.add_system(despawn_particle_emitter)
			.add_system(activate_particle_emitters);
	}
}

#[derive(Component)]
struct ParticleEmitter {
	interval: f32,
	time_since_emitting: f32,
}

fn spawn_particle_emitter(
	commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
) {
	spawn_gizmo(
		commands,
		windows,
		action_state,
		Action::SpawnEmitter,
		Vec2::ONE * 15.0,
		draw_order::EMITTER,
		Color::GREEN,
		ParticleEmitter {
			interval: 0.1,
			time_since_emitting: 0.0,
		},
	);
}

fn despawn_particle_emitter(
	commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
	emitters: Query<(Entity, &Transform), With<ParticleEmitter>>,
) {
	despawn_gizmo(
		commands,
		windows,
		action_state,
		Action::DespawnEmitter,
		&emitters,
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
			spawn_particle_at_location(&mut commands, location);
			emitter.time_since_emitting -= emitter.interval;
		} else {
			emitter.time_since_emitting += time.delta_seconds();
		}
	}
}
