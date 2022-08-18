use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
	common::wrapping_offset_2d,
	draw_properties::{self, DrawProperties},
	input::Action,
	particle_attractor::activate_particle_attractors,
	unwrap_or_return,
};

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<Inertia>()
			.add_startup_system(spawn_initial_particles)
			.add_system(spawn_particle)
			.add_system(toggle_inertia)
			.add_system(particles_applying_forces)
			.add_system(particles_cancelling)
			.add_system(despawn_cancelled_particles.after(particles_cancelling))
			.add_system(
				clamp_particle_speed
					.after(particles_applying_forces)
					.after(activate_particle_attractors),
			)
			.add_system(move_particles.after(clamp_particle_speed));
	}
}

/// The theoretical amount of force applied at 1 pixel distance.
const BASE_FORCE: f32 = 10000.0;
/// The force will be applied as if it always has at least this distance.
const PROXIMITY_FORCE_CAP: f32 = 5.0;
/// The power to which the distance is raised to diminish force. A higher number means force more quickly diminishes with distance.
const DIMINISHING_POWER: f32 = 2.0;
/// Maximum speed of a particle in units/second.
const MAX_PARTICLE_SPEED: f32 = 200.0;
/// The distance within which opposing-charge particles will cancel out.
const PARTICLE_CANCEL_DISTANCE: f32 = 4.0;
/// How many particles to spawn when the application first launches.
const INITIAL_PARTICLE_COUNT: u32 = 1000;

#[derive(Default)]
struct Inertia(bool);

fn toggle_inertia(mut inertia: ResMut<Inertia>, action_state: Query<&ActionState<Action>>) {
	if action_state.single().just_pressed(Action::ToggleInertia) {
		inertia.0 = !inertia.0;
	}
}

#[derive(Default, Component)]
pub struct Particle {
	positive: bool,
}

impl Particle {
	pub fn new(positive: bool) -> Self {
		Self { positive }
	}
}

#[derive(Default, Component)]
pub struct Movement(Vec2);

impl Movement {
	pub fn add(&mut self, movement: Vec2) {
		self.0 += movement;
	}
}

#[derive(Default, Component)]
pub struct Cancelled(bool);

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

	spawn_particle_at_location(&mut commands, cursor_pos, true);
}

fn particles_applying_forces(
	time: Res<Time>,
	windows: Res<Windows>,
	mut particles: Query<(&mut Movement, &Particle, &Transform)>,
	action_state: Query<&ActionState<Action>>,
) {
	let window = unwrap_or_return!(windows.get_primary());
	let action_state = action_state.single();

	if action_state.pressed(Action::SuspendRepulsion) {
		return;
	}

	let mut combinations = particles.iter_combinations_mut();
	while let Some(
		[(mut movement_a, particle_a, transform_a), (mut movement_b, particle_b, transform_b)],
	) = combinations.fetch_next()
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
		let invert_force = if particle_a.positive != particle_b.positive {
			-1.0
		} else {
			1.0
		};
		let force = BASE_FORCE
			/ offset
				.length()
				.max(PROXIMITY_FORCE_CAP)
				.powf(DIMINISHING_POWER)
			* offset.normalize()
			* time.delta_seconds()
			* invert_force;

		movement_a.0 += force;
		movement_b.0 -= force;
	}
}

fn particles_cancelling(
	windows: Res<Windows>,
	mut particles: Query<(&mut Cancelled, &Particle, &Transform)>,
) {
	let window = unwrap_or_return!(windows.get_primary());

	let mut combinations = particles.iter_combinations_mut();
	while let Some(
		[(mut cancelled_a, particle_a, transform_a), (mut cancelled_b, particle_b, transform_b)],
	) = combinations.fetch_next()
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

		if offset.length_squared() < PARTICLE_CANCEL_DISTANCE.powi(2)
			&& particle_a.positive != particle_b.positive
			&& !cancelled_a.0
			&& !cancelled_b.0
		{
			cancelled_a.0 = true;
			cancelled_b.0 = true;
		}
	}
}

fn despawn_cancelled_particles(mut commands: Commands, particles: Query<(Entity, &Cancelled)>) {
	for (entity, cancelled) in &particles {
		if cancelled.0 {
			commands.entity(entity).despawn();
		}
	}
}

fn clamp_particle_speed(
	time: Res<Time>,
	inertia: Res<Inertia>,
	mut particles: Query<&mut Movement, With<Particle>>,
) {
	if !inertia.0 {
		for mut particle in &mut particles {
			particle.0 = particle
				.0
				.clamp_length_max(MAX_PARTICLE_SPEED * time.delta_seconds());
		}
	}
}

fn move_particles(
	time: Res<Time>,
	windows: Res<Windows>,
	inertia: Res<Inertia>,
	mut particles: Query<(&mut Transform, &mut Movement), With<Particle>>,
) {
	let window = unwrap_or_return!(windows.get_primary());

	for (mut transform, mut movement) in &mut particles {
		let movement_to_apply = if inertia.0 {
			movement.0 * time.delta_seconds() * 0.5
		} else {
			movement.0
		};
		transform.translation += movement_to_apply.extend(0.0);
		transform.translation.x = transform.translation.x.rem_euclid(window.width());
		transform.translation.y = transform.translation.y.rem_euclid(window.height());
		if !inertia.0 {
			movement.0 = Vec2::ZERO;
		}
	}
}

fn spawn_initial_particles(mut commands: Commands, windows: Res<Windows>) {
	let window = unwrap_or_return!(windows.get_primary());
	let middle = Vec2::new(window.width(), window.height()) / 2.0;
	let smallest_dimension = f32::min(window.width(), window.height());
	let offset = Vec2::Y * smallest_dimension * 0.9 / 2.0;
	for n in 0..INITIAL_PARTICLE_COUNT {
		let position = middle
			+ Mat2::from_angle(
				n as f32 * std::f32::consts::PI * 2.0 / INITIAL_PARTICLE_COUNT as f32,
			) * offset;
		spawn_particle_at_location(&mut commands, position, true);
	}
}

#[derive(Default, Bundle)]
struct ParticleBundle {
	#[bundle]
	sprite_bundle: SpriteBundle,
	particle: Particle,
	movement: Movement,
	cancelled: Cancelled,
}

pub fn spawn_particle_at_location(commands: &mut Commands, position: Vec2, positive: bool) {
	let draw_properties = if positive {
		draw_properties::POSITIVE_PARTICLE
	} else {
		draw_properties::NEGATIVE_PARTICLE
	};
	let DrawProperties {
		draw_priority,
		size,
		color,
	} = draw_properties;

	commands.spawn_bundle(ParticleBundle {
		sprite_bundle: SpriteBundle {
			sprite: Sprite { color, ..default() },
			transform: Transform {
				translation: position.extend(draw_priority),
				scale: (Vec2::ONE * size).extend(1.0),
				..default()
			},
			..default()
		},
		particle: Particle::new(positive),
		..default()
	});
}
