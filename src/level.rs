use super::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Game, spawn_level)
            .add_system(
                update_level_selection
                    .run_in_state(GameState::Game)
                    .run_not_in_state(PauseState::Paused),
            );
    }
}

fn spawn_level(mut commands: Commands, level: Res<LevelsAsset>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: level.map.clone(),
        ..default()
    });
}

fn update_level_selection(
    level_query: Query<(&Handle<LdtkLevel>, &Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    for (level_handle, level_transform) in &level_query {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            for player_transform in &player_query {
                if player_transform.translation.x
                    < level_transform.translation.x + ldtk_level.level.px_wid as f32
                    && player_transform.translation.x > level_transform.translation.x
                    && player_transform.translation.y
                        < level_transform.translation.y + ldtk_level.level.px_hei as f32
                    && player_transform.translation.y > level_transform.translation.y
                    && !level_selection.is_match(&0, &ldtk_level.level)
                {
                    *level_selection = LevelSelection::Iid(ldtk_level.level.iid.clone());
                }
            }
        }
    }
}
