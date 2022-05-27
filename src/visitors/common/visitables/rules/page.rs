use crate::visitors::common::prelude::*;

use parcel_css::rules::page::PageRule;

impl<'a> Visitable<'a> for PageRule<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.declarations.accept(visitor))
    }
}
