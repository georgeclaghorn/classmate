use crate::visitors::common::prelude::*;

use parcel_css::properties::masking::{ClipPath, Mask, MaskBorder};

impl<'a> Visitable<'a> for ClipPath<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
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

impl<'a> Visitable<'a> for Mask<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.image.accept(visitor))
    }
}

impl<'a> Visitable<'a> for MaskBorder<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.source.accept(visitor))
    }
}
