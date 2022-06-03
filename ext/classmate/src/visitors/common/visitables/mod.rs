mod stylesheet;
mod rules;
mod declaration;
mod properties;
mod values;

use crate::visitors::{Visitor, Visitable};

impl<'a, T> Visitable for [T] where T: Visitable {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        self.iter_mut().try_for_each(|item| item.accept(visitor))
    }
}

impl<'a, T> Visitable for Vec<T> where T: Visitable {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        self.as_mut_slice().accept(visitor)
    }
}

use smallvec::{SmallVec, Array};

impl<'a, T, const N: usize> Visitable for SmallVec<[T; N]>
where
    T: Visitable,
    [T; N]: Array<Item=T>
{
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        self.as_mut_slice().accept(visitor)
    }
}

use std::cell::RefMut;

impl<'a, T> Visitable for RefMut<'a, T> where T: Visitable {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        T::accept(self, visitor)
    }
}
