use bevy::prelude::*;

use crate::input::Action;
use crate::unwrap_or_return;

use leafwing_input_manager::prelude::ActionState;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<Inertia>()
			.add_system(toggle_inertia)
			.add_system(clamp_speed)
			.add_system(apply_movement.after(clamp_speed));
	}
}

/// Maximum speed of any movement in units/second.
const MAX_SPEED: f32 = 200.0;

#[derive(Default, Component)]
pub struct Movement(Vec2);

impl Movement {
	pub fn add(&mut self, movement: Vec2) {
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

	for (mut transform, mut movement) in &mut movers {
		let movement_to_apply = if inertia.0 {
			movement.0 * time.delta_seconds() * 0.5
		} else {
			movement.0
		};
		transform.translation += movement_to_apply.extend(0.0);
		transform.translation.x = transform.translation.x.rem_euclid(window.width());
		transform.translation.y = transform.translation.y.rem_euclid(window.height());
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

pub fn clamp_speed(time: Res<Time>, inertia: Res<Inertia>, mut particles: Query<&mut Movement>) {
	if !inertia.0 {
		for mut particle in &mut particles {
			particle.0 = particle
				.0
				.clamp_length_max(MAX_SPEED * time.delta_seconds());
		}
	}
}
