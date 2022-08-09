use super::*;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<DoorBundle>("Door")
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Game)
                    .run_not_in_state(PauseState::Paused)
                    .with_system(opening_door)
                    .with_system(track_door_switches)
                    .with_system(link_door_inputs)
                    .into(),
            );
    }
}

#[derive(Component, Default)]
pub struct Door;

#[derive(Bundle, Default, LdtkEntity)]
struct DoorBundle {
    door: Door,

    #[from_entity_instance]
    switches: DoorActivationControl,

    #[from_entity_instance]
    instance: EntityInstance,

    #[bundle]
    collision: DoorCollision,

    #[bundle]
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Component, Default)]
struct DoorActivationControl {
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

impl From<EntityInstance> for DoorActivationControl {
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
            collider: Collider::cuboid(12.0, 12.0),
            rigid_body: RigidBody::Fixed,
        }
    }
}

fn opening_door(
    mut door_query: Query<
        (&mut TextureAtlasSprite, &DoorActivationControl, Entity),
        (With<Door>, Changed<DoorActivationControl>),
    >,
    mut commands: Commands,
) {
    for (mut sprite, activators, entity) in &mut door_query {
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

fn link_door_inputs(
    mut door_query: Query<(&mut DoorActivationControl, &EntityInstance), Added<Door>>,
    switch_query: Query<(Entity, &EntityInstance), Added<Switch>>,
) {
    let switches = switch_query.iter().collect::<Vec<_>>();
    for (mut activation_control, door_instance) in &mut door_query {
        if let FieldValue::EntityRefs(door_references) = &door_instance
            .field_instances
            .iter()
            .find(|v| v.identifier == "Switches")
            .expect("Door entity must have a switches field")
            .value
        {
            for reference in door_references.iter().flatten() {
                if let Some((switch, _)) = switches
                    .iter()
                    .find(|(_, switch_instance)| switch_instance.iid == reference.entity_iid)
                {
                    activation_control.switches.push(switch.clone());
                }
            }
        } else {
            panic!("Door entity switches field must be an entity refs");
        }
    }
}

fn track_door_switches(
    mut door_query: Query<&mut DoorActivationControl, With<Door>>,
    mut pressed_event: EventReader<SwitchPressedEvent>,
) {
    for SwitchPressedEvent(button_entity, state) in pressed_event.iter() {
        for mut door_activator in &mut door_query {
            if door_activator.switches.contains(&button_entity) {
                match state {
                    SwitchState::Pressed => {
                        door_activator.pressed.push(button_entity.clone());
                    }
                    SwitchState::Released => {
                        door_activator.pressed.retain(|x| x != button_entity);
                    }
                }
            }
        }
    }
}
