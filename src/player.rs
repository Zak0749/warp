use super::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Game)
                    .run_not_in_state(PauseState::Paused)
                    .with_system(player_movement)
                    .with_system(player_animation)
                    .with_system(player_state_tracker)
                    .with_system(update_ability)
                    .with_system(spawn_past_player)
                    .with_system(update_past_player)
                    .with_system(remove_past_when_not_preforming)
                    .into(),
            );
    }
}

#[derive(Bundle, Default, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    past_states: PlayerPastStates,
    ability_state: PlayerAbilityState,

    #[bundle]
    collider: PlayerColliderBundle,

    #[bundle]
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,

    #[bundle]
    input_manager: PlayerInput,
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
struct PlayerColliderBundle {
    collider: Collider,
    rigid_body: RigidBody,
    velocity: Velocity,
    gravity_scale: GravityScale,
    locked_axis: LockedAxes,
    friction: Friction,
    dampening: Damping,
}

impl Default for PlayerColliderBundle {
    fn default() -> Self {
        Self {
            collider: Collider::cuboid(7.0, 9.0),
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

#[derive(Component, Default, Clone)]
struct PlayerPastStates(Vec<PlayerPastState>);

#[derive(Clone)]
struct PlayerPastState {
    translation: Vec3,
    index: usize,
}

#[derive(Component, Default, Debug)]
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
    for (mut vel, action_state) in &mut player_query {
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
    mut player_query: Query<
        (
            &mut TextureAtlasSprite,
            &ActionState<PlayerAction>,
            &PlayerAbilityState,
        ),
        With<Player>,
    >,
    mut timer: Local<AnimationTimer>,
    time: Res<Time>,
) {
    for (mut sprite, action_state, ability_state) in &mut player_query {
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

        match ability_state {
            PlayerAbilityState::Cooldown => sprite.color = Color::GRAY,
            _ => sprite.color = Color::WHITE,
        }
    }
}

struct StateTimer(Timer);

impl Default for StateTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, false))
    }
}

fn player_state_tracker(
    mut player_query: Query<(&mut PlayerPastStates, &TextureAtlasSprite, &Transform), With<Player>>,
    mut timer: Local<StateTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        for (mut past_states, sprite, transform) in player_query.iter_mut() {
            if past_states.0.len() > 100 {
                past_states.0.remove(0);
            }
            past_states.0.push(PlayerPastState {
                translation: transform.translation,
                index: sprite.index,
            });
        }
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
    for (mut ability_state, action_state) in &mut player_query {
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

fn spawn_past_player(
    mut commands: Commands,
    player_query: Query<
        (
            &PlayerAbilityState,
            &PlayerPastStates,
            &Handle<TextureAtlas>,
        ),
        Changed<PlayerAbilityState>,
    >,
) {
    for (ability_state, past_states, atlas) in &player_query {
        if matches!(ability_state, PlayerAbilityState::Preforming) {
            commands
                .spawn()
                .insert(PastPlayer)
                .insert_bundle(PlayerColliderBundle::default())
                .insert_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        color: Color::CYAN,
                        index: past_states.0.first().unwrap().index,
                        ..default()
                    },
                    texture_atlas: atlas.clone(),
                    transform: Transform {
                        translation: past_states.0.first().unwrap().translation,
                        // scale: Vec3::new(10.0, 10.0, 1.0),
                        ..default()
                    },
                    ..default()
                });
        };
    }
}

fn update_past_player(
    player_query: Query<&PlayerPastStates, With<Player>>,
    mut past_player_query: Query<(&mut Transform, &mut TextureAtlasSprite), With<PastPlayer>>,
) {
    for past_states in &player_query {
        for (mut transform, mut sprite) in &mut past_player_query {
            transform.translation = past_states.0.first().unwrap().translation;
            sprite.index = past_states.0.first().unwrap().index;
        }
    }
}

fn remove_past_when_not_preforming(
    mut commands: Commands,
    player_query: Query<&PlayerAbilityState, Changed<PlayerAbilityState>>,
    past_player_query: Query<Entity, (With<PastPlayer>, Without<Player>)>,
) {
    for ability_state in &player_query {
        if !matches!(ability_state, PlayerAbilityState::Preforming) {
            for past_player_entity in &past_player_query {
                commands.entity(past_player_entity).despawn_recursive();
            }
        }
    }
}

#[derive(Component, Default)]
struct PastPlayer;
