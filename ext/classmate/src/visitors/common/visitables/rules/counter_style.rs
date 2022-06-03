use crate::visitors::common::prelude::*;

use parcel_css::rules::counter_style::CounterStyleRule;

impl Visitable for CounterStyleRule<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.declarations.accept(visitor))
    }
}
