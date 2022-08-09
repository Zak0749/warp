use super::*;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<LevelsAsset>()
                .with_collection::<AudioAssets>()
                .with_collection::<FontAssets>()
                .continue_to_state(GameState::Game),
        );
    }
}

#[derive(AssetCollection)]
pub struct LevelsAsset {
    #[asset(path = "Levels.ldtk")]
    pub map: Handle<LdtkAsset>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/music.ogg")]
    pub music: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/game.ttf")]
    pub game: Handle<Font>,
}
