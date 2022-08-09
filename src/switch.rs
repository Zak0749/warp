use super::*;

pub struct SwitchPlugin;

impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<SwitchBundle>("Switch")
            .add_event::<SwitchPressedEvent>()
            .add_system(button_collisons.run_in_state(GameState::Game));
    }
}

pub enum SwitchState {
    Pressed,
    Released,
}

pub struct SwitchPressedEvent(pub Entity, pub SwitchState);

#[derive(Bundle, Default, LdtkEntity)]
struct SwitchBundle {
    switch: Switch,
    collision_count: CollisionCount,

    #[from_entity_instance]
    instance: EntityInstance,

    #[bundle]
    collider_bundle: SwitchCollider,

    #[bundle]
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Component, Default)]
pub struct Switch;

#[derive(Bundle)]
struct SwitchCollider {
    collider: Collider,
    event: ActiveEvents,
    sensor: Sensor,
}

impl Default for SwitchCollider {
    fn default() -> Self {
        SwitchCollider {
            collider: Collider::cuboid(7.0, 7.0),
            event: ActiveEvents::COLLISION_EVENTS,
            sensor: Sensor,
        }
    }
}

#[derive(Component, Default)]
struct CollisionCount(i32);

fn button_collisons(
    mut collision_events: EventReader<CollisionEvent>,
    mut button_query: Query<(&mut TextureAtlasSprite, &mut CollisionCount, Entity), With<Switch>>,
    mut pressed_event: EventWriter<SwitchPressedEvent>,
) {
    for collison in collision_events.iter() {
        for (mut sprite, mut collisions, entity) in &mut button_query {
            match collison {
                CollisionEvent::Started(object_1, object_2, _) => {
                    if &entity == object_1 || &entity == object_2 {
                        collisions.0 += 1;
                        if collisions.0 > 0 {
                            sprite.index = 1;
                            pressed_event.send(SwitchPressedEvent(entity, SwitchState::Pressed))
                        };
                    }
                }
                CollisionEvent::Stopped(object_1, object_2, _) => {
                    if &entity == object_1 || &entity == object_2 {
                        collisions.0 -= 1;
                        if collisions.0 == 0 {
                            sprite.index = 0;
                            pressed_event.send(SwitchPressedEvent(entity, SwitchState::Released))
                        };
                    }
                }
            }
        }
    }
}
