use crate::visitors::common::prelude::*;

use parcel_css::properties::list::ListStyle;

impl<'a> Visitable<'a> for ListStyle<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.image.accept(visitor))
    }
}
