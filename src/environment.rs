use bevy::prelude::*;
use crate::physics::{Collides, Solid};
use crate::AssetLoadingState;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(SystemSet::on_enter(AssetLoadingState::Finished).with_system(setup_environment.system()));
    }
}

fn setup_environment(
	mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let floor_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let floor_thickness = 10.0;

    commands
        .spawn(SpriteBundle {
            material: floor_material.clone(),
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            sprite: Sprite::new(Vec2::new(900.0, floor_thickness)),
            ..Default::default()
        })
        .with(Collides)
        .with(Solid);
}