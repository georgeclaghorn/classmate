use crate::visitors::common::prelude::*;

use parcel_css::properties::custom::{UnparsedProperty, CustomProperty, TokenList, TokenOrValue};

impl Visitable for UnparsedProperty<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.value.accept(visitor))
    }
}

impl Visitable for CustomProperty<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.value.accept(visitor))
    }
}

impl Visitable for TokenList<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.0.accept(visitor))
    }
}

impl Visitable for TokenOrValue<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                TokenOrValue::Url(url) => url.accept(visitor),
                TokenOrValue::Token(_) | TokenOrValue::Color(_) => Ok(())
            }
        })
    }
}
