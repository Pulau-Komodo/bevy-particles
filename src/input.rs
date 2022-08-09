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
	SpawnEmitter,
	SpawnDeleter,
	DespawnEmitter,
	DespawnDeleter,
}

fn set_binds(mut commands: Commands) {
	use Action::*;

	let mut input_map = InputMap::default();
	input_map.insert(MouseButton::Left, SpawnParticle);
	input_map.insert(KeyCode::Equals, SpawnEmitter);
	input_map.insert(KeyCode::Minus, SpawnDeleter);
	input_map.insert_chord([KeyCode::LShift, KeyCode::Equals], DespawnEmitter);
	input_map.insert_chord([KeyCode::RShift, KeyCode::Equals], DespawnEmitter);
	input_map.insert_chord([KeyCode::LShift, KeyCode::Minus], DespawnDeleter);
	input_map.insert_chord([KeyCode::RShift, KeyCode::Minus], DespawnDeleter);

	commands.spawn_bundle(InputManagerBundle::<Action> {
		input_map,
		..default()
	});
}
