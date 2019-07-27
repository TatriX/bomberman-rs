use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Default)]
pub struct Bomberman {
    pub velocity: [f32; 2],
}

impl Component for Bomberman {
    type Storage = DenseVecStorage<Self>;
}
