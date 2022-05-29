use crate::visitors::common::prelude::*;

use parcel_css::rules::counter_style::CounterStyleRule;

impl Visitable for CounterStyleRule<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.declarations.accept(visitor))
    }
}
