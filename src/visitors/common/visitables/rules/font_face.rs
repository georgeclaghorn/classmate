use crate::visitors::common::prelude::*;

use parcel_css::rules::font_face::{
  FontFaceRule,
  FontFaceProperty,
  Source,
  UrlSource
};

impl<'a> Visitable<'a> for FontFaceRule<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.properties.accept(visitor))
    }
}

impl<'a> Visitable<'a> for FontFaceProperty<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                FontFaceProperty::Source(property) => property.accept(visitor),
                FontFaceProperty::Custom(property) => property.accept(visitor),

                FontFaceProperty::FontFamily(_)    |
                FontFaceProperty::FontStyle(_)     |
                FontFaceProperty::FontWeight(_)    |
                FontFaceProperty::FontStretch(_)   |
                FontFaceProperty::UnicodeRange(_)  => Ok(())
            }
        })
    }
}

impl<'a> Visitable<'a> for Source<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                Source::Url(source) => source.accept(visitor),
                Source::Local(_)    => Ok(())
            }
        })
    }
}

impl<'a> Visitable<'a> for UrlSource<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.url.accept(visitor))
    }
}
