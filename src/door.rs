use super::prelude::*;
use crate::switch::{Switch, SwitchPressedEvent};

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<DoorBundle>("Door")
            .add_system(door_opening)
            .add_system(door_mapping)
            .add_system(door_open_tracker);
    }
}

#[derive(Bundle, Default, LdtkEntity)]
struct DoorBundle {
    #[bundle]
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,

    #[from_entity_instance]
    switches: Activators,

    #[from_entity_instance]
    entity_instance: EntityInstance,

    door: Door,
}

#[derive(Component, Default)]
pub struct Door;

#[derive(Component, Default)]
struct Activators {
    switches: Vec<Entity>,
    control: DoorControl,
    pressed: Vec<Entity>,
}

#[derive(Default)]
enum DoorControl {
    #[default]
    Or,
    And,
}

impl From<EntityInstance> for Activators {
    fn from(entity_instance: EntityInstance) -> Self {
        Self {
            switches: vec![],
            pressed: vec![],
            control: match &entity_instance
                .field_instances
                .iter()
                .find(|v| v.identifier == "DoorControl")
                .expect("Door entity must have a switches field")
                .value
            {
                FieldValue::Enum(Some(v)) if v == "Or" => DoorControl::Or,
                FieldValue::Enum(Some(v)) if v == "And" => DoorControl::And,
                _ => panic!("Door entity switches field must be an enum"),
            },
        }
    }
}

#[derive(Bundle)]
struct DoorCollision {
    collider: Collider,
    rigid_body: RigidBody,
}

impl Default for DoorCollision {
    fn default() -> Self {
        Self {
            collider: Collider::cuboid(24.0 / 2.0, 24.0 / 2.0),
            rigid_body: RigidBody::Fixed,
        }
    }
}

fn door_mapping(
    mut door_query: Query<(&mut Activators, &EntityInstance), Added<Door>>,
    switch_query: Query<(Entity, &EntityInstance), Added<Switch>>,
) {
    let switches = switch_query.iter().collect::<Vec<_>>();
    for (mut door_switches, door_instance) in door_query.iter_mut() {
        if let FieldValue::EntityRefs(v) = &door_instance
            .field_instances
            .iter()
            .find(|v| v.identifier == "Switches")
            .expect("Door entity must have a switches field")
            .value
        {
            for z in v.iter().flatten() {
                let x = switches
                    .iter()
                    .find(|(_, i)| i.iid == z.entity_iid)
                    .expect("Ref must be valid");

                door_switches.switches.push(x.0);
            }
        } else {
            panic!("Door entity switches field must be an entity refs");
        }
    }
}

fn door_opening(
    mut door_query: Query<(&mut TextureAtlasSprite, &Activators, Entity), With<Door>>,
    mut commands: Commands,
) {
    for (mut sprite, activators, entity) in door_query.iter_mut() {
        if (matches!(activators.control, DoorControl::Or) && activators.pressed.len() > 0)
            || (matches!(activators.control, DoorControl::And)
                && activators.pressed.len() == activators.switches.len())
        {
            sprite.index = 3;
            commands.entity(entity).remove_bundle::<DoorCollision>();
        } else {
            sprite.index = 0;
            commands
                .entity(entity)
                .insert_bundle(DoorCollision::default());
        }
    }
}

fn door_open_tracker(
    mut door_query: Query<&mut Activators, With<Door>>,
    mut pressed_event: EventReader<SwitchPressedEvent>,
) {
    for event in pressed_event.iter() {
        for mut door_activator in door_query.iter_mut() {
            if door_activator.switches.contains(&event.0) {
                if event.1 {
                    door_activator.pressed.push(event.0);
                } else {
                    door_activator.pressed.retain(|e| e != &event.0);
                }
            }
        }
    }
}
