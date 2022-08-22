use bevy::prelude::*;

use crate::{
	common::Positive,
	particle::{spawn_particle_at_location, NextBatch},
};

#[derive(Component)]
pub struct Emitter {
	interval: f32,
	time_since_emitting: f32,
}

impl Emitter {
	pub const fn new() -> Self {
		Self {
			interval: 0.1,
			time_since_emitting: 0.0,
		}
	}
}

impl Default for Emitter {
	fn default() -> Self {
		Self::new()
	}
}

pub fn activate_emitters(
	mut commands: Commands,
	time: Res<Time>,
	mut next_batch: ResMut<NextBatch>,
	mut emitters: Query<(&mut Emitter, Option<&Positive>, &Transform)>,
) {
	for (mut emitter, positive, transform) in &mut emitters {
		let location = transform.translation.truncate();
		if emitter.time_since_emitting > emitter.interval {
			spawn_particle_at_location(
				&mut commands,
				&mut next_batch,
				location,
				positive.is_some(),
			);
			emitter.time_since_emitting -= emitter.interval;
		} else {
			emitter.time_since_emitting += time.delta_seconds();
		}
	}
}
