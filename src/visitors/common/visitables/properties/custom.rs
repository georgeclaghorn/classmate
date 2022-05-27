use crate::visitors::common::prelude::*;

use parcel_css::properties::custom::{UnparsedProperty, CustomProperty, TokenList, TokenOrValue};

impl<'a> Visitable<'a> for UnparsedProperty<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.value.accept(visitor))
    }
}

impl<'a> Visitable<'a> for CustomProperty<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.value.accept(visitor))
    }
}

impl<'a> Visitable<'a> for TokenList<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.0.accept(visitor))
    }
}

impl<'a> Visitable<'a> for TokenOrValue<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                TokenOrValue::Url(url) => url.accept(visitor),
                TokenOrValue::Token(_) | TokenOrValue::Color(_) => Ok(())
            }
        })
    }
}
