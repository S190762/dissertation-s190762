use specs::{
    prelude::*,
    saveload::{
        U64Marker,
        U64MarkerAllocator,
        Marker,
        MarkerAllocator
    },
};
use serde::{
    Deserialize,
    Serialize
};
use nalgebra::Vector2;
use specs_derive::Component;

#[derive(Component)]
pub struct ActionComponent {
    pub remaining_actions: Vec<Actions>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Actions {
    NoActions,
    MoveEntity(Vector2<f64>),
    DeleteEntity,
    ModifyEntity(Option<U64Marker>, Box<Actions>),
    ModifyEntities(Option<U64Marker>, Vec<Actions>),
    MultipleActions(Vec<Actions>),
    PrintText(String)
}