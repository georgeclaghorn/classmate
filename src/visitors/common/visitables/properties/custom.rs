use crate::visitors::common::prelude::*;

use parcel_css::properties::custom::{UnparsedProperty, CustomProperty};

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

use parcel_css::properties::custom::{TokenList, TokenOrValue, Variable};

impl Visitable for TokenList<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.0.accept(visitor))
    }
}

impl Visitable for TokenOrValue<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                TokenOrValue::Url(url)      => url.accept(visitor),
                TokenOrValue::Var(variable) => variable.accept(visitor),

                TokenOrValue::Token(_)      |
                TokenOrValue::Color(_)      => Ok(())
            }
        })
    }
}

impl Visitable for Variable<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            if let Some(fallback) = &mut self.fallback {
                fallback.accept(visitor)
            } else {
                Ok(())
            }
        })
    }
}
