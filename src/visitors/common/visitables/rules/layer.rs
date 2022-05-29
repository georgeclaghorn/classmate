use crate::visitors::common::prelude::*;

use parcel_css::rules::layer::LayerBlockRule;

impl Visitable for LayerBlockRule<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.rules.accept(visitor))
    }
}
