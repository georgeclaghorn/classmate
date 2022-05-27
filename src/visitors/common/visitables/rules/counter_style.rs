use crate::visitors::common::prelude::*;

use parcel_css::rules::counter_style::CounterStyleRule;

impl<'a> Visitable<'a> for CounterStyleRule<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.declarations.accept(visitor))
    }
}
