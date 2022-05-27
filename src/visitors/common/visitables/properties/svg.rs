use crate::visitors::common::prelude::*;

use parcel_css::properties::svg::{SVGPaint, Marker};

impl<'a> Visitable<'a> for SVGPaint<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
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

impl<'a> Visitable<'a> for Marker<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                Marker::Url(url) => url.accept(visitor),
                Marker::None     => Ok(())
            }
        })
    }
}
