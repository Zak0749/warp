use super::prelude::*;

pub struct SwitchPlugin;

impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_collisons)
            .add_event::<SwitchPressedEvent>()
            .register_ldtk_entity::<SwitchBundle>("Switch");
    }
}

pub struct SwitchPressedEvent(pub Entity, pub bool);

#[derive(Bundle, Default, LdtkEntity)]
struct SwitchBundle {
    #[from_entity_instance]
    entity_instance: EntityInstance,

    #[bundle]
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,

    #[from_entity_instance]
    #[bundle]
    collider_bundle: SwitchCollider,

    switch: Switch,
}

#[derive(Component, Default)]
pub struct Switch;

#[derive(Bundle, Default)]
struct SwitchCollider {
    collider: Collider,
    event: ActiveEvents,
    sensor: Sensor,
}

impl From<EntityInstance> for SwitchCollider {
    fn from(entity_instance: EntityInstance) -> Self {
        let tile = entity_instance
            .tile
            .expect("Switch entity must have a tile");

        SwitchCollider {
            collider: Collider::cuboid((tile.w as f32) / 2.0, (tile.h as f32) / 2.0),
            event: ActiveEvents::COLLISION_EVENTS,
            sensor: Sensor,
        }
    }
}

fn button_collisons(
    mut collision_events: EventReader<CollisionEvent>,
    mut button_query: Query<(&mut TextureAtlasSprite, Entity), With<Switch>>,
    mut pressed_event: EventWriter<SwitchPressedEvent>,
) {
    for collison in collision_events.iter() {
        for (mut sprite, entity) in button_query.iter_mut() {
            match collison {
                CollisionEvent::Started(_, e, _) => {
                    if &entity == e {
                        sprite.index = 1;
                        pressed_event.send(SwitchPressedEvent(entity, true));
                    }
                }
                CollisionEvent::Stopped(_, e, _) => {
                    if &entity == e {
                        sprite.index = 0;
                        pressed_event.send(SwitchPressedEvent(entity, false));
                    }
                }
            }
        }
    }
}
