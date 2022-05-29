use crate::visitors::common::prelude::*;

use parcel_css::rules::nesting::NestingRule;

impl Visitable for NestingRule<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.style.accept(visitor))
    }
}
