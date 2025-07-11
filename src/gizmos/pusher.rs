use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::prelude::ActionState;

use crate::{
	assets::TextureMap,
	draw_properties,
	input::Action,
	movement::{Movement, MovementTrait},
	particle::Particle,
	unwrap_or_return,
};

#[derive(Component, Debug, Clone, Copy, Default)]
pub(super) struct PusherPlacer;

#[derive(Component, Debug, Clone, Copy, Default)]
pub(super) struct Pusher;

pub(super) fn place_pusher(
	mut commands: Commands,
	texture_map: Res<TextureMap>,
	window: Query<&Window, With<PrimaryWindow>>,
	action_state: Query<&ActionState<Action>>,
	mut pusher_placer: Query<(Entity, &mut Transform), With<PusherPlacer>>,
) {
	let action_state = unwrap_or_return!(action_state.single().ok());
	let window = unwrap_or_return!(window.single().ok());
	let cursor_pos = unwrap_or_return!(window.cursor_position());
	let cursor_pos = Vec2::new(cursor_pos.x, window.height() - cursor_pos.y);

	let sprite_component = || Sprite {
		color: draw_properties::PUSHER.color,
		image: draw_properties::PUSHER
			.texture
			.and_then(|texture| texture_map.0.get(&texture))
			.cloned()
			.unwrap_or_default(),
		..default()
	};

	if action_state.just_pressed(&Action::Pusher) {
		commands.spawn((
			PusherPlacer,
			sprite_component(),
			Transform {
				translation: cursor_pos.extend(100.0),
				scale: (Vec2::ONE * draw_properties::PUSHER.size).extend(1.0),
				..default()
			},
		));
	} else if action_state.pressed(&Action::Pusher) {
		let Ok((_, mut transform)) = pusher_placer.single_mut() else {
			return;
		};
		let offset = cursor_pos - transform.translation.truncate();
		transform.rotation = Quat::from_rotation_z(offset.to_angle());
	} else if action_state.just_released(&Action::Pusher) {
		let Some((entity, transform)) = pusher_placer.iter_mut().next() else {
			return;
		};
		let offset = cursor_pos - transform.translation.truncate();
		commands.entity(entity).despawn();
		commands.spawn((
			Pusher,
			sprite_component(),
			Transform {
				translation: transform
					.translation
					.with_z(draw_properties::PUSHER.draw_priority),
				scale: (Vec2::ONE * draw_properties::PUSHER.size).extend(1.0),
				rotation: Quat::from_rotation_z(offset.to_angle()),
			},
		));
	} else {
		for (entity, _) in pusher_placer {
			commands.entity(entity).despawn();
		}
	}
}

pub(super) fn activate_pushers(
	pushers: Query<&Transform, With<Pusher>>,
	particles: Query<(&mut Movement, &Transform), With<Particle>>,
) {
	for (mut movement, particle_transform) in particles {
		for pusher_transform in pushers {
			let offset =
				pusher_transform.translation.truncate() - particle_transform.translation.truncate();
			let local_point = (pusher_transform.rotation.inverse() * offset.extend(0.0)).truncate();
			if local_point.x.abs() <= 200.0 && local_point.y.abs() <= 100.0 {
				movement.add((pusher_transform.rotation * Vec3::X).truncate() * 3.0);
			}
		}
	}
}
