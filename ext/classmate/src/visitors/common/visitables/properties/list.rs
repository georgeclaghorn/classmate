use crate::visitors::common::prelude::*;

use parcel_css::properties::list::ListStyle;

impl Visitable for ListStyle<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.image.accept(visitor))
    }
}
