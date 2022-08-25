use bevy::prelude::*;

use crate::{common::wrapping_offset_2d, particle::Particle, WindowDimensions};

#[derive(Component)]
pub struct Deleter {
	radius_squared: f32,
}

impl Deleter {
	pub fn _new(radius: f32) -> Self {
		Self {
			radius_squared: radius.powi(2),
		}
	}
}

impl Default for Deleter {
	fn default() -> Self {
		Self {
			radius_squared: 10_000.0,
		}
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
