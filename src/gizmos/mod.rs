use bevy::{ecs::system::EntityCommands, prelude::*, window::PrimaryWindow};
use deleter::{SlowDeleter, activate_slow_deleters, recharge_slow_deleters};
use leafwing_input_manager::prelude::ActionState;

use crate::{
	WindowDimensions,
	assets::TextureMap,
	common::{Positive, find_entity_by_cursor},
	draw_properties::{self, DrawProperties},
	gizmos::pusher::{Pusher, activate_pushers},
	input::Action,
	movement::{Movement, merge_speed},
	unwrap_or_return,
};

use self::{
	attractor::{Attractor, activate_attractors},
	deleter::{Deleter, activate_deleters},
	eater::{
		Eater, activate_eaters, apply_eater_scale, eaters_chasing_particles, process_dormant_eaters,
	},
	emitter::{Emitter, activate_emitters, adjust_particle_limit},
};

pub use self::emitter::ParticleLimit;

mod attractor;
mod deleter;
mod eater;
mod emitter;
mod pusher;

pub struct GizmoPlugin;

impl Plugin for GizmoPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (spawn_or_despawn_gizmos, adjust_particle_limit))
			.add_systems(
				FixedUpdate,
				(
					(
						activate_attractors,
						eaters_chasing_particles,
						activate_pushers,
					)
						.before(merge_speed),
					(
						(activate_deleters, recharge_slow_deleters),
						activate_slow_deleters,
					)
						.chain(),
					activate_emitters,
					activate_eaters,
					apply_eater_scale,
					process_dormant_eaters,
				),
			)
			.init_resource::<ParticleLimit>();
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlacementStyle {
	Instant,
	WithRotation,
}

#[derive(Component, Debug, Clone, Copy)]
struct BeingPlaced;

enum GizmoVariants {
	Neutral(GizmoVariant),
	Polar {
		negative: GizmoVariant,
		positive: GizmoVariant,
	},
}

#[derive(Debug, Clone, Copy)]
struct GizmoVariant {
	action: Action,
	draw_properties: DrawProperties,
}

struct Gizmo {
	gizmo_type: GizmoType,
	variants: GizmoVariants,
	has_movement: bool,
	placement_style: PlacementStyle,
}

const GIZMOS: [Gizmo; 7] = [
	Gizmo {
		gizmo_type: GizmoType::Emitter,
		variants: GizmoVariants::Polar {
			negative: GizmoVariant {
				action: Action::NegativeEmitter,
				draw_properties: draw_properties::NEGATIVE_EMITTER,
			},
			positive: GizmoVariant {
				action: Action::PositiveEmitter,
				draw_properties: draw_properties::POSITIVE_EMITTER,
			},
		},
		has_movement: false,
		placement_style: PlacementStyle::Instant,
	},
	Gizmo {
		gizmo_type: GizmoType::Deleter,
		variants: GizmoVariants::Neutral(GizmoVariant {
			action: Action::Deleter,
			draw_properties: draw_properties::DELETER,
		}),
		has_movement: false,
		placement_style: PlacementStyle::Instant,
	},
	Gizmo {
		gizmo_type: GizmoType::SlowDeleter,
		variants: GizmoVariants::Neutral(GizmoVariant {
			action: Action::SlowDeleter,
			draw_properties: draw_properties::SLOW_DELETER,
		}),
		has_movement: false,
		placement_style: PlacementStyle::Instant,
	},
	Gizmo {
		gizmo_type: GizmoType::Attractor,
		variants: GizmoVariants::Neutral(GizmoVariant {
			action: Action::Attractor,
			draw_properties: draw_properties::ATTRACTOR,
		}),
		has_movement: false,
		placement_style: PlacementStyle::Instant,
	},
	Gizmo {
		gizmo_type: GizmoType::Repulsor,
		variants: GizmoVariants::Neutral(GizmoVariant {
			action: Action::Repulsor,
			draw_properties: draw_properties::REPULSOR,
		}),
		has_movement: false,
		placement_style: PlacementStyle::Instant,
	},
	Gizmo {
		gizmo_type: GizmoType::Pusher,
		variants: GizmoVariants::Neutral(GizmoVariant {
			action: Action::Pusher,
			draw_properties: draw_properties::PUSHER,
		}),
		has_movement: false,
		placement_style: PlacementStyle::WithRotation,
	},
	Gizmo {
		gizmo_type: GizmoType::Eater,
		variants: GizmoVariants::Polar {
			negative: GizmoVariant {
				action: Action::NegativeEater,
				draw_properties: draw_properties::NEGATIVE_EATER,
			},
			positive: GizmoVariant {
				action: Action::PositiveEater,
				draw_properties: draw_properties::POSITIVE_EATER,
			},
		},
		has_movement: true,
		placement_style: PlacementStyle::Instant,
	},
];

#[derive(Clone, Copy, PartialEq, Component)]
enum GizmoType {
	Emitter,
	Deleter,
	SlowDeleter,
	Attractor,
	Repulsor,
	Pusher,
	Eater,
}

impl GizmoType {
	fn insert_using<'l, 'a>(
		self,
		entity_commands: &'l mut EntityCommands<'a>,
	) -> &'l mut EntityCommands<'a> {
		match self {
			Self::Emitter => entity_commands.insert(Emitter::default()),
			Self::Deleter => entity_commands.insert(Deleter::default()),
			Self::SlowDeleter => entity_commands.insert(SlowDeleter::default()),
			Self::Attractor => entity_commands.insert(Attractor::default()),
			Self::Repulsor => entity_commands.insert(Attractor::repulsor()),
			Self::Pusher => entity_commands.insert(Pusher),
			Self::Eater => entity_commands.insert(Eater::default()),
		}
	}
}

