use crate::visitors::common::prelude::*;

use parcel_css::rules::page::PageRule;

impl Visitable for PageRule<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.declarations.accept(visitor))
    }
}
