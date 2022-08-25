use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

pub struct InputPlugin;

impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(InputManagerPlugin::<Action>::default())
			.add_startup_system(set_binds);
	}
}

#[derive(Debug, Clone, Actionlike)]
pub enum Action {
	SpawnParticle,
	PositiveEmitter,
	NegativeEmitter,
	Deleter,
	Attractor,
	PositiveEater,
	NegativeEater,
	DespawnModifier,
	DespawnAllModifier,
	RaiseParticleLimit,
	LowerParticleLimit,
	ToggleInertia,
}

fn set_binds(mut commands: Commands) {
	use Action::*;
	use KeyCode::*;

	let actions = vec![
		(Equals, PositiveEmitter),
		(Minus, NegativeEmitter),
		(Key1, Deleter),
		(Key2, Attractor),
		(LBracket, NegativeEater),
		(RBracket, PositiveEater),
	];

	let left_click: InputKind = MouseButton::Left.into();

	let mut input_map = InputMap::default();
	input_map.insert(left_click, SpawnParticle);
	input_map.insert(LAlt, DespawnAllModifier);
	input_map.insert(RAlt, DespawnAllModifier);
	input_map.insert(LShift, DespawnModifier);
	input_map.insert(RShift, DespawnModifier);
	input_map.insert(Up, RaiseParticleLimit);
	input_map.insert(Down, LowerParticleLimit);
	input_map.insert(I, ToggleInertia);

	for (key, action) in actions {
		input_map.insert(key, action);
	}

	commands.spawn_bundle(InputManagerBundle::<Action> {
		input_map,
		..default()
	});
}
