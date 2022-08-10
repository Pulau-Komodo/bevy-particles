use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
	common::{find_entity_by_cursor, wrapping_offset_2d},
	draw_order,
	input::Action,
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
	let action_state = action_state.single();
	if !action_state.just_pressed(Action::SpawnAttractor) {
		return;
	}
	let cursor_pos = unwrap_or_return!(windows
		.get_primary()
		.and_then(|window| window.cursor_position()));

	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::PURPLE,
				..default()
			},
			transform: Transform {
				translation: cursor_pos.extend(draw_order::ATTRACTOR),
				scale: Vec3::new(15.0, 15.0, 1.0),
				..default()
			},
			..default()
		})
		.insert(ParticleAttractor { force: 10000.0 });
}

fn despawn_particle_attractor(
	mut commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
	deleters: Query<(Entity, &Transform), With<ParticleAttractor>>,
) {
	let action_state = action_state.single();
	if !action_state.just_pressed(Action::DespawnAttractor) {
		return;
	}
	let window = unwrap_or_return!(windows.get_primary());
	let cursor_pos = unwrap_or_return!(window.cursor_position());

	if let Some(attractor) = find_entity_by_cursor(
		cursor_pos,
		Vec2::new(window.width(), window.height()),
		deleters.iter(),
	) {
		commands.entity(attractor).despawn();
	}
}

pub fn activate_particle_attractors(
	time: Res<Time>,
	windows: Res<Windows>,
	attractors: Query<(&ParticleAttractor, &Transform)>,
	mut particles: Query<(&mut Particle, &Transform)>,
) {
	let window = unwrap_or_return!(windows.get_primary());

	for (attractor, attractor_transform) in &attractors {
		let attractor_position = attractor_transform.translation.truncate();
		for (mut particle, particle_transform) in &mut particles {
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

			particle.add_movement(force);
		}
	}
}
