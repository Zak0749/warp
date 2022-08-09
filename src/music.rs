use super::*;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Game, start_background_music)
            .add_exit_system(GameState::Game, stop_background_music);
    }
}

fn start_background_music(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play_looped(audio_assets.music.clone());
    audio.set_volume(0.3);
}

fn stop_background_music(audio: Res<Audio>) {
    audio.stop();
}