fn spawn_or_despawn_gizmos(
	mut commands: Commands,
	texture_map: Res<TextureMap>,
	window: Query<&Window, With<PrimaryWindow>>,
	window_dimensions: Res<WindowDimensions>,
	action_state: Query<&ActionState<Action>>,
	gizmos: Query<(Entity, &Transform, &GizmoType, Option<&Positive>), Without<BeingPlaced>>,
	mut placers: Query<(Entity, &mut Transform, &GizmoType), With<BeingPlaced>>,
) {
	let action_state = action_state.single().unwrap();
	let window = unwrap_or_return!(window.single().ok());
	let cursor_pos = unwrap_or_return!(window.cursor_position());
	let cursor_pos = Vec2::new(cursor_pos.x, window.height() - cursor_pos.y);

	for gizmo in GIZMOS {
		let variants = match gizmo.variants {
			GizmoVariants::Neutral(variant) => [Some((variant, false)), None],
			GizmoVariants::Polar { negative, positive } => {
				[Some((negative, false)), Some((positive, true))]
			}
		};
		for (variant, positive) in variants.into_iter().flatten() {
			if action_state.just_pressed(&variant.action) {
				if action_state.pressed(&Action::DespawnAllModifier) {
					despawn_all_gizmos(&mut commands, &gizmo, gizmos, positive);
				} else if action_state.pressed(&Action::DespawnModifier) {
					despawn_gizmo(
						&mut commands,
						cursor_pos,
						window_dimensions.0,
						&gizmo,
						gizmos,
						positive,
					);
				} else {
					let is_placer = gizmo.placement_style == PlacementStyle::WithRotation;
					spawn_gizmo(
						&mut commands,
						&texture_map,
						cursor_pos,
						&gizmo,
						&variant,
						positive,
						is_placer,
					);
				}
			} else if gizmo.placement_style == PlacementStyle::WithRotation {
				if action_state.pressed(&variant.action) {
					for (_, mut transform, gizmo_type) in &mut placers {
						if *gizmo_type == gizmo.gizmo_type {
							let offset = cursor_pos - transform.translation.truncate();
							transform.rotation = Quat::from_rotation_z(offset.to_angle());
						}
					}
				} else if action_state.just_released(&variant.action) {
					for (entity, _, gizmo_type) in &placers {
						if *gizmo_type == gizmo.gizmo_type {
							commands.entity(entity).remove::<BeingPlaced>();
						}
					}
				} else {
					for (entity, _, gizmo_type) in &placers {
						if *gizmo_type == gizmo.gizmo_type {
							commands.entity(entity).despawn();
						}
					}
				}
			}
		}
	}
}

fn spawn_gizmo<'a>(
	commands: &'a mut Commands,
	texture_map: &Res<TextureMap>,
	position: Vec2,
	gizmo: &'a Gizmo,
	variant: &'a GizmoVariant,
	positive: bool,
	is_placer: bool,
) {
	let DrawProperties {
		draw_priority,
		size,
		color,
		texture,
	} = variant.draw_properties;

	let image = texture
		.and_then(|texture| texture_map.0.get(&texture))
		.cloned()
		.unwrap_or_default();

	let mut entity_commands = commands.spawn((
		Sprite {
			color,
			image,
			..default()
		},
		Transform {
			translation: position.extend(draw_priority),
			scale: (Vec2::ONE * size).extend(1.0),
			..default()
		},
		gizmo.gizmo_type,
	));

	gizmo.gizmo_type.insert_using(&mut entity_commands);

	if is_placer {
		entity_commands.insert(BeingPlaced);
	}
	if positive {
		entity_commands.insert(Positive);
	}
	if gizmo.has_movement {
		entity_commands.insert(Movement::default());
	}
}

fn despawn_gizmo<'a>(
	commands: &mut Commands,
	coordinates: Vec2,
	window_dimensions: Vec2,
	gizmo: &'a Gizmo,
	gizmos: impl IntoIterator<Item = (Entity, &'a Transform, &'a GizmoType, Option<&'a Positive>)>,
	positive: bool,
) {
	if let Some(gizmo) = find_entity_by_cursor(
		coordinates,
		window_dimensions,
		gizmos
			.into_iter()
			.filter_map(|(entity, transform, gizmo_type, positive_component)| {
				(&gizmo.gizmo_type == gizmo_type && positive_component.is_some() == positive)
					.then_some((entity, transform))
			}),
	) {
		commands.entity(gizmo).despawn();
	}
}

fn despawn_all_gizmos<'a>(
	commands: &'a mut Commands,
	gizmo: &'a Gizmo,
	gizmos: impl IntoIterator<Item = (Entity, &'a Transform, &'a GizmoType, Option<&'a Positive>)>,
	positive: bool,
) {
	for gizmo in gizmos
		.into_iter()
		.filter_map(|(entity, _, gizmo_type, positive_component)| {
			(&gizmo.gizmo_type == gizmo_type && positive_component.is_some() == positive)
				.then_some(entity)
		}) {
		commands.entity(gizmo).despawn();
	}
}
