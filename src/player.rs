use bevy::{math::vec2, prelude::*};
use crate::physics::{Collides, Gravity, PhysicsBundle, Solid, Velocity};
use crate::AssetLoadingState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
			.add_system_set(SystemSet::on_enter(AssetLoadingState::Finished)
				.with_system(setup_player.system())
			)
			.add_system_set(SystemSet::on_update(AssetLoadingState::Finished)
				.with_system(control_player.system())
			);
	}
}

pub struct Player;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PlayerState {
	OnFloor,
	Airborne,
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
		.with_bundle((Collides, Solid, Gravity, Player, PlayerState::Airborne))
		.with_bundle(PhysicsBundle::player());
}

fn control_player(
	mut query: Query<(&mut Velocity, &mut PlayerState), With<Player>>,
	keyboard_input: Res<Input<KeyCode>>,
) {
	let (mut velocity, mut player_state) = query.single_mut().unwrap();

	if keyboard_input.just_pressed(KeyCode::Space) {	
		velocity.0.y = 500.0;
		*player_state = PlayerState::Airborne;
	}

	if keyboard_input.pressed(KeyCode::Right) {
		velocity.0 += vec2(50.0, 0.0);
	} else if keyboard_input.pressed(KeyCode::Left) {
		velocity.0 += vec2(-50.0, 0.0);
	} else {
		velocity.0.x = 0.0;
	}
}