use bevy::prelude::*;

use crate::CLICK_RADIUS;

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
	entities: impl IntoIterator<Item = (Entity, &'a Transform)>,
) -> Option<Entity> {
	find_nearest_within_radius(window_dimensions, cursor_pos, CLICK_RADIUS, entities)
}

pub fn find_nearest_within_radius<'a, T>(
	window_dimensions: Vec2,
	position: Vec2,
	radius: f32,
	items: impl IntoIterator<Item = (T, &'a Transform)>,
) -> Option<T> {
	let radius_squared = radius.powi(2);

	items
		.into_iter()
		.filter_map(|(item, transform)| {
			let item_position = transform.translation.truncate();
			let distance_squared =
				wrapping_offset_2d(position, item_position, window_dimensions).length_squared();
			(distance_squared < radius_squared).then_some((item, distance_squared))
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
		.map(|(item, _)| item)
}

/// Generates a series of points in a circle around the midpoint.
pub fn circular_points(midpoint: Vec2, radius: f32, count: u32) -> impl Iterator<Item = Vec2> {
	let offset = Vec2::Y * radius;
	(0..count).map(move |n| {
		midpoint + Mat2::from_angle(n as f32 * std::f32::consts::PI * 2.0 / count as f32) * offset
	})
}

#[derive(Component)]
pub struct Positive;

#[inline]
pub fn calculate_force(base_force: f32, proximity_cap: f32, exponent: f32, offset: Vec2) -> Vec2 {
	if offset == Vec2::ZERO {
		return Vec2::ZERO;
	}

	base_force / offset.length().max(proximity_cap).powf(exponent) * offset.normalize()
}
