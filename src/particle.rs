use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
	common::wrapping_offset_2d, draw_order, input::Action,
	particle_attractor::activate_particle_attractors, unwrap_or_return,
};

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
	fn build(&self, app: &mut App) {
		app.add_system(spawn_particle)
			.add_system(particles_repelling_each_other)
			.add_system(
				clamp_particle_speed
					.after(particles_repelling_each_other)
					.after(activate_particle_attractors),
			)
			.add_system(move_particles.after(clamp_particle_speed));
	}
}

/// Maximum speed of a particle in units/second.
const MAX_PARTICLE_SPEED: f32 = 200.0;

#[derive(Component, Default)]
pub struct Particle {
	movement: Vec2,
}

impl Particle {
	pub fn add_movement(&mut self, movement: Vec2) {
		self.movement += movement;
	}
}

fn spawn_particle(
	mut commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
) {
	let action_state = action_state.single();
	if !action_state.just_pressed(Action::SpawnParticle) {
		return;
	}
	let cursor_pos = unwrap_or_return!(windows
		.get_primary()
		.and_then(|window| window.cursor_position()));

	spawn_particle_at_location(&mut commands, cursor_pos);
}

fn particles_repelling_each_other(
	time: Res<Time>,
	windows: Res<Windows>,
	mut particles: Query<(&mut Particle, &Transform)>,
	action_state: Query<&ActionState<Action>>,
) {
	let window = unwrap_or_return!(windows.get_primary());
	let action_state = action_state.single();

	if action_state.pressed(Action::SuspendRepulsion) {
		return;
	}

	let mut combinations = particles.iter_combinations_mut();
	while let Some([(mut particle_a, transform_a), (mut particle_b, transform_b)]) =
		combinations.fetch_next()
	{
		let position_a = transform_a.translation.truncate();
		let position_b = transform_b.translation.truncate();
		if position_a == position_b {
			continue;
		}

		let offset = wrapping_offset_2d(
			position_a,
			position_b,
			Vec2::new(window.width(), window.height()),
		);
		let force = 10000.0
			* offset.length_recip().min(0.2).powf(2.0)
			* offset.normalize()
			* time.delta_seconds();

		particle_a.movement += force;
		particle_b.movement -= force;
	}
}

fn clamp_particle_speed(time: Res<Time>, mut particles: Query<&mut Particle>) {
	for mut particle in &mut particles {
		particle.movement = particle
			.movement
			.clamp_length_max(MAX_PARTICLE_SPEED * time.delta_seconds());
	}
}

fn move_particles(windows: Res<Windows>, mut particles: Query<(&mut Transform, &mut Particle)>) {
	let window = unwrap_or_return!(windows.get_primary());

	for (mut transform, mut particle) in &mut particles {
		transform.translation += particle.movement.extend(0.0);
		transform.translation.x = transform.translation.x.rem_euclid(window.width());
		transform.translation.y = transform.translation.y.rem_euclid(window.height());
		particle.movement = Vec2::ZERO;
	}
}

#[derive(Bundle)]
struct ParticleBundle {
	#[bundle]
	sprite_bundle: SpriteBundle,
	particle: Particle,
}

pub fn spawn_particle_at_location(commands: &mut Commands, position: Vec2) {
	commands.spawn_bundle(ParticleBundle {
		sprite_bundle: SpriteBundle {
			sprite: Sprite {
				color: Color::WHITE,
				..default()
			},
			transform: Transform {
				translation: position.extend(draw_order::PARTICLE),
				scale: Vec3::new(5.0, 5.0, 1.0),
				..default()
			},
			..default()
		},
		particle: Particle::default(),
	});
}
