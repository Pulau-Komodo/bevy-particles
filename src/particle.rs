use bevy::{
	ecs::query::{QueryFilter, WorldQuery},
	prelude::*,
	window::PrimaryWindow,
};
use leafwing_input_manager::prelude::ActionState;

use crate::{
	TIMESTEP, WindowDimensions, WrappingForce,
	assets::TextureMap,
	common::{Positive, calculate_force, circular_points, offset_2d, wrapping_offset_2d},
	draw_properties::{self, DrawProperties},
	input::Action,
	movement::{Movement, MovementBatch2, MovementTrait, merge_speed},
	unwrap_or_return,
};

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<NextBatch>()
			.add_systems(Startup, spawn_initial_particles)
			.add_systems(
				Update,
				(
					(spawn_particle, give_particles_sprites).chain(),
					despawn_all_particles,
				),
			)
			.add_systems(
				FixedUpdate,
				(
					(
						particles_applying_forces::<Movement, Without<BatchTwo>, With<BatchTwo>>,
						particles_applying_forces::<
							MovementBatch2,
							With<BatchTwo>,
							Without<BatchTwo>,
						>,
					)
						.before(merge_speed),
					(particles_cancelling, despawn_cancelled_particles).chain(),
				),
			);
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

#[derive(Default, Component)]
pub struct Cancelled(pub bool);

fn spawn_particle(
	mut commands: Commands,
	window: Query<&Window, With<PrimaryWindow>>,
	mut next_batch: ResMut<NextBatch>,
	action_state: Query<&ActionState<Action>>,
) {
	let action_state = action_state.single().unwrap();
	if !action_state.just_pressed(&Action::SpawnParticle)
		|| action_state.pressed(&Action::DespawnModifier)
	{
		return;
	}
	let cursor_pos = unwrap_or_return!(window.single().ok().and_then(|window| {
		window
			.cursor_position()
			.map(|pos| Vec2::new(pos.x, window.height() - pos.y))
	}));

	spawn_particle_at_location(&mut commands, &mut next_batch, cursor_pos, true);
}

fn despawn_all_particles(
	mut commands: Commands,
	action_state: Query<&ActionState<Action>>,
	particles: Query<Entity, With<Particle>>,
) {
	let action_state = action_state.single().unwrap();
	if !action_state.just_pressed(&Action::SpawnParticle)
		|| !action_state.pressed(&Action::DespawnModifier)
	{
		return;
	}

	for particle in &particles {
		commands.entity(particle).despawn();
	}
}

fn particles_applying_forces<M, F, F2>(
	window_dimensions: Res<WindowDimensions>,
	wrapping: Res<WrappingForce>,
	mut particles: Query<(&mut M, Option<&Positive>, &Transform), (With<Particle>, F)>,
	other_particles: Query<(Option<&Positive>, &Transform), (With<Particle>, F2)>,
) where
	M: Component<Mutability = bevy::ecs::component::Mutable> + MovementTrait,
	F: WorldQuery + QueryFilter,
	F2: WorldQuery + QueryFilter,
{
	let mut combinations = particles.iter_combinations_mut();
	while let Some(
		[
			(mut movement_a, positive_a, transform_a),
			(mut movement_b, positive_b, transform_b),
		],
	) = combinations.fetch_next()
	{
		let force = calculate_force(
			BASE_FORCE,
			PROXIMITY_FORCE_CAP,
			DIMINISHING_POWER,
			offset_2d(
				transform_a.translation.truncate(),
				transform_b.translation.truncate(),
				wrapping.0.then_some(window_dimensions.0),
			),
		);
		let invert_force = if positive_a.is_some() != positive_b.is_some() {
			-1.0
		} else {
			1.0
		};
		let force = force * TIMESTEP * invert_force;

		movement_a.add(force);
		movement_b.add(-force);
	}
	drop(combinations);
	for (mut movement, positive_a, transform_a) in &mut particles.iter_mut() {
		for (positive_b, transform_b) in &other_particles {
			let force = calculate_force(
				BASE_FORCE,
				PROXIMITY_FORCE_CAP,
				DIMINISHING_POWER,
				offset_2d(
					transform_a.translation.truncate(),
					transform_b.translation.truncate(),
					wrapping.0.then_some(window_dimensions.0),
				),
			);
			let invert_force = if positive_a.is_some() != positive_b.is_some() {
				-1.0
			} else {
				1.0
			};
			let force = force * TIMESTEP * invert_force;

			movement.add(force);
		}
	}
}

fn particles_cancelling(
	window_dimensions: Res<WindowDimensions>,
	mut positive_particles: Query<(&mut Cancelled, &Transform), (With<Particle>, With<Positive>)>,
	mut negative_particles: Query<
		(&mut Cancelled, &Transform),
		(With<Particle>, Without<Positive>),
	>,
) {
	for (mut cancelled_pos, transform_pos) in &mut positive_particles {
		for (mut cancelled_neg, transform_neg) in &mut negative_particles {
			if !cancelled_pos.0 && !cancelled_neg.0 {
				let offset = wrapping_offset_2d(
					transform_pos.translation.truncate(),
					transform_neg.translation.truncate(),
					window_dimensions.0,
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
	window_dimensions: Res<WindowDimensions>,
) {
	let middle = window_dimensions.0 / 2.0;
	let smallest_dimension = f32::min(window_dimensions.0.x, window_dimensions.0.y);

	for point in circular_points(
		middle,
		smallest_dimension * 0.9 / 2.0,
		INITIAL_PARTICLE_COUNT,
	) {
		spawn_particle_at_location(&mut commands, &mut next_batch, point, true);
	}
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
		..
	} = draw_properties;

	let mut entity_commands = commands.spawn((
		Particle,
		Movement::default(),
		Cancelled::default(),
		Transform {
			translation: position.extend(draw_priority),
			scale: (Vec2::ONE * size).extend(1.0),
			..default()
		},
	));
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

fn give_particles_sprites(
	mut commands: Commands,
	texture_map: Res<TextureMap>,
	particles: Query<(Entity, Has<Positive>), (With<Particle>, Without<Sprite>)>,
) {
	for (entity, positive) in particles {
		let draw_properties = if positive {
			draw_properties::POSITIVE_PARTICLE
		} else {
			draw_properties::NEGATIVE_PARTICLE
		};
		let image = draw_properties
			.texture
			.and_then(|texture| texture_map.0.get(&texture))
			.cloned()
			.unwrap_or_default();
		commands.entity(entity).insert(Sprite {
			color: draw_properties.color,
			image,
			..default()
		});
	}
}

#[derive(Component)]
pub struct BatchTwo;

#[derive(Resource, Default)]
pub struct NextBatch(u8);
