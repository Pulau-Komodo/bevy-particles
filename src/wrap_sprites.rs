use bevy::prelude::*;

use crate::WindowDimensions;

const NEIGHBOURS: [Vec2; 8] = [
	Vec2::NEG_ONE,
	Vec2::NEG_Y,
	Vec2::new(1.0, -1.0),
	Vec2::NEG_X,
	Vec2::X,
	Vec2::new(-1.0, 1.0),
	Vec2::Y,
	Vec2::ONE,
];

#[derive(Debug, Component)]
pub struct WrapSprite {
	direction: Vec2,
	parent: Entity,
}

#[derive(Debug, Component)]
pub struct WrapChildren([Entity; 8]);

/// To do: consider not bothering doing this for particles. But so far it seems to work fine, and strictly speaking it does affect their rendering.
pub fn add_wrap_sprites(
	mut commands: Commands,
	sprites: Query<(Entity, &Sprite, &Transform), (Without<WrapChildren>, Without<WrapSprite>)>,
) {
	for (entity, sprite, transform) in sprites {
		let children = NEIGHBOURS.map(|direction| {
			commands
				.spawn((
					sprite.clone(),
					*transform,
					WrapSprite {
						direction,
						parent: entity,
					},
				))
				.id()
		});
		commands.entity(entity).insert(WrapChildren(children));
	}
}

pub fn update_wrap_sprites(
	sprites: Query<&Transform, (With<WrapChildren>, Without<WrapSprite>)>,
	mut wrap_sprites: Query<(&mut Transform, &WrapSprite)>,
	window_dimensions: Res<WindowDimensions>,
) {
	wrap_sprites
		.par_iter_mut()
		.for_each(|(mut transform, wrap_sprite)| {
			let Ok(mut new_transform) = sprites.get(wrap_sprite.parent).copied() else {
				return;
			};
			new_transform.translation +=
				(wrap_sprite.direction * window_dimensions.get()).extend(0.0);
			*transform = new_transform;
		});
}

pub fn despawn_wrap_sprites(
	event: On<Despawn<WrapChildren>>,
	mut commands: Commands,
	parents: Query<&WrapChildren>,
) {
	let Ok(wrap_children) = parents.get(event.entity) else {
		return;
	};
	for child in wrap_children.0 {
		commands.entity(child).despawn();
	}
}
