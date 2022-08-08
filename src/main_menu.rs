use super::*;
use std::f32::consts::PI;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<MainMenuAction>::default())
            .add_enter_system(GameState::MainMenu, spawn_main_menu_bundles)
            .add_system(resize_main_title.run_in_state(GameState::MainMenu))
            .add_system(game_start.run_in_state(GameState::MainMenu))
            .add_exit_system(GameState::MainMenu, despawn_main_menu_items);
    }
}

#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MainMenuAction {
    StartGame,
}

#[derive(Default, Component)]
struct MainMenuTitle;

#[derive(Default, Component)]
struct MainMenuItem;

struct TitleTimer(Timer);

impl Default for TitleTimer {
    fn default() -> Self {
        TitleTimer(Timer::from_seconds(5.0, true))
    }
}

fn spawn_main_menu_bundles(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                ..default()
            },
            color: Color::NONE.into(),
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
                            bottom: Val::Px(50.0),
                            right: Val::Auto,
                        },
                        ..default()
                    },
                    text: Text::with_section(
                        "Warp".to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/game.ttf"),
                            font_size: 200.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..default()
                })
                .insert(MainMenuTitle);
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
                    "Press Enter to start".to_string(),
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
            });
        })
        .insert(MainMenuItem);

    commands
        .spawn()
        .insert_bundle(InputManagerBundle {
            input_map: InputMap::new([
                (KeyCode::Return, MainMenuAction::StartGame),
                (KeyCode::Space, MainMenuAction::StartGame),
            ]),
            action_state: ActionState::default(),
        })
        .insert(MainMenuItem);
}

fn resize_main_title(
    mut title_query: Query<&mut Transform, With<MainMenuTitle>>,
    time: Res<Time>,
    mut timer: Local<TitleTimer>,
) {
    timer.0.tick(time.delta());
    for mut transform in title_query.iter_mut() {
        transform.scale = Vec3::splat(((timer.0.percent() * PI).sin() - 0.5) * -2.0);
    }
}

fn game_start(
    mut commands: Commands,
    action_query: Query<&ActionState<MainMenuAction>, Changed<ActionState<MainMenuAction>>>,
) {
    for action in action_query.iter() {
        if action.pressed(MainMenuAction::StartGame) {
            commands.insert_resource(NextState(GameState::Loading));
        }
    }
}

fn despawn_main_menu_items(items_query: Query<Entity, With<MainMenuItem>>, mut commands: Commands) {
    for entity in items_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
