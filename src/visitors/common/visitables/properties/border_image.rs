use crate::visitors::common::prelude::*;

use parcel_css::properties::border_image::BorderImage;

impl<'a> Visitable<'a> for BorderImage<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.source.accept(visitor))
    }
}
