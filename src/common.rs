use bevy::prelude::*;

use crate::CLICK_RADIUS_SQUARED;

pub fn wrapping_offset_2d(first: Vec2, second: Vec2, wrap: Vec2) -> Vec2 {
	Vec2::new(
		wrapping_offset(first.x, second.x, wrap.x),
		wrapping_offset(first.y, second.y, wrap.y),
	)
}

fn wrapping_offset(first: f32, second: f32, wrap: f32) -> f32 {
	let offset = first - second;
	if offset.abs() > wrap / 2.0 {
		if first > second {
			offset - wrap
		} else {
			offset + wrap
		}
	} else {
		offset
	}
}

/// Find the entity closest to cursor within the click radius
pub fn find_entity_by_cursor<'a>(
	cursor_pos: Vec2,
	window_dimensions: Vec2,
	entities: impl Iterator<Item = (Entity, &'a Transform)>,
) -> Option<Entity> {
	entities
		.into_iter()
		.filter_map(|(entity, transform)| {
			let position = transform.translation.truncate();
			let distance_squared =
				wrapping_offset_2d(cursor_pos, position, window_dimensions).length_squared();
			(distance_squared < CLICK_RADIUS_SQUARED).then_some((entity, distance_squared))
		})
		.min_by(|(_, distance_a), (_, distance_b)| {
			if distance_a < distance_b {
				std::cmp::Ordering::Less
			} else if distance_a > distance_b {
				std::cmp::Ordering::Greater
			} else {
				std::cmp::Ordering::Equal
			}
		})
		.map(|(entity, _)| entity)
}
