use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
	common::find_entity_by_cursor, draw_order, input::Action, particle::spawn_particle_at_location,
	unwrap_or_return,
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
	mut commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
) {
	let action_state = action_state.single();
	if !action_state.just_pressed(Action::SpawnEmitter) {
		return;
	}
	let cursor_pos = unwrap_or_return!(windows
		.get_primary()
		.and_then(|window| window.cursor_position()));

	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::GREEN,
				..default()
			},
			transform: Transform {
				translation: cursor_pos.extend(draw_order::EMITTER),
				scale: Vec3::new(15.0, 15.0, 1.0),
				..default()
			},
			..default()
		})
		.insert(ParticleEmitter {
			interval: 0.1,
			time_since_emitting: 0.0,
		});
}

fn despawn_particle_emitter(
	mut commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
	deleters: Query<(Entity, &Transform), With<ParticleEmitter>>,
) {
	let action_state = action_state.single();
	if !action_state.just_pressed(Action::DespawnEmitter) {
		return;
	}
	let window = unwrap_or_return!(windows.get_primary());
	let cursor_pos = unwrap_or_return!(window.cursor_position());

	if let Some(deleter) = find_entity_by_cursor(
		cursor_pos,
		Vec2::new(window.width(), window.height()),
		deleters.iter(),
	) {
		commands.entity(deleter).despawn();
	}
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
