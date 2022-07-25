use super::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(InGameState::Playing)
            .add_plugin(InputManagerPlugin::<PauseMenuAction>::default())
            .add_enter_system(
                InGameState::Paused,
                spawn_pause_menu_ui.run_in_state(GameState::InGame),
            )
            .add_enter_system(GameState::InGame, spawn_pause_menu_detector)
            .add_system(
                unpause_game
                    .run_in_state(InGameState::Paused)
                    .run_in_state(GameState::InGame),
            )
            .add_system(pause_game.run_in_state(GameState::InGame))
            .add_exit_system(GameState::InGame, despawn_pause_menu_detector)
            .add_exit_system(
                InGameState::Paused,
                despawn_pause_menu_items.run_in_state(GameState::InGame),
            );
    }
}

#[derive(Default, Component)]
struct PauseMenuItem;

#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PauseMenuAction {
    Close,
    Opening,
}

#[derive(Default, Component)]
struct PauseMenuInput;

fn spawn_pause_menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::Center,
                margin: Rect {
                    top: Val::Px(0.0),
                    left: Val::Auto,
                    bottom: Val::Px(0.0),
                    right: Val::Auto,
                },
                padding: Rect::all(Val::Px(10.0)),
                ..default()
            },
            color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        margin: Rect {
                            top: Val::Px(0.0),
                            left: Val::Auto,
                            bottom: Val::Px(0.0),
                            right: Val::Auto,
                        },
                        ..default()
                    },
                    text: Text::with_section(
                        "Paused".to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/game.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..default()
                })
                .insert(PauseMenuItem);
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: Rect {
                        top: Val::Px(0.0),
                        left: Val::Auto,
                        bottom: Val::Px(0.0),
                        right: Val::Auto,
                    },
                    ..default()
                },
                text: Text::with_section(
                    "Press Enter to continue".to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/game.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..default()
            });
        })
        .insert(PauseMenuItem);
}

fn spawn_pause_menu_detector(mut commands: Commands) {
    commands.spawn().insert_bundle(InputManagerBundle {
        input_map: InputMap::new([
            (KeyCode::Escape, PauseMenuAction::Opening),
            (KeyCode::Return, PauseMenuAction::Close),
        ]),
        action_state: ActionState::default(),
    });
}

fn unpause_game(
    mut commands: Commands,
    action_query: Query<&ActionState<PauseMenuAction>, Changed<ActionState<PauseMenuAction>>>,
) {
    for action in action_query.iter() {
        if action.pressed(PauseMenuAction::Close) {
            commands.insert_resource(NextState(InGameState::Playing));
        }
    }
}

fn pause_game(
    mut commands: Commands,
    action_query: Query<&ActionState<PauseMenuAction>, Changed<ActionState<PauseMenuAction>>>,
) {
    for action in action_query.iter() {
        if action.pressed(PauseMenuAction::Opening) {
            commands.insert_resource(NextState(InGameState::Paused));
        }
    }
}

fn despawn_pause_menu_detector(
    mut commands: Commands,
    manager_query: Query<Entity, With<ActionState<PauseMenuAction>>>,
) {
    for entity in manager_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_pause_menu_items(
    items_query: Query<Entity, With<PauseMenuItem>>,
    mut commands: Commands,
) {
    for entity in items_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
