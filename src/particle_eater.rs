use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
	common::{
		circular_points, despawn_gizmo, find_nearest_within_radius, spawn_gizmo, wrapping_offset_2d,
	},
	draw_properties,
	input::Action,
	movement::Movement,
	particle::{spawn_particle_at_location, Cancelled, Particle},
	unwrap_or_return,
};

pub struct ParticleEaterPlugin;

impl Plugin for ParticleEaterPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(spawn_positive_particle_eater)
			.add_system(spawn_negative_particle_eater)
			.add_system(despawn_positive_particle_eater)
			.add_system(despawn_negative_particle_eater)
			.add_system(activate_particle_eaters)
			.add_system(eaters_chasing_particles)
			.add_system(apply_eater_scale)
			.add_system(wake_up_dormant);
	}
}

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
pub struct ParticleEater {
	positive: bool,
	eaten: u8,
	target: u8,
}

impl ParticleEater {
	const fn new(positive: bool) -> Self {
		Self {
			positive,
			eaten: 0,
			target: 10,
		}
	}
	fn is_full(&self) -> bool {
		self.eaten >= self.target
	}
}

/// An entity that should suspend activity for attached number of seconds.
#[derive(Component)]
pub struct Dormant(f32);

fn spawn_positive_particle_eater(
	mut commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
) {
	spawn_gizmo(
		&mut commands,
		windows,
		action_state,
		Action::SpawnPositiveEater,
		draw_properties::POSITIVE_EATER,
		(ParticleEater::new(true), Movement::default()),
	);
}

fn spawn_negative_particle_eater(
	mut commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
) {
	spawn_gizmo(
		&mut commands,
		windows,
		action_state,
		Action::SpawnNegativeEater,
		draw_properties::NEGATIVE_EATER,
		(ParticleEater::new(false), Movement::default()),
	);
}

fn despawn_positive_particle_eater(
	commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
	eaters: Query<(&ParticleEater, Entity, &Transform)>,
) {
	despawn_gizmo(
		commands,
		windows,
		action_state,
		Action::DespawnPositiveEater,
		Action::DespawnAllPositiveEaters,
		eaters
			.iter()
			.filter_map(|(eater, entity, transform)| eater.positive.then_some((entity, transform))),
	);
}

fn despawn_negative_particle_eater(
	commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&ActionState<Action>>,
	eaters: Query<(&ParticleEater, Entity, &Transform)>,
) {
	despawn_gizmo(
		commands,
		windows,
		action_state,
		Action::DespawnNegativeEater,
		Action::DespawnAllNegativeEaters,
		eaters.iter().filter_map(|(eater, entity, transform)| {
			(!eater.positive).then_some((entity, transform))
		}),
	);
}

fn activate_particle_eaters(
	mut commands: Commands,
	windows: Res<Windows>,
	mut eaters: Query<(Entity, &mut ParticleEater, &Transform), Without<Dormant>>,
	mut particles: Query<(&Particle, &mut Cancelled, &Transform)>,
) {
	let window = unwrap_or_return!(windows.get_primary());

	for (particle, mut cancelled, particle_transform) in particles
		.iter_mut()
		.filter(|(_, cancelled, _)| !cancelled.0)
	{
		let particle_position = particle_transform.translation.truncate();
		if let Some((entity, mut eater, eater_location)) = find_nearest_within_radius(
			Vec2::new(window.width(), window.height()),
			particle_position,
			EATER_RADIUS,
			eaters.iter_mut().filter_map(|(entity, eater, transform)| {
				(!eater.is_full() && particle.is_positive() != eater.positive)
					.then_some(((entity, eater, transform.translation.truncate()), transform))
			}),
		) {
			eater.eaten += 1;
			cancelled.0 = true;
			if eater.is_full() {
				commands.entity(entity).insert(Dormant(10.0));
				for position in circular_points(eater_location, 25.0, eater.target as u32) {
					spawn_particle_at_location(&mut commands, position, eater.positive);
				}
			}
			continue;
		}
	}
}

pub fn eaters_chasing_particles(
	time: Res<Time>,
	windows: Res<Windows>,
	mut eaters: Query<(&ParticleEater, &mut Movement, &Transform), Without<Dormant>>,
	particles: Query<(&Particle, &Transform)>,
) {
	let window = unwrap_or_return!(windows.get_primary());

	for (eater, mut eater_movement, eater_transform) in &mut eaters {
		let eater_position = eater_transform.translation.truncate();

		for particle_transform in particles.iter().filter_map(|(particle, transform)| {
			(particle.is_positive() != eater.positive).then_some(transform)
		}) {
			let offset = wrapping_offset_2d(
				eater_position,
				particle_transform.translation.truncate(),
				Vec2::new(window.width(), window.height()),
			);
			let force = BASE_PURSUIT_FORCE
				/ offset
					.length()
					.max(PROXIMITY_FORCE_CAP)
					.powf(DIMINISHING_POWER)
				* offset.normalize()
				* time.delta_seconds();

			eater_movement.add(-force);
		}
	}
}

fn apply_eater_scale(mut eaters: Query<(&ParticleEater, &mut Transform)>) {
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

fn wake_up_dormant(
	mut commands: Commands,
	time: Res<Time>,
	mut eaters: Query<(Entity, &mut ParticleEater, &mut Dormant)>,
) {
	for (entity, mut eater, mut dormant) in &mut eaters {
		dormant.0 -= time.delta_seconds();
		if dormant.0 <= 0.0 {
			eater.eaten = 0;
			commands.entity(entity).remove::<Dormant>();
		}
	}
}
