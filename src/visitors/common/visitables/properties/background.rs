use crate::visitors::common::prelude::*;

use parcel_css::properties::background::Background;

impl Visitable for Background<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.image.accept(visitor))
    }
}
