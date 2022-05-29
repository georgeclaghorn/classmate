use crate::visitors::common::prelude::*;

use parcel_css::properties::background::Background;

impl Visitable for Background<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.image.accept(visitor))
    }
}
