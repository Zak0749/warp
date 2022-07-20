use super::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_system(player_movement_system)
            .add_system(player_animation_system)
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

#[derive(Bundle, Default, LdtkEntity)]
struct PlayerBundle {
    player: Player,

    #[from_entity_instance]
    entity_instance: EntityInstance,

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
    dampening: Damping,
}

impl From<EntityInstance> for PlayerCollision {
    fn from(entity_instance: EntityInstance) -> Self {
        let width = if let FieldValue::Float(Some(v)) = entity_instance
            .field_instances
            .iter()
            .find(|v| v.identifier == "Width")
            .expect("Player entity must have a width field")
            .value
        {
            v
        } else {
            panic!("Player entity width field must be a float")
        };

        let height = if let FieldValue::Float(Some(v)) = entity_instance
            .field_instances
            .iter()
            .find(|v| v.identifier == "Height")
            .expect("Player entity must have a height field")
            .value
        {
            v
        } else {
            panic!("Player entity height field must be a float")
        };

        Self {
            collider: Collider::cuboid(width as f32 / 2.0, height as f32 / 2.0),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            gravity_scale: GravityScale(0.0),
            locked_axis: LockedAxes::ROTATION_LOCKED,
            friction: Friction::coefficient(0.0),
            dampening: Damping {
                linear_damping: 30.0,
                angular_damping: 0.0,
            },
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
struct PlayerInput {
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
                    (KeyCode::Up, Up),
                    (KeyCode::Down, Down),
                    (KeyCode::Left, Left),
                    (KeyCode::Right, Right),
                ]),
                ..default()
            },
        }
    }
}

fn player_movement_system(
    mut player_query: Query<(&mut Velocity, &ActionState<PlayerAction>), With<Player>>,
) {
    for (mut vel, action_state) in player_query.iter_mut() {
        if action_state.pressed(PlayerAction::Up) {
            vel.linvel.y = 100.0;
        }
        if action_state.pressed(PlayerAction::Down) {
            vel.linvel.y = -100.0;
        }
        if action_state.pressed(PlayerAction::Right) {
            vel.linvel.x = 100.0;
        }
        if action_state.pressed(PlayerAction::Left) {
            vel.linvel.x = -100.0;
        }
    }
}

struct AnimationTimer(Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, false))
    }
}

fn player_animation_system(
    mut player_query: Query<(&mut TextureAtlasSprite, &ActionState<PlayerAction>), With<Player>>,
    mut timer: Local<AnimationTimer>,
    time: Res<Time>,
) {
    for (mut sprite, action_state) in player_query.iter_mut() {
        sprite.index = if action_state.pressed(PlayerAction::Left) {
            timer.0.tick(time.delta());
            8 + ((sprite.index
                + if timer.0.finished() {
                    timer.0.reset();
                    1
                } else {
                    0
                })
                % 4)
        } else if action_state.pressed(PlayerAction::Right) {
            timer.0.tick(time.delta());
            12 + ((sprite.index
                + if timer.0.finished() {
                    timer.0.reset();
                    1
                } else {
                    0
                })
                % 4)
        } else if action_state.pressed(PlayerAction::Up) {
            timer.0.tick(time.delta());
            4 + ((sprite.index
                + if timer.0.finished() {
                    timer.0.reset();
                    1
                } else {
                    0
                })
                % 4)
        } else if action_state.pressed(PlayerAction::Down) {
            timer.0.tick(time.delta());
            0 + ((sprite.index
                + if timer.0.finished() {
                    timer.0.reset();
                    1
                } else {
                    0
                })
                % 4)
        } else if timer.0.finished() {
            timer.0.reset();
            sprite.index - sprite.index % 4
        } else {
            timer.0.tick(time.delta());
            sprite.index
        };
    }
}
