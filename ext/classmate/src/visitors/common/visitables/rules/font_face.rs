use crate::visitors::common::prelude::*;

use parcel_css::rules::font_face::{
  FontFaceRule,
  FontFaceProperty,
  Source,
  UrlSource
};

impl Visitable for FontFaceRule<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.properties.accept(visitor))
    }
}

impl Visitable for FontFaceProperty<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
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

impl Visitable for Source<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                Source::Url(source) => source.accept(visitor),
                Source::Local(_)    => Ok(())
            }
        })
    }
}

impl Visitable for UrlSource<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.url.accept(visitor))
    }
}
