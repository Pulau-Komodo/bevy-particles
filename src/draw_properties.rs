use bevy::prelude::*;

pub struct DrawProperties {
	/// Higher means more towards front.
	pub draw_priority: f32,
	pub size: f32,
	pub color: Color,
}

pub const POSITIVE_PARTICLE: DrawProperties = DrawProperties {
	draw_priority: 5.0,
	size: 5.0,
	color: Color::rgb(1.0, 0.5, 0.5),
};

pub const NEGATIVE_PARTICLE: DrawProperties = DrawProperties {
	draw_priority: 5.0,
	size: 5.0,
	color: Color::rgb(0.5, 0.5, 1.0),
};

pub const POSITIVE_EMITTER: DrawProperties = DrawProperties {
	draw_priority: 1.0,
	size: 15.0,
	color: Color::RED,
};

pub const NEGATIVE_EMITTER: DrawProperties = DrawProperties {
	draw_priority: 1.0,
	size: 15.0,
	color: Color::BLUE,
};

pub const DELETER: DrawProperties = DrawProperties {
	draw_priority: 2.0,
	size: 20.0,
	color: Color::WHITE,
};

pub const ATTRACTOR: DrawProperties = DrawProperties {
	draw_priority: 1.5,
	size: 15.0,
	color: Color::PURPLE,
};

pub const POSITIVE_EATER: DrawProperties = DrawProperties {
	draw_priority: 4.0,
	size: 10.0,
	color: Color::rgb(1.0, 0.75, 0.75),
};

pub const NEGATIVE_EATER: DrawProperties = DrawProperties {
	draw_priority: 4.0,
	size: 10.0,
	color: Color::rgb(0.75, 0.75, 1.0),
};
