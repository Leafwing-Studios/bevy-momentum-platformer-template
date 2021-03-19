use bevy::{math::vec2, prelude::*};

use crate::AssetLoadingState;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_system_set(SystemSet::on_update(AssetLoadingState::Finished).with_system(kinematics.system()));
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