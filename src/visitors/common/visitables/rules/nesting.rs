use crate::visitors::common::prelude::*;

use parcel_css::rules::nesting::NestingRule;

impl<'a> Visitable<'a> for NestingRule<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.style.accept(visitor))
    }
}
