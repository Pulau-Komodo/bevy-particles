use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use crate::gizmos::ParticleLimit;
use crate::particle::Particle;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(FrameTimeDiagnosticsPlugin)
			.add_startup_system(set_up_panels)
			.add_system(update_fps)
			.add_system(update_particle_count)
			.add_system(update_particle_limit);
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
	commands
		.spawn_bundle(
			TextBundle::from_section(
				"-",
				TextStyle {
					font: font.clone(),
					font_size: 50.0,
					color: Color::WHITE,
				},
			)
			.with_style(Style {
				align_self: AlignSelf::FlexEnd,
				position_type: PositionType::Absolute,
				position: UiRect {
					top: Val::Px(5.0),
					right: Val::Px(15.0),
					..default()
				},
				..default()
			}),
		)
		.insert(FpsDisplay);

	commands
		.spawn_bundle(
			TextBundle::from_section(
				"-",
				TextStyle {
					font: font.clone(),
					font_size: 50.0,
					color: Color::WHITE,
				},
			)
			.with_style(Style {
				align_self: AlignSelf::FlexEnd,
				position_type: PositionType::Absolute,
				position: UiRect {
					top: Val::Px(60.0),
					right: Val::Px(15.0),
					..default()
				},
				..default()
			}),
		)
		.insert(ParticleCountDisplay);

	commands
		.spawn_bundle(
			TextBundle::from_section(
				"-",
				TextStyle {
					font,
					font_size: 30.0,
					color: Color::WHITE,
				},
			)
			.with_style(Style {
				align_self: AlignSelf::FlexEnd,
				position_type: PositionType::Absolute,
				position: UiRect {
					top: Val::Px(110.0),
					right: Val::Px(15.0),
					..default()
				},
				..default()
			}),
		)
		.insert(ParticleLimitDisplay);
}

fn update_fps(diagnostics: Res<Diagnostics>, mut text: Query<&mut Text, With<FpsDisplay>>) {
	let mut text = text.single_mut();

	if let Some(fps) = diagnostics
		.get(FrameTimeDiagnosticsPlugin::FPS)
		.and_then(bevy::diagnostic::Diagnostic::average)
	{
		text.sections[0].value = format!("{}", fps.round());
	}
}

fn update_particle_count(
	particles: Query<(), With<Particle>>,
	mut text: Query<&mut Text, With<ParticleCountDisplay>>,
) {
	let mut text = text.single_mut();
	let count = particles.iter().len();

	text.sections[0].value = format!("{count}");
}

fn update_particle_limit(
	limit: Res<ParticleLimit>,
	mut text: Query<&mut Text, With<ParticleLimitDisplay>>,
) {
	let mut text = text.single_mut();

	text.sections[0].value = format!("/ {}", limit.current());
}
