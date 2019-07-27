use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
};

pub struct Bomberman {}

impl Component for Bomberman {
    type Storage = DenseVecStorage<Self>;
}
