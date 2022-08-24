use bevy::prelude::*;

use crate::{
	common::{calculate_force, wrapping_offset_2d},
	movement::{Movement, MovementTrait},
	particle::Particle,
	unwrap_or_return,
};

#[derive(Component)]
pub struct Attractor {
	force: f32,
}

impl Default for Attractor {
	fn default() -> Self {
		Self { force: 10000.0 }
	}
}

pub fn activate_attractors(
	time: Res<Time>,
	windows: Res<Windows>,
	attractors: Query<(&Attractor, &Transform)>,
	mut particles: Query<(&mut Movement, &Transform), With<Particle>>,
) {
	let window = unwrap_or_return!(windows.get_primary());

	for (attractor, attractor_transform) in &attractors {
		let attractor_position = attractor_transform.translation.truncate();
		for (mut movement, particle_transform) in &mut particles {
			let offset = wrapping_offset_2d(
				attractor_position,
				particle_transform.translation.truncate(),
				Vec2::new(window.requested_width(), window.requested_height()),
			);
			let force = calculate_force(attractor.force, 10.0, 1.05, offset) * time.delta_seconds();

			movement.add(force);
		}
	}
}
