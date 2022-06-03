use crate::visitors::common::prelude::*;

use parcel_css::rules::supports::SupportsRule;

impl Visitable for SupportsRule<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.rules.accept(visitor))
    }
}
