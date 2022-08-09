use bevy::prelude::*;
use gui::GuiPlugin;

mod gui;
mod macros;

fn main() {
	App::new()
		.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
		.insert_resource(WindowDescriptor {
			width: 1600.0,
			height: 900.0,
			title: String::from("Particle simulator"),
			..default()
		})
		.insert_resource(ParticlePositions::default())
		.add_plugins(DefaultPlugins)
		.add_plugin(GuiPlugin)
		.add_startup_system(spawn_camera)
		.add_system(spawn_particle)
		.add_system(store_particle_positions)
		.add_system(set_particle_movement.after(store_particle_positions))
		.add_system(move_particles.after(set_particle_movement))
		.run();
}

fn spawn_camera(mut commands: Commands) {
	commands.spawn_bundle(Camera2dBundle {
		projection: OrthographicProjection {
			left: 0.0,
			right: 1.0,
			bottom: 0.0,
			top: 1.0,
			window_origin: bevy::render::camera::WindowOrigin::BottomLeft,
			..default()
		},
		..default()
	});
}

#[derive(Component, Default)]
struct Particle {
	movement: Vec2,
}

fn spawn_particle(mut commands: Commands, windows: Res<Windows>, mouse: Res<Input<MouseButton>>) {
	if !mouse.just_pressed(MouseButton::Left) {
		return;
	}
	let cursor_pos = unwrap_or_return!(windows
		.get_primary()
		.and_then(|window| window.cursor_position()));

	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::WHITE,
				..default()
			},
			transform: Transform {
				translation: cursor_pos.extend(0.0),
				scale: Vec3::new(5.0, 5.0, 1.0),
				..default()
			},
			..default()
		})
		.insert(Particle::default());
}

#[derive(Default)]
struct ParticlePositions {
	positions: Vec<Vec2>,
}

fn store_particle_positions(
	mut positions: ResMut<ParticlePositions>,
	particles: Query<&Transform, With<Particle>>,
) {
	positions.positions = particles
		.iter()
		.map(|transform| transform.translation.truncate())
		.collect();
}

fn set_particle_movement(
	windows: Res<Windows>,
	time: Res<Time>,
	positions: Res<ParticlePositions>,
	mut particles: Query<(&mut Particle, &Transform)>,
) {
	let window = unwrap_or_return!(windows.get_primary());

	for (mut particle, transform) in &mut particles {
		let this_position = transform.translation.truncate();
		particle.movement = Vec2::ZERO;
		for position in &positions.positions {
			if position == &this_position {
				continue;
			}
			let offset = Vec2::new(
				wrapping_offset(this_position.x, position.x, window.width()),
				wrapping_offset(this_position.y, position.y, window.height()),
			);
			let force = offset.length_recip().powf(2.0);
			particle.movement += 10000.0 * force * offset.normalize() * time.delta_seconds();
		}
		if particle.movement.length_squared() > 100.0 {
			println!("Fast particle! {:?}", particle.movement.length());
			particle.movement = particle.movement.clamp_length_max(10.0);
		}
	}
}

fn wrapping_offset(first: f32, second: f32, wrap: f32) -> f32 {
	let offset = first - second;
	if offset.abs() > wrap / 2.0 {
		if first > second {
			offset - wrap
		} else {
			offset + wrap
		}
	} else {
		offset
	}
}

fn move_particles(windows: Res<Windows>, mut particles: Query<(&mut Transform, &Particle)>) {
	let window = unwrap_or_return!(windows.get_primary());

	for (mut transform, particle) in &mut particles {
		transform.translation += particle.movement.extend(0.0);
		transform.translation.x = transform.translation.x.rem_euclid(window.width());
		transform.translation.y = transform.translation.y.rem_euclid(window.height());
	}
}
