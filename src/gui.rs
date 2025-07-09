use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use crate::gizmos::ParticleLimit;
use crate::particle::Particle;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(FrameTimeDiagnosticsPlugin::default())
			.add_systems(Startup, set_up_panels)
			.add_systems(
				Update,
				(update_fps, update_particle_count, update_particle_limit),
			);
	}
}

#[derive(Component)]
struct FpsDisplay;

#[derive(Component)]
struct ParticleCountDisplay;

#[derive(Component)]
struct ParticleLimitDisplay;

fn set_up_panels(mut commands: Commands, asset_server: Res<AssetServer>) {
	let font = asset_server.load("fonts/FiraSans-Bold.ttf");
	commands.spawn((
		Text(String::from("-")),
		TextFont::from_font(font.clone()).with_font_size(50.0),
		TextColor(Color::WHITE),
		Node {
			align_self: AlignSelf::FlexEnd,
			position_type: PositionType::Absolute,
			top: Val::Px(5.0),
			right: Val::Px(15.0),
			..default()
		},
		FpsDisplay,
	));

	commands.spawn((
		Text(String::from("-")),
		TextFont::from_font(font.clone()).with_font_size(50.0),
		TextColor(Color::WHITE),
		Node {
			align_self: AlignSelf::FlexEnd,
			position_type: PositionType::Absolute,
			top: Val::Px(60.0),
			right: Val::Px(15.0),
			..default()
		},
		ParticleCountDisplay,
	));

	commands.spawn((
		Text(String::from("-")),
		TextFont::from_font(font.clone()).with_font_size(30.0),
		TextColor(Color::WHITE),
		Node {
			align_self: AlignSelf::FlexEnd,
			position_type: PositionType::Absolute,
			top: Val::Px(110.0),
			right: Val::Px(15.0),
			..default()
		},
		ParticleLimitDisplay,
	));
}

fn update_fps(diagnostics: Res<DiagnosticsStore>, mut text: Query<&mut Text, With<FpsDisplay>>) {
	let mut text = text.single_mut().unwrap();

	if let Some(fps) = diagnostics
		.get(&FrameTimeDiagnosticsPlugin::FPS)
		.and_then(bevy::diagnostic::Diagnostic::average)
	{
		text.0 = format!("{}", fps.round());
	}
}

fn update_particle_count(
	particles: Query<(), With<Particle>>,
	mut text: Query<&mut Text, With<ParticleCountDisplay>>,
) {
	let mut text = text.single_mut().unwrap();
	let count = particles.iter().len();

	text.0 = format!("{count}");
}

fn update_particle_limit(
	limit: Res<ParticleLimit>,
	mut text: Query<&mut Text, With<ParticleLimitDisplay>>,
) {
	let mut text = text.single_mut().unwrap();

	text.0 = format!("/ {}", limit.current());
}
