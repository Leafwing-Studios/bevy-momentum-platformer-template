use std::env;

use bevy::{math::vec2, prelude::*};
use crate::{AssetLoadingState, player::Player, player::PlayerState, utils::compute_overlap};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
			.add_system_set(SystemSet::on_update(AssetLoadingState::Finished)
				.with_system(kinematics.system())
				.with_system(floor_collision.system())
			);
	}
}

const GRAVITY_COEFFICIENT: f32 = -9.8 * 50.0;

/// Marker component for things that obey gravity
pub struct Gravity;

/// Marker component for things that have collision. This is purely for registering collisions, not necessarily things that would push each other around.
pub struct Collides;

/// Marker component for things that take up space, and push each other around. Solid entities cannot occupy the same space as other solid entities
pub struct Solid;

/// How much mass does this thing have?
pub struct Mass(pub f32); // TODO: enforce this is positive

/// Applying a force to something
pub struct Force(pub Vec2);

pub struct Velocity(pub Vec2);

pub struct Acceleration(pub Vec2);

#[derive(Bundle)]
pub struct PhysicsBundle {
	mass: Mass,
	velocity: Velocity,
	acceleraction: Acceleration,
}

impl PhysicsBundle {
	pub fn player() -> Self {
		PhysicsBundle {
			mass: Mass(1.0),
			velocity: Velocity(Vec2::ZERO),
			acceleraction: Acceleration(vec2(0.0, GRAVITY_COEFFICIENT))
		}
	}
}

fn kinematics(
	mut query: Query<(&Acceleration, &mut Velocity, &mut Transform)>,
	time: Res<Time>,
) {
	let delta_time = time.delta_seconds();

	for (acceleration, mut velocity, mut transform) in query.iter_mut() {
		velocity.0 += acceleration.0 * delta_time;
		transform.translation += (velocity.0 * delta_time).extend(0.0);
	}
}

fn floor_collision(
	mut player_query: Query<(&mut Transform, &Velocity, &Sprite, &mut PlayerState), With<Player>>,
	environment_query: Query<(&Transform, &Sprite), (With<Collides>, With<Solid>, Without<Player>)>,
) {
	let (player_transform, player_velocity, player_sprite, player_state) = player_query.single_mut().unwrap();

	if *player_state == PlayerState::Airborne {
		for (platform_transform, platform_sprite) in environment_query.iter() {
			// println!("Transform: {:?}\nSprite: {:?}", platform_transform, platform_sprite);

			let overlap = compute_overlap(
				*player_transform,
				*platform_transform,
				player_sprite,
				platform_sprite,
			);

			if overlap.length_squared() <= 0.0 {
				
			}
		}
	}
}