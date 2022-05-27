use crate::visitors::common::prelude::*;

use std::cell::RefMut;
use parcel_css::declaration::DeclarationBlock;

impl<'a> Visitable<'a> for RefMut<'a, DeclarationBlock<'a>> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            self.important_declarations.accept(visitor)?;
            self.declarations.accept(visitor)
        })
    }
}

impl<'a> Visitable<'a> for DeclarationBlock<'a> {
  fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
          self.important_declarations.accept(visitor)?;
          self.declarations.accept(visitor)
      })
  }
}
