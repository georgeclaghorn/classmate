use crate::visitors::common::prelude::*;

use parcel_css::rules::page::PageRule;

impl Visitable for PageRule<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.declarations.accept(visitor))
    }
}
