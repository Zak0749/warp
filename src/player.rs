use super::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_system(player_movement_system)
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

#[derive(Bundle, Default, LdtkEntity)]
struct PlayerBundle {
    player: Player,

    #[bundle]
    #[from_entity_instance]
    collider: PlayerCollision,
    #[bundle]
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[bundle]
    input_manager: PlayerInput,
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
struct PlayerCollision {
    collider: Collider,
    rigid_body: RigidBody,
    velocity: Velocity,
    gravity_scale: GravityScale,
    locked_axis: LockedAxes,
    friction: Friction,
}

impl From<EntityInstance> for PlayerCollision {
    fn from(entity_instance: EntityInstance) -> Self {
        let tile = entity_instance
            .tile
            .expect("Player entity must have a tile");

        Self {
            collider: Collider::cuboid((tile.w as f32 / 4.0) - 0.1, (tile.h as f32 / 4.0) - 0.1),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            gravity_scale: GravityScale(0.0),
            locked_axis: LockedAxes::ROTATION_LOCKED,
            friction: Friction::coefficient(0.0),
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Bundle)]
pub struct PlayerInput {
    #[bundle]
    input_manager: InputManagerBundle<PlayerAction>,
}

impl Default for PlayerInput {
    fn default() -> Self {
        use PlayerAction::*;

        Self {
            input_manager: InputManagerBundle::<PlayerAction> {
                input_map: InputMap::new([
                    (KeyCode::W, Up),
                    (KeyCode::S, Down),
                    (KeyCode::A, Left),
                    (KeyCode::D, Right),
                ]),
                ..default()
            },
        }
    }
}

fn player_movement_system(
    mut query: Query<(&mut Velocity, &ActionState<PlayerAction>), With<Player>>,
) {
    for (mut vel, action_state) in query.iter_mut() {
        vel.linvel = Vec2::new(0.0, 0.0);
        if action_state.pressed(PlayerAction::Up) {
            vel.linvel.y += 100.0;
        }
        if action_state.pressed(PlayerAction::Down) {
            vel.linvel.y -= 100.0;
        }
        if action_state.pressed(PlayerAction::Right) {
            vel.linvel.x += 100.0;
        }
        if action_state.pressed(PlayerAction::Left) {
            vel.linvel.x -= 100.0;
        }
    }
}
