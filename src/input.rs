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
	SpawnPositiveEmitter,
	SpawnNegativeEmitter,
	SpawnDeleter,
	SpawnAttractor,
	SpawnPositiveEater,
	SpawnNegativeEater,
	DespawnPositiveEmitter,
	DespawnNegativeEmitter,
	DespawnDeleter,
	DespawnAttractor,
	DespawnPositiveEater,
	DespawnNegativeEater,
	DespawnAllParticles,
	DespawnAllPositiveEmitters,
	DespawnAllNegativeEmitters,
	DespawnAllPositiveEaters,
	DespawnAllNegativeEaters,
	DespawnAllDeleters,
	DespawnAllAttractors,
	SuspendRepulsion,
	ToggleInertia,
}

fn set_binds(mut commands: Commands) {
	use Action::*;
	use KeyCode::*;

	let actions = vec![
		(
			Equals,
			SpawnPositiveEmitter,
			DespawnPositiveEmitter,
			DespawnAllPositiveEmitters,
		),
		(
			Minus,
			SpawnNegativeEmitter,
			DespawnNegativeEmitter,
			DespawnAllNegativeEmitters,
		),
		(Key1, SpawnDeleter, DespawnDeleter, DespawnAllDeleters),
		(Key2, SpawnAttractor, DespawnAttractor, DespawnAllAttractors),
		(
			LBracket,
			SpawnNegativeEater,
			DespawnNegativeEater,
			DespawnAllNegativeEaters,
		),
		(
			RBracket,
			SpawnPositiveEater,
			DespawnPositiveEater,
			DespawnAllPositiveEaters,
		),
	];

	let left_click: InputKind = MouseButton::Left.into();

	let mut input_map = InputMap::default();
	input_map.insert(left_click, SpawnParticle);
	input_map.insert(left_click, SpawnParticle);
	input_map.insert_chord([LAlt.into(), left_click], DespawnAllParticles);
	input_map.insert_chord([RAlt.into(), left_click], DespawnAllParticles);
	input_map.insert(Space, SuspendRepulsion);
	input_map.insert(I, ToggleInertia);

	for (key, spawn, despawn, despawn_all) in actions {
		input_map.insert(key, spawn);
		input_map.insert_chord([LShift, key], despawn.clone());
		input_map.insert_chord([RShift, key], despawn);
		input_map.insert_chord([LAlt, key], despawn_all.clone());
		input_map.insert_chord([RAlt, key], despawn_all);
	}

	commands.spawn_bundle(InputManagerBundle::<Action> {
		input_map,
		..default()
	});
}
