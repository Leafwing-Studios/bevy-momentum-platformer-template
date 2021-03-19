use bevy::prelude::*;
use crate::physics::{Collides, Gravity, PhysicsBundle, Solid};
use crate::AssetLoadingState;

pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
			.add_system_set(SystemSet::on_enter(AssetLoadingState::Finished).with_system(setup_player.system()));
	}
}
fn setup_player(
	mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let texture_handle = asset_server.get_handle("player.png");

    commands
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
		.with_bundle((Collides, Solid, Gravity))
		.with_bundle(PhysicsBundle::player());
}