use crate::visitors::common::prelude::*;

use parcel_css::properties::background::Background;

impl<'a> Visitable<'a> for Background<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.image.accept(visitor))
    }
}
