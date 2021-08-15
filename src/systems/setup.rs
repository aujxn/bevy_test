use crate::components::*;
use crate::entities::*;
use bevy::prelude::*;

pub fn setup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn().insert(UserControls::new());
    commands.spawn_bundle(PlayerBundle::new(&asset_server, &mut materials));
    commands.spawn_bundle(MobBundle::new(&asset_server, &mut materials));
    commands.spawn_bundle(DashBundle::new());
}
