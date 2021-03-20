use bevy::prelude::*;

pub fn compute_overlap(
  transform_1: Transform, 
  transform_2: Transform, 
  sprite_1: &Sprite, 
  sprite_2: &Sprite
) -> Vec2 {

  // the minimum distance that the two sprites could be on each axis without overlapping
  let total_half_size = sprite_1.size / 2.0 + sprite_2.size / 2.0;
  // the distance they two sprites are apart on each axis
  let total_distance = (transform_1.translation - transform_2.translation).truncate();
  let overlap = total_half_size - total_distance.abs(); // the amount of overlap on each axis

  overlap
}

#[cfg(test)]
mod tests {
  use bevy::prelude::*;
  use super::*;

  fn test_wrapper(
    position_1: (f32, f32),
    position_2: (f32, f32),
    size_1: (f32, f32),
    size_2: (f32, f32),
    collision: Vec2,
  ) {
    let transform_1 = Transform::from_xyz(position_1.0, position_1.1, 0.0);
    let transform_2 = Transform::from_xyz(position_2.0, position_2.1, 0.0);
    let sprite_1 = Sprite::new(Vec2::new(size_1.0, size_1.1));
    let sprite_2 = Sprite::new(Vec2::new(size_2.0, size_2.1));

    assert_eq!(compute_overlap(transform_1, transform_2, &sprite_1, &sprite_2), collision);
  }

  #[test]
  fn basic_test() {
    let collision = Vec2::new(5.0, 5.0);

    test_wrapper(
      (0.0, 0.0),
      (5.0, 5.0),
      (10.0, 10.0),
      (10.0, 10.0),
      collision
    );
  }
}