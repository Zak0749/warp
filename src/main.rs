use bevy::render::texture::ImageSettings;
use warp::*;

fn main() {
    App::new()
        .add_loopless_state(GameState::Loading)
        .add_loopless_state(PauseState::Unpaused)
        .insert_resource(WindowDescriptor {
            width: 512.0,
            height: 512.0,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(LdtkPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(AssetPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(MusicPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(WallsPlugin)
        .add_plugin(BoxPlugin)
        .add_plugin(SwitchPlugin)
        .add_plugin(DoorPlugin)
        .add_plugin(PausedPlugin)
        .run();
}
