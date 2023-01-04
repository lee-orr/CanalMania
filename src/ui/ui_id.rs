use bevy::prelude::*;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Component, Debug)]
pub struct UiId<T: Debug + PartialEq + Eq + Hash>(T);

impl<T: Debug + PartialEq + Eq + Hash> UiId<T> {
    pub fn val(&self) -> &T {
        &self.0
    }

    pub fn new(val: T) -> Self {
        println!("Creating ID {val:?}");
        Self(val)
    }
}
