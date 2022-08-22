use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
	common::{calculate_force, circular_points, wrapping_offset_2d, Positive},
	draw_properties::{self, DrawProperties},
	input::Action,
	movement::{merge_speed, Movement, MovementBatch2},
	unwrap_or_return,
};

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<NextBatch>()
			.add_startup_system(spawn_initial_particles)
			.add_system(spawn_particle)
			.add_system(despawn_all_particles)
			.add_system(particles_applying_forces.before(merge_speed))
			.add_system(particles_applying_forces_batch_2.before(merge_speed))
			.add_system(particles_cancelling)
			.add_system(despawn_cancelled_particles.after(particles_cancelling));
	}
}

/// The theoretical amount of force applied at 1 pixel distance.
const BASE_FORCE: f32 = 10_000.0;
/// The force will be applied as if it always has at least this distance.
const PROXIMITY_FORCE_CAP: f32 = 5.0;
/// The power to which the distance is raised to diminish force. A higher number means force more quickly diminishes with distance.
const DIMINISHING_POWER: f32 = 2.0;
/// The distance within which opposing-charge particles will cancel out.
const PARTICLE_CANCEL_DISTANCE: f32 = 4.0;
/// How many particles to spawn when the application first launches.
const INITIAL_PARTICLE_COUNT: u32 = 1000;

#[derive(Default, Component)]
pub struct Particle;

impl Particle {
	pub fn new() -> Self {
		Self
	}
}

#[derive(Default, Component)]
pub struct Cancelled(pub bool);

fn spawn_particle(
	mut commands: Commands,
	windows: Res<Windows>,
	mut next_batch: ResMut<NextBatch>,
	action_state: Query<&ActionState<Action>>,
) {
	let action_state = action_state.single();
	if !action_state.just_pressed(Action::SpawnParticle)
		|| action_state.pressed(Action::DespawnModifier)
	{
		return;
	}
	let cursor_pos = unwrap_or_return!(windows
		.get_primary()
		.and_then(|window| window.cursor_position()));

	spawn_particle_at_location(&mut commands, &mut next_batch, cursor_pos, true);
}

fn despawn_all_particles(
	mut commands: Commands,
	action_state: Query<&ActionState<Action>>,
	particles: Query<Entity, With<Particle>>,
) {
	let action_state = action_state.single();
	if !action_state.just_pressed(Action::SpawnParticle)
		|| !action_state.pressed(Action::DespawnModifier)
	{
		return;
	}

	for particle in &particles {
		commands.entity(particle).despawn();
	}
}

fn particles_applying_forces(
	time: Res<Time>,
	windows: Res<Windows>,
	mut particles: Query<
		(&mut Movement, Option<&Positive>, &Transform),
		(With<Particle>, Without<BatchTwo>),
	>,
	other_particles: Query<(Option<&Positive>, &Transform), (With<Particle>, With<BatchTwo>)>,
) {
	let window = unwrap_or_return!(windows.get_primary());

	let mut combinations = particles.iter_combinations_mut();
	while let Some(
		[(mut movement_a, positive_a, transform_a), (mut movement_b, positive_b, transform_b)],
	) = combinations.fetch_next()
	{
		if let Some(force) = calculate_force(
			BASE_FORCE,
			PROXIMITY_FORCE_CAP,
			DIMINISHING_POWER,
			transform_a.translation.truncate(),
			transform_b.translation.truncate(),
			Vec2::new(window.width(), window.height()),
		) {
			let invert_force = if positive_a.is_some() != positive_b.is_some() {
				-1.0
			} else {
				1.0
			};
			let force = force * time.delta_seconds() * invert_force;

			movement_a.add(force);
			movement_b.add(-force);
		}
	}
	for (mut movement, positive_a, transform_a) in &mut particles.iter_mut() {
		for (positive_b, transform_b) in &other_particles {
			if let Some(force) = calculate_force(
				BASE_FORCE,
				PROXIMITY_FORCE_CAP,
				DIMINISHING_POWER,
				transform_a.translation.truncate(),
				transform_b.translation.truncate(),
				Vec2::new(window.width(), window.height()),
			) {
				let invert_force = if positive_a.is_some() != positive_b.is_some() {
					-1.0
				} else {
					1.0
				};
				let force = force * time.delta_seconds() * invert_force;

				movement.add(force);
			}
		}
	}
}

