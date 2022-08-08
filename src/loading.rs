use super::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::MainMenu);

        AssetLoader::new(GameState::Loading)
            .continue_to_state(GameState::InGame)
            .with_collection::<LevelAsset>()
            .build(app);
    }
}

#[derive(AssetCollection)]
pub struct LevelAsset {
    #[asset(path = "Levels.ldtk")]
    pub handle: Handle<LdtkAsset>,
}
