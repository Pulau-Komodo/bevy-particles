use bevy::prelude::*;

use crate::{
	common::{calculate_force, wrapping_offset_2d},
	movement::{Movement, MovementTrait},
	particle::Particle,
	WindowDimensions, TIMESTEP,
};

#[derive(Component)]
pub struct Attractor {
	force: f32,
}

impl Attractor {
	pub fn invert(self) -> Self {
		Self { force: -self.force }
	}
}

impl Default for Attractor {
	fn default() -> Self {
		Self { force: 10000.0 }
	}
}

pub fn activate_attractors(
	window_dimensions: Res<WindowDimensions>,
	attractors: Query<(&Attractor, &Transform)>,
	mut particles: Query<(&mut Movement, &Transform), With<Particle>>,
) {
	for (attractor, attractor_transform) in &attractors {
		let attractor_position = attractor_transform.translation.truncate();
		for (mut movement, particle_transform) in &mut particles {
			let offset = wrapping_offset_2d(
				attractor_position,
				particle_transform.translation.truncate(),
				window_dimensions.0,
			);
			let force = calculate_force(attractor.force, 10.0, 1.05, offset) * TIMESTEP;

			movement.add(force);
		}
	}
}
