use super::*;

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PauseMenuAction>::default())
            .add_enter_system(PauseState::Paused, spawn_pause_menu)
            .add_exit_system(PauseState::Paused, despawn_entities_with::<PauseMenuItem>)
            .add_startup_system(spawn_pause_menu_detector)
            .add_system(change_pause_state.run_in_state(GameState::Game));
    }
}

#[derive(Default, Component)]
struct PauseMenuItem;

#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PauseMenuAction {
    Close,
    Open,
}

fn spawn_pause_menu(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::Center,
                margin: UiRect {
                    top: Val::Px(0.0),
                    left: Val::Auto,
                    bottom: Val::Px(0.0),
                    right: Val::Auto,
                },
                padding: UiRect::all(Val::Px(10.0)),
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
                        margin: UiRect {
                            top: Val::Px(0.0),
                            left: Val::Auto,
                            bottom: Val::Px(0.0),
                            right: Val::Auto,
                        },
                        ..default()
                    },
                    text: Text::from_section(
                        "Paused".to_string(),
                        TextStyle {
                            font: font_assets.game.clone(),
                            font_size: 60.0,
                            color: Color::WHITE,
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
                    margin: UiRect {
                        top: Val::Px(0.0),
                        left: Val::Auto,
                        bottom: Val::Px(0.0),
                        right: Val::Auto,
                    },
                    ..default()
                },
                text: Text::from_section(
                    "Press Enter to continue".to_string(),
                    TextStyle {
                        font: font_assets.game.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        })
        .insert(PauseMenuItem);
}

fn spawn_pause_menu_detector(mut commands: Commands) {
    commands.spawn_bundle(InputManagerBundle {
        input_map: InputMap::new([
            (KeyCode::Escape, PauseMenuAction::Open),
            (KeyCode::Return, PauseMenuAction::Close),
        ]),
        action_state: ActionState::default(),
    });
}

fn change_pause_state(
    mut commands: Commands,
    action_query: Query<&ActionState<PauseMenuAction>>,
    current_state: Res<CurrentState<PauseState>>,
) {
    for action in &action_query {
        if action.pressed(PauseMenuAction::Close) && matches!(current_state.0, PauseState::Paused) {
            commands.insert_resource(NextState(PauseState::Unpaused));
        } else if action.pressed(PauseMenuAction::Open)
            && matches!(current_state.0, PauseState::Unpaused)
        {
            commands.insert_resource(NextState(PauseState::Paused));
        }
    }
}
