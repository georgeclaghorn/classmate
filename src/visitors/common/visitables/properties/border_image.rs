use crate::visitors::common::prelude::*;

use parcel_css::properties::border_image::BorderImage;

impl Visitable for BorderImage<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.source.accept(visitor))
    }
}