fn particles_applying_forces_batch_2(
	time: Res<Time>,
	windows: Res<Windows>,
	mut particles: Query<
		(&mut MovementBatch2, Option<&Positive>, &Transform),
		(With<Particle>, With<BatchTwo>),
	>,
	other_particles: Query<(Option<&Positive>, &Transform), (With<Particle>, Without<BatchTwo>)>,
) {
	let window = unwrap_or_return!(windows.get_primary());

	let mut combinations = particles.iter_combinations_mut();
	while let Some(
		[(mut movement_a, positive_a, transform_a), (mut movement_b, positive_b, transform_b)],
	) = combinations.fetch_next()
	{
		if let Some(force) = calculate_force(
			BASE_FORCE,
			PROXIMITY_FORCE_CAP,
			DIMINISHING_POWER,
			transform_a.translation.truncate(),
			transform_b.translation.truncate(),
			Vec2::new(window.width(), window.height()),
		) {
			let invert_force = if positive_a.is_some() != positive_b.is_some() {
				-1.0
			} else {
				1.0
			};
			let force = force * time.delta_seconds() * invert_force;

			movement_a.add(force);
			movement_b.add(-force);
		}
	}
	for (mut movement, positive_a, transform_a) in &mut particles.iter_mut() {
		for (positive_b, transform_b) in &other_particles {
			if let Some(force) = calculate_force(
				BASE_FORCE,
				PROXIMITY_FORCE_CAP,
				DIMINISHING_POWER,
				transform_a.translation.truncate(),
				transform_b.translation.truncate(),
				Vec2::new(window.width(), window.height()),
			) {
				let invert_force = if positive_a.is_some() != positive_b.is_some() {
					-1.0
				} else {
					1.0
				};
				let force = force * time.delta_seconds() * invert_force;

				movement.add(force);
			}
		}
	}
}

fn particles_cancelling(
	windows: Res<Windows>,
	mut positive_particles: Query<(&mut Cancelled, &Transform), (With<Particle>, With<Positive>)>,
	mut negative_particles: Query<
		(&mut Cancelled, &Transform),
		(With<Particle>, Without<Positive>),
	>,
) {
	let window = unwrap_or_return!(windows.get_primary());

	for (mut cancelled_pos, transform_pos) in &mut positive_particles {
		for (mut cancelled_neg, transform_neg) in &mut negative_particles {
			if !cancelled_pos.0 && !cancelled_neg.0 {
				let offset = wrapping_offset_2d(
					transform_pos.translation.truncate(),
					transform_neg.translation.truncate(),
					Vec2::new(window.width(), window.height()),
				);
				if offset.length_squared() < PARTICLE_CANCEL_DISTANCE.powi(2) {
					cancelled_pos.0 = true;
					cancelled_neg.0 = true;
				}
			}
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

fn spawn_initial_particles(
	mut commands: Commands,
	mut next_batch: ResMut<NextBatch>,
	windows: Res<Windows>,
) {
	let window = unwrap_or_return!(windows.get_primary());

	let middle = Vec2::new(window.width(), window.height()) / 2.0;
	let smallest_dimension = f32::min(window.width(), window.height());

	for point in circular_points(
		middle,
		smallest_dimension * 0.9 / 2.0,
		INITIAL_PARTICLE_COUNT,
	) {
		spawn_particle_at_location(&mut commands, &mut next_batch, point, true);
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

pub fn spawn_particle_at_location(
	commands: &mut Commands,
	next_batch: &mut ResMut<NextBatch>,
	position: Vec2,
	positive: bool,
) {
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

	let mut entity_commands = commands.spawn_bundle(ParticleBundle {
		sprite_bundle: SpriteBundle {
			sprite: Sprite { color, ..default() },
			transform: Transform {
				translation: position.extend(draw_priority),
				scale: (Vec2::ONE * size).extend(1.0),
				..default()
			},
			..default()
		},
		particle: Particle::new(),
		..default()
	});
	if positive {
		entity_commands.insert(Positive);
	}
	if next_batch.0 == 1 {
		entity_commands.insert(BatchTwo);
		entity_commands.insert(MovementBatch2::default());
		next_batch.0 = 0;
	} else {
		next_batch.0 += 1;
	}
}

#[derive(Component)]
pub struct BatchTwo;

#[derive(Default)]
pub struct NextBatch(u8);
