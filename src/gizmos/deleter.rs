use bevy::prelude::*;

use crate::{common::wrapping_offset_2d, particle::Particle, WindowDimensions, TIMESTEP};

#[derive(Component)]
pub struct Deleter {
	radius_squared: f32,
}

impl Deleter {
	pub fn new(radius: f32) -> Self {
		Self {
			radius_squared: radius.powi(2),
		}
	}
}

impl Default for Deleter {
	fn default() -> Self {
		Self::new(100.0)
	}
}

pub fn activate_deleters(
	mut commands: Commands,
	window_dimensions: Res<WindowDimensions>,
	deleters: Query<(&Deleter, &Transform)>,
	particles: Query<(Entity, &Transform), With<Particle>>,
) {
	'particle: for (particle, particle_transform) in &particles {
		let particle_position = particle_transform.translation.truncate();
		for (deleter, deleter_transform) in &deleters {
			let deleter_position = deleter_transform.translation.truncate();
			let distance_squared =
				wrapping_offset_2d(particle_position, deleter_position, window_dimensions.0)
					.length_squared();

			if distance_squared < deleter.radius_squared {
				commands.entity(particle).despawn();
				continue 'particle;
			}
		}
	}
}

#[derive(Component)]
pub struct SlowDeleter {
	radius_squared: f32,
	rate: f32,
	charge: f32,
}

impl SlowDeleter {
	pub fn new(radius: f32, rate: f32) -> Self {
		Self {
			radius_squared: radius.powi(2), rate, charge: 0.0,
		}
	}
}

impl Default for SlowDeleter {
	fn default() -> Self {
		Self::new(100.0, 2.0)
	}
}

pub fn activate_slow_deleters(
	mut commands: Commands,
	window_dimensions: Res<WindowDimensions>,
	mut deleters: Query<(&mut SlowDeleter, &Transform)>,
	particles: Query<(Entity, &Transform), With<Particle>>,
) {
	'particle: for (particle, particle_transform) in &particles {
		let particle_position = particle_transform.translation.truncate();
		for (mut deleter, deleter_transform) in &mut deleters {
			if deleter.charge < 1.0 {
				continue;
			}
			let deleter_position = deleter_transform.translation.truncate();
			let distance_squared =
				wrapping_offset_2d(particle_position, deleter_position, window_dimensions.0)
					.length_squared();

			if distance_squared < deleter.radius_squared {
				commands.entity(particle).despawn();
				deleter.charge = 0.0;
				continue 'particle;
			}
		}
	}
}

pub(crate) fn recharge_slow_deleters(
	mut deleters: Query<&mut SlowDeleter>,
) {
	for mut deleter in &mut deleters {
		deleter.charge += deleter.rate / TIMESTEP;
	}
}