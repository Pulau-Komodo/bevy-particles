use bevy::prelude::*;

use crate::{
	common::{
		calculate_force, circular_points, find_nearest_within_radius, wrapping_offset_2d, Positive,
	},
	draw_properties,
	movement::{Movement, MovementTrait},
	particle::{spawn_particle_at_location, Cancelled, NextBatch, Particle},
	unwrap_or_return,
};

/// The radius inside the particle eater will eat particles.
const EATER_RADIUS: f32 = 20.0;
/// The size the particle eater will be multiplied as it fills up. This is the size it would have at full, but it won't actually reach it, because being full shrinks it.
const EATER_FULL_SCALE: f32 = 2.0;
/// The theoretical amount of force applied to the particle eater at 1 pixel distance.
const BASE_PURSUIT_FORCE: f32 = 5_000.0;
/// The force will be applied as if it always has at least this distance.
const PROXIMITY_FORCE_CAP: f32 = 5.0;
/// The power to which the distance is raised to diminish force. A higher number means force more quickly diminishes with distance.
const DIMINISHING_POWER: f32 = 1.5;

#[derive(Component)]
pub struct Eater {
	eaten: u8,
	target: u8,
}

impl Eater {
	pub const fn new(target: u8) -> Self {
		Self { eaten: 0, target }
	}
	fn is_full(&self) -> bool {
		self.eaten >= self.target
	}
}

impl Default for Eater {
	fn default() -> Self {
		Self::new(10)
	}
}

/// An entity that should suspend activity for attached number of seconds.
#[derive(Component)]
pub struct Dormant(f32);

pub fn activate_eaters(
	mut commands: Commands,
	windows: Res<Windows>,
	mut next_batch: ResMut<NextBatch>,
	mut eaters: Query<(Entity, &mut Eater, Option<&Positive>, &Transform), Without<Dormant>>,
	mut particles: Query<(Option<&Positive>, &mut Cancelled, &Transform), With<Particle>>,
) {
	let window = unwrap_or_return!(windows.get_primary());

	for (particle_positive, mut cancelled, particle_transform) in particles
		.iter_mut()
		.filter(|(_, cancelled, _)| !cancelled.0)
	{
		let particle_position = particle_transform.translation.truncate();
		if let Some((entity, mut eater, eater_positive, eater_location)) =
			find_nearest_within_radius(
				Vec2::new(window.width(), window.height()),
				particle_position,
				EATER_RADIUS,
				eaters
					.iter_mut()
					.filter_map(|(entity, eater, positive, transform)| {
						(!eater.is_full() && particle_positive.is_some() != positive.is_some())
							.then_some((
								(entity, eater, positive, transform.translation.truncate()),
								transform,
							))
					}),
			) {
			eater.eaten += 1;
			cancelled.0 = true;
			if eater.is_full() {
				commands.entity(entity).insert(Dormant(10.0));
				for position in circular_points(eater_location, 25.0, eater.target as u32) {
					spawn_particle_at_location(
						&mut commands,
						&mut next_batch,
						position,
						eater_positive.is_some(),
					);
				}
			}
			continue;
		}
	}
}

pub fn eaters_chasing_particles(
	time: Res<Time>,
	windows: Res<Windows>,
	mut eaters: Query<
		(Option<&Positive>, &mut Movement, &Transform),
		(With<Eater>, Without<Dormant>),
	>,
	particles: Query<(Option<&Positive>, &Transform), With<Particle>>,
) {
	let window = unwrap_or_return!(windows.get_primary());

	for (eater_positive, mut eater_movement, eater_transform) in &mut eaters {
		let eater_position = eater_transform.translation.truncate();

		for particle_transform in particles.iter().filter_map(|(positive, transform)| {
			(positive.is_some() != eater_positive.is_some()).then_some(transform)
		}) {
			let offset = wrapping_offset_2d(
				eater_position,
				particle_transform.translation.truncate(),
				Vec2::new(window.width(), window.height()),
			);
			let force = calculate_force(
				BASE_PURSUIT_FORCE,
				PROXIMITY_FORCE_CAP,
				DIMINISHING_POWER,
				offset,
			) * time.delta_seconds();

			eater_movement.add(-force);
		}
	}
}

pub fn apply_eater_scale(mut eaters: Query<(&Eater, &mut Transform)>) {
	for (eater, mut transform) in &mut eaters {
		if eater.is_full() {
			transform.scale = Vec3::ONE * draw_properties::POSITIVE_EATER.size * 0.5;
		} else {
			transform.scale = Vec3::ONE
				* draw_properties::POSITIVE_EATER.size
				* (1.0 + eater.eaten as f32 / eater.target as f32 * (EATER_FULL_SCALE - 1.0));
		}
	}
}

pub fn process_dormant_eaters(
	mut commands: Commands,
	time: Res<Time>,
	mut eaters: Query<(Entity, &mut Eater, &mut Dormant)>,
) {
	for (entity, mut eater, mut dormant) in &mut eaters {
		dormant.0 -= time.delta_seconds();
		if dormant.0 <= 0.0 {
			eater.eaten = 0;
			commands.entity(entity).remove::<Dormant>();
		}
	}
}
