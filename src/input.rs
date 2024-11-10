use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

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
	SlowDeleter,
	Attractor,
	Repulsor,
	PositiveEater,
	NegativeEater,
	DespawnModifier,
	DespawnAllModifier,
	RaiseParticleLimit,
	LowerParticleLimit,
	ToggleInertia,
	ToggleWrap,
}

fn set_binds(mut commands: Commands) {
	use Action::*;
	use KeyCode::*;

	let actions = vec![
		(Equal, PositiveEmitter),
		(Minus, NegativeEmitter),
		(Digit1, Deleter),
		(Backquote, SlowDeleter),
		(Digit2, Attractor),
		(Digit3, Repulsor),
		(BracketLeft, NegativeEater),
		(BracketRight, PositiveEater),
	];

	let mut input_map = InputMap::default();
	input_map.insert(SpawnParticle, MouseButton::Left);
	input_map.insert(DespawnAllModifier, ControlLeft);
	input_map.insert(DespawnAllModifier, ControlRight);
	input_map.insert(DespawnModifier, ShiftLeft);
	input_map.insert(DespawnModifier, ShiftRight);
	input_map.insert(RaiseParticleLimit, ArrowUp);
	input_map.insert(LowerParticleLimit, ArrowDown);
	input_map.insert(ToggleInertia, KeyI);
	input_map.insert(ToggleWrap, KeyW);

	for (key, action) in actions {
		input_map.insert(action, key);
	}

	commands.spawn(InputManagerBundle::<Action> {
		input_map,
		..default()
	});
}
