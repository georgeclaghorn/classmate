mod stylesheet;
mod rules;
mod declaration;
mod properties;
mod values;

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

use smallvec::{SmallVec, Array};

impl<'a, T, const N: usize> Visitable<'a> for SmallVec<[T; N]>
where
    T: Visitable<'a>,
    [T; N]: Array<Item=T>
{
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        self.as_mut_slice().accept(visitor)
    }
}

use std::cell::RefMut;

impl<'a, T> Visitable<'a> for RefMut<'a, T> where T: Visitable<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        T::accept(self, visitor)
    }
}
