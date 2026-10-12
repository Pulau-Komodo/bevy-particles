use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(InputManagerPlugin::<Action>::default())
			.add_systems(Startup, set_binds);
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Reflect, Actionlike)]
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
	Pusher,
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
		(Period, Pusher),
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

	print_binds(&input_map);

	commands.spawn(input_map);
}

fn print_binds<A: Actionlike + Ord + bevy::reflect::enums::Enum>(input_map: &InputMap<A>) {
	let mut longest = 0;
	fn input_to_debug<A: Actionlike, I: std::fmt::Debug>((action, input): (&A, I)) -> (&A, String) {
		(action, format!("{input:?}"))
	}
	let mut actions = input_map
		.iter_buttonlike()
		.map(input_to_debug)
		.chain(input_map.iter_axislike().map(input_to_debug))
		.chain(input_map.iter_dual_axislike().map(input_to_debug))
		.chain(input_map.iter_triple_axislike().map(input_to_debug))
		.inspect(|(a, _)| longest = a.variant_name().len().max(longest))
		.collect::<Vec<_>>();

	actions.sort_by_key(|(a, _)| *a);

	for (action, buttons) in actions {
		let action = format!("{action:?}");
		println!("{action:<longest$} {buttons}");
	}
}
