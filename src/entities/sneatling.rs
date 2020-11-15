use amethyst::{
    assets::Handle,
    core::transform::Transform,
    prelude::*,
    renderer::{ SpriteRender, SpriteSheet, },
};

use crate::components::{Shape, Sneatling, Velocity, Collides};
use crate::constants::ARENA_HEIGHT;

const SNEATLING_WIDTH: f32 = 3.0;
const SNEATLING_HEIGHT: f32 = 2.0;

pub fn initialise_sneatling(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    let mut default_transform = Transform::default();
    let center = ARENA_HEIGHT / 2.0;
    default_transform.set_translation_xyz(center, center, 0.75);

    world
        .create_entity()
        .with(Sneatling::new())
        .with(Collides::new())
        .with(Shape::new(SNEATLING_WIDTH, SNEATLING_HEIGHT))
        .with(Velocity::new())
        .with(default_transform)
        .with(sprite_render)
        .build();
}
