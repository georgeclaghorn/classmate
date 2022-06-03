use crate::visitors::common::prelude::*;

use parcel_css::properties::border_image::BorderImage;

impl Visitable for BorderImage<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.source.accept(visitor))
    }
}
