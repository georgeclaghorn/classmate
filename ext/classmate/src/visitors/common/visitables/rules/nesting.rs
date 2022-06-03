use crate::visitors::common::prelude::*;

use parcel_css::rules::nesting::NestingRule;

impl Visitable for NestingRule<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.style.accept(visitor))
    }
}
