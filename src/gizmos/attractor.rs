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
	fall_off: f32,
	proximity_cap: f32,
}

impl Attractor {
	pub fn repulsor() -> Self {
		Self {
			force: -30000.0,
			fall_off: 1.5,
			proximity_cap: 2.0,
		}
	}
}

impl Default for Attractor {
	fn default() -> Self {
		Self {
			force: 10000.0,
			fall_off: 1.05,
			proximity_cap: 10.0,
		}
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
			let force =
				calculate_force(attractor.force, attractor.proximity_cap, attractor.fall_off, offset) * TIMESTEP;

			movement.add(force);
		}
	}
}
