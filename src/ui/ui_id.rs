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
        Self(val)
    }
}

pub trait WithUiId {
    fn id<T: Debug + PartialEq + Eq + Hash>(self, id: T) -> (Self, UiId<T>)
    where
        Self: Sized,
    {
        (self, UiId(id))
    }
}
