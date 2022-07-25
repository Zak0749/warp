use super::*;

pub struct UiCameraPlugin;

impl Plugin for UiCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ui_camera);
    }
}

#[derive(Component, Default)]
struct UiCamera;

fn spawn_ui_camera(mut commands: Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);
}
