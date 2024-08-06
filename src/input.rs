use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

pub struct InputPlugin;

impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(InputManagerPlugin::<Action>::default())
			.add_systems(Startup, set_binds);
	}
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Reflect, Actionlike)]
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
		(Equal, PositiveEmitter),
		(Minus, NegativeEmitter),
		(Digit1, Deleter),
		(Digit2, Attractor),
		(BracketLeft, NegativeEater),
		(BracketRight, PositiveEater),
	];

	let left_click: InputKind = MouseButton::Left.into();

	let mut input_map = InputMap::default();
	input_map.insert(SpawnParticle, left_click);
	input_map.insert(DespawnAllModifier, AltLeft);
	input_map.insert(DespawnAllModifier, AltRight);
	input_map.insert(DespawnModifier, ShiftLeft);
	input_map.insert(DespawnModifier, ShiftRight);
	input_map.insert(RaiseParticleLimit, ArrowUp);
	input_map.insert(LowerParticleLimit, ArrowDown);
	input_map.insert(ToggleInertia, KeyI);

	for (key, action) in actions {
		input_map.insert(action, key);
	}

	commands.spawn(InputManagerBundle::<Action> {
		input_map,
		..default()
	});
}
