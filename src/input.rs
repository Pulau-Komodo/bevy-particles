use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

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
	DespawnPositiveEmitter,
	DespawnNegativeEmitter,
	DespawnDeleter,
	DespawnAttractor,
	SuspendRepulsion,
}

fn set_binds(mut commands: Commands) {
	use Action::*;
	use KeyCode::*;

	let actions = vec![
		(Equals, SpawnPositiveEmitter, DespawnPositiveEmitter),
		(Minus, SpawnNegativeEmitter, DespawnNegativeEmitter),
		(Key1, SpawnDeleter, DespawnDeleter),
		(Key2, SpawnAttractor, DespawnAttractor),
	];

	let mut input_map = InputMap::default();
	input_map.insert(MouseButton::Left, SpawnParticle);
	input_map.insert(KeyCode::Space, SuspendRepulsion);

	for (key, spawn, despawn) in actions {
		input_map.insert(key, spawn);
		input_map.insert_chord([LShift, key], despawn.clone());
		input_map.insert_chord([RShift, key], despawn);
	}

	commands.spawn_bundle(InputManagerBundle::<Action> {
		input_map,
		..default()
	});
}
