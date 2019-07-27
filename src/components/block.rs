use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub enum Block {
    Background,
    Explodable,
    Solid,
    Portal,
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}
