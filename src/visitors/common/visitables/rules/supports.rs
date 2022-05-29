use crate::visitors::common::prelude::*;

use parcel_css::rules::supports::SupportsRule;

impl Visitable for SupportsRule<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.rules.accept(visitor))
    }
}
