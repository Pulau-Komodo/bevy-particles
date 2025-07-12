use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct DrawProperties {
	/// Higher means more towards front.
	pub draw_priority: f32,
	pub size: f32,
	pub color: Color,
	pub texture: Option<crate::assets::Texture>,
}

pub const POSITIVE_PARTICLE: DrawProperties = DrawProperties {
	draw_priority: 5.0,
	size: 1.0,
	color: Color::srgb(1.0, 0.5, 0.5),
	texture: Some(crate::assets::Texture::Particle),
};

pub const NEGATIVE_PARTICLE: DrawProperties = DrawProperties {
	draw_priority: 5.0,
	size: 1.0,
	color: Color::srgb(0.5, 0.5, 1.0),
	texture: Some(crate::assets::Texture::Particle),
};

pub const POSITIVE_EMITTER: DrawProperties = DrawProperties {
	draw_priority: 1.0,
	size: 15.0,
	color: Color::srgb(1.0, 0.0, 0.0),
	texture: None,
};

pub const NEGATIVE_EMITTER: DrawProperties = DrawProperties {
	draw_priority: 1.0,
	size: 15.0,
	color: Color::srgb(0.0, 0.0, 1.0),
	texture: None,
};

pub const DELETER: DrawProperties = DrawProperties {
	draw_priority: 2.0,
	size: 1.0,
	color: Color::WHITE,
	texture: Some(crate::assets::Texture::Deleter),
};

pub const SLOW_DELETER: DrawProperties = DrawProperties {
	draw_priority: 2.2,
	size: 1.0,
	color: Color::srgb(0.8, 0.8, 0.8),
	texture: Some(crate::assets::Texture::Deleter),
};

pub const ATTRACTOR: DrawProperties = DrawProperties {
	draw_priority: 1.5,
	size: 15.0,
	color: Color::srgb(0.5, 0.0, 0.5),
	texture: None,
};

pub const REPULSOR: DrawProperties = DrawProperties {
	draw_priority: 1.4,
	size: 16.0,
	color: Color::srgb(0.5, 1.0, 0.5),
	texture: None,
};

pub const PUSHER: DrawProperties = DrawProperties {
	draw_priority: 5.0,
	size: 1.0,
	color: Color::WHITE,
	texture: Some(crate::assets::Texture::Pusher),
};

pub const POSITIVE_EATER: DrawProperties = DrawProperties {
	draw_priority: 4.0,
	size: 10.0,
	color: Color::srgb(1.0, 0.75, 0.75),
	texture: None,
};

pub const NEGATIVE_EATER: DrawProperties = DrawProperties {
	draw_priority: 4.0,
	size: 10.0,
	color: Color::srgb(0.75, 0.75, 1.0),
	texture: None,
};
