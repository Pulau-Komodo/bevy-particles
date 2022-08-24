use bevy::prelude::*;

use crate::input::Action;
use crate::unwrap_or_return;

use leafwing_input_manager::prelude::ActionState;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<Inertia>()
			.add_system(toggle_inertia)
			.add_system(merge_speed)
			.add_system(clamp_speed.after(merge_speed))
			.add_system(apply_movement.after(clamp_speed));
	}
}

/// Maximum speed of any movement in units/second.
const MAX_SPEED: f32 = 200.0;

pub trait MovementTrait {
	fn add(&mut self, other: Vec2);
}

#[derive(Default, Component)]
pub struct Movement(Vec2);

impl MovementTrait for Movement {
	fn add(&mut self, movement: Vec2) {
		self.0 += movement;
	}
}

#[derive(Default, Component)]
pub struct MovementBatch2(Vec2);

impl MovementTrait for MovementBatch2 {
	fn add(&mut self, movement: Vec2) {
		self.0 += movement;
	}
}

pub fn apply_movement(
	time: Res<Time>,
	windows: Res<Windows>,
	inertia: Res<Inertia>,
	mut movers: Query<(&mut Transform, &mut Movement)>,
) {
	let window = unwrap_or_return!(windows.get_primary());
	let window_dimensions = Vec2::new(window.requested_width(), window.requested_height());

	for (mut transform, mut movement) in &mut movers {
		let movement_to_apply = if inertia.0 {
			movement.0 * time.delta_seconds() * 0.5
		} else {
			movement.0
		};
		transform.translation += movement_to_apply.extend(0.0);
		transform.translation.x = transform.translation.x.rem_euclid(window_dimensions.x);
		transform.translation.y = transform.translation.y.rem_euclid(window_dimensions.y);
		if !inertia.0 {
			movement.0 = Vec2::ZERO;
		}
	}
}

#[derive(Default)]
pub struct Inertia(bool);

pub fn toggle_inertia(mut inertia: ResMut<Inertia>, action_state: Query<&ActionState<Action>>) {
	if action_state.single().just_pressed(Action::ToggleInertia) {
		inertia.0 = !inertia.0;
	}
}

pub fn merge_speed(mut movers: Query<(&mut Movement, &mut MovementBatch2)>) {
	for (mut movement, mut movement_2) in &mut movers {
		movement.0 += movement_2.0;
		movement_2.0 = Vec2::ZERO;
	}
}

pub fn clamp_speed(time: Res<Time>, inertia: Res<Inertia>, mut movers: Query<&mut Movement>) {
	if !inertia.0 {
		for mut movement in &mut movers {
			movement.0 = movement
				.0
				.clamp_length_max(MAX_SPEED * time.delta_seconds());
		}
	}
}
