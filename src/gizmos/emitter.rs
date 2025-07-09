use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
	TIMESTEP,
	common::Positive,
	input::Action,
	particle::{NextBatch, Particle, spawn_particle_at_location},
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
	limit: Res<ParticleLimit>,
	mut next_batch: ResMut<NextBatch>,
	particles: Query<(), With<Particle>>,
	mut emitters: Query<(&mut Emitter, Option<&Positive>, &Transform)>,
) {
	if particles.iter().len() >= limit.current() as usize {
		return;
	}

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
			emitter.time_since_emitting += TIMESTEP;
		}
	}
}

#[derive(Resource)]
pub struct ParticleLimit(u32);

impl ParticleLimit {
	fn raise(&mut self) {
		self.0 = self.0.saturating_add(100).min(u32::MAX - u32::MAX % 100);
	}
	fn lower(&mut self) {
		self.0 = self.0.saturating_sub(100);
	}
	pub fn current(&self) -> u32 {
		self.0
	}
}

impl Default for ParticleLimit {
	fn default() -> Self {
		Self(1_200)
	}
}

pub fn adjust_particle_limit(
	mut limit: ResMut<ParticleLimit>,
	action_state: Query<&ActionState<Action>>,
) {
	let action_state = action_state.single().unwrap();

	match (
		action_state.just_pressed(&Action::RaiseParticleLimit),
		action_state.just_pressed(&Action::LowerParticleLimit),
	) {
		(true, false) => limit.raise(),
		(false, true) => limit.lower(),
		_ => (),
	}
}
