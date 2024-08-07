use bevy::prelude::*;

use crate::{
	common::{calculate_force, offset_2d},
	movement::{Movement, MovementTrait},
	particle::Particle,
	WindowDimensions, WrappingForce, TIMESTEP,
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
	wrapping: Res<WrappingForce>,
	attractors: Query<(&Attractor, &Transform)>,
	mut particles: Query<(&mut Movement, &Transform), With<Particle>>,
) {
	for (attractor, attractor_transform) in &attractors {
		let attractor_position = attractor_transform.translation.truncate();
		for (mut movement, particle_transform) in &mut particles {
			let offset = offset_2d(
				attractor_position,
				particle_transform.translation.truncate(),
				wrapping.0.then_some(window_dimensions.0),
			);
			let force = calculate_force(attractor.force, 10.0, 1.05, offset) * TIMESTEP;

			movement.add(force);
		}
	}
}
