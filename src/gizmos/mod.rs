use bevy::{ecs::system::EntityCommands, prelude::*};
use leafwing_input_manager::prelude::ActionState;

use crate::{
	common::{find_entity_by_cursor, Positive},
	draw_properties::{self, DrawProperties},
	input::Action,
	movement::{merge_speed, Movement},
	unwrap_or_return,
};

use self::{
	attractor::{activate_attractors, Attractor},
	deleter::{activate_deleters, Deleter},
	eater::{
		activate_eaters, apply_eater_scale, eaters_chasing_particles, process_dormant_eaters, Eater,
	},
	emitter::{activate_emitters, Emitter},
};

mod attractor;
mod deleter;
mod eater;
mod emitter;

pub struct GizmoPlugin;

impl Plugin for GizmoPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(activate_attractors.before(merge_speed))
			.add_system(activate_deleters)
			.add_system(activate_emitters)
			.add_system(activate_eaters)
			.add_system(eaters_chasing_particles.before(merge_speed))
			.add_system(apply_eater_scale)
			.add_system(process_dormant_eaters)
			.add_system(spawn_or_despawn_gizmos);
	}
}

struct GizmoVariant {
	action: Action,
	draw_properties: DrawProperties,
}

struct Gizmo {
	gizmo_type: GizmoType,
	neutral_or_negative_variant: GizmoVariant,
	/// The presence of this implies the other variant is negative.
	positive_variant: Option<GizmoVariant>,
	has_movement: bool,
}

const GIZMOS: [Gizmo; 4] = [
	Gizmo {
		gizmo_type: GizmoType::Emitter,
		neutral_or_negative_variant: GizmoVariant {
			action: Action::NegativeEmitter,
			draw_properties: draw_properties::NEGATIVE_EMITTER,
		},
		positive_variant: Some(GizmoVariant {
			action: Action::PositiveEmitter,
			draw_properties: draw_properties::POSITIVE_EMITTER,
		}),
		has_movement: false,
	},
	Gizmo {
		gizmo_type: GizmoType::Deleter,
		neutral_or_negative_variant: GizmoVariant {
			action: Action::Deleter,
			draw_properties: draw_properties::DELETER,
		},
		positive_variant: None,
		has_movement: false,
	},
	Gizmo {
		gizmo_type: GizmoType::Attractor,
		neutral_or_negative_variant: GizmoVariant {
			action: Action::Attractor,
			draw_properties: draw_properties::ATTRACTOR,
		},
		positive_variant: None,
		has_movement: false,
	},
	Gizmo {
		gizmo_type: GizmoType::Eater,
		neutral_or_negative_variant: GizmoVariant {
			action: Action::NegativeEater,
			draw_properties: draw_properties::NEGATIVE_EATER,
		},
		positive_variant: Some(GizmoVariant {
			action: Action::PositiveEater,
			draw_properties: draw_properties::POSITIVE_EATER,
		}),
		has_movement: true,
	},
];

#[derive(Clone, Copy, PartialEq, Component)]
enum GizmoType {
	Emitter,
	Deleter,
	Attractor,
	Eater,
}

enum GizmoComponent {
	Emitter(Emitter),
	Deleter(Deleter),
	Attractor(Attractor),
	Eater(Eater),
}

impl GizmoComponent {
	fn default_of_type(gizmo_type: GizmoType) -> Self {
		match gizmo_type {
			GizmoType::Emitter => Self::Emitter(Emitter::default()),
			GizmoType::Deleter => Self::Deleter(Deleter::default()),
			GizmoType::Attractor => Self::Attractor(Attractor::default()),
			GizmoType::Eater => Self::Eater(Eater::default()),
		}
	}
	fn insert_using<'l, 'a, 'b, 'c>(
		self,
		entity_commands: &'l mut EntityCommands<'a, 'b, 'c>,
	) -> &'l mut EntityCommands<'a, 'b, 'c> {
		match self {
			Self::Emitter(c) => entity_commands.insert(c),
			Self::Deleter(c) => entity_commands.insert(c),
			Self::Attractor(c) => entity_commands.insert(c),
			Self::Eater(c) => entity_commands.insert(c),
		}
	}
}

fn spawn_or_despawn_gizmos<'a>(
	mut commands: Commands,
	windows: Res<Windows>,
	action_state: Query<&'a ActionState<Action>>,
	gizmos: Query<(Entity, &'a Transform, &'a GizmoType, Option<&'a Positive>)>,
) {
	let action_state = action_state.single();
	let window = unwrap_or_return!(windows.get_primary());
	let cursor_pos = unwrap_or_return!(window.cursor_position());

	let window_dimensions = Vec2::new(window.requested_width(), window.requested_height());

	for gizmo in GIZMOS {
		for (variant, positive) in [
			(Some(&gizmo.neutral_or_negative_variant), false),
			(gizmo.positive_variant.as_ref(), true),
		]
		.iter()
		.filter_map(|(x, p)| x.map(|x| (x, p)))
		{
			if action_state.just_pressed(variant.action.clone()) {
				if action_state.pressed(Action::DespawnAllModifier) {
					despawn_all_gizmos(&mut commands, &gizmo, &gizmos, *positive);
				} else if action_state.pressed(Action::DespawnModifier) {
					despawn_gizmo(
						&mut commands,
						cursor_pos,
						window_dimensions,
						&gizmo,
						&gizmos,
						*positive,
					);
				} else {
					spawn_gizmo(&mut commands, cursor_pos, &gizmo, *positive);
				}
			}
		}
	}
}

fn spawn_gizmo<'a>(commands: &'a mut Commands, position: Vec2, gizmo: &'a Gizmo, positive: bool) {
	let variant = if positive {
		gizmo.positive_variant.as_ref().unwrap()
	} else {
		&gizmo.neutral_or_negative_variant
	};
	let DrawProperties {
		draw_priority,
		size,
		color,
	} = variant.draw_properties;

	let mut entity_commands = commands.spawn();

	entity_commands
		.insert_bundle(SpriteBundle {
			sprite: Sprite { color, ..default() },
			transform: Transform {
				translation: position.extend(draw_priority),
				scale: (Vec2::ONE * size).extend(1.0),
				..default()
			},
			..default()
		})
		.insert(gizmo.gizmo_type);

	let component = GizmoComponent::default_of_type(gizmo.gizmo_type);
	component.insert_using(&mut entity_commands);

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
