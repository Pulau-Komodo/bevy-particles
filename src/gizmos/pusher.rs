use bevy::prelude::*;

use crate::{
	WindowDimensions,
	common::wrapping_offset_2d,
	gizmos::BeingPlaced,
	movement::{Movement, MovementTrait},
	particle::Particle,
};

#[derive(Component, Debug, Clone, Copy, Default)]
pub(super) struct Pusher;

pub(super) fn activate_pushers(
	window_dimensions: Res<WindowDimensions>,
	pushers: Query<&Transform, (With<Pusher>, Without<BeingPlaced>)>,
	particles: Query<(&mut Movement, &Transform), With<Particle>>,
) {
	for (mut movement, particle_transform) in particles {
		for pusher_transform in pushers {
			let offset = wrapping_offset_2d(
				pusher_transform.translation.truncate(),
				particle_transform.translation.truncate(),
				window_dimensions.0,
			);
			let _offset =
				pusher_transform.translation.truncate() - particle_transform.translation.truncate();
			let local_point = (pusher_transform.rotation.inverse() * offset.extend(0.0)).truncate();
			if local_point.x.abs() <= 200.0 && local_point.y.abs() <= 100.0 {
				movement.add((pusher_transform.rotation * Vec3::X).truncate() * 3.0);
			}
		}
	}
}
