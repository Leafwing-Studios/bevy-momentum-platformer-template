#![allow(dead_code, unused_variables, unused_mut, unused_imports)]
use bevy::prelude::*;
use bevy::asset::LoadState;

mod physics;
mod player;
mod environment;

fn main() {
    App::build()
    .init_resource::<SpriteHandles>()
    .add_plugins(DefaultPlugins)
    .add_state(AssetLoadingState::Loading)
    .add_system_set(SystemSet::on_enter(AssetLoadingState::Loading).with_system(load_textures.system()))
    .add_system_set(SystemSet::on_update(AssetLoadingState::Loading).with_system(check_textures.system()))
    .add_system_set(SystemSet::on_enter(AssetLoadingState::Finished).with_system(setup.system()))
    .add_plugin(environment::EnvironmentPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(physics::PhysicsPlugin)
    .run();
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AssetLoadingState {
    Loading,
    Finished,
}

#[derive(Default)]
struct SpriteHandles {
    handles: Vec<Handle<ColorMaterial>>,
}

fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    sprite_handles.handles.push(asset_server.load("player.png"));
}

fn check_textures(
    mut state: ResMut<State<AssetLoadingState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set_next(AssetLoadingState::Finished).unwrap();
    }
}

fn setup(
    mut commands: Commands,
) {
    commands
        .spawn(OrthographicCameraBundle::new_2d())
        .spawn(UiCameraBundle::default());
}