use bevy::prelude::*;

use crate::{common::wrapping_offset_2d, movement::Movement, particle::Particle, unwrap_or_return};

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
