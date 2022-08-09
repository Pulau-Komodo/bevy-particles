use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
	common::{find_entity_by_cursor, wrapping_offset_2d},
	draw_order,
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
	mut commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
) {
	let action_state = action_state.single();
	if !action_state.just_pressed(Action::SpawnDeleter) {
		return;
	}
	let cursor_pos = unwrap_or_return!(windows
		.get_primary()
		.and_then(|window| window.cursor_position()));

	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::RED,
				..default()
			},
			transform: Transform {
				translation: cursor_pos.extend(draw_order::DELETER),
				scale: Vec3::new(15.0, 15.0, 15.0),
				..default()
			},
			..default()
		})
		.insert(ParticleDeleter::new(100.0));
}

fn despawn_particle_deleter(
	mut commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
	deleters: Query<(Entity, &Transform), With<ParticleDeleter>>,
) {
	let action_state = action_state.single();
	if !action_state.just_pressed(Action::DespawnDeleter) {
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
