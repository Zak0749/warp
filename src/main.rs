use warp::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 512.0,
            height: 512.0,
            ..Default::default()
        })
        .add_loopless_state(GameState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(PlayerPlugin)
        .add_plugin(WallPlugin)
        .add_plugin(DoorPlugin)
        .add_plugin(SwitchPlugin)
        .add_plugin(BoxPlugin)
        .add_plugin(GameCameraPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(InGamePlugin)
        .add_plugin(PauseMenuPlugin)
        .add_plugin(UiCameraPlugin)
        .run();
}
