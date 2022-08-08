use super::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_system(
                player_movement
                    .run_in_state(GameState::InGame)
                    .run_not_in_state(InGameState::Paused),
            )
            .add_system(
                player_animation
                    .run_in_state(GameState::InGame)
                    .run_not_in_state(InGameState::Paused),
            )
            .register_ldtk_entity::<PlayerBundle>("Player")
            // .add_system(setup_test)
            // .add_system(
            //     follower_moving
            //         .run_in_state(GameState::InGame)
            //         .run_not_in_state(InGameState::Paused),
            // )
            .add_system(
                player_state_tracker
                    .run_in_state(GameState::InGame)
                    .run_not_in_state(InGameState::Paused),
            )
            .add_system(
                test_state
                    .run_in_state(GameState::InGame)
                    .run_not_in_state(InGameState::Paused),
            )
            .add_system(
                update_ability
                    .run_in_state(GameState::InGame)
                    .run_not_in_state(InGameState::Paused),
            );
    }
}

#[derive(Bundle, Default, LdtkEntity)]
struct PlayerBundle {
    player: Player,

    past_states: PlayerPastStates,

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

    ability_state: PlayerAbilityState,
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
    Ability,
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
                    (KeyCode::Space, Ability),
                ]),
                ..default()
            },
        }
    }
}

#[derive(Component, Default)]
struct PlayerPastStates(Vec<PlayerPastState>);

struct PlayerPastState {
    translation: Vec3,
    index: usize,
}

#[derive(Component, Default)]
enum PlayerAbilityState {
    Preforming,
    Cooldown,

    #[default]
    Idle,
}

fn player_movement(
    mut player_query: Query<
        (&mut Velocity, &ActionState<PlayerAction>),
        (With<Player>, Changed<ActionState<PlayerAction>>),
    >,
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

fn player_animation(
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

struct StateTimer(Timer);

impl Default for StateTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, false))
    }
}

fn player_state_tracker(
    mut player_query: Query<
        (
            &mut PlayerPastStates,
            &TextureAtlasSprite,
            &Transform,
        ),
        With<Player>,
    >,
    mut timer: Local<StateTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        for (mut past_states, sprite, transform) in player_query.iter_mut() {
            if past_states.0.len() > 200 {
                past_states.0.remove(0);
            }
            past_states.0.push(PlayerPastState {
                translation: transform.translation,
                index: sprite.index,
            });
        }
    }
}

fn test_state(
    mut player_query: Query<(&PlayerAbilityState, &mut TextureAtlasSprite), With<Player>>,
) {
    for (ability_state, mut sprite) in player_query.iter_mut() {
        sprite.color = match ability_state {
            PlayerAbilityState::Preforming => Color::RED,
            PlayerAbilityState::Cooldown => Color::GRAY,
            PlayerAbilityState::Idle => Color::WHITE,
        };
    }
}

struct CooldownTimer(Timer);

impl Default for CooldownTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(3.0, false))
    }
}

struct UsageTimer(Timer);

impl Default for UsageTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(5.0, false))
    }
}

fn update_ability(
    mut player_query: Query<(&mut PlayerAbilityState, &ActionState<PlayerAction>), With<Player>>,
    mut cooldown_timer: Local<CooldownTimer>,
    mut usage_timer: Local<UsageTimer>,
    time: Res<Time>,
) {
    for (mut ability_state, action_state) in player_query.iter_mut() {
        match *ability_state {
            PlayerAbilityState::Idle => {
                if action_state.pressed(PlayerAction::Ability)
                    && action_state.get_pressed().iter().any(|&x| {
                        x == PlayerAction::Up
                            || x == PlayerAction::Down
                            || x == PlayerAction::Left
                            || x == PlayerAction::Right
                    })
                {
                    *ability_state = PlayerAbilityState::Preforming;
                }
            }
            PlayerAbilityState::Preforming => {
                usage_timer.0.tick(time.delta());
                if !action_state.get_pressed().iter().any(|&x| {
                    x == PlayerAction::Up
                        || x == PlayerAction::Down
                        || x == PlayerAction::Left
                        || x == PlayerAction::Right
                }) || usage_timer.0.finished()
                {
                    usage_timer.0.reset();
                    *ability_state = PlayerAbilityState::Cooldown;
                }
            }
            PlayerAbilityState::Cooldown => {
                cooldown_timer.0.tick(time.delta());
                if cooldown_timer.0.finished() {
                    cooldown_timer.0.reset();
                    *ability_state = PlayerAbilityState::Idle;
                }
            }
        }
    }
}

// #[derive(Component, Default)]
// struct PastPlayer;

// #[derive(Bundle, Default)]
// struct PastPlayerBundle {
//     past_player: PastPlayer,

//     #[bundle]
//     sprite: SpriteBundle,

//     rigid_body: RigidBody,
//     collider: Collider,
//     locked_axis: LockedAxes,
// }

// fn setup_test(mut commands: Commands, player_query: Query<&Transform, Added<Player>>) {
//     for transform in player_query.iter() {
//         commands.spawn_bundle(PastPlayerBundle {
//             sprite: SpriteBundle {
//                 sprite: Sprite {
//                     color: Color::GOLD,
//                     ..default()
//                 },
//                 transform: Transform {
//                     scale: Vec3::new(12.0, 18.0, 1.0),
//                     translation: player_query.single().translation,
//                     ..default()
//                 },
//                 ..default()
//             },
//             rigid_body: RigidBody::KinematicPositionBased,
//             collider: Collider::cuboid(6.0, 9.0),
//             locked_axis: LockedAxes::ROTATION_LOCKED,
//             ..default()
//         });
//     }
// }

// fn follower_moving(
//     mut follower_query: Query<&mut Transform, With<PastPlayer>>,
//     player_query: Query<&PlayerPastStates, With<Player>>,
// ) {
//     for mut transform in follower_query.iter_mut() {
//         for past_state in player_query.iter() {
//             if let Some(past_state) = past_state.0.first() {
//                 transform.translation = past_state.translation;
//             }
//         }
//     }
// }
