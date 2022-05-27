use crate::visitors::common::prelude::*;

use parcel_css::rules::layer::LayerBlockRule;

impl<'a> Visitable<'a> for LayerBlockRule<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.rules.accept(visitor))
    }
}
