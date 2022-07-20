use super::prelude::*;

pub struct BoxPlugin;

impl Plugin for BoxPlugin {
    fn build(&self, app: &mut App) {
        println!("BoxPlugin::build");
        app.register_ldtk_entity::<BoxBundle>("Box");
    }
}

#[derive(Bundle, Default, LdtkEntity)]
struct BoxBundle {
    r#box: Box,

    #[bundle]
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,

    #[bundle]
    collider: BoxCollider,
}

#[derive(Component, Default)]
pub struct Box;

#[derive(Bundle)]
struct BoxCollider {
    collider: Collider,
    rigid_body: RigidBody,
    velocity: Velocity,
    gravity_scale: GravityScale,
    locked_axis: LockedAxes,
    friction: Friction,
    dampening: Damping,
}

impl Default for BoxCollider {
    fn default() -> Self {
        BoxCollider {
            collider: Collider::cuboid(8.0, 8.0),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::default(),
            gravity_scale: GravityScale(0.0),
            locked_axis: LockedAxes::ROTATION_LOCKED,
            friction: Friction::coefficient(10.0),
            dampening: Damping {
                linear_damping: 15.0,
                angular_damping: 0.0,
            },
        }
    }
}
