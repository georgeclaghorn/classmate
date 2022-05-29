mod stylesheet;
mod rules;
mod declaration;
mod properties;
mod values;

use std::cell::RefMut;
use crate::visitors::{Visitor, Visitable};

impl<'a, T> Visitable<'a> for [T] where T: Visitable<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        self.iter_mut().try_for_each(|item| item.accept(visitor))
    }
}

impl<'a, T> Visitable<'a> for Vec<T> where T: Visitable<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        self.as_mut_slice().accept(visitor)
    }
}

impl<'a, T> Visitable<'a> for RefMut<'a, T> where T: Visitable<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        T::accept(self, visitor)
    }
}
