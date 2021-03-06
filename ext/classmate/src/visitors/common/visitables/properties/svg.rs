use crate::visitors::common::prelude::*;

use parcel_css::properties::svg::{SVGPaint, Marker};

impl Visitable for SVGPaint<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                SVGPaint::Url(url, _)   => url.accept(visitor),

                SVGPaint::None          |
                SVGPaint::Color(_)      |
                SVGPaint::ContextFill   |
                SVGPaint::ContextStroke => Ok(())
            }
        })
    }
}

impl Visitable for Marker<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                Marker::Url(url) => url.accept(visitor),
                Marker::None     => Ok(())
            }
        })
    }
}
