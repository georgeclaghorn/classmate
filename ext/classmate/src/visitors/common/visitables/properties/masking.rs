use crate::visitors::common::prelude::*;

use parcel_css::properties::masking::{ClipPath, Mask, MaskBorder};

impl Visitable for ClipPath<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                ClipPath::Url(url)    => url.accept(visitor),

                ClipPath::None        |
                ClipPath::Shape(_, _) |
                ClipPath::Box(_)      => Ok(())
            }
        })
    }
}

impl Visitable for Mask<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.image.accept(visitor))
    }
}

impl Visitable for MaskBorder<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.source.accept(visitor))
    }
}
